use aws_sdk_s3::{primitives::ByteStream, Client};

use crate::error::FileServerError;

/// Save a JSON file to the S3 bucket
///
/// # Arguments
///
/// * `client` - Oracle S3 client
/// * `bucket_name` - Bucket name
/// * `file_path` - File path
/// * `json` - JSON content in string format
pub(crate) async fn save_json(
    client: &Client,
    bucket_name: &String,
    file_path: &String,
    json: String,
) -> Result<(), FileServerError> {
    client
        .put_object()
        .bucket(bucket_name)
        .key(file_path)
        .body(ByteStream::from(json.into_bytes()))
        .send()
        .await
        .map_err(|_err| FileServerError::S3Error {
            message: _err.to_string(),
        })?;

    Ok(())
}

/// Read a JSON file from the S3 bucket
///
/// # Arguments
///
/// * `client` - Oracle S3 client
/// * `bucket_name` - Bucket name
/// * `file_path` - File path
///
/// # Returns
///
/// * `String` - JSON content in string format
pub(crate) async fn read_json(
    client: &Client,
    bucket_name: &String,
    file_path: &String,
) -> Result<String, FileServerError> {
    let object = client
        .get_object()
        .bucket(bucket_name)
        .key(file_path)
        .send()
        .await
        .map_err(|_err| FileServerError::S3Error {
            message: _err.to_string(),
        })?;

    let body = object
        .body
        .collect()
        .await
        .map_err(|_err| FileServerError::S3Error {
            message: _err.to_string(),
        })?;

    Ok(
        String::from_utf8(body.to_vec()).map_err(|_err| FileServerError::S3Error {
            message: _err.to_string(),
        })?,
    )
}

/// Delete a JSON file from the S3 bucket
///
/// # Arguments
///
/// * `client` - Oracle S3 client
/// * `bucket_name` - Bucket name
/// * `file_path` - File path
pub(crate) async fn delete_json(
    client: &Client,
    bucket_name: &String,
    file_path: &String,
) -> Result<(), FileServerError> {
    let _ = client
        .delete_object()
        .bucket(bucket_name)
        .key(file_path)
        .send()
        .await
        .map_err(|_err| FileServerError::S3Error {
            message: _err.to_string(),
        });

    Ok(())
}
