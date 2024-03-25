use crate::db_setup::DbError;
use crate::schema::lecture;
use chrono::NaiveTime;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, RunQueryDsl, Selectable};
use diesel::{ExpressionMethods, QueryDsl};
use itertools::Itertools;

use super::appointment::AppointmentsEndpoint;
use super::course_description::{CourseDescription, CourseDescriptionEndpoint};
use super::course_variant::{CourseVariantEndpoint, CourseVariantFromXml};
use super::DataAquisitionError;
use super::{appointment::AppointmentFromXml, course::CourseFromXml};

pub struct Lectures(Vec<Lecture>);

#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Selectable)]
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
    pub description: String,
    pub organization: String,
    pub ects: f64,
}

pub struct LecturesBuilder {
    pub templates: Vec<LectureTemplate>,
}

#[derive(Default, Clone)]
pub struct LectureTemplate {
    pub id: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub weekday: Option<String>,
    pub subject: Option<String>,
    pub course_type: Option<String>,
    pub name_en: Option<String>,
    pub name_de: Option<String>,
    pub semester: Option<String>,
    pub curriculum: Option<String>,
    pub description: Option<String>,
    pub organization: Option<String>,
    pub ects: Option<f64>,
}

impl From<Vec<CourseFromXml>> for LecturesBuilder {
    fn from(value: Vec<CourseFromXml>) -> Self {
        let templates = value
            .into_iter()
            .map(|course| LectureTemplate {
                id: Some(course.id),
                course_type: Some(course.course_type),
                name_en: Some(course.name_en),
                name_de: Some(course.name_de),
                ects: Some(course.sws * 1.5),
                semester: Some(course.semester),
                ..Default::default()
            })
            .collect();
        Self { templates }
    }
}

impl Lectures {
    pub fn build_from(course: CourseFromXml) -> LecturesBuilder {
        LecturesBuilder::from(vec![course])
    }

    pub fn get_all_subjects(
        conn: &mut PgConnection,
        semester: &str,
    ) -> Result<Vec<String>, DbError> {
        lecture::table
            .select(lecture::subject)
            .filter(lecture::semester.eq(semester))
            .load::<String>(conn)
            .map_err(|_| DbError::QueryError("getting all subjects".to_string()))
    }
}

impl LecturesBuilder {
    pub fn with_appointments(mut self, appointments: &[AppointmentFromXml]) -> Self {
        self.templates
            .iter_mut()
            .flat_map(|template| {
                appointments.into_iter().flat_map(|appoint| {
                    appoint
                        .weekdays
                        .clone()
                        .into_iter()
                        .map(|weekday| LectureTemplate {
                            start_time: Some(appoint.start_time),
                            end_time: Some(appoint.end_time),
                            weekday: Some(weekday.to_owned()),
                            ..template.clone()
                        })
                })
            })
            .collect_vec();
        self
    }

    pub fn with_varaints(mut self, variants: &[CourseVariantFromXml]) -> Self {
        self.templates
            .iter_mut()
            .flat_map(|template| {
                variants.into_iter().map(|variant| LectureTemplate {
                    curriculum: Some(variant.curriculum.clone()),
                    subject: Some(variant.subject.clone()),
                    organization: Some(variant.organization.clone()),
                    ..template.clone()
                })
            })
            .collect_vec();
        self
    }
    pub fn with_description(mut self, description: &CourseDescription) -> Self {
        self.templates.iter_mut().for_each(|template| {
            template.description = Some(description.subject_description.clone())
        });
        self
    }

    pub fn finalize(self) -> Vec<Lecture> {
        self.templates
            .into_iter()
            .map(Lecture::from_template)
            .collect()
    }

    pub fn add_to_db(self, conn: &mut PgConnection) -> Result<(), DbError> {
        use crate::schema::lecture::dsl::*;

        diesel::insert_into(lecture)
            .values(self.finalize())
            .on_conflict_do_nothing()
            .execute(conn)
            .map_err(|e| DbError::InsertionFailed(e.to_string()))?;
        Ok(())
    }
}

impl Lecture {
    pub fn from_template(template: LectureTemplate) -> Self {
        Self {
            id: template.id.expect("id has to be set"),
            start_time: template.start_time.expect("start time has to be set"),
            end_time: template.end_time.expect("end time has to be set"),
            weekday: template.weekday.expect("weekday has to be set"),
            subject: template.subject.expect("subject has to be set"),
            course_type: template.course_type.expect("course_type has to be set"),
            name_en: template.name_en.expect("name_en has to be set"),
            name_de: template.name_de.expect("name_de has to be set"),
            semester: template.semester.expect("semester has to be set"),
            curriculum: template.curriculum.expect("curriculum has to be set"),
            description: template.description.expect("description has to be set"),
            organization: template.organization.expect("organization has to be set"),
            ects: template.ects.expect("ects has to be set"),
        }
    }
}
