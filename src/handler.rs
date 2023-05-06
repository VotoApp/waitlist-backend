use super::add_email_record::add_email_record;
use crate::add_email_record::AddEmailRecordError;
use crate::dynamodb::DynamoDBPutItemClient;
use crate::response_helpers::{
    get_all_good_response, get_bad_gateway_response, get_invalid_input_response,
    get_resource_conflict_response,
};
use email_address::EmailAddress;
use lambda_http::{Body, Error as LambdaError, Request, Response};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
struct RequestWaitlistResponseBody {
    email: String,
}

pub async fn function_handler(
    event: Request,
    dynamodb_client: &impl DynamoDBPutItemClient,
) -> Result<Response<Body>, LambdaError> {
    let body = event.body();
    let text_body = match body {
        Body::Text(text) => text,
        _ => return get_invalid_input_response(),
    };
    let waitlist_body = match serde_json::from_str::<RequestWaitlistResponseBody>(text_body) {
        Ok(body) => body,
        Err(e) => {
            info!("Encountered Serialization Error {}", e);
            return get_invalid_input_response();
        }
    };
    let email = waitlist_body.email;
    if !EmailAddress::is_valid(&email) {
        return get_invalid_input_response();
    }
    let email_persisted_result = add_email_record(dynamodb_client, &email).await;
    if let Err(e) = email_persisted_result {
        match e {
            AddEmailRecordError::DynamoDBInternalError => return get_bad_gateway_response(),
            AddEmailRecordError::EmailAlreadyExists => return get_resource_conflict_response(),
        }
    }

    get_all_good_response()
}
