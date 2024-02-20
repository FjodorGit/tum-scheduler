use std::env;

use crate::schema::curriculum;
use anyhow::Result;
use diesel::RunQueryDsl;
use diesel::{deserialize::Queryable, prelude::Insertable};
use roxmltree::{Document, Node};

use super::{TumApiError, TumXmlError, TumXmlNode};

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = curriculum)]
pub struct Curriculum {
    pub id: String,
    pub name_de: String,
    pub name_en: String,
}

pub struct CurriculumEndpoint {
    pub base_request_url: String,
}

impl TryFrom<TumXmlNode<'_, '_>> for Curriculum {
    type Error = TumXmlError;
    fn try_from(resource_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let id = resource_node.get_text_of_next("id")?;
        let (name_de, name_en) = resource_node.get_translations()?;
        let curriculum = Curriculum {
            id,
            name_de,
            name_en,
        };
        Ok(curriculum)
    }
}

impl Curriculum {
    fn read_all_from_page(xml: String) -> Result<Vec<Curriculum>, TumApiError> {
        let mut curricula: Vec<Curriculum> = vec![];
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode(document.root_element());
        for resource_element in root_element.resource_elements() {
            let appointment = Curriculum::try_from(resource_element)?;
            curricula.push(appointment);
        }
        Ok(curricula)
    }
}

impl CurriculumEndpoint {
    pub fn new() -> Self {
        let base_request_url = env::var("CURRICULUM_URL")
            .expect("APPOINTMENT_URL should exist in environment variables");
        CurriculumEndpoint { base_request_url }
    }

    pub async fn get_all(&self, course_id: String) -> Result<Vec<Curriculum>, TumApiError> {
        println!("Requesting appointement for {}", course_id);
        let request_result = reqwest::get(&self.base_request_url).await?;
        let xml: String = request_result.text().await?;
        Curriculum::read_all_from_page(xml)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::curriculum::Curriculum;

    #[test]
    fn test_reading_curricula() {
        let test_xml: String = fs::read_to_string("test_xmls/curricula.xml")
            .expect("Should be able to read curricula test file");
        let curricula =
            Curriculum::read_all_from_page(test_xml).expect("should be able to read curricula");
        println!("{:#?}", curricula);
        assert_eq!(curricula.len(), 1148);
    }
}
