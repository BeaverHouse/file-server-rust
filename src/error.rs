use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Debug, Display, Error};

use crate::models::BaseResponse;

#[derive(Debug, Display, Error)]
pub enum FileServerError {
    #[display("File not found with ID: {}", id)]
    FileNotFound { id: String },

    #[display("File format is invalid: {}", path)]
    FileFormatInvalid { path: String },

    #[display("Failed to read image data: {}", message)]
    ImageParsingError { message: String },

    #[display("Serialization error")]
    SerializationError,

    #[display("Deserialization error: {}", json_str)]
    DeserializationError { json_str: String },

    #[display("PostgreSQL DB error: {}", message)]
    PostgresDBError { message: String },

    #[display("S3 error: {}", message)]
    ObjectStorageError { message: String },

    #[display("User is not registered")]
    NotRegistered,

    #[display("Unauthorized: {}", message)]
    Unauthorized { message: String },
}

impl error::ResponseError for FileServerError {
    fn error_response(&self) -> HttpResponse {
        log::error!("{:?}", self);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(BaseResponse {
                status: u16::from(self.status_code()),
                message: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            FileServerError::FileNotFound { .. } => StatusCode::BAD_REQUEST,
            FileServerError::FileFormatInvalid { .. } => StatusCode::BAD_REQUEST,
            FileServerError::ImageParsingError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            FileServerError::SerializationError => StatusCode::INTERNAL_SERVER_ERROR,
            FileServerError::DeserializationError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            FileServerError::PostgresDBError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            FileServerError::ObjectStorageError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            FileServerError::NotRegistered => StatusCode::BAD_REQUEST,
            FileServerError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}
