use std::fs;
use std::io::Write;

use crate::error::FileServerError;

/// Save a JSON file to the file system
///
/// # Arguments
///
/// * `file_path` - File path
/// * `json` - JSON content in string format
pub(crate) async fn save_json(file_path: &String, json: String) -> Result<(), FileServerError> {
    let mut file =
        fs::File::create(&file_path).map_err(|_err| FileServerError::FileCreateError {
            path: file_path.clone(),
        })?;
    file.write_all(json.as_bytes())
        .map_err(|_err| FileServerError::FileWriteError {
            path: file_path.to_string(),
        })?;
    Ok(())
}

/// Read a JSON file from the file system
///
/// # Arguments
///
/// * `file_path` - File path
///
/// # Returns
///
/// * `String` - JSON content in string format
pub(crate) async fn read_json(file_path: &String) -> Result<String, FileServerError> {
    fs::read_to_string(&file_path).map_err(|_err| FileServerError::FileReadError {
        path: file_path.to_string(),
    })
}

/// Delete a JSON file from the file system
///
/// # Arguments
///
/// * `file_path` - File path
pub(crate) async fn delete_json(file_path: &String) -> Result<(), FileServerError> {
    fs::remove_file(&file_path).map_err(|_err| FileServerError::FileDeleteError {
        path: file_path.to_string(),
    })
}
