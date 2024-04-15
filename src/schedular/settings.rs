use std::collections::HashMap;

use serde::Deserialize;

type CoursesPerFaculty = Vec<(String, i32)>;

pub struct ConstraintSettings {
    pub min_num_ects: Option<i32>,
    pub max_num_days: Option<i32>,
    pub max_courses_per_faculty: Option<CoursesPerFaculty>,
}

#[derive(Debug)]
pub struct FilterSettings<'a> {
    pub semester: Option<&'a str>,
    pub courses: Option<&'a [&'a str]>,
    pub excluded_courses: Option<&'a Vec<String>>,
    pub faculties: Option<&'a Vec<String>>,
    pub curriculum: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub enum SolutionObjective {
    #[serde(rename = "noobjective")]
    NoObjective,
    #[serde(rename = "mincourses")]
    MinimizeNumCourses,
    #[serde(rename = "minweekdays")]
    MinimizeNumWeekdays,
    #[serde(rename = "maxects")]
    MaximizeNumEcts,
}

impl From<&HashMap<String, i32>> for ConstraintSettings {
    fn from(value: &HashMap<String, i32>) -> Self {
        let mut max_num_days = None;
        let mut min_num_ects = None;
        let mut max_courses_per_faculty_vec = vec![];
        for (key, amount) in value.iter() {
            match key.as_str() {
                "maxweekdays" => max_num_days = Some(amount),
                "minects" => min_num_ects = Some(amount),
                _ => max_courses_per_faculty_vec.push((key.to_owned(), *amount)),
            }
        }
        let mut max_courses_per_faculty = None;
        if !max_courses_per_faculty_vec.is_empty() {
            max_courses_per_faculty = Some(max_courses_per_faculty_vec);
        }
        Self {
            max_num_days: max_num_days.copied(),
            min_num_ects: min_num_ects.copied(),
            max_courses_per_faculty,
        }
    }
}
