use actix_web::error::{ErrorInternalServerError, ErrorServiceUnavailable};
use actix_web::get;
use actix_web::{post, web::Json, Responder, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db_setup::connection;
use crate::schedular::course_selection::CourseSelection;
use crate::schedular::scheduling_problem::SchedulingProblem;
use crate::schedular::settings::{ConstraintSettings, FilterSettings, SolutionObjective};
use crate::scraper::organization::TumOrganization;

use super::ApiError;

#[derive(Deserialize, Debug)]
struct OptimizeRequest {
    courses: Vec<String>,
    curriculum: String,
    semester: String,
    constraints: ConstraintSettings,
    objective: SolutionObjective,
    num_of_schedules: Option<usize>,
}

#[derive(Serialize, Debug)]
struct OptimizeResponse {
    schedules: Vec<Vec<CourseSelection>>,
}

#[get("/api/departments")]
pub async fn deparments() -> Result<impl Responder> {
    let conn = &mut connection().map_err(|err| ErrorServiceUnavailable(err))?;
    let department_names =
        TumOrganization::get_all_departments(conn).map_err(|err| ErrorInternalServerError(err))?;
    Ok(Json(department_names))
}

#[post("/api/optimize")]
pub async fn optimize(optimize_request: Json<OptimizeRequest>) -> Result<impl Responder, ApiError> {
    tracing::info!("Handling optimization request");
    let mut scheduling_problem = SchedulingProblem::new();
    let filter_settings = FilterSettings {
        courses: Some(&optimize_request.courses),
        semester: Some(&optimize_request.semester),
        excluded_courses: None,
        faculties: None,
        curriculum: Some(&optimize_request.curriculum),
    };

    let all_solutions = scheduling_problem.solve(
        filter_settings,
        &optimize_request.constraints,
        &optimize_request.objective,
    );

    let num_solutions = match optimize_request.num_of_schedules {
        Some(amount) => amount,
        None => 1,
    };

    match all_solutions {
        Ok(solutions) => Ok(Json(
            solutions.into_iter().take(num_solutions).collect_vec(),
        )),
        Err(_) => return Err(ApiError::InternalError),
    }
}
