use std::env;

use crate::utils::element_has_name;
use crate::{db, schema::curriculum};
use anyhow::Result;
use diesel::RunQueryDsl;
use diesel::{deserialize::Queryable, prelude::Insertable};
use roxmltree::{Document, Node};
use thiserror::Error;

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = curriculum)]
pub struct Curriculum {
    pub id: String,
    pub en: String,
    pub de: String,
}

#[derive(Error, Debug)]
pub enum CurriculumError {
    #[error("Failed to parse curriculum: {0}")]
    NodeParseError(String),
    #[error("Failed while requesting curriculum: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed while requesting curriculum: {0}")]
    DocumentParseError(#[from] roxmltree::Error),
    #[error("Failed to get the correct element in curriculum xml: {0}")]
    DocumentIncomplete(String),
    #[error("Failed to insert the element into curriculum table")]
    DbInsertError(#[from] diesel::result::Error),
}

impl TryFrom<Node<'_, '_>> for Curriculum {
    type Error = CurriculumError;
    fn try_from(value: Node<'_, '_>) -> Result<Self, Self::Error> {
        let id = value
            .descendants()
            .filter(|n| element_has_name(n, "id"))
            .find_map(|n| n.text())
            .ok_or(CurriculumError::NodeParseError(
                "No id found in node".to_owned(),
            ))?;
        let mut language_iter = value
            .descendants()
            .filter(|n| element_has_name(n, "translation"))
            .filter_map(|n| n.text());
        let de = language_iter.next().ok_or(CurriculumError::NodeParseError(
            "No german language node found".to_owned(),
        ))?;
        let en: &str = language_iter.next().ok_or(CurriculumError::NodeParseError(
            "No english language node found".to_owned(),
        ))?;
        let curriculum = Curriculum::new(id, de, en);
        Ok(curriculum)
    }
}

impl Curriculum {
    pub fn new(id: &str, de: &str, en: &str) -> Self {
        Curriculum {
            id: id.to_string(),
            en: en.to_string(),
            de: de.to_string(),
        }
    }

    pub async fn get_all() -> Result<(), CurriculumError> {
        use crate::schema::curriculum::dsl::*;

        let mut conn = db::db_setup::connection()
            .expect("should be able to connect to database for curriculum");

        let request_url = env::var("CURRICULUM_URL")
            .expect("CURRICULUM_URL should exist in environment variables");
        let request_result = reqwest::get(request_url).await?;
        let xml = request_result.text().await?;
        let document = Document::parse(&xml)?;
        let mut some_resource_element = document.root_element().first_element_child();
        while let Some(resource_element) = some_resource_element {
            let curr = Curriculum::try_from(resource_element)?;
            diesel::insert_into(curriculum)
                .values(&curr)
                .execute(&mut conn)?;
            some_resource_element = resource_element.next_sibling_element();
        }

        Ok(())
    }
}
