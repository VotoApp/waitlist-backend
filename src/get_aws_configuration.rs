use aws_config::environment::EnvironmentVariableCredentialsProvider;
use aws_credential_types::Credentials;
use aws_sdk_dynamodb::{config::Region, Config};

#[allow(dead_code)]
pub fn get_prod_aws_configuration() -> Config {
    Config::builder()
        .region(Region::new("us-east-1"))
        .credentials_provider(EnvironmentVariableCredentialsProvider::new())
        .build()
}

#[allow(dead_code)]
pub fn get_test_aws_configuration() -> Config {
    Config::builder()
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::for_tests())
        .build()
}
