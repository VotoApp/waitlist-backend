use std::{collections::HashMap, error::Error};

use async_trait::async_trait;

use aws_sdk_dynamodb::{
    error::SdkError,
    operation::put_item::{PutItemError, PutItemOutput},
    types::AttributeValue,
    Client,
};
use mockall::automock;

/**
 *  This file aims to encapsulate usage of dynamodb and provide
 *  package owned types that can be swapped and mocked at runtime
 *
 */

pub type DynamoDBItem = Option<HashMap<std::string::String, AttributeValue>>;

#[automock]
#[async_trait]
pub trait DynamoDBPutItemClient {
    async fn put_item_with_condition(
        &self,
        table_name: &str,
        item: DynamoDBItem,
        condition_expression: &str,
    ) -> Result<PutItemOutput, PutItemErrorWrapper>;
}

/**
 * This type encapsulates usage of dynamodb and provides a convenient layer
 * of indirect for mocking out
 */
pub struct DynamoDBCore {
    client: Client,
}

impl DynamoDBCore {
    pub fn new(client: Client) -> Self {
        DynamoDBCore { client }
    }
}

pub enum PutItemErrorWrapper {
    InternalDynamoDBError(Box<dyn Error>),
    ConditionalCheckFailedException(Box<dyn Error>),
}

impl From<SdkError<PutItemError>> for PutItemErrorWrapper {
    fn from(value: SdkError<PutItemError>) -> Self {
        // let internal_error = Box::new(value);
        match value {
            SdkError::ServiceError(ref service_error) => {
                let put_item_error = service_error.err();
                if put_item_error.is_conditional_check_failed_exception() {
                    Self::ConditionalCheckFailedException(Box::new(value))
                } else {
                    Self::InternalDynamoDBError(Box::new(value))
                }
            }
            _ => Self::InternalDynamoDBError(Box::new(value)),
        }
    }
}

#[async_trait]
impl DynamoDBPutItemClient for DynamoDBCore {
    async fn put_item_with_condition(
        &self,
        table_name: &str,
        item: DynamoDBItem,
        condition_expression: &str,
    ) -> Result<PutItemOutput, PutItemErrorWrapper> {
        let request = self
            .client
            .put_item()
            .table_name(table_name)
            .set_item(item)
            .condition_expression(condition_expression);
        let put_item_output = request.send().await?;
        return Ok(put_item_output);
    }
}
