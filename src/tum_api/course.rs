use core::panic;
use std::env;

use diesel::{
    deserialize::Queryable, prelude::Insertable, query_dsl::methods::SelectDsl, PgConnection,
    RunQueryDsl,
};
use lazy_static::lazy_static;
use roxmltree::Document;
use tracing::info;

use crate::{
    db_setup::{connection, DbError},
    tum_api::organization::TumOrganization,
};

use super::{tum_xml_node::TumXmlNode, DataAquisitionError, TumXmlError};

#[derive(Debug)]
pub struct CourseFromXml {
    pub id: String,
    pub course_type: String,
    pub sws: f64,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
}

#[derive(Debug)]
pub struct CourseEndpoint {
    pub base_request_url: String,
    pub current_page: usize,
}

impl TryFrom<TumXmlNode<'_, '_>> for CourseFromXml {
    type Error = TumXmlError;
    fn try_from(resource_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let id = resource_node.get_text_of_next("id")?;
        let semester_node = resource_node.get_next("semesterDto")?;
        let semester = semester_node.get_text_of_next("key")?;

        let course_title_node = semester_node.get_next_sibling()?;
        let (name_de, name_en) = course_title_node.get_translations()?;

        let course_type_node = resource_node.get_next("courseTypeDto")?;
        let course_type = course_type_node.get_text_of_next("key")?;

        let course_norm_node = resource_node.get_next("courseNormConfigs")?;
        let sws_text = course_norm_node.get_text_of_last("value")?;
        let sws = sws_text
            .parse::<f64>()
            .expect("sws should be parsable to float");

        let course_basic_data = CourseFromXml {
            id,
            course_type,
            sws,
            name_de,
            name_en,
            semester,
        };
        Ok(course_basic_data)
    }
}

impl CourseFromXml {
    fn read_all_from_page(xml: String) -> Result<Vec<CourseFromXml>, DataAquisitionError> {
        let mut result = vec![];
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode::new(document.root_element());

        // println!("{:#?}", some_resource_element.unwrap().tag_name().name());
        for resource_element in root_element.resource_elements() {
            let course = Self::try_from(resource_element)?;
            // println!("{:#?}", course);
            result.push(course);
        }
        // println!("{:#?}", result.len());
        Ok(result)
    }
}

impl CourseEndpoint {
    pub fn for_semester(semester_id: &str) -> Self {
        let base_url = env::var("BASE_COURSES_URL")
            .expect("BASE_COURSES_URL should exist in environment variables");
        let base_request_url = format!("{}{}&$skip=", base_url, semester_id);
        Self {
            base_request_url,
            current_page: 0,
        }
    }

    pub async fn fetch_next_page(&mut self) -> Result<Vec<CourseFromXml>, DataAquisitionError> {
        let request_url = format!("{}{}", self.base_request_url, self.current_page * 100);
        info!("Fetching from page {}", self.current_page);
        let request_result = reqwest::get(request_url).await?;
        let xml = request_result.text().await?;
        let courses = CourseFromXml::read_all_from_page(xml)?;
        self.current_page += 1;
        if courses.is_empty() {
            return Err(DataAquisitionError::ZeroCoursesFound(self.current_page));
        }
        Ok(courses)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::course::CourseFromXml;

    #[test]
    fn test_reading_courses_page() {
        let test_xml: String = fs::read_to_string("test_xmls/course.xml")
            .expect("Should be able to read course test file");
        let courses =
            CourseFromXml::read_all_from_page(test_xml).expect("should be able to read courses");
        assert_eq!(courses.len(), 100);
    }
}
