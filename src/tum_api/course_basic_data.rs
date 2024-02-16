use roxmltree::Node;
use thiserror::Error;

pub struct CourseBasicData {
    id: String,
    course_type: String,
    semester: String,
}

#[derive(Debug, Error)]
pub enum CourseBasicDataError {}

impl TryFrom<Node<'_, '_>> for CourseBasicData {
    type Error = CourseBasicDataError;
    fn try_from(value: Node<'_, '_>) -> Result<Self, Self::Error> {
        todo!()
    }
}
