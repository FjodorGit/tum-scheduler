use super::{tum_xml_node::TumXmlNode, ScraperError, TumXmlError};
use crate::schema::{self, course};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::query_dsl::methods::{DistinctDsl, SelectDsl};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{prelude::Insertable, Queryable};
use diesel::{result, PgConnection, RunQueryDsl};
use reqwest::Client;
use roxmltree::Document;
use std::env;
use std::io::Write;
use tracing::info;

#[derive(Debug, PartialEq, Clone, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = schema::sql_types::ProcessingError)]
pub enum ProcessingError {
    None,
    MissingDescription,
    MissingOrganization,
    MissingVariants,
    MissingAppointments,
}

impl ToSql<schema::sql_types::ProcessingError, Pg> for ProcessingError {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ProcessingError::None => out.write_all(b"None")?,
            ProcessingError::MissingDescription => out.write_all(b"MissingDescription")?,
            ProcessingError::MissingOrganization => out.write_all(b"MissingOrganization")?,
            ProcessingError::MissingVariants => out.write_all(b"MissingVariants")?,
            ProcessingError::MissingAppointments => out.write_all(b"MissingAppointments")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::ProcessingError, Pg> for ProcessingError {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"None" => Ok(ProcessingError::None),
            b"MissingAppointments" => Ok(ProcessingError::MissingAppointments),
            b"MissingVariants" => Ok(ProcessingError::MissingVariants),
            b"MissingOrganization" => Ok(ProcessingError::MissingOrganization),
            b"MissingDescription" => Ok(ProcessingError::MissingDescription),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = course)]
pub struct CourseFromXml {
    pub id: String,
    pub course_type: String,
    pub sws: f64,
    pub name_en: String,
    pub name_de: String,
    pub semester: String,
    pub processing_error: ProcessingError,
}

#[derive(Debug)]
pub struct CourseEndpoint {
    pub base_request_url: String,
    pub current_page: usize,
    pub client: Client,
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
            processing_error: ProcessingError::None,
        };
        Ok(course_basic_data)
    }
}

impl CourseFromXml {
    fn read_all_from_page(xml: String) -> Result<Vec<CourseFromXml>, ScraperError> {
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

    pub fn add_to_db(&self, conn: &mut PgConnection) -> Result<usize, result::Error> {
        diesel::insert_into(course::table)
            .values(self)
            .execute(conn)
    }
}

impl CourseEndpoint {
    pub fn for_semester(semester_id: &str) -> Self {
        let base_url = env::var("BASE_COURSES_URL")
            .expect("BASE_COURSES_URL should exist in environment variables");
        let base_request_url = format!("{}{}&$skip=", base_url, semester_id);
        let client = Client::new();
        Self {
            base_request_url,
            current_page: 0,
            client,
        }
    }

    pub async fn fetch_next_page(&mut self) -> Result<Vec<CourseFromXml>, ScraperError> {
        let request_url = format!("{}{}", self.base_request_url, self.current_page * 100);
        info!("Fetching from page {}", self.current_page);
        let mut request_result = self.client.get(&request_url).send().await;
        if request_result.is_err() {
            self.client = Client::new();
            request_result = self.client.get(request_url).send().await;
        }
        let xml = request_result?.text().await?;
        let courses = CourseFromXml::read_all_from_page(xml)?;
        self.current_page += 1;
        if courses.is_empty() {
            return Err(ScraperError::ZeroCoursesFound(self.current_page));
        }
        Ok(courses)
    }

    pub fn get_all_processed(conn: &mut PgConnection) -> Result<Vec<String>, result::Error> {
        course::table.select(course::id).load(conn)
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
