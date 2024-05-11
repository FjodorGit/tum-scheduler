use clap::{Parser, Subcommand};

use crate::{
    api::run_server,
    meilisearch::init_melisearch,
    schedular::scheduling_problem::test_run,
    scraper::{aquire_curriculum_data, aquire_lecture_data},
};
use anyhow::Result;
use dotenv::dotenv;

pub mod api;
pub mod db_setup;
pub mod meilisearch;
pub mod schedular;
pub mod schema;
pub mod scraper;
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
    Debug,
}

#[tokio::main]
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

    match runmode {
        RunMode::Server => {
            tracing::info!("Initializing meilisearch server");
            init_melisearch().await?;
            tracing::info!("Starting web server");
            run_server().await?;
        }
        RunMode::Debug => {
            tracing::info!("Running Schedular Testrun");
            test_run()?;
        }
        RunMode::Scraper {
            mode: ScraperMode::Lectures,
            semester,
        } => {
            tracing::info!("Starting to scrape the courses from TUM");
            aquire_lecture_data(&semester).await?;
        }
        RunMode::Scraper {
            mode: ScraperMode::Curriculum,
            semester,
        } => {
            tracing::info!("Starting to scrape the curricula from TUM");
            aquire_curriculum_data(&semester).await?;
        }
    }

    Ok(())
}
