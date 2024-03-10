use thiserror::Error;

use self::tum_xml_node::TumXmlError;
use crate::db_setup;

pub mod appointment;
pub mod course;
pub mod course_variant;
pub mod curriculum;
pub mod lecture;
pub mod tum_xml_node;

#[derive(Debug, Error)]
pub enum DataAquisitionError {
    #[error("Failed to parse resource node: {0}")]
    NodeParseError(#[from] TumXmlError),
    #[error("Failed to parse response document")]
    DocumentParseError(#[from] roxmltree::Error),
    #[error("Failed to request course basic data")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed interact with database `{0}`")]
    DbError(#[from] db_setup::DbError),
}
