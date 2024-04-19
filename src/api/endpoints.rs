use std::collections::HashMap;

use actix_web::error::{ErrorInternalServerError, ErrorServiceUnavailable};
use actix_web::get;
use actix_web::{post, web::Json, Responder, Result};
use serde::Deserialize;
use tracing::info;

use crate::db_setup::connection;
use crate::schedular::scheduling_problem::SchedulingProblem;
use crate::schedular::settings::{ConstraintSettings, FilterSettings, SolutionObjective};
use crate::scraper::organization::TumOrganization;

#[derive(Deserialize, Debug)]
struct FrontendConfiguration {
    semester: String,
    curriculum: String,
    #[serde(rename = "selectedPrefixes")]
    selected_prefixes: Vec<String>,
    #[serde(rename = "excludedCourses")]
    excluded_courses: Vec<String>,
    #[serde(rename = "additionalConstraints")]
    additional_constraints: HashMap<String, i32>,
    objective: SolutionObjective,
}

#[get("/api/departments")]
pub async fn deparments() -> Result<impl Responder> {
    let conn = &mut connection().map_err(|err| ErrorServiceUnavailable(err))?;
    let department_names =
        TumOrganization::get_all_departments(conn).map_err(|err| ErrorInternalServerError(err))?;
    Ok(Json(department_names))
}

#[post("/api/optimize")]
pub async fn optimize(configuration: Json<FrontendConfiguration>) -> Result<impl Responder> {
    let mut scheduling_problem = SchedulingProblem::new();
    let additional_contraints: ConstraintSettings =
        ConstraintSettings::from(&configuration.additional_constraints);
    let filter_settings = FilterSettings {
        semester: Some(&configuration.semester),
        excluded_courses: Some(&configuration.excluded_courses),
        faculties: Some(&configuration.selected_prefixes),
        curriculum: Some(&configuration.curriculum),
        courses: None,
    };

    let solution = scheduling_problem.solve(
        filter_settings,
        additional_contraints,
        &configuration.objective,
    );

    println!("Result: {:#?}", solution);
    Ok(Json(solution.unwrap()))
}
