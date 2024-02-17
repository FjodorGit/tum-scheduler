use std::io::prelude::*;
use std::{env, fs::File};

use roxmltree::Document;

use crate::{db, utils::element_has_name};

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
    fn read_all_data_from_page(xml: String) -> Result<Vec<Course>, TumApiError> {
        let document = Document::parse(&xml)?;

        let mut result = vec![];
        let mut some_resource_element = document
            .root_element()
            .descendants()
            .filter(|n| element_has_name(n, "resource"))
            .next();
        // println!("{:#?}", some_resource_element.unwrap().tag_name().name());
        while let Some(resource_element) = some_resource_element {
            let basic_data = Self::try_from(TumXmlNode(resource_element))?;
            println!("{:#?}", basic_data);
            result.push(basic_data);
            some_resource_element = resource_element.next_sibling_element();
        }
        // println!("{:#?}", result.len());
        Ok(result)
    }

    pub async fn get_all() -> Result<(), TumApiError> {
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
