use crate::constants;
use deadpool_postgres::{tokio_postgres::Error, Client};

/// Check the registered name by user ID
///
/// # Arguments
///
/// * `conn` - PostgreSQL connection
/// * `user_id` - User ID
///
/// # Returns
///
/// * `String` - Registered name, or "Not Exist"
pub async fn check_registered_name(conn: &Client, user_id: String) -> Result<String, Error> {
    let query = format!(
        "SELECT name FROM family_app.users WHERE user_id = '{}'",
        user_id
    );
    log::info!("{}", query);

    let row = conn.query(&query, &[]).await?;
    if row.len() == 0 {
        return Ok(constants::NON_EXIST.to_string());
    }

    let name: String = row[0].get(0);
    Ok(name)
}

/// Save alarm info
///
/// # Arguments
///
/// * `conn` - PostgreSQL connection
/// * `user_id` - User ID
/// * `file_path` - File path
pub async fn insert_alarm_info(
    conn: &Client,
    user_id: String,
    file_path: &String,
) -> Result<(), Error> {
    let query = format!(
        "INSERT INTO family_app.alarms (user_id, file_path, delete_flag) VALUES ('{}', '{}', 'N')",
        user_id, file_path
    );
    log::info!("{}", query);

    conn.execute(&query, &vec![]).await?;
    Ok(())
}

/// Get alarm file path by user ID
///
/// # Arguments
///
/// * `conn` - PostgreSQL connection
/// * `user_id` - User ID
///
/// # Returns
///
/// * `String` - File path for saved alarm information, or "Not Exist"
pub async fn get_alarm_file_path(conn: &Client, user_id: String) -> Result<String, Error> {
    let query = format!(
        "SELECT file_path FROM family_app.alarms WHERE user_id = '{}' AND delete_flag = 'N'",
        user_id
    );
    log::info!("{}", query);

    let row = conn.query(&query, &vec![]).await?;
    if row.len() == 0 {
        return Ok(constants::NON_EXIST.to_string());
    }

    let path = row[0].get(0);
    Ok(path)
}

/// Update alarm info
///
/// # Arguments
///
/// * `conn` - PostgreSQL connection
/// * `user_id` - User ID
/// * `file_path` - New file path
pub async fn update_alarm_info(
    conn: &Client,
    user_id: String,
    file_path: &String,
) -> Result<(), Error> {
    let query = format!(
        "UPDATE family_app.alarms SET file_path = '{}' WHERE user_id = '{}' AND delete_flag = 'N'",
        file_path, user_id
    );
    log::info!("{}", query);

    conn.execute(&query, &vec![]).await?;
    Ok(())
}

/// Soft delete alarm info
///
/// # Arguments
///
/// * `conn` - PostgreSQL connection
/// * `user_id` - User ID
pub async fn delete_alarm_info(conn: &Client, user_id: String) -> Result<(), Error> {
    let query = format!(
        "UPDATE family_app.alarms SET delete_flag = 'Y' WHERE user_id = '{}'",
        user_id
    );
    log::info!("{}", query);

    conn.execute(&query, &vec![]).await?;
    Ok(())
}
