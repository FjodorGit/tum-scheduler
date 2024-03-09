type CoursesPerFaculty = Vec<(String, i32)>;

pub struct ConstraintSettings {
    pub min_num_ects: Option<i32>,
    pub max_num_days: Option<i32>,
    pub max_courses_per_faculty: Option<CoursesPerFaculty>,
}

pub struct FilterSettings {
    pub subjects: Option<Vec<String>>,
    pub exclude_subject: Option<Vec<String>>,
    pub faculties: Option<Vec<String>>,
    pub curriculum: Option<String>,
}

pub enum SolutionObjective {
    NoObjective,
    MinimizeNumCourses,
    MaximizeNumEcts,
}
