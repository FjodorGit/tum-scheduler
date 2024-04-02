use std::str::FromStr;

use crate::schema::lecture;
use chrono::NaiveTime;
use diesel::{deserialize::Queryable, prelude::Insertable, PgConnection, RunQueryDsl, Selectable};
use diesel::{result, ExpressionMethods, QueryDsl};
use itertools::Itertools;

use super::appointment::SingleAppointment;
use super::course_description::{CourseDescription, CourseDescriptionEndpoint};
use super::course_variant::CourseVariantFromXml;
use super::organization::{TumOrganization, TumOrganizationFromXml};
use super::{appointment::AppointmentFromXml, course::CourseFromXml};

pub struct Lectures;

#[derive(Debug, Default, Clone, Insertable, Queryable, PartialEq, Selectable)]
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

#[derive(Debug)]
pub struct LecturesBuilder {
    pub templates: Vec<LectureTemplate>,
}

#[derive(Default, Clone, Debug)]
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
    pub fn build_from(course: &CourseFromXml) -> LecturesBuilder {
        LecturesBuilder::from(vec![course.clone()])
    }
}

impl LecturesBuilder {
    pub fn with_appointments(mut self, appointments: &[AppointmentFromXml]) -> Self {
        let new_templates = self
            .templates
            .iter()
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
        self.templates = new_templates;
        self
    }

    pub fn with_varaints(mut self, variants: &[CourseVariantFromXml]) -> Self {
        let new_templates = self
            .templates
            .iter_mut()
            .flat_map(|template| {
                variants.into_iter().map(|variant| LectureTemplate {
                    curriculum: Some(variant.curriculum.clone()),
                    subject: Some(variant.subject.clone()),
                    ..template.clone()
                })
            })
            .collect_vec();
        self.templates = new_templates;
        self
    }
    pub fn with_description(mut self, description: &CourseDescription) -> Self {
        self.templates
            .iter_mut()
            .for_each(|template| template.description = Some(description.0.clone()));
        self
    }

    pub fn by_organization(mut self, orga: &TumOrganizationFromXml) -> Self {
        self.templates.iter_mut().for_each(|template| {
            template.organization = Some(orga.to_owned());
        });
        self
    }

    pub fn finalize(self) -> Vec<Lecture> {
        self.templates
            .into_iter()
            .map(Lecture::from_template)
            .collect()
    }

    pub fn add_to_db(self, conn: &mut PgConnection) -> Result<(), result::Error> {
        use crate::schema::lecture::dsl::*;

        diesel::insert_into(lecture)
            .values(self.finalize())
            .on_conflict_do_nothing()
            .execute(conn)?;
        Ok(())
    }
}

impl Lecture {
    pub fn new(
        start_time: &str,
        end_time: &str,
        weekday: &str,
        course_type: &str,
        subject: &str,
        name: &str,
        ects: f64,
    ) -> Self {
        Self {
            start_time: NaiveTime::from_str(start_time).unwrap(),
            end_time: NaiveTime::from_str(end_time).unwrap(),
            weekday: weekday.to_owned(),
            course_type: course_type.to_owned(),
            subject: subject.to_owned(),
            name_en: name.to_owned(),
            ects,
            ..Default::default()
        }
    }
    pub fn appointment(&self) -> SingleAppointment {
        SingleAppointment {
            from: self.start_time,
            to: self.end_time,
            weekday: self.weekday.clone(),
            course_type: self.course_type.to_owned(),
        }
    }

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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::NaiveTime;

    use crate::tum_api::{
        appointment::AppointmentFromXml, course::CourseFromXml,
        course_variant::CourseVariantFromXml,
    };

    use super::Lectures;

    #[test]
    fn test_adding_appointments() {
        let course = CourseFromXml {
            id: "11111".to_string(),
            course_type: "VO".to_string(),
            sws: 4.,
            name_en: "TestKurs".to_string(),
            name_de: "TestCourse".to_string(),
            semester: "200".to_string(),
            processing_error: crate::tum_api::course::ProcessingError::None,
        };
        let appointment1 = AppointmentFromXml {
            weekdays: vec!["Monday".to_string(), "Tuesday".to_string()],
            start_time: NaiveTime::from_str("13:30").expect("should be able to parse time"),
            end_time: NaiveTime::from_str("15:30").expect("should be able to parse time"),
        };
        let appointment2 = AppointmentFromXml {
            weekdays: vec!["Tuesday".to_string()],
            start_time: NaiveTime::from_str("9:30").expect("should be able to parse time"),
            end_time: NaiveTime::from_str("11:30").expect("should be able to parse time"),
        };
        let lectures =
            Lectures::build_from(&course).with_appointments(&[appointment1, appointment2]);
        assert_eq!(lectures.templates.len(), 3);
        for lec in lectures.templates.iter() {
            assert!(lec.start_time.is_some());
            assert!(lec.end_time.is_some());
            assert!(lec.weekday.is_some());
        }
        assert_eq!(
            lectures.templates.first().unwrap().start_time.unwrap(),
            NaiveTime::from_str("13:30").expect("should be able to parse time")
        );
        assert_eq!(
            lectures.templates.first().unwrap().end_time.unwrap(),
            NaiveTime::from_str("15:30").expect("should be able to parse time")
        );
        assert_eq!(
            lectures.templates.last().unwrap().weekday.clone().unwrap(),
            "Tuesday".to_string()
        );
    }

    #[test]
    fn test_with_variants() {
        let course = CourseFromXml {
            id: "11111".to_string(),
            course_type: "VO".to_string(),
            sws: 4.,
            name_en: "TestKurs".to_string(),
            name_de: "TestCourse".to_string(),
            semester: "200".to_string(),
            processing_error: crate::tum_api::course::ProcessingError::None,
        };

        let variant1 = CourseVariantFromXml {
            curriculum: "4321".to_string(),
            subject: "HAHAH".to_string(),
        };
        let variant2 = CourseVariantFromXml {
            curriculum: "1243".to_string(),
            subject: "JOJOJ".to_string(),
        };
        let lectures = Lectures::build_from(&course).with_varaints(&[variant1, variant2]);
        println!("{:#?}", lectures);
        assert_eq!(lectures.templates.len(), 2);
        for lec in lectures.templates.iter() {
            assert!(lec.curriculum.is_some());
            assert!(lec.subject.is_some());
        }
        // assert_eq!(
        //     lectures.templates.first().unwrap().curriculum.unwrap(),
        //     NaiveTime::from_str("13:30").expect("should be able to parse time")
        // );
        // assert_eq!(
        //     lectures.templates.first().unwrap().end_time.unwrap(),
        //     NaiveTime::from_str("15:30").expect("should be able to parse time")
        // );
        // assert_eq!(
        //     lectures.templates.last().unwrap().weekday.clone().unwrap(),
        //     "Tuesday".to_string()
        // );
    }
}
