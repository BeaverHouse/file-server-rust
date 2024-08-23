use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::FileServerError;

/// Get the local file path with the base directory.
/// If the folder for the category does not exist, it will be created.
///
/// # Arguments
///
/// * `category` - Category
/// * `file_name` - File name
///
/// # Returns
///
/// * `String` - Local file path
pub fn get_file_path(category: String, file_name: &String) -> String {
    let base_dir = env::var("BASE_DIR").expect("BASE_DIR must be set");
    create_folder(&base_dir, &category);

    format!("{}/{}/{}", base_dir, category, file_name)
}

/// Create a folder in the local file system
///
/// # Arguments
///
/// * `base_dir` - Base directory
/// * `category` - Category
fn create_folder(base_dir: &String, category: &String) {
    let path = format!("{}/{}", base_dir, category);
    fs::create_dir_all(&path)
        .map_err(|_err| FileServerError::FolderCreateError { path })
        .unwrap();
}

/// Get current time in milliseconds
///
/// # Returns
///
/// * `u128` - Current time in milliseconds
pub fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
