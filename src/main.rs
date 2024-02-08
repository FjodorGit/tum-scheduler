use std::fmt::format;
use std::fs;
use std::io::Write;
use std::{fmt::write, fs::File};

use db::course::{self, Course};
use diesel::{Insertable, RunQueryDsl};
use dotenv::dotenv;
use reqwest::{self, Request};
use roxmltree::{Attribute, Document, Node};
use tokio;

pub mod db;
pub mod schema;

const IDS_REQUEST_URL: &str = "https://campus.tum.de/tumonline/ee/rest/slc.tm.cp/student/courses?$filter=courseNormKey-eq=LVEAB;orgId-eq=1;termId-eq=199&$orderBy=title=ascnf&$skip={}&$top=100";

const IDS_FILE_NAME: &str = "test_ids.txt";

// use paging mechnism to get course ids then use allCurriculum to get type of course
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let mut conn = db::db_setup::connection().expect("should be able to establish connection");
    let example_course = Course {
        id: 923329999,
        subject: "MA".to_string(),
        semester: "W2023".to_string(),
    };
    let _ = diesel::insert_into(schema::course::table)
        .values(example_course)
        .get_result::<Course>(&mut conn);
    Ok(())
}

async fn aquire_ids() -> Result<(), reqwest::Error> {
    let mut page = 0;
    let mut ids_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .truncate(true)
        .open(IDS_FILE_NAME)
        .expect("Should be able to open file");
    loop {
        let request_url = format!("https://campus.tum.de/tumonline/ee/rest/slc.tm.cp/student/courses?$filter=courseNormKey-eq=LVEAB;orgId-eq=1;termId-eq=199&$orderBy=title=ascnf&$skip={}&$top=100", page * 100);
        let request_result = reqwest::get(request_url).await?;
        let xml = request_result.text().await?;
        let ids = filter_ids(xml);
        if ids.is_empty() {
            break;
        }
        let mut ids_string = ids.join("\n");
        ids_string.push_str("\n");
        ids_file
            .write(ids_string.as_ref())
            .expect("Should be able to append ids to file");
        println!("Next page");
        page += 1;
    }
    Ok(())
}

async fn aquire_course_data(id: String) {}

fn filter_ids(courses_xml: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let document = Document::parse(&courses_xml).expect("Returned XML should be valid");
    let root = document.root_element();
    let mut maybe_resource_node: Option<Node> = root.first_element_child();
    for _ in 0..6 {
        maybe_resource_node = maybe_resource_node
            .expect("root should have link nodes")
            .next_sibling_element();
    }
    while let Some(resource_node) = maybe_resource_node {
        let link_node = resource_node
            .first_element_child()
            .expect("resource node should have link child node");
        let id: String = link_node
            .attribute("key")
            .expect("link node should have key attribute")
            .to_owned();
        println!("Found Id: {:#?}", id);
        result.push(id);
        maybe_resource_node = resource_node.next_sibling_element();
    }
    return result;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::filter_ids;

    #[test]
    fn test_filtering_ids() {
        let test_xml: String =
            fs::read_to_string("test.txt").expect("Should be able to read test file");
        let result = filter_ids(&test_xml);
        assert_eq!("950697421", result.last().unwrap());
    }
    #[test]
    fn test_filtering_no_ids() {
        let test_xml: String =
            fs::read_to_string("empty_xml.txt").expect("Should be able to read test file");
        let result = filter_ids(&test_xml);
        assert!(result.is_empty());
    }
}
