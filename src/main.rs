use std::thread::sleep;
use std::time::Duration;

use crate::schedular::scheduling_problem::test_grb;
use crate::tum_api::course::CourseFromXml;
use crate::tum_api::course_variant::CourseVariantEndpoint;
use crate::tum_api::curriculum::{Curriculum, CurriculumEndpoint};
use crate::tum_api::lecture::LectureFromXml;
use crate::{db_setup::connection, tum_api::course::CourseEndpoint};
use anyhow::Result;
use db_setup::DbError;
use dotenv::dotenv;
use tum_api::DataAquisitionError;

use crate::tum_api::appointment::AppointmentEndpoint;

pub mod db_setup;
pub mod schedular;
pub mod schema;
pub mod tum_api;
use tracing::{debug, info};
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
    test_grb()?;
    // println!("{:#?}", week_in_15min_intervalls());
    // aquire_lecture_data("199").await?;
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
    let already_processed_courses = CourseFromXml::get_all_ids(conn)?;
    info!(
        "{} courses are already in the database",
        already_processed_courses.len()
    );
    info!("Requesting all other lectures");
    loop {
        let courses = course_endpoint.fetch_next_page().await?;
        debug!("courses len: {}", courses.len());
        if courses.len() == 0 {
            info!("Downloaded all courses for semester {}.", semester_id);
            break;
        }
        let mut courses_count = 0;
        let courses_to_process = courses
            .iter()
            .filter(|c| !already_processed_courses.contains(&c.id));
        for course in courses_to_process.into_iter() {
            let appointments = appointment_endpoint.get_recurring_by_id(&course.id).await?;
            let variants = course_variant_endpoint.get_all_by_id(&course.id).await;
            if let Err(_) = &variants {
                continue;
            }
            let variants = variants?;

            for appointment in appointments.iter() {
                for variant in variants.iter() {
                    let lectures = LectureFromXml::build(&course, appointment, variant);
                    LectureFromXml::insert(conn, lectures)?;
                }
            }
            CourseFromXml::insert(conn, course)?;
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
