mod json;
mod utils;

use std::env;

use actix_web::{delete, get, post, web, HttpResponse, Scope};
use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::Client;
use deadpool_postgres::Pool;
use log;
use serde_json;

use crate::{
    constants, database,
    error::FileServerError,
    guard::check_api_key,
    models::{Alarm, AlarmList, AlarmListResponse, BaseResponse, StringResponse},
};

pub fn file_handler() -> Scope {
    web::scope("/file")
        .service(upload_alarms)
        .service(download_alarms)
        .service(delete_alarms)
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

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .accelerate(true)
        .build();
    let client = Client::from_conf(config);

    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

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
    let new_path = utils::get_file_path(constants::ALARM_TABLE_NAME.to_string(), &file_name);
    let _ = json::save_json(&client, &bucket_name, &new_path, json).await;

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
        let _ = json::delete_json(&client, &bucket_name, &old_path.to_string()).await;
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

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .accelerate(true)
        .build();
    let client = Client::from_conf(config);

    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

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

    let json_str = json::read_json(&client, &bucket_name, &file_path).await?;
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

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .accelerate(true)
        .build();
    let client = Client::from_conf(config);

    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

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

    let _ = json::delete_json(&client, &bucket_name, &file_path.to_string()).await;

    Ok(HttpResponse::Ok().json(BaseResponse {
        status: 200,
        message: "Delete alarms successfully".to_string(),
    }))
}
