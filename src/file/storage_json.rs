use reqwest::Client;

use crate::error::FileServerError;

/// Save a JSON file to the S3 bucket
///
/// # Arguments
///
/// * `endpoint` - Oracle S3 endpoint
/// * `file_path` - File path
/// * `json` - JSON content in string format
pub(crate) async fn save_json(
    endpoint: &String,
    file_path: &String,
    json: String,
) -> Result<(), FileServerError> {
    if file_path.ends_with(".json") == false {
        return Err(FileServerError::FileFormatInvalid {
            path: file_path.to_string(),
        });
    }
    let url = format!("{}/o/{}", endpoint, file_path);
    let _ = Client::new()
        .put(url)
        .body(json.into_bytes())
        .send()
        .await
        .map_err(|_err| FileServerError::ObjectStorageError {
            message: _err.to_string(),
        })?;

    Ok(())
}

/// Read a JSON file from the S3 bucket
///
/// # Arguments
///
/// * `endpoint` - Oracle S3 endpoint
/// * `file_path` - File path
///
/// # Returns
///
/// * `String` - JSON content in string format
pub(crate) async fn read_json(
    endpoint: &String,
    file_path: &String,
) -> Result<String, FileServerError> {
    if file_path.ends_with(".json") == false {
        return Err(FileServerError::FileFormatInvalid {
            path: file_path.to_string(),
        });
    }
    let url = format!("{}/o/{}", endpoint, file_path);
    let object = Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|_err| FileServerError::ObjectStorageError {
            message: _err.to_string(),
        })?;

    let body = object
        .bytes()
        .await
        .map_err(|_err| FileServerError::ObjectStorageError {
            message: _err.to_string(),
        })?;

    Ok(
        String::from_utf8(body.to_vec()).map_err(|_err| FileServerError::ObjectStorageError {
            message: _err.to_string(),
        })?,
    )
}

