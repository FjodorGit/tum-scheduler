use itertools::Itertools;

use crate::{schema::lecture, tum_api::appointment::SingleAppointment};
use diesel::result;
use diesel::PgConnection;

use super::lecture::LectureAppointment;

pub struct FilterSettings {
    pub subjects: Option<Vec<String>>,
    pub exclude_subject: Option<Vec<String>>,
    pub facultys: Option<Vec<String>>,
    pub curriculum: Option<String>,
}

#[derive(Debug)]
pub struct CourseSelection {
    pub abbr: String,
    pub name_en: String,
    pub appointments: Vec<SingleAppointment>,
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
    ) -> Result<Vec<LectureAppointment>, result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        let mut lectures = lecture::table.into_boxed();
        if let Some(curr) = filters.curriculum {
            lectures = lectures.filter(lecture::curriculum.eq(curr));
        }
        if let Some(fac) = filters.facultys {
            lectures = lectures.filter(lecture::faculty.eq_any(fac));
        }
        if let Some(subj) = filters.subjects {
            lectures = lectures.filter(lecture::subject.eq_any(subj));
        }
        if let Some(exclude_subj) = filters.exclude_subject {
            lectures = lectures.filter(lecture::subject.ne_all(exclude_subj));
        }

        let addmissiable_lectures = lectures
            .select((
                lecture::id,
                lecture::start_time,
                lecture::end_time,
                lecture::weekday,
                lecture::subject,
                lecture::course_type,
                lecture::name_en,
                lecture::ects,
            ))
            .filter(lecture::course_type.eq_any(["VO", "VI", "UE"]))
            .order((lecture::subject.asc(), lecture::course_type.desc()))
            .distinct()
            .load::<LectureAppointment>(conn)?;
        Ok(addmissiable_lectures)
    }

    pub fn build_from_lectures(lecture_appointments: Vec<LectureAppointment>) -> Vec<Self> {
        let mut course_choices: Vec<CourseSelection> = vec![];
        for (subject, subject_group) in &lecture_appointments.iter().group_by(|l| l.subject.clone())
        {
            let course_type_groups = &subject_group.group_by(|s| s.course_type.clone());
            let mut vos = vec![];
            let mut vis = vec![];
            let mut ues = vec![];
            for (course_type, course_type_group) in course_type_groups {
                match course_type.as_str() {
                    "VO" => vos = course_type_group.collect(),
                    "VI" => vis = course_type_group.collect(),
                    "UE" => ues = course_type_group.collect(),
                    _ => (),
                }
            }

            if vis.len() != 0 && ues.len() != 0 && vos.len() != 0 {
                println!("Messed up subject {:#?}", subject)
            }
            let mut ects = 0.;
            if let Some(c) = vos.get(0) {
                ects += c.ects;
            }
            if let Some(c) = vis.get(0) {
                ects += c.ects;
            }
            if let Some(c) = ues.get(0) {
                ects += c.ects;
            }
            ects = f64::ceil(ects);
            let lectures = [vos, vis].concat();
            let mut course_selection_for_subject =
                Self::course_selection_from_course_groups(lectures, ues, ects);
            // println!(
            //     "Course selection for subject: {:#?}",
            //     course_selection_for_subject
            // );
            course_choices.append(&mut course_selection_for_subject);
        }
        course_choices
    }

    fn from_teaching_lectures(lec: &Vec<&LectureAppointment>, ects: &f64) -> Self {
        let abbr = lec[0].subject.to_owned();
        let name_en = lec[0].name_en.to_owned();
        let appointments = lec.into_iter().map(|l| l.appointment()).collect_vec();
        CourseSelection {
            abbr,
            name_en,
            appointments,
            ects: f64::ceil(*ects),
        }
    }

    fn from_exercise_lectures(lec: &Vec<&LectureAppointment>, ects: f64) -> Vec<Self> {
        lec.into_iter()
            .map(|l| {
                let abbr = l.subject.to_owned();
                let name_en = l.name_en.to_owned();
                let appointment = l.appointment();
                Self {
                    abbr,
                    name_en,
                    appointments: vec![appointment],
                    ects,
                }
            })
            .collect_vec()
    }

    fn from_lecture_with_exercises(
        lec: &Vec<&LectureAppointment>,
        exer: &Vec<&LectureAppointment>,
        ects: &f64,
    ) -> Vec<Self> {
        let mut selections = vec![];
        let abbr = &lec[0].subject;
        let name_en = &lec[0].name_en;
        let teaching_appointments = lec.into_iter().map(|l| l.appointment()).collect_vec();

        for ex in exer.into_iter() {
            let mut appointments = teaching_appointments.clone();
            appointments.push(ex.appointment());
            let selection = CourseSelection {
                abbr: abbr.to_owned(),
                name_en: name_en.to_owned(),
                appointments,
                ects: *ects,
            };
            selections.push(selection);
        }
        selections
    }

    fn course_selection_from_course_groups<'a>(
        lec: Vec<&'a LectureAppointment>,
        exec: Vec<&'a LectureAppointment>,
        ects: f64,
    ) -> Vec<CourseSelection> {
        match (lec.len(), exec.len()) {
            (0, 0) => vec![],
            (0, _) => Self::from_exercise_lectures(&exec, ects),
            (_, 0) => Self::from_teaching_lectures(&lec, &ects).to_vec(),
            (_, _) => Self::from_lecture_with_exercises(&lec, &exec, &ects),
        }
    }

    fn to_vec(self) -> Vec<Self> {
        vec![self]
    }
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;

    use crate::db_setup::connection;

    use super::{CourseSelection, FilterSettings};

    #[test]
    fn test_building_subject_appointment() {
        dotenv().ok();
        let filters = FilterSettings {
            subjects: None,
            exclude_subject: None,
            facultys: None, //Some("IN".to_string()),
            curriculum: None,
        };
        let conn = &mut connection().expect("should be able to establish connection");
        let lectures = CourseSelection::addmissiable_lectures(conn, filters)
            .expect("should be able to find addmissable lectures");
        CourseSelection::build_from_lectures(lectures);
    }

    #[test]
    fn test_getting_all_subjects_from_curriculum() {
        dotenv().ok();
        let conn = &mut connection().expect("should be able to establish connection");
    }
}
