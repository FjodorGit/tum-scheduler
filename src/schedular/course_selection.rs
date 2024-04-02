use itertools::Either;
use itertools::Itertools;

use diesel::result;
use diesel::PgConnection;
use serde::Serialize;

use crate::schema::lecture;
use crate::tum_api::appointment::SingleAppointment;
use crate::tum_api::lecture::Lecture;

use super::settings::FilterSettings;

#[derive(Debug, Serialize)]
pub struct CourseSelection {
    pub subject: String,
    pub name_en: String,
    pub appointments: Vec<SingleAppointment>,
    pub faculty: String,
    pub ects: f64,
}

impl CourseSelection {
    pub fn takes_place_on(&self, weekday: &String) -> bool {
        self.appointments.iter().any(|a| a.weekday == *weekday)
    }

    pub fn weekdays(&self) -> impl Iterator<Item = String> + '_ {
        self.appointments
            .iter()
            .map(|appo| appo.weekday.to_owned())
            .unique()
    }

    pub fn addmissiable_lectures(
        conn: &mut PgConnection,
        filters: FilterSettings,
    ) -> Result<Vec<Lecture>, result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        let mut lectures = lecture::table.into_boxed();
        if let Some(sems) = filters.semester {
            lectures = lectures.filter(lecture::semester.eq(sems));
        }
        if let Some(curr) = filters.curriculum {
            lectures = lectures.filter(lecture::curriculum.eq(curr));
        }
        if let Some(fac) = filters.faculties {
            lectures = lectures.filter(lecture::organization.eq_any(fac));
        }
        if let Some(exclude_subj) = filters.excluded_courses {
            lectures = lectures.filter(lecture::subject.ne_all(exclude_subj));
        }

        let addmissiable_lectures = lectures
            .filter(lecture::course_type.eq_any(["VO", "VI", "UE"]))
            .order((lecture::subject.asc(), lecture::course_type.desc()))
            .distinct()
            .load::<Lecture>(conn)?;
        Ok(addmissiable_lectures)
    }

    pub fn build_from_lectures(lectures: Vec<Lecture>) -> Vec<Self> {
        lectures
            .iter()
            .group_by(|l| &l.subject)
            .into_iter()
            .flat_map(|(_, subject_group)| {
                let (teaching_lectures, exercise_lectures): (Vec<&Lecture>, Vec<&Lecture>) =
                    subject_group
                        .into_iter()
                        .partition_map(|lec| match lec.course_type.as_str() {
                            "VO" | "VI" => Either::Left(lec),
                            "UE" => Either::Right(lec),
                            _ => unreachable!(),
                        });

                let ects: f64 = teaching_lectures.first().map_or(0.0, |&l| l.ects)
                    + exercise_lectures.first().map_or(0.0, |&l| l.ects);
                let ects = ects.ceil();
                let course_selection_for_subject = Self::course_selection_from_course_group(
                    teaching_lectures,
                    exercise_lectures,
                    ects,
                );
                course_selection_for_subject
            })
            .collect()
    }

    fn from_teaching_lectures(lec: &[&Lecture], ects: &f64) -> Vec<Self> {
        let subject = lec[0].subject.to_owned();
        let name_en = lec[0].name_en.to_owned();
        let faculty = lec[0].organization.to_owned();
        let appointments = lec.iter().map(|l| l.appointment()).collect_vec();
        vec![Self {
            subject,
            name_en,
            appointments,
            ects: f64::ceil(*ects),
            faculty,
        }]
    }

    fn from_exercise_lectures(lec: &[&Lecture], ects: f64) -> Vec<Self> {
        lec.iter()
            .map(|l| {
                let subject = l.subject.to_owned();
                let name_en = l.name_en.to_owned();
                let appointment = l.appointment();
                let faculty = l.organization.to_owned();
                Self {
                    subject,
                    name_en,
                    appointments: vec![appointment],
                    ects,
                    faculty,
                }
            })
            .collect_vec()
    }

    fn from_lecture_with_exercises(lec: &[&Lecture], exer: &[&Lecture], ects: &f64) -> Vec<Self> {
        let mut selections = vec![];
        let subject = &lec[0].subject;
        let name_en = &lec[0].name_en;
        let faculty = &lec[0].organization;
        let teaching_appointments = lec.iter().map(|l| l.appointment()).collect_vec();

        for ex in exer.into_iter() {
            let mut appointments = teaching_appointments.clone();
            appointments.push(ex.appointment());
            let selection = Self {
                subject: subject.to_owned(),
                name_en: name_en.to_owned(),
                appointments,
                ects: *ects,
                faculty: faculty.to_owned(),
            };
            selections.push(selection);
        }
        selections
    }

    fn course_selection_from_course_group<'a>(
        lec: Vec<&'a Lecture>,
        exec: Vec<&'a Lecture>,
        ects: f64,
    ) -> Vec<CourseSelection> {
        match (lec.len(), exec.len()) {
            (0, 0) => vec![],
            (0, _) => Self::from_exercise_lectures(&exec, ects),
            (_, 0) => Self::from_teaching_lectures(&lec, &ects),
            (_, _) => Self::from_lecture_with_exercises(&lec, &exec, &ects),
        }
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use itertools::Itertools;

    use crate::{
        db_setup::connection,
        tum_api::{
            course::CourseFromXml,
            lecture::{Lecture, Lectures},
        },
    };

    use super::{CourseSelection, FilterSettings};

    fn generate_test_lectures() -> Vec<Lecture> {
        let l1t1 = Lecture::new("9:30", "11:30", "Monday", "VO", "JO1111", "First", 4.);
        let l1t2 = Lecture::new("9:30", "11:30", "Tuesday", "VO", "JO1111", "First", 4.);
        let l1e1 = Lecture::new("12:30", "14:30", "Monday", "UE", "JO1111", "First", 4.);
        let l1e2 = Lecture::new("12:30", "14:30", "Friday", "UE", "JO1111", "First", 4.);

        let l2t1 = Lecture::new("8:30", "10:30", "Wednesday", "VO", "NE9999", "Second", 7.);
        let l2e1 = Lecture::new("12:30", "14:30", "Monday", "UE", "NE9999", "Second", 7.);
        let l2e2 = Lecture::new("12:30", "14:30", "Friday", "UE", "NE9999", "Second", 7.);
        let l2e3 = Lecture::new("06:30", "09:45", "Friday", "UE", "NE9999", "Second", 7.);

        vec![l1t1, l1t2, l1e1, l1e2, l2t1, l2e1, l2e2, l2e3]
    }

    #[test]
    fn test_building_subject_appointment() {
        dotenv().ok();
        let filters = FilterSettings {
            semester: Some("23W"),
            excluded_courses: None,
            faculties: None, //Some("IN".to_string()),
            curriculum: None,
        };
        let conn = &mut connection().expect("should be able to establish connection");
        let lectures = CourseSelection::addmissiable_lectures(conn, filters)
            .expect("should be able to find addmissable lectures");
        CourseSelection::build_from_lectures(lectures);
    }

    #[test]
    fn test_building_selections_teching_and_exercise() {
        let lectures = generate_test_lectures();
        let selections = CourseSelection::build_from_lectures(lectures);
        let first_selections = selections
            .iter()
            .filter(|selection| selection.subject == "JO1111".to_owned())
            .collect_vec();
        let second_selection = selections
            .iter()
            .filter(|selection| selection.subject == "NE9999".to_owned())
            .collect_vec();
        assert_eq!(selections.len(), 5);
        assert_eq!(first_selections.iter().count(), 2);
        assert_eq!(second_selection.iter().count(), 3);
        first_selections.iter().for_each(|selection| {
            assert_eq!(selection.appointments.len(), 3);
        });
        second_selection.iter().for_each(|selection| {
            assert_eq!(selection.appointments.len(), 2);
        });
    }
}
