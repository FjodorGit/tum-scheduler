use std::{thread::sleep, time::Duration};

use thiserror::Error;

use self::tum_xml_node::TumXmlError;
use crate::{
    db_setup::{self, connection},
    tum_api::{
        appointment::AppointmentEndpoint,
        course::{CourseEndpoint, CourseFromXml},
        course_variant::CourseVariantEndpoint,
        curriculum::{CurriculumEndpoint, CurriculumFromXml},
        lecture::LectureSessionFromXml,
    },
};

pub mod appointment;
pub mod course;
pub mod course_description;
pub mod course_variant;
pub mod curriculum;
pub mod lecture;
pub mod tum_xml_node;

#[derive(Debug, Error)]
pub enum DataAquisitionError {
    #[error("Failed to parse resource node: {0}")]
    NodeParseError(#[from] TumXmlError),
    #[error("Failed to parse response document")]
    DocumentParseError(#[from] roxmltree::Error),
    #[error("Failed to request course basic data")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed interact with database `{0}`")]
    DbError(#[from] db_setup::DbError),
}

pub async fn aquire_lecture_data(
    semester_id: &str,
    with_curricula: bool,
) -> Result<(), DataAquisitionError> {
    let conn = &mut connection()?;
    let appointment_endpoint = AppointmentEndpoint::new();
    let course_variant_endpoint = CourseVariantEndpoint::new();
    let mut course_endpoint = CourseEndpoint::new(semester_id);
    if with_curricula {
        let curriculum_endpoint = CurriculumEndpoint::new();
        let curricula = curriculum_endpoint.get_all(semester_id).await?;
        tracing::info!(
            "Got {} curricula currently in the database",
            &curricula.len()
        );
        CurriculumFromXml::db_insert(conn, curricula)?;
        tracing::info!("Updated all curricula");
    }
    let already_processed_courses = CourseFromXml::get_all_ids(conn)?;
    tracing::info!(
        "{} courses are already in the database",
        already_processed_courses.len()
    );
    tracing::info!("Requesting all other lectures");
    loop {
        let courses = course_endpoint.fetch_next_page().await?;
        tracing::debug!("courses len: {}", courses.len());
        if courses.len() == 0 {
            tracing::info!("Downloaded all courses for semester {}.", semester_id);
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
                    let lectures = LectureSessionFromXml::build(&course, appointment, variant);
                    LectureSessionFromXml::insert(conn, lectures)?;
                }
            }
            CourseFromXml::insert(conn, course)?;
            tracing::info!("Finished downloading course {}", course.id);
            courses_count += 1;
            if courses_count % 100 == 0 {
                tracing::info!(
                    "Downloaded {} courses. Taking a 30 seconds break.",
                    courses_count
                );
                sleep(Duration::new(30, 0));
            }
        }
    }
    Ok(())
}
