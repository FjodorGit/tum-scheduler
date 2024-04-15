use std::env;

use diesel::{
    deserialize::Queryable, prelude::Insertable, query_dsl::methods::SelectDsl, result,
    PgConnection, RunQueryDsl, Selectable,
};
use lazy_static::lazy_static;
use reqwest::Client;
use roxmltree::Document;

use crate::{db_setup::connection, schema::organization};

use super::{
    tum_xml_node::{TumXmlError, TumXmlNode},
    ScraperError,
};

lazy_static! {
    static ref ORGANIZATIONS: Vec<String> = {
        let conn: &mut PgConnection =
            &mut connection().expect("should be able to get connections for organizations");
        TumOrganization::get_all(conn).expect("should be able to get all organization ids")
    };
}

pub type TumOrganizationFromXml = String;

#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Selectable)]
#[diesel(table_name = organization)]
pub struct TumOrganization {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub kind: String,
}

pub struct TumOrganizationEndpoint {
    pub base_url: String,
    pub client: Client,
}

impl TumOrganization {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<String>, result::Error> {
        organization::table.select(organization::id).load(conn)
    }

    pub fn read_organization_id(
        document: Document,
    ) -> Result<Option<TumOrganizationFromXml>, TumXmlError> {
        let root_node = TumXmlNode::new(document.root_element());
        let organization_node = root_node.get_next("organisationResponsibleDto")?;
        let mut organization = organization_node.get_text_of_next("id")?;
        let parent_organization = organization_node.get_text_of_next("parentOrganisationId")?;

        if !ORGANIZATIONS.contains(&organization) && !ORGANIZATIONS.contains(&parent_organization) {
            return Ok(None);
        }

        if !ORGANIZATIONS.contains(&organization) && ORGANIZATIONS.contains(&parent_organization) {
            organization = parent_organization;
        }

        Ok(Some(organization))
    }
}

impl TumOrganizationEndpoint {
    pub fn new() -> Self {
        let base_url = env::var("ORGANIZATION_URL")
            .expect("ORGANIZATION_URL should exist in environment variables");
        let client = reqwest::Client::new();
        Self { base_url, client }
    }

    pub async fn get_organization(
        &self,
        course_id: &str,
    ) -> Result<Option<TumOrganizationFromXml>, ScraperError> {
        let course_url = format!("{}{}", self.base_url, course_id);
        let xml_response = self.client.get(course_url).send().await?.text().await?;
        let document = Document::parse(&xml_response)?;
        Ok(TumOrganization::read_organization_id(document)?)
    }
}

#[cfg(test)]
mod test {

    use dotenv::dotenv;

    use crate::scraper::organization::TumOrganizationEndpoint;

    #[tokio::test]
    async fn test_getting_course_organization() {
        dotenv().ok();
        dotenv::from_filename("request_urls").ok();

        let org_id = "950731629";
        let organization_endpoint = TumOrganizationEndpoint::new();
        let organization = organization_endpoint
            .get_organization(org_id)
            .await
            .expect("should be able to fetch course organization");
        assert_eq!("53219".to_string(), organization.unwrap());
    }
}
