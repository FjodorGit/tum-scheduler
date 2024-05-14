use actix_files::Files;
use actix_web::{error, http::header::ContentType, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use reqwest::StatusCode;
use thiserror::Error;

use self::endpoints::{deparments, optimize};

pub mod endpoints;

#[derive(Debug, Error)]
enum ApiError {
    #[error("internal server error")]
    InternalError,

    #[error("bad request")]
    BadClientData,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn run_server() -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::JsonConfig::default()
                    // limit request payload size
                    .limit(4096)
                    // use custom error handler
                    .error_handler(|err, req| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().into())
                            .into()
                    }),
            )
            .service(optimize) //order here matters
            .service(deparments)
            .service(Files::new("/", "./frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
