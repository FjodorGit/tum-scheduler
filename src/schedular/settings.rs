pub struct ConstraintSettings {
    pub min_num_ects: Option<i32>,
    pub max_num_days: Option<i32>,
}

pub enum SolutionObjective {
    NoObjective,
    MinimizeNumCourses,
    MaximizeNumEcts,
}
