use grb::prelude::*;
use std::array;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;

use crate::tum_api::{
    appointment::SingleAppointment,
    lecture::NewLecture,
    subject_appointments::{CourseSelection, FilterSettings},
};
use chrono::{Duration, NaiveTime};
use grb::{add_binvar, c, expr::LinExpr, param, Env, Expr, Model, Var, VarType::Binary};
use thiserror::Error;

pub const NUM_INTERVALS_PER_WEEK: usize = 355;
pub const NUM_INTERVALS_PER_DAY: usize = 71; // from 6:00 to 23:45
pub const NUM_DAYS_PER_WEEK: usize = 5;
pub const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

#[derive(Error, Debug)]
pub enum SchedularError {
    #[error("Failed to add vairiable {0}")]
    VariableAddingError(#[from] grb::Error),
}

pub struct SchedulingProblem {
    model: Model,
    vars: Vec<Var>,
    weekday_exprs: HashMap<String, LinExpr>,
    on_weekday_vars: HashMap<String, Vec<Var>>,
    interval_exprs: HashMap<String, LinExpr>,
    amount_ects: LinExpr,
}

pub struct ConstraintSettings {
    min_num_ects: Option<i32>,
    max_num_days: Option<i32>,
}

pub enum SolutionObjective {
    NoObjective,
    MinimizeNumCourses,
    MaximizeNumEcts,
}

impl SchedulingProblem {
    pub fn new() -> Self {
        let model = Model::new("schedular").expect("should be able to create grb model");
        Self {
            model,
            vars: vec![],
            weekday_exprs: HashMap::new(),
            on_weekday_vars: HashMap::new(),
            interval_exprs: HashMap::new(),
            amount_ects: LinExpr::new(),
        }
    }

    pub fn add_courses(
        &mut self,
        subject_aps: &Vec<CourseSelection>,
    ) -> Result<(), SchedularError> {
        let mut vars_count = 0;
        for subject_ap in subject_aps.into_iter() {
            self.add_course(subject_ap, vars_count)?;
            vars_count += 1;
        }
        Ok(())
    }

    pub fn add_course(
        &mut self,
        subject_schedule: &CourseSelection,
        schedule_num: i32,
    ) -> Result<(), SchedularError> {
        let appointment_var_name = format!("{}_v{}", subject_schedule.abbr, schedule_num);
        println!("Name {}", appointment_var_name);
        let appointment_var = self
            .model
            .add_var(&appointment_var_name, Binary, 0., 0., 1., [])?;

        self.vars.push(appointment_var);
        for appointment in subject_schedule.appointments.iter() {
            self.add_session(appointment_var, &appointment);
        }

        for weekday in subject_schedule.weekdays() {
            println!("Takes place on {}", weekday);
            self.add_weekday(appointment_var, weekday)
        }

        self.amount_ects
            .add_term(subject_schedule.ects, appointment_var);
        Ok(())
    }

    fn add_session(&mut self, appointment_var: Var, appointment: &SingleAppointment) {
        let mut time_point = appointment.from;
        while time_point < appointment.to {
            let constraint_name = format!("{}_{}", appointment.weekday, time_point);
            if let Some(expr) = self.interval_exprs.get_mut(&constraint_name) {
                expr.add_term(1.0, appointment_var);
            } else {
                let mut expr = LinExpr::new();
                expr.add_term(1.0, appointment_var);
                self.interval_exprs.insert(constraint_name, expr);
            }
            time_point += Duration::minutes(15);
        }
    }

    fn add_weekday(&mut self, session_var: Var, weekday: String) {
        if let Some(expr) = self.weekday_exprs.get_mut(&weekday) {
            expr.add_term(1.0, session_var);
            self.on_weekday_vars
                .get_mut(&weekday)
                .unwrap()
                .push(session_var);
        } else {
            let mut expr = LinExpr::new();
            expr.add_term(1.0, session_var);
            self.weekday_exprs.insert(weekday.clone(), expr);
            self.on_weekday_vars.insert(weekday, vec![session_var]);
        }
    }

    pub fn add_additional_constraints(
        &mut self,
        constraints: ConstraintSettings,
    ) -> Result<(), SchedularError> {
        if let Some(min_ects) = constraints.min_num_ects {
            self.model
                .add_constr("min_ects", c!(self.amount_ects.clone() >= min_ects))?;
        }
        if let Some(max_days) = constraints.max_num_days {
            let mut weekday_sum_expr = LinExpr::new();
            for weekday in WEEKDAYS {
                let weekday_var_name = format!("{}_v", weekday);
                let weekday_var = self
                    .model
                    .add_var(&weekday_var_name, Binary, 0., 0., 1., [])?;
                if let Some(weekday_expr) = self.weekday_exprs.get(weekday) {
                    self.model.add_constr(
                        &format!("{}_is_off", weekday),
                        c!(weekday_expr.clone() >= weekday_var),
                    )?;
                    for (num, on_this_day_var) in self
                        .on_weekday_vars
                        .get(weekday)
                        .expect("should contain elements because of previous call")
                        .iter()
                        .enumerate()
                    {
                        self.model.add_constr(
                            &format!("{}_is_on_{}", weekday, num),
                            c!(on_this_day_var <= weekday_var),
                        )?;
                    }
                    weekday_sum_expr.add_term(1., weekday_var);
                }
            }
            self.model
                .add_constr("weekday_sum_constr", c!(weekday_sum_expr <= max_days))?;
        }
        Ok(())
    }

    pub fn solve(&mut self, objective: SolutionObjective) -> Result<Vec<f64>, SchedularError> {
        let interval_constraints = self
            .interval_exprs
            .iter()
            .map(|(name, expr)| (name, c!(expr.clone() <= 1)));
        match objective {
            SolutionObjective::MinimizeNumCourses => self
                .model
                .set_objective(self.vars.iter().grb_sum(), Minimize),
            SolutionObjective::MaximizeNumEcts => {
                self.model.set_objective(self.amount_ects.clone(), Maximize)
            }
            SolutionObjective::NoObjective => self.model.set_objective(0, Minimize),
        }?;
        for (constr, expr) in interval_constraints {
            self.model.add_constr(constr, expr)?;
        }
        self.model.update()?;
        println!("Writing model to file");
        self.model.write("schedular.lp")?;
        self.model.optimize()?;
        let vals = self.model.get_obj_attr_batch(attr::X, self.vars.clone())?;
        Ok(vals)
    }
}

pub fn test_grb() -> Result<(), SchedularError> {
    dotenv::dotenv().ok();
    use crate::db_setup::connection;
    let conn = &mut connection().expect("should be able to establish connection to db");
    let mut scheduling_problem = SchedulingProblem::new();

    let filters = FilterSettings {
        subjects: None,
        exclude_subject: Some(vec![
            "MA5617".to_string(),
            "MA0003".to_string(),
            "MA3601".to_string(),
            "MA5120".to_string(),
            "MA2409".to_string(),
            "MA3405".to_string(),
            "MA3407".to_string(),
            "MA3442".to_string(),
            "MA3703".to_string(),
            "CIT4130023".to_string(),
            "CIT4130024".to_string(),
            "MA4304".to_string(),
            "MA5619".to_string(),
        ]),
        facultys: Some(vec!["MA".to_string(), "CIT".to_string()]),
        curriculum: Some("5244".to_string()),
    };

    let constraints = ConstraintSettings {
        min_num_ects: None,
        max_num_days: Some(3),
    };

    let possible_lectures = CourseSelection::addmissiable_lectures(conn, filters)
        .expect("should be able to request possible lectures");
    let course_selections = CourseSelection::build_from_lectures(possible_lectures);
    scheduling_problem.add_courses(&course_selections)?;
    scheduling_problem.add_additional_constraints(constraints)?;
    let solution = scheduling_problem.solve(SolutionObjective::MaximizeNumEcts)?;
    let resulting_courses: Vec<CourseSelection> = course_selections
        .into_iter()
        .zip(solution.iter())
        .filter_map(|(value, &mask)| if mask == 1. { Some(value) } else { None })
        .collect();
    println!("Result: {:#?}", resulting_courses);
    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::NaiveTime;

    use crate::{
        db_setup::connection,
        tum_api::{
            lecture::NewLecture,
            subject_appointments::{self},
        },
    };

    use super::SchedulingProblem;

    #[test]
    fn test_solving_simple_ip() {
        dotenv::dotenv().ok();
        let conn = &mut connection().expect("should be able to establish connection to db");
        let mut scheduling_problem = SchedulingProblem::new();
        for subject in ["MA3080", "MA3205", "MA3442", "MA4804", "CIT4100003"] {}
        println!("Added appointements");
        scheduling_problem
            .solve(super::SolutionObjective::NoObjective)
            .expect("should be able to start solve");
    }
}
