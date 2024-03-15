use std::collections::HashMap;

use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::schedular::settings::SolutionObjective;

#[derive(Deserialize, Debug)]
struct FrontendConfiguration {
    curriculum: String,
    #[serde(rename = "selectedPrefixes")]
    selected_prefixes: Vec<String>,
    #[serde(rename = "excludedCourses")]
    excluded_courses: Vec<String>,
    #[serde(rename = "additionalConstraints")]
    additional_constraints: HashMap<String, i32>,
    objective: SolutionObjective,
}

#[post("/api/optimize")]
pub async fn optimize(configuration: Json<FrontendConfiguration>) -> impl Responder {
    println!("Welcome {:#?}!", configuration);
    HttpResponse::Ok()
}
