use std::thread::sleep;
use std::time::Duration;

use crate::tum_api::course_variant::CourseVariantEndpoint;
use crate::tum_api::curriculum::{Curriculum, CurriculumEndpoint};
use crate::tum_api::lecture::Lecture;
use crate::{db_setup::connection, tum_api::course::CourseEndpoint};
use anyhow::Result;
use db_setup::DbError;
use dotenv::dotenv;
use tum_api::DataAquisitionError;

use crate::tum_api::appointment::AppointmentEndpoint;

pub mod db_setup;
pub mod schema;
pub mod tum_api;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(thiserror::Error, Debug)]
pub enum FillDbError {
    #[error("Encountered {0}")]
    DbError(#[from] DbError),
    #[error("Request Error while filling db")]
    RequestError(#[from] reqwest::Error),
}

// use paging mechnism to get course ids then use allCurriculum to get type of course
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

    info!("Starting web server!");
    aquire_lecture_data("199").await?;
    Ok(())
}

pub async fn aquire_lecture_data(semester_id: &str) -> Result<(), DataAquisitionError> {
    let conn = &mut connection()?;
    let appointment_endpoint = AppointmentEndpoint::new();
    let course_variant_endpoint = CourseVariantEndpoint::new();
    let mut course_endpoint = CourseEndpoint::new(semester_id);
    let curricula = Curriculum::get_all(conn, semester_id)?;
    info!(
        "Got {} curricula currently in the database",
        curricula.len()
    );
    if curricula.len() == 0 {
        let curriculum_endpoint = CurriculumEndpoint::new();
        let curricula = curriculum_endpoint.get_all(semester_id).await?;
        Curriculum::insert(conn, curricula)?;
        info!("Updated all curricula")
    }
    let lectures_in_db = db_setup::get_all_lecture_ids(conn)?;
    info!(
        "{} lectures are already in the database",
        lectures_in_db.len()
    );
    info!("Requesting all other lectures");
    loop {
        let courses = course_endpoint.fetch_next_page().await?;
        if courses.len() == 0 {
            info!("Downloaded all courses for semester {}.", semester_id);
            break;
        }
        let mut courses_count = 0;
        let courses = courses.iter().filter(|c| !lectures_in_db.contains(&c.id));
        for course in courses.into_iter() {
            let appointments = appointment_endpoint.get_recurring_by_id(&course.id).await?;
            let variants = course_variant_endpoint.get_all_by_id(&course.id).await;
            if let Err(_) = &variants {
                continue;
            }
            let variants = variants?;

            for appointment in appointments.iter() {
                for variant in variants.iter() {
                    let lectures = Lecture::build(&course, appointment, variant);
                    Lecture::insert(conn, lectures)?;
                }
            }
            info!("Finished downloading course {}", course.id);
            courses_count += 1;
            if courses_count % 100 == 0 {
                info!(
                    "Downloaded {} courses. Taking a 30 seconds break.",
                    courses_count
                );
                sleep(Duration::new(30, 0));
            }
        }
    }
    Ok(())
}
