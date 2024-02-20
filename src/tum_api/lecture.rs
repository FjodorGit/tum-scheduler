use crate::{db_setup::DbError, schema::lecture};
use chrono::NaiveTime;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection};

use super::{
    appointment::{self, Appointment},
    course::Course,
    course_variant::CourseVariant,
};

#[derive(Debug, Clone, Insertable, Queryable, PartialEq)]
#[diesel(table_name = lecture)]
pub struct Lecture {
    pub id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: String,
    pub subject: String,
    pub course_type: String,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
    pub curriculum: String,
    pub faculty: String,
    pub ects: f64,
}

impl Lecture {
    pub fn build(course: &Course, appointment: &Appointment, variant: &CourseVariant) -> Vec<Self> {
        let mut lectures = vec![];
        for weekday in appointment.weekdays.iter() {
            let ects = course
                .sws
                .parse::<f64>()
                .expect("should be able to convert sws string to number")
                * 1.5;
            let lecture = Self {
                id: course.id.to_owned(),
                start_time: appointment.from,
                end_time: appointment.to,
                weekday: weekday.to_owned(),
                subject: variant.abbreviation.to_owned(),
                course_type: course.course_type.to_owned(),
                name_en: course.name_en.to_owned(),
                name_de: course.name_de.to_owned(),
                semester: course.semester.to_owned(),
                curriculum: variant.curriculum.to_owned(),
                faculty: variant.facultiy.to_owned(),
                ects,
            };
            println!("{:#?}", lecture);
            lectures.push(lecture);
        }
        lectures
    }
}
