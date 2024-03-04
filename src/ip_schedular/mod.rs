use grb::prelude::*;
use std::array;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;

use crate::tum_api::{
    appointment::SingleAppointment, lecture::NewLecture, subject_appointments::CourseSelection,
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
    weekday_expr: HashMap<String, LinExpr>,
    interval_expr: HashMap<String, LinExpr>,
    min_amount_ects: LinExpr,
}

impl SchedulingProblem {
    pub fn new() -> Self {
        let model = Model::new("schedular").expect("should be able to create grb model");
        Self {
            model,
            vars: vec![],
            weekday_expr: HashMap::new(),
            interval_expr: HashMap::new(),
            min_amount_ects: LinExpr::new(),
        }
    }

    pub fn add_subjects(
        &mut self,
        subject_aps: Vec<CourseSelection>,
    ) -> Result<(), SchedularError> {
        let mut vars_count = 0;
        for subject_ap in subject_aps.into_iter() {
            self.add_subject_schedule(subject_ap, vars_count)?;
            vars_count += 1;
        }
        Ok(())
    }

    pub fn add_subject_schedule(
        &mut self,
        subject_schedule: CourseSelection,
        schedule_num: i32,
    ) -> Result<(), SchedularError> {
        let appointment_var_name = format!("{}_v{}", subject_schedule.name, schedule_num);
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

        self.min_amount_ects
            .add_term(subject_schedule.ects, appointment_var);
        Ok(())
    }

    fn add_session(&mut self, appointment_var: Var, appointment: &SingleAppointment) {
        let mut time_point = appointment.from;
        while time_point < appointment.to {
            let constraint_name = format!("{}_{}", appointment.weekday, time_point);
            if let Some(expr) = self.interval_expr.get_mut(&constraint_name) {
                expr.add_term(1.0, appointment_var);
            } else {
                let mut expr = LinExpr::new();
                expr.add_term(1.0, appointment_var);
                self.interval_expr.insert(constraint_name, expr);
            }
            time_point += Duration::minutes(15);
        }
    }

    fn add_weekday(&mut self, session_var: Var, weekday: String) {
        if let Some(expr) = self.weekday_expr.get_mut(&weekday) {
            expr.add_term(1.0, session_var);
        } else {
            let mut expr = LinExpr::new();
            expr.add_term(1.0, session_var);
            self.weekday_expr.insert(weekday, expr);
        }
    }

    pub fn solve(&mut self) -> Result<(), SchedularError> {
        // let mut env = Env::new("schedular.log").expect("should be able to init env");
        // env.set(param::LogToConsole, 0)
        //     .expect("should be able to init logger");
        let interval_constraints = self
            .interval_expr
            .iter()
            .map(|(name, expr)| (name, c!(expr.clone() <= 1)));
        let weekday_constraints = self
            .weekday_expr
            .iter()
            .map(|(name, expr)| (name, c!(expr.clone() <= 1)));
        self.model.set_objective(0, Minimize)?;
        for (constr, expr) in weekday_constraints {
            self.model.add_constr(constr, expr)?;
        }
        for (constr, expr) in interval_constraints {
            self.model.add_constr(constr, expr)?;
        }
        self.model
            .add_constr("min_ects", c!(self.min_amount_ects.clone() >= 13))?;
        self.model.update()?;
        println!("Writing model to file");
        self.model.write("schedular.lp")?;
        self.model.optimize()?;
        let vals = self.model.get_obj_attr_batch(attr::X, self.vars.clone())?;
        println!("{:#?}", vals);
        Ok(())
    }
}

pub fn test_grb() -> Result<(), SchedularError> {
    dotenv::dotenv().ok();
    use crate::db_setup::connection;
    let conn = &mut connection().expect("should be able to establish connection to db");
    let mut scheduling_problem = SchedulingProblem::new();
    for subject in ["MA3080", "MA3205", "MA3442", "MA4804", "CIT4100003"] {}
    scheduling_problem.solve()?;
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
            .solve()
            .expect("should be able to start solve");
    }
}
