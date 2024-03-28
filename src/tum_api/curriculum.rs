use std::env;

use crate::{db_setup::DbError, schema::curriculum};
use anyhow::Result;
use diesel::{
    deserialize::Queryable, prelude::Insertable, result, ExpressionMethods, PgConnection, QueryDsl,
    RunQueryDsl,
};
use roxmltree::Document;

use super::{tum_xml_node::TumXmlNode, ScraperError, TumXmlError};

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = curriculum)]
pub struct CurriculumFromXml {
    pub id: String,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
}

pub struct CurriculumEndpoint {
    pub base_request_url: String,
}

impl CurriculumFromXml {
    fn build(resource_node: TumXmlNode<'_, '_>, semester: &str) -> Result<Self, TumXmlError> {
        let id = resource_node.get_text_of_next("id")?;
        let (name_de, name_en) = resource_node.get_translations()?;
        let curriculum = CurriculumFromXml {
            id,
            name_de,
            name_en,
            semester: semester.into(),
        };
        Ok(curriculum)
    }
    fn read_all_from_page(
        xml: String,
        semester: &str,
    ) -> Result<Vec<CurriculumFromXml>, ScraperError> {
        let mut curricula: Vec<CurriculumFromXml> = vec![];
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode::new(document.root_element());
        for resource_element in root_element.resource_elements() {
            let appointment = CurriculumFromXml::build(resource_element, semester)?;
            curricula.push(appointment);
        }
        Ok(curricula)
    }

    pub fn db_insert(conn: &mut PgConnection, curricula: Vec<Self>) -> Result<(), result::Error> {
        use crate::schema::curriculum::dsl::*;

        diesel::insert_into(curriculum)
            .values(curricula)
            .on_conflict_do_nothing()
            .execute(conn)?;

        Ok(())
    }

    pub fn db_get_all(
        conn: &mut PgConnection,
        semester_id: &str,
    ) -> Result<Vec<CurriculumFromXml>, DbError> {
        use crate::schema::curriculum::dsl::*;

        let curricula = curriculum
            .filter(semester.eq(semester_id))
            .load(conn)
            .map_err(|e| DbError::QueryError(e.to_string()))?;
        Ok(curricula)
    }
}

impl CurriculumEndpoint {
    pub fn new() -> Self {
        let base_request_url = env::var("CURRICULUM_URL")
            .expect("CURRICULUM_URL should exist in environment variables");
        CurriculumEndpoint { base_request_url }
    }

    pub async fn get_all(&self, semester: &str) -> Result<Vec<CurriculumFromXml>, ScraperError> {
        let request_url = format!("{}/{}", self.base_request_url, semester);
        let request_result = reqwest::get(request_url).await?;
        let xml: String = request_result.text().await?;
        CurriculumFromXml::read_all_from_page(xml, semester)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::curriculum::CurriculumFromXml;

    #[test]
    fn test_reading_curricula() {
        let test_xml: String = fs::read_to_string("test_xmls/curricula.xml")
            .expect("Should be able to read curricula test file");
        let curricula = CurriculumFromXml::read_all_from_page(test_xml, "199")
            .expect("should be able to read curricula");
        println!("{:#?}", curricula);
        assert_eq!(curricula.len(), 1148);
    }
}
