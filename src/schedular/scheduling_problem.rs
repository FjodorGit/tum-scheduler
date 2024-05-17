use crate::{
    db_setup::connection, schedular::settings::FilterSettings,
    scraper::appointment::SingleAppointment,
};

use super::{
    course_selection::CourseSelection,
    settings::{ConstraintSettings, SolutionObjective},
    WEEKDAYS,
};
use grb::{
    attribute::{ModelDoubleAttr::ObjVal, ModelIntAttr::SolCount},
    parameter::IntParam::{PoolSearchMode, PoolSolutions, SolutionLimit, SolutionNumber},
    prelude::*,
};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;

use chrono::Duration;
use grb::{c, expr::LinExpr};

use super::SchedularError;

pub struct SchedulingProblem {
    model: Model,
    vars: Vec<Var>,
    weekday_exprs: HashMap<String, LinExpr>,
    on_weekday_vars: HashMap<String, Vec<Var>>,
    interval_exprs: HashMap<String, LinExpr>,
    amount_ects: LinExpr,
    faculties: HashMap<String, LinExpr>,
}

#[derive(Serialize, Debug)]
pub struct SolutionSchedule {
    objective_value: f64,
    total_ects: f64,
    course_selections: Vec<CourseSelection>,
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
            faculties: HashMap::new(),
        }
    }

    pub fn add_courses<I: IntoIterator<Item = CourseSelection>>(
        &mut self,
        subject_aps: I,
    ) -> Result<(), SchedularError> {
        for (var_num, subject_ap) in subject_aps.into_iter().enumerate() {
            self.add_course(&subject_ap, var_num)?;
        }
        Ok(())
    }

    pub fn add_course(
        &mut self,
        course_selection: &CourseSelection,
        schedule_num: usize,
    ) -> Result<(), SchedularError> {
        let course_var_name = format!("{}_v{}", course_selection.subject, schedule_num);
        let course_var = self
            .model
            .add_var(&course_var_name, Binary, 0., 0., 1., [])?;

        self.vars.push(course_var);
        self.add_faculty(course_var, course_selection);
        for appointment in course_selection.appointments.iter() {
            self.add_session(course_var, appointment);
        }

        for weekday in course_selection.weekdays() {
            self.add_weekday(course_var, weekday)
        }

        self.amount_ects.add_term(course_selection.ects, course_var);
        Ok(())
    }

    fn add_faculty(&mut self, course_var: Var, course_selection: &CourseSelection) {
        if let Some(expr) = self.faculties.get_mut(&course_selection.faculty) {
            expr.add_term(1., course_var);
        } else {
            let mut expr = LinExpr::new();
            expr.add_term(1.0, course_var);
            self.faculties
                .insert(course_selection.faculty.clone(), expr);
        }
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

    pub fn add_constraints(
        &mut self,
        constraints: &ConstraintSettings,
    ) -> Result<(), SchedularError> {
        let interval_constraints = self
            .interval_exprs
            .iter()
            .map(|(name, expr)| (name, c!(expr.clone() <= 1)));
        for (constr, expr) in interval_constraints {
            self.model.add_constr(constr, expr)?;
        }

        let solution_num = constraints.max_num_solutions.unwrap_or(1);
        self.model.set_param(PoolSolutions, solution_num)?;

        if let Some(min_ects) = constraints.min_num_ects {
            self.model
                .add_constr("min_ects", c!(self.amount_ects.clone() >= min_ects))?;
        }

        if let Some(courses_per_faculty) = &constraints.max_courses_per_faculty {
            for (fac, num) in courses_per_faculty.iter() {
                if let Some(expr) = self.faculties.get(fac) {
                    self.model.add_constr(fac, c!(expr.clone() <= num))?;
                }
            }
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

    fn set_objective(&mut self, objective: &SolutionObjective) -> Result<(), SchedularError> {
        match objective {
            SolutionObjective::MinimizeNumCourses => self
                .model
                .set_objective(self.vars.iter().grb_sum(), Minimize),
            SolutionObjective::MaximizeNumEcts => {
                self.model.set_objective(self.amount_ects.clone(), Maximize)
            }
            SolutionObjective::MinimizeNumWeekdays => {
                todo!()
            }
            SolutionObjective::NoObjective => self.model.set_objective(0, Minimize),
        }?;
        Ok(())
    }

    pub fn solve(
        &mut self,
        filter_settings: FilterSettings,
        constraint_settings: &ConstraintSettings,
        objective: &SolutionObjective,
    ) -> Result<Vec<SolutionSchedule>, SchedularError> {
        let conn = &mut connection().expect("should be able to establish connection to db");
        let possible_lectures = CourseSelection::addmissiable_lectures(conn, filter_settings)
            .expect("should be able to request possible lectures");

        let course_selections = CourseSelection::build_from_lectures(possible_lectures);
        self.add_courses(course_selections.clone())?;
        self.add_constraints(constraint_settings)?;
        self.set_objective(objective)?;
        self.model.set_param(PoolSearchMode, 2)?;
        self.model.update()?;
        println!("Writing model to file");
        self.model.write("schedular.lp")?;
        self.model.optimize()?;

        let solution_count = self.model.get_attr(SolCount)?;

        let courses_iterator = course_selections.iter();
        (0..solution_count)
            .map(|index| {
                self.model.set_param(SolutionNumber, index)?;
                let objective_value = self.model.get_attr(ObjVal)?;
                let solution_vec = self.model.get_obj_attr_batch(attr::Xn, self.vars.clone())?;
                let course_selections = courses_iterator
                    .clone()
                    .zip(solution_vec.iter())
                    .filter_map(|(course, &val)| {
                        if val == 1. {
                            Some(course.clone())
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                let total_ects = course_selections.iter().fold(0., |acc, val| acc + val.ects);
                let schedule = SolutionSchedule {
                    objective_value,
                    total_ects,
                    course_selections,
                };
                Ok(schedule)
            })
            .collect()
    }
}

pub fn test_run() -> Result<(), SchedularError> {
    dotenv::dotenv().ok();
    let mut scheduling_problem = SchedulingProblem::new();

    let courses = vec![
        "MA3241".to_string(),
        "MA4405".to_string(),
        "MA3005".to_string(),
        "MA5934".to_string(),
        "MA5012".to_string(),
        "MA4408".to_string(),
        "MA5442".to_string(),
        "MA5059".to_string(),
        "MA5306".to_string(),
        "MA3081".to_string(),
        "CIT413031".to_string(),
        "MA4502".to_string(),
    ];

    let filters = FilterSettings {
        semester: Some("24S"),
        excluded_courses: None,
        courses: Some(&courses),
        faculties: None,
        curriculum: Some("5244"),
    };

    let constraints = ConstraintSettings {
        min_num_ects: Some(23),
        max_num_days: None,
        max_num_solutions: Some(2),
        max_courses_per_faculty: None,
    };

    let solutions = scheduling_problem.solve(
        filters,
        &constraints,
        &SolutionObjective::MinimizeNumCourses,
    )?;
    println!("Result: {:#?}", solutions);
    Ok(())
}

#[cfg(test)]
mod test {}
