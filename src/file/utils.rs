use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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

    format!("{}/{}/{}", base_dir, category, file_name)
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
