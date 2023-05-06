use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
use http::Request;
use lambda_http::Body;
use mockall::predicate::always;
use voto_waitlist::dynamodb::{
    DynamoDBPutItemClient, MockDynamoDBPutItemClient, PutItemErrorWrapper,
};
use voto_waitlist::handler::function_handler;

#[tokio::test]
async fn test_handler_gives_bad_request_with_invalid_body() {
    // Arrange
    let input_fixture = "asdkjfaljsdhfad";
    let req = get_request_from_body(input_fixture);
    let dynamodb_client = get_test_dynamo_db_client();
    let result = function_handler(req, &dynamodb_client).await.unwrap();

    // Act
    let output_snapshot =
        "Response { status: 400, version: HTTP/1.1, headers: {}, body: Text(\"\") }";

    let result_string = format!("{:?}", result);
    // Assert
    assert_eq!(result_string, output_snapshot);
}

#[tokio::test]
async fn test_handler_gives_bad_request_with_invalid_email() {
    // Arrange
    let input_fixture = r#"{"email":"testexample.com"}"#;
    let req = get_request_from_body(input_fixture);
    let dynamodb_client = get_test_dynamo_db_client();
    let result = function_handler(req, &dynamodb_client).await.unwrap();

    // Act
    let output_snapshot =
        "Response { status: 400, version: HTTP/1.1, headers: {}, body: Text(\"\") }";

    let result_string = format!("{:?}", result);
    // Assert
    assert_eq!(result_string, output_snapshot);
}

#[tokio::test]
async fn test_handler_gives_version_conflict_with_already_existing_email() {
    // Arrange
    let input_fixture = r#"{"email":"test@example.com"}"#;
    let req = get_request_from_body(input_fixture);
    let dynamodb_client = get_test_dynamo_db_client_condition_fails();
    let result = function_handler(req, &dynamodb_client).await.unwrap();

    // Act
    let output_snapshot =
        "Response { status: 409, version: HTTP/1.1, headers: {}, body: Text(\"\") }";

    let result_string = format!("{:?}", result);
    // Assert
    assert_eq!(result_string, output_snapshot);
}

#[tokio::test]
async fn test_handler_gives_bad_gateway_dynamodb_down() {
    // Arrange
    let input_fixture = r#"{"email":"test@example.com"}"#;
    let req = get_request_from_body(input_fixture);
    let dynamodb_client = get_test_dynamo_db_client_internal_error();
    let result = function_handler(req, &dynamodb_client).await.unwrap();

    // Act
    let output_snapshot =
        "Response { status: 502, version: HTTP/1.1, headers: {}, body: Text(\"\") }";

    let result_string = format!("{:?}", result);

    // Assert
    assert_eq!(result_string, output_snapshot);
}

#[tokio::test]
async fn test_handler_gives_success() {
    // Arrange
    let input_fixture = r#"{"email":"test@example.com"}"#;
    let req = get_request_from_body(input_fixture);
    let dynamodb_client = get_test_dynamo_db_client();
    let result = function_handler(req, &dynamodb_client).await.unwrap();

    // Act
    let output_snapshot =
        "Response { status: 200, version: HTTP/1.1, headers: {}, body: Text(\"\") }";

    let result_string = format!("{:?}", result);
    // Assert
    assert_eq!(result_string, output_snapshot);
}

fn get_request_from_body(body_string: &str) -> Request<Body> {
    Request::builder()
        .body(Body::Text(String::from(body_string)))
        .unwrap()
}

fn get_test_dynamo_db_client() -> impl DynamoDBPutItemClient {
    let mut mock_client = MockDynamoDBPutItemClient::default();
    mock_client
        .expect_put_item_with_condition()
        .with(always(), always(), always())
        .returning(|_, _, _| Ok(PutItemOutput::builder().build()));
    mock_client
}

fn get_test_dynamo_db_client_condition_fails() -> impl DynamoDBPutItemClient {
    // MockDyn
    let mut mock_client = MockDynamoDBPutItemClient::default();
    mock_client
        .expect_put_item_with_condition()
        .with(always(), always(), always())
        .returning(|_, _, _| {
            Err(PutItemErrorWrapper::ConditionalCheckFailedException(
                "".into(),
            ))
        });
    mock_client
}

fn get_test_dynamo_db_client_internal_error() -> impl DynamoDBPutItemClient {
    // MockDyn
    let mut mock_client = MockDynamoDBPutItemClient::default();
    mock_client
        .expect_put_item_with_condition()
        .with(always(), always(), always())
        .returning(|_, _, _| Err(PutItemErrorWrapper::InternalDynamoDBError("".into())));
    mock_client
}
