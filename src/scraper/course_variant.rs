use std::env;

use reqwest::Client;
use roxmltree::Document;

use super::{tum_xml_node::TumXmlNode, ScraperError, TumXmlError};

#[derive(Debug)]
pub struct CourseVariantFromXml {
    pub curriculum: String,
    pub subject: String,
}

#[derive(Debug)]
pub struct CourseVariantEndpoint {
    pub client: Client,
    pub base_request_url: String,
    pub request_url_end: String,
}

impl TryFrom<TumXmlNode<'_, '_>> for CourseVariantFromXml {
    type Error = TumXmlError;
    fn try_from(resource_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let curriculum = resource_node.get_text_of_next("curriculumVersionId")?;
        let first_back_node = resource_node.get_next("back")?;
        let designation = first_back_node.get_text_of_next("designation")?;
        let variant = CourseVariantFromXml {
            curriculum,
            subject: designation,
        };
        Ok(variant)
    }
}

impl CourseVariantFromXml {
    fn read_all_from_page(xml: String) -> Result<Vec<CourseVariantFromXml>, ScraperError> {
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode::new(document.root_element());

        let mut variants = vec![];
        for resource_elem in root_element.resource_elements() {
            let variant = Self::try_from(resource_elem)?;
            variants.push(variant);
        }
        Ok(variants)
    }

    pub fn empty_variant(course_count: usize) -> Vec<CourseVariantFromXml> {
        vec![Self {
            curriculum: "0000".to_string(),
            subject: format!("XX{}", course_count),
        }]
    }
}

impl CourseVariantEndpoint {
    pub fn new() -> Self {
        let base_request_url = env::var("COURSE_VARIANTS_URL")
            .expect("COURSE_VARIANT_URL should exist in environment variables");
        let request_url_end = "/course/allCurriculumPositions".to_string();
        let client = reqwest::Client::new();
        Self {
            client,
            base_request_url,
            request_url_end,
        }
    }
    pub async fn get_all_by_id(&self, id: &str) -> Result<Vec<CourseVariantFromXml>, ScraperError> {
        let request_url = format!("{}{}{}", self.base_request_url, id, self.request_url_end);
        let request_result = self
            .client
            .get(request_url)
            .header("Accept", "application/xml")
            .send()
            .await?;
        let xml: String = request_result.text().await?;
        // println!("Xml: {:#?}", xml);
        CourseVariantFromXml::read_all_from_page(xml)
    }
}
#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::fs;

    use crate::scraper::course_variant::{CourseVariantEndpoint, CourseVariantFromXml};

    #[test]
    fn test_reading_variants() {
        let test_xml: String = fs::read_to_string("test_xmls/course_variants.xml")
            .expect("Should be able to read course variant test file");
        let variants = CourseVariantFromXml::read_all_from_page(test_xml)
            .expect("should be able to read variants");
        assert_eq!(variants.len(), 26);
    }

    #[test]
    fn test_reading_variants_other() {
        let test_xml: String = fs::read_to_string("test_xmls/course_variants2.xml")
            .expect("Should be able to read course variant test file");
        let variants = CourseVariantFromXml::read_all_from_page(test_xml)
            .expect("should be able to read variants");
        assert_eq!(variants.len(), 11);
    }

    #[tokio::test]
    async fn test_requesting_variants() {
        dotenv().ok();
        dotenv::from_filename("request_urls").ok();

        let variant_endpoint = CourseVariantEndpoint::new();
        let variants = variant_endpoint
            .get_all_by_id("950701818")
            .await
            .expect("should be able to aquire variants");
        assert_eq!(variants.len(), 13);
    }
}
