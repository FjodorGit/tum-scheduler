use crate::{db_setup::DbError, schema::lecture};
use chrono::NaiveTime;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, RunQueryDsl, Selectable};

use super::{
    appointment::{AppointmentFromXml, SingleAppointment},
    course::CourseFromXml,
    course_variant::CourseVariant,
};

#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Selectable)]
#[diesel(table_name = lecture)]
pub struct LectureFromXml {
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

#[derive(Debug, Clone, Queryable, PartialEq, Selectable)]
#[diesel(table_name = lecture)]
pub struct LectureAppointment {
    pub id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: String,
    pub subject: String,
    pub course_type: String,
    pub name_en: String,
    pub ects: f64,
}

impl LectureFromXml {
    pub fn build(
        course: &CourseFromXml,
        appointment: &AppointmentFromXml,
        variant: &CourseVariant,
    ) -> Vec<Self> {
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
            // println!("{:#?}", lecture);
            lectures.push(lecture);
        }
        lectures
    }

    pub fn insert(conn: &mut PgConnection, lectures: Vec<Self>) -> Result<(), DbError> {
        use crate::schema::lecture::dsl::*;

        diesel::insert_into(lecture)
            .values(lectures)
            .on_conflict_do_nothing()
            .execute(conn)
            .map_err(|e| DbError::InsertionFailed(e.to_string()))?;
        Ok(())
    }
}

impl LectureAppointment {
    pub fn appointment(&self) -> SingleAppointment {
        SingleAppointment {
            from: self.start_time,
            to: self.end_time,
            weekday: self.weekday.clone(),
            course_type: self.course_type.to_owned(),
        }
    }
}
