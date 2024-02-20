use std::io::prelude::*;
use std::{env, fs::File};

use roxmltree::Document;

use crate::tum_api::lecture::Lecture;

use super::appointment::AppointmentEndpoint;
use super::{TumApiError, TumXmlError, TumXmlNode};

#[derive(Debug)]
pub struct Course {
    pub id: String,
    pub course_type: String,
    pub sws: String,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
}

impl TryFrom<TumXmlNode<'_, '_>> for Course {
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
        let sws = course_norm_node.get_text_of_last("value")?;

        let course_basic_data = Course {
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

impl Course {
    fn read_all_from_page(xml: String) -> Result<Vec<Course>, TumApiError> {
        let mut result = vec![];
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode(document.root_element());

        // println!("{:#?}", some_resource_element.unwrap().tag_name().name());
        for resource_element in root_element.resource_elements() {
            let course = Self::try_from(resource_element)?;
            println!("{:#?}", course);
            result.push(course);
        }
        // println!("{:#?}", result.len());
        Ok(result)
    }

    pub async fn fetch_all() -> Result<(), TumApiError> {
        let mut page = 0;
        loop {
            let mut request_url = env::var("BASE_URL_IDS")
                .expect("BASE_URL_IDS should exist in environment variables");
            request_url.push_str(&(page * 100).to_string());
            println!("url {:#?}", request_url);
            let request_result = reqwest::get(request_url).await?;
            let xml = request_result.text().await?;
            let basic_data = Self::read_all_from_page(xml)?;
            if basic_data.is_empty() {
                break;
            }
            page += 1;
        }
        Ok(())
    }

    pub async fn build_lectues(&self) -> Result<Vec<Lecture>, TumApiError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::course::Course;

    #[test]
    fn test_reading_courses_page() {
        let test_xml: String = fs::read_to_string("test_xmls/course.xml")
            .expect("Should be able to read course test file");
        let courses = Course::read_all_from_page(test_xml).expect("should be able to read courses");
        assert_eq!(courses.len(), 100);
    }
}
