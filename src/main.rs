use anyhow::Result;
use std::{env, io};
use tum_api::TumApiError;

use crate::tum_api::course::CourseEndpoint;
use crate::tum_api::course_variant::CourseVariantEndpoint;
use crate::tum_api::lecture::Lecture;
use db_setup::DbError;
use dotenv::dotenv;
use tokio;

use crate::tum_api::appointment::{self, AppointmentEndpoint};

pub mod db_setup;
pub mod schema;
pub mod tum_api;

const IDS_FILE_NAME: &str = "ids.txt";

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
    aquire_lecture_data().await?;
    Ok(())
}

pub async fn aquire_lecture_data() -> Result<(), TumApiError> {
    let appointment_endpoint = AppointmentEndpoint::new();
    let course_variant_endpoint = CourseVariantEndpoint::new();
    let mut course_endpoint = CourseEndpoint::new();
    loop {
        let courses = course_endpoint.fetch_next_page().await?;
        println!("Got {:?} courses", courses.len());
        if courses.len() == 0 {
            break;
        }
        for course in courses.into_iter() {
            let appointments = appointment_endpoint.get_recurring_by_id(&course.id).await?;
            println!("Got {:?} recurring appointments", appointments.len());
            let variants = course_variant_endpoint.get_all_by_id(&course.id).await;
            if let Err(err) = &variants {
                println!("Skipping {:#?} due to {:#?}.", course.id, err.to_string());
                continue;
            }
            let variants = variants?;
            println!("{:#?}", variants);
            println!("Got {:?} variants", variants.len());

            for appointment in appointments.iter() {
                for variant in variants.iter() {
                    let _lectures = Lecture::build(&course, appointment, variant);
                }
            }
        }
    }
    Ok(())
}
