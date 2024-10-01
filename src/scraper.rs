use std::{collections::HashMap, thread::sleep, time::Duration};

use diesel::result;
use lazy_static::lazy_static;
use thiserror::Error;

use self::tum_xml_node::TumXmlError;
use crate::{
    db_setup::connection,
    scraper::{
        appointment::AppointmentsEndpoint,
        course::{CourseEndpoint, ProcessingError},
        course_description::CourseDescriptionEndpoint,
        course_variant::CourseVariantEndpoint,
        curriculum::{CurriculumEndpoint, CurriculumFromXml},
        lecture::Lectures,
        organization::TumOrganizationEndpoint,
    },
};

pub mod appointment;
pub mod course;
pub mod course_description;
pub mod course_variant;
pub mod curriculum;
pub mod lecture;
pub mod organization;
pub mod tum_xml_node;

lazy_static! {
    static ref TOSEMESTERID: HashMap<&'static str, &'static str> = {
        let mut hm = HashMap::new();
        hm.insert("24W", "203");
        hm.insert("24S", "200");
        hm.insert("23W", "199");
        hm.insert("23S", "198");
        hm
    };
}

#[derive(Debug, Error)]
pub enum ScraperError {
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
    DbError(#[from] result::Error),
    #[error("Failed interact with database `{0}`")]
    DbConnectionError(#[from] result::ConnectionError),
}

pub async fn aquire_curriculum_data(semester_name: &str) -> Result<(), ScraperError> {
    let conn = &mut connection()?;
    let semester_id = TOSEMESTERID
        .get(semester_name)
        .expect("should be present in static lazy semester hashmap");

    let curriculum_endpoint = CurriculumEndpoint::new();
    let curricula = curriculum_endpoint.get_all(semester_id).await?;
    tracing::info!(
        "{} downloaded curricula for semester {}",
        curricula.len(),
        semester_name,
    );
    CurriculumFromXml::db_insert(conn, curricula)?;
    Ok(())
}

pub async fn aquire_lecture_data(semester_name: &str) -> Result<(), ScraperError> {
    let conn = &mut connection()?;
    let semester_id = TOSEMESTERID
        .get(semester_name)
        .expect("should be present in static lazy semester hashmap");
    let mut course_endpoint = CourseEndpoint::for_semester(semester_id);
    let appointment_endpoint = AppointmentsEndpoint::new();
    let variants_endpoint = CourseVariantEndpoint::new();
    let organization_endpoint = TumOrganizationEndpoint::new();
    let already_processed_courses = CourseEndpoint::get_all_processed_ids(conn)?;
    let description_endpoint = CourseDescriptionEndpoint::for_semester(semester_id);
    tracing::info!(
        "{} courses are already in the database",
        already_processed_courses.len()
    );
    tracing::info!("Requesting all other lectures");
    let mut course_count = 0;
    while let Ok(mut courses) = course_endpoint.fetch_next_page().await {
        let courses_to_process = courses
            .iter_mut()
            .filter(|c| !already_processed_courses.contains(&c.id));
        for course in courses_to_process {
            tracing::info!("Downloading course {}", course.id);
            let variants = variants_endpoint
                .get_all_by_id(&course.id)
                .await
                .unwrap_or_default();
            if variants.is_empty() {
                course.processing_error = ProcessingError::MissingVariants;
                course.add_to_db(conn)?;
                continue;
            }
            let appointments = appointment_endpoint
                .get_recurring_by_id(&course.id)
                .await
                .unwrap_or_default();
            if appointments.is_empty() {
                course.processing_error = ProcessingError::MissingAppointments;
                course.add_to_db(conn)?;
                continue;
            }
            let organization = organization_endpoint
                .get_organization(&course.id)
                .await
                .unwrap_or_default();
            if organization.is_none() {
                course.processing_error = ProcessingError::MissingOrganization;
                course.add_to_db(conn)?;
                continue;
            }

            let description = description_endpoint
                .get_subject_description(&variants.first().unwrap().subject)
                .await;
            if description.is_err() {
                course.processing_error = ProcessingError::MissingDescription;
                course.add_to_db(conn)?;
                continue;
            }

            tracing::info!("Finished downloading course {}", course.id);
            Lectures::build_from(course)
                .with_appointments(&appointments)
                .with_varaints(&variants)
                .with_description(&description.unwrap())
                .by_organization(&organization.unwrap())
                .add_to_db(conn)?;
            course.add_to_db(conn)?;
            course_count += 1;
            if course_count % 100 == 0 {
                tracing::info!(
                    "Downloaded {} courses. Taking a 10 second break.",
                    course_count
                );
                sleep(Duration::new(10, 0));
            }
        }
    }

    tracing::info!(
        "Downloaded {} courses for semester {}.",
        course_count,
        semester_name
    );
    Ok(())
}
