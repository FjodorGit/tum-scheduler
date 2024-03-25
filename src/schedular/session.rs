use chrono::NaiveTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;

use crate::schema::lecture;

#[derive(Debug, Clone, Queryable, PartialEq, Selectable)]
#[diesel(table_name = lecture)]
pub struct LectureSession {
    pub id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: String,
    pub subject: String,
    pub course_type: String,
    pub name_en: String,
    pub organization: String,
    pub ects: f64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize)]
pub struct SingleAppointment {
    pub weekday: String,
    pub from: NaiveTime,
    pub to: NaiveTime,
    pub course_type: String,
}

impl LectureSession {
    pub fn appointment(&self) -> SingleAppointment {
        SingleAppointment {
            from: self.start_time,
            to: self.end_time,
            weekday: self.weekday.clone(),
            course_type: self.course_type.to_owned(),
        }
    }
}
impl SingleAppointment {
    pub fn takes_place_at(&self, time: NaiveTime, weekday: &str) -> bool {
        self.from <= time && self.to > time && self.weekday == weekday
    }
}
