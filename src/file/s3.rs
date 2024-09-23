use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{self, config::Credentials};
use std::env;

pub async fn get_s3_client() -> aws_sdk_s3::Client {
    let region = env::var("S3_REGION").expect("S3_REGION must be set");
    let endpoint = env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set");
    let access_key = env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set");
    let secret_key = env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set");

    let credentials = Credentials::new(access_key, secret_key, None, None, "s3");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(Region::new(region))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .endpoint_url(endpoint)
        .build();

    aws_sdk_s3::Client::from_conf(s3_config)
}
