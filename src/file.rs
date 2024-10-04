mod storage_image;
mod storage_json;
mod utils;

use std::env;

use actix_web::{delete, get, post, web, HttpResponse, Scope};
use data_url::DataUrl;
use deadpool_postgres::Pool;
use log;
use serde_json;

use crate::{
    constants, database,
    error::FileServerError,
    guard::check_api_key,
    models::{
        Alarm, AlarmList, AlarmListResponse, BaseResponse, StringResponse, UploadImageRequest,
    },
};

pub fn file_handler() -> Scope {
    web::scope("/file")
        .service(upload_alarms)
        .service(download_alarms)
        .service(delete_alarms)
        .service(upload_aecheck_image)
}

#[utoipa::path(
    post,
    tag = "File",
    path = "/file/upload/alarms/{id}",
    request_body = AlarmList,
    responses(
        (status = 200, description = "Upload alarms successfully", body = StringResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/upload/alarms/{id}")]
async fn upload_alarms(
    _req: actix_web::HttpRequest,
    id: web::Path<String>,
    body: web::Json<AlarmList>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, FileServerError> {
    check_api_key(&_req)?;

    let endpoint =
        env::var("ORACLE_FAMILY_RW_ENDPOINT").expect("ORACLE_FAMILY_RW_ENDPOINT must be set");

    let connection = pool
        .get()
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    let name = database::alarms::check_registered_name(&connection, id.to_string())
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;
    if name == constants::NON_EXIST {
        return Err(FileServerError::NotRegistered);
    }

    log::info!("Upload alarms - ID: {}", id);

    let json =
        serde_json::to_string(&body.alarms).map_err(|_| FileServerError::SerializationError)?;
    let file_name = format!("alarms_{}_{}.json", id, utils::get_epoch_ms());
    let new_path = format!(
        "family/{}/{}",
        constants::ALARM_TABLE_NAME.to_string(),
        &file_name
    );
    let _ = storage_json::save_json(&endpoint, &new_path, json).await;

    let old_path = database::alarms::get_alarm_file_path(&connection, id.to_string())
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    if old_path == constants::NON_EXIST.to_string() {
        let _ = database::alarms::insert_alarm_info(&connection, id.to_string(), &new_path)
            .await
            .map_err(|err| FileServerError::PostgresDBError {
                message: err.to_string(),
            })?;
    } else {
        log::info!("Replace old file: {}", old_path);
        let _ = database::alarms::update_alarm_info(&connection, id.to_string(), &new_path)
            .await
            .map_err(|err| FileServerError::PostgresDBError {
                message: err.to_string(),
            })?;
    }

    Ok(HttpResponse::Ok().json(StringResponse {
        status: 200,
        message: "Upload alarms successfully".to_string(),
        data: name,
    }))
}

#[utoipa::path(
    get,
    tag = "File",
    path = "/file/download/alarms/{id}",
    responses(
        (status = 200, description = "Download alarms successfully", body = AlarmListResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[get("/download/alarms/{id}")]
async fn download_alarms(
    _req: actix_web::HttpRequest,
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, FileServerError> {
    check_api_key(&_req)?;

    let endpoint =
        env::var("ORACLE_FAMILY_RW_ENDPOINT").expect("ORACLE_FAMILY_RW_ENDPOINT must be set");

    let connection = pool
        .get()
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    let file_path = database::alarms::get_alarm_file_path(&connection, id.to_string())
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    if file_path == "Not Exist" {
        return Err(FileServerError::FileNotFound { id: id.to_string() });
    }

    log::info!("Download alarms - ID: {}", id);

    let json_str = storage_json::read_json(&endpoint, &file_path).await?;
    let alarms: Vec<Alarm> = serde_json::from_str(&json_str)
        .map_err(|_err| FileServerError::DeserializationError { json_str })?;

    Ok(HttpResponse::Ok().json(AlarmListResponse {
        status: 200,
        message: "Download alarms successfully".to_string(),
        data: AlarmList { alarms },
    }))
}

#[utoipa::path(
    delete,
    tag = "File",
    path = "/file/delete/alarms/{id}",
    responses(
        (status = 200, description = "Delete alarms successfully", body = BaseResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[delete("/delete/alarms/{id}")]
async fn delete_alarms(
    _req: actix_web::HttpRequest,
    id: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, FileServerError> {
    check_api_key(&_req)?;

    let connection = pool
        .get()
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    let file_path = database::alarms::get_alarm_file_path(&connection, id.to_string())
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    if file_path == "Not Exist" {
        return Err(FileServerError::FileNotFound { id: id.to_string() });
    }

    log::info!("Delete alarms - ID: {}", id);

    let _ = database::alarms::delete_alarm_info(&connection, id.to_string())
        .await
        .map_err(|err| FileServerError::PostgresDBError {
            message: err.to_string(),
        })?;

    Ok(HttpResponse::Ok().json(BaseResponse {
        status: 200,
        message: "Delete alarms successfully".to_string(),
    }))
}

#[utoipa::path(
    post,
    tag = "File",
    path = "/file/aecheck",
    responses(
        (status = 200, description = "Upload image successfully", body = StringResponse),
    ),
    security(
        ("bearerAuth" = [])
    )
)]
#[post("/aecheck")]
async fn upload_aecheck_image(
    _req: actix_web::HttpRequest,
    body: web::Json<UploadImageRequest>,
) -> Result<HttpResponse, FileServerError> {
    check_api_key(&_req)?;

    let endpoint =
        env::var("ORACLE_AECHECK_W_ENDPOINT").expect("ORACLE_AECHECK_W_ENDPOINT must be set");

    let data_url =
        DataUrl::process(&body.file).map_err(|_err| FileServerError::ImageParsingError {
            message: _err.to_string(),
        })?;
    let (img_bytes, _) =
        data_url
            .decode_to_vec()
            .map_err(|_err| FileServerError::ImageParsingError {
                message: _err.to_string(),
            })?;

    let url = storage_image::save_aecheck_image(&endpoint, &img_bytes).await?;

    log::info!("Upload image - URL: {}", url);

    Ok(HttpResponse::Ok().json(StringResponse {
        status: 200,
        message: "Upload image successfully".to_string(),
        data: url,
    }))
}
