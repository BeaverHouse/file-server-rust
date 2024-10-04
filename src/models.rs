use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PostgresMapper, ToSchema)]
#[pg_mapper(table = "alarms")]
pub(crate) struct Alarm {
    #[schema(example = 1)]
    id: u32,

    #[schema(example = "Alarm 1")]
    name: String,

    #[schema(example = 30)]
    seconds: u32,
}

// API Models below

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct AlarmList {
    pub(crate) alarms: Vec<Alarm>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct AlarmListResponse {
    pub(crate) status: u16,
    pub(crate) message: String,
    pub(crate) data: AlarmList,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct StringResponse {
    pub(crate) status: u16,
    pub(crate) message: String,
    pub(crate) data: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BaseResponse {
    pub status: u16,
    pub message: String,
}

// https://github.com/juhaku/utoipa/issues/740#issuecomment-2163105436
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadImageRequest {
    pub file: String,
}
