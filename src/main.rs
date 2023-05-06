mod add_email_record;
mod dynamodb;
mod get_aws_configuration;
mod handler;
mod response_helpers;
use aws_sdk_dynamodb::Client;
use dynamodb::DynamoDBCore;
use get_aws_configuration::get_prod_aws_configuration;
use handler::function_handler;
use http::Response;
use lambda_http::{run, service_fn, Body, Error as LambdaError, Request};

async fn bound_function_handler(request: Request) -> Result<Response<Body>, LambdaError> {
    let aws_conf = get_prod_aws_configuration();
    let dynamodb_client = Client::from_conf(aws_conf);
    let dynamodb_core = DynamoDBCore::new(dynamodb_client);
    // let confi
    function_handler(request, &dynamodb_core).await
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(bound_function_handler)).await
}
