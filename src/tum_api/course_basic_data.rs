use std::io::prelude::*;
use std::{env, fs::File};

use roxmltree::{Document, Node};
use thiserror::Error;

use crate::{db, tum_api::curriculum::CurriculumError, utils::element_has_name};

#[derive(Debug)]
pub struct CourseBasicData {
    pub id: String,
    pub course_type: String,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
}

#[derive(Debug, Error)]
pub enum CourseBasicDataError {
    #[error("Failed to parse resource node: {0}")]
    NodeParseError(String),
    #[error("Failed to parse course basic data document")]
    DocumentParseError(#[from] roxmltree::Error),
    #[error("Failed to request course basic data")]
    RequestError(#[from] reqwest::Error),
}

impl TryFrom<Node<'_, '_>> for CourseBasicData {
    type Error = CourseBasicDataError;
    fn try_from(resource_node: Node<'_, '_>) -> Result<Self, Self::Error> {
        let id = resource_node
            .descendants()
            .filter(|n| element_has_name(n, "id"))
            .find_map(|n| n.text())
            .ok_or(CourseBasicDataError::NodeParseError(
                "No id node found".to_owned(),
            ))?;
        let semester_node = resource_node
            .descendants()
            .filter(|n| element_has_name(n, "semesterDto"))
            .next()
            .ok_or(CourseBasicDataError::NodeParseError(
                "Node semesterDto node found".to_owned(),
            ))?;

        let semester = semester_node
            .descendants()
            .filter(|n| element_has_name(n, "key"))
            .find_map(|n| n.text())
            .ok_or(CourseBasicDataError::NodeParseError(
                "No semester key found".to_owned(),
            ))?;

        let course_title_node =
            semester_node
                .next_sibling_element()
                .ok_or(CourseBasicDataError::NodeParseError(
                    "course title node not found".to_owned(),
                ))?;

        let mut course_titles = course_title_node
            .descendants()
            .filter(|n| element_has_name(n, "translation"))
            .filter_map(|n| n.text());
        let name_de: &str = course_titles
            .next()
            .ok_or(CourseBasicDataError::NodeParseError(
                "no german name found".to_owned(),
            ))?;
        let name_en: &str = course_titles
            .next()
            .ok_or(CourseBasicDataError::NodeParseError(
                "no english name found".to_owned(),
            ))?;

        let course_type_node = resource_node
            .descendants()
            .filter(|n| element_has_name(n, "courseTypeDto"))
            .next()
            .ok_or(CourseBasicDataError::NodeParseError(
                "Node courseTypeDto not found".to_owned(),
            ))?;

        let course_type = course_type_node
            .descendants()
            .filter(|n| element_has_name(n, "key"))
            .find_map(|n| n.text())
            .ok_or(CourseBasicDataError::NodeParseError(
                "No course type key found".to_owned(),
            ))?;

        let course_basic_data = CourseBasicData {
            id: id.to_owned(),
            course_type: course_type.to_owned(),
            name_de: name_de.to_owned(),
            name_en: name_en.to_owned(),
            semester: semester.to_owned(),
        };
        Ok(course_basic_data)
    }
}

impl CourseBasicData {
    fn read_all_data_from_page(xml: String) -> Result<Vec<CourseBasicData>, CourseBasicDataError> {
        let document = Document::parse(&xml)?;

        let mut result = vec![];
        let mut some_resource_element = document
            .root_element()
            .descendants()
            .filter(|n| element_has_name(n, "resource"))
            .next();
        // println!("{:#?}", some_resource_element.unwrap().tag_name().name());
        while let Some(resource_element) = some_resource_element {
            let basic_data = Self::try_from(resource_element)?;
            println!("{:#?}", basic_data);
            result.push(basic_data);
            some_resource_element = resource_element.next_sibling_element();
        }
        // println!("{:#?}", result.len());
        Ok(result)
    }

    pub async fn get_all() -> Result<(), CourseBasicDataError> {
        let mut conn = db::db_setup::connection()
            .expect("should be able to connect to database for course basic data");

        let mut page = 0;
        loop {
            let mut request_url = env::var("BASE_URL_IDS")
                .expect("BASE_URL_IDS should exist in environment variables");
            request_url.push_str(&(page * 100).to_string());
            println!("url {:#?}", request_url);
            let request_result = reqwest::get(request_url).await?;
            let mut file = File::create("foo.txt").unwrap();
            let xml = request_result.text().await?;
            file.write_all(&xml.as_bytes()).unwrap();
            let basic_data = Self::read_all_data_from_page(xml)?;
            if basic_data.is_empty() {
                break;
            }
            page += 1;
        }
        Ok(())
    }
}
