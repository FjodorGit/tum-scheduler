use std::{thread::sleep, time::Duration};

use thiserror::Error;

use self::{
    lecture::{LectureTemplate, LecturesBuilder},
    tum_xml_node::TumXmlError,
};
use crate::{
    db_setup::{self, connection},
    tum_api::{
        appointment::AppointmentsEndpoint,
        course::{CourseEndpoint, CourseFromXml},
        course_description::CourseDescriptionEndpoint,
        course_variant::CourseVariantEndpoint,
        curriculum::{CurriculumEndpoint, CurriculumFromXml},
        lecture::{Lecture, Lectures},
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
    #[error("Failed to parse response document: {0}")]
    DocumentParseError(String),
    #[error("Zero courses found on page {0}")]
    ZeroCoursesFound(usize),
    #[error("roxml failed to parse xml: {0}")]
    InvalidXml(#[from] roxmltree::Error),
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
    let mut course_endpoint = CourseEndpoint::for_semester(semester_id);
    let already_processed_courses = Lectures::get_all_subjects(conn, semester_id)?;
    let appointment_endpoint = AppointmentsEndpoint::new();
    let variants_endpoint = CourseVariantEndpoint::new();
    let description_endpoint = CourseDescriptionEndpoint::for_semester(semester_id);
    tracing::info!(
        "{} courses are already in the database",
        already_processed_courses.len()
    );
    tracing::info!("Requesting all other lectures");
    let mut courses_count = 0;
    while let Ok(courses) = course_endpoint.fetch_next_page().await {
        let courses_to_process = courses
            .into_iter()
            .filter(|c| !already_processed_courses.contains(&c.id));
        for course in courses_to_process {
            let variants = variants_endpoint.get_all_by_id(&course.id).await?;
            let appointments = appointment_endpoint.get_recurring_by_id(&course.id).await?;
            let representative_subject = &variants
                .first()
                .expect("variants should have at least one")
                .subject;
            let description = description_endpoint
                .get_subject_description(representative_subject)
                .await?;
            tracing::info!("Finished downloading course {}", course.id);
            Lectures::build_from(course)
                .with_appointments(&appointments)
                .with_varaints(&variants)
                .with_description(&description)
                .add_to_db(conn)?;
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

    tracing::info!(
        "Downloaded {} courses for semester {}.",
        courses_count,
        semester_id
    );
    Ok(())
}
