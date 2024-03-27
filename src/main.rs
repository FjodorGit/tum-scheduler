use clap::{Parser, Subcommand};

use crate::{
    api::run_server,
    tum_api::{aquire_curriculum_data, aquire_lecture_data},
};
use anyhow::Result;
use dotenv::dotenv;

pub mod api;
pub mod db_setup;
pub mod schedular;
pub mod schema;
pub mod tum_api;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Subcommand)]
enum ScraperMode {
    Lectures,
    Curriculum,
}

#[derive(Parser)]
#[command(name = "tum-schedular")]
#[command(version, about)]
enum RunMode {
    /// Starts the server for the schedular website
    Server,
    /// Starts scraping the TUM-API for course data
    Scraper {
        #[command(subcommand)]
        mode: ScraperMode,
        #[arg(long)]
        semester: String,
    },
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    dotenv::from_filename("request_urls").ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tum_scheduler=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let runmode = RunMode::parse();

    if let RunMode::Scraper { mode, semester } = runmode {
        match mode {
            ScraperMode::Lectures => {
                tracing::info!("Starting to scrape the courses from TUM");
                aquire_lecture_data(&semester).await?;
            }
            ScraperMode::Curriculum => {
                tracing::info!("Starting to scrape the curricula from TUM");
                aquire_curriculum_data(&semester).await?;
            }
        }
    } else {
        tracing::info!("Starting web server");
        run_server().await?;
    }
    Ok(())
}
