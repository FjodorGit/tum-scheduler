use serde::Deserialize;

type CoursesPerFaculty = Vec<(String, i32)>;

#[derive(Debug, Deserialize)]
pub struct ConstraintSettings {
    pub min_num_ects: Option<i32>,
    pub max_num_solutions: Option<i32>,
    pub max_num_days: Option<i32>,
    pub max_courses_per_faculty: Option<CoursesPerFaculty>,
}

#[derive(Debug)]
pub struct FilterSettings<'a> {
    pub semester: Option<&'a str>,
    pub courses: Option<&'a Vec<String>>,
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
