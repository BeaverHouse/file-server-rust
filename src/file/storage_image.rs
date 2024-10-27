use reqwest::Client;
use uuid::Uuid;

use crate::error::FileServerError;

use super::utils::get_epoch_ms;

/// Save an image file to the S3 bucket
///
/// # Arguments
///
/// * `endpoint` - Oracle S3 endpoint
/// * `data` - Binary data of the image
///
/// # Returns
///
/// * `String` - URL of the uploaded image
pub(crate) async fn save_aecheck_image(
    endpoint: &String,
    img_bytes: &Vec<u8>,
) -> Result<String, FileServerError> {
    let file_path = format!(
        "aecheck/user-image/img_{}_{}.jpg",
        Uuid::new_v4(),
        get_epoch_ms()
    );
    let url = format!("{}/o/{}", endpoint, file_path);

    let _ = Client::new()
        .put(url)
        .body(img_bytes.clone())
        .send()
        .await
        .map_err(|_err| FileServerError::ObjectStorageError {
            message: _err.to_string(),
        })?;

    Ok(format!("https://objectstorage.ap-chuncheon-1.oraclecloud.com/p/kZNz1CCojL7rRFkNrpihiudQViOGdkUHFrldnh8pGUzpJtA_IRbgSJ0hSovJQ2p2/n/axpzmkynkrby/b/austin-oracle-bucket/o/{}", file_path))
}
