use std::io::Write;
use std::{fmt::write, fs::File};

use reqwest::{self, Request};
use roxmltree::{Attribute, Document, Node};
use tokio;

const IDS_REQUEST_URL: &str = "https://campus.tum.de/tumonline/ee/rest/slc.tm.cp/student/courses?$filter=courseNormKey-eq=LVEAB;orgId-eq=1;termId-eq=199&$orderBy=title=ascnf&$skip={}&$top={}";

// use paging mechnism to get course ids then use allCurriculum to get type of course
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut valid_ids: Vec<i32> = vec![];
    Ok(())
}

fn filter_ids(courses_xml: &String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let document = Document::parse(courses_xml).expect("Returned XML should be valid");
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
