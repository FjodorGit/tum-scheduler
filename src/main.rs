use anyhow::Result;
use std::fs;
use std::io::Write;
use std::{env, io};
use std::{fmt::write, fs::File};
use tum_api::course::Course;
use tum_api::curriculum::Curriculum;

use crate::tum_api::lecture::Lecture;
use db_setup::DbError;
use diesel::QueryDsl;
use diesel::{Insertable, RunQueryDsl};
use dotenv::dotenv;
use reqwest::{self, Request};
use roxmltree::{Attribute, Document, Node};
use tokio;

use crate::tum_api::appointment::AppointmentEndpoint;

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
    // let mut conn = db::db_setup::connection().expect("should be able to establish connection");
    // let example_course = Course {
    //     id: 923329999,
    //     subject: "MA".to_string(),
    //     semester: "W2023".to_string(),
    // };
    // let _ = diesel::insert_into(schema::course::table)
    //     .values(example_course)
    //     .get_result::<Course>(&mut conn);
    // fill_db().await?;
    Course::fetch_all().await?;
    Ok(())
}
