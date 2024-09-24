use std::time::{SystemTime, UNIX_EPOCH};

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
