mod constants;
mod database;
mod error;
mod file;
mod guard;
mod models;

use std::net::Ipv4Addr;

use actix_cors::Cors;
use database::config::PostgresConfig;
use models::{
    Alarm, AlarmList, AlarmListResponse, BaseResponse, StringResponse, UploadImageRequest,
};

use actix_web::{
    get, http,
    middleware::Logger,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use confik::{Configuration, EnvSource};
use dotenvy::dotenv;
use env_logger::Env;
use tokio_postgres::NoTls;
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "RS File Server", description = "File Server made by Rust"),
    paths(
        file::upload_alarms,
        file::download_alarms,
        file::delete_alarms,
        file::upload_aecheck_image,
        healthcheck
    ),
    components(schemas(Alarm, AlarmList, AlarmListResponse, BaseResponse, StringResponse, UploadImageRequest)),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if openapi.components.is_none() {
            openapi.components = Some(Components::new());
        }

        openapi.components.as_mut().unwrap().add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[utoipa::path(
    get,
    tag = "Health Check",
    path = "/file",
    responses(
        (status = 200, description = "Health check", body = String)
    ),
)]
#[get("")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("This is RS File Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = PostgresConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://dev.aecheck.com")
            .allowed_origin("https://aecheck.com")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::PayloadConfig::new(constants::MAX_PAYLOAD_SIZE))
            .app_data(web::JsonConfig::default().limit(constants::MAX_PAYLOAD_SIZE))
            .wrap(Logger::new("%a %t %r - %s"))
            .wrap(cors)
            .service(
                SwaggerUi::new("/file/docs/{_:.*}")
                    .url("/file/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .service(file::file_handler().service(healthcheck))
    })
    .workers(3)
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
