use actix_files::Files;
use actix_web::{App, HttpServer};
use anyhow::Result;

use self::endpoints::{deparments, optimize};

pub mod endpoints;

pub async fn run_server() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(optimize) //order here matters
            .service(deparments)
            .service(Files::new("/", "./frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
