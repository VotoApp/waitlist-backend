use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_dynamo::to_item;

use crate::dynamodb::{DynamoDBPutItemClient, PutItemErrorWrapper};

const WAITLIST_EMAIL_RECORD_TABLE_NAME: &str = "waitlist-email-records";
const WAITLIST_TABLE_EMAIL_ALREADY_EXISTS_CONDITION: &str = "attribute_not_exists(PK)";

#[derive(Serialize, Deserialize)]
pub struct WaitlistItemRecord {
    #[serde(rename = "PK")]
    email: String,
    #[serde(rename = "DateReceived")]
    date_received: u64,
}

#[derive(Debug, PartialEq)]
pub enum AddEmailRecordError {
    DynamoDBInternalError,
    EmailAlreadyExists,
}

pub async fn add_email_record(
    client: &impl DynamoDBPutItemClient,
    email: &str,
) -> Result<(), AddEmailRecordError> {
    let epoch_seconds_time_string = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let waitlist_item = WaitlistItemRecord {
        email: String::from(email),
        date_received: epoch_seconds_time_string,
    };
    let waitlist_item_record = to_item(waitlist_item)
        .expect("Waitlist item record should always have all necessary feels to be deserialized");

    let dynamodb_result = client
        .put_item_with_condition(
            WAITLIST_EMAIL_RECORD_TABLE_NAME,
            Some(waitlist_item_record),
            WAITLIST_TABLE_EMAIL_ALREADY_EXISTS_CONDITION,
        )
        .await;
    let sdk_error = match dynamodb_result {
        Ok(_) => return Ok(()),
        Err(e) => e,
    };
    let email_record_error = match sdk_error {
        PutItemErrorWrapper::ConditionalCheckFailedException(_) => {
            AddEmailRecordError::EmailAlreadyExists
        }
        PutItemErrorWrapper::InternalDynamoDBError(_) => AddEmailRecordError::DynamoDBInternalError,
    };
    Err(email_record_error)
}

#[cfg(test)]
mod tests {
    use aws_sdk_dynamodb::{operation::put_item::PutItemOutput, types::AttributeValue};
    use mockall::predicate::{always, eq, function};

    use crate::dynamodb::{DynamoDBItem, MockDynamoDBPutItemClient};

    use super::*;

    const TEST_EMAIL: &'static str = "test@example.com";

    #[tokio::test]
    async fn test_add_email_record_calls_dynamodb_correctly() {
        // Arrange
        let mut mock_client = MockDynamoDBPutItemClient::new();
        let item_equals_test = move |item: &DynamoDBItem| {
            let actual = item.clone().unwrap();
            assert_eq!(actual["PK"], AttributeValue::S(String::from(TEST_EMAIL)));
            let _: u64 = actual["DateReceived"].as_n().unwrap().parse().unwrap();
            true
        };
        mock_client
            .expect_put_item_with_condition()
            .with(
                eq(WAITLIST_EMAIL_RECORD_TABLE_NAME),
                function(item_equals_test),
                eq(WAITLIST_TABLE_EMAIL_ALREADY_EXISTS_CONDITION),
            )
            .times(1)
            .returning(|_, _, _| Err(PutItemErrorWrapper::InternalDynamoDBError("".into())));

        // Act
        let _ = add_email_record(&mock_client, TEST_EMAIL).await;
    }

    #[tokio::test]
    async fn test_add_email_record_calls_will_error_with_dynamodb_error() {
        // Arrange
        let mut mock_client = MockDynamoDBPutItemClient::new();
        mock_client
            .expect_put_item_with_condition()
            .with(always(), always(), always())
            .times(1)
            .returning(|_, _, _| Err(PutItemErrorWrapper::InternalDynamoDBError("".into())));
        let expected = AddEmailRecordError::DynamoDBInternalError;

        // Act
        let actual = add_email_record(&mock_client, TEST_EMAIL)
            .await
            .unwrap_err();

        // Assert
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_add_email_record_calls_will_error_with_condition_error() {
        // Arrange
        let mut mock_client = MockDynamoDBPutItemClient::new();
        mock_client
            .expect_put_item_with_condition()
            .with(always(), always(), always())
            .times(1)
            .returning(|_, _, _| {
                Err(PutItemErrorWrapper::ConditionalCheckFailedException(
                    "".into(),
                ))
            });
        let expected = AddEmailRecordError::EmailAlreadyExists;

        // Act
        let actual = add_email_record(&mock_client, TEST_EMAIL)
            .await
            .unwrap_err();

        // Assert
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_add_email_record_calls_can_succeed() {
        // Arrange
        let mut mock_client = MockDynamoDBPutItemClient::new();
        mock_client
            .expect_put_item_with_condition()
            .with(always(), always(), always())
            .times(1)
            .returning(|_, _, _| Ok(PutItemOutput::builder().build()));

        // Act
        let result = add_email_record(&mock_client, TEST_EMAIL).await;

        // Assert
        result.expect("The result should be successfull");
    }
}
