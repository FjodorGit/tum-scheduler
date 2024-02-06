use std::io::Write;
use std::{fmt::write, fs::File};

use reqwest::{self, Request};
use roxmltree::Document;
use tokio;

// use paging mechnism to get course ids then use allCurriculum to get type of course
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let first_valid_id: i32 = 950695000;
    let mut course_id: i32 = first_valid_id;
    let mut previous_invalid = false;
    let mut valid_ids: Vec<i32> = vec![];
    loop {
        if id_is_valid(course_id).await? {
            println!("Found valid id: {:#?}", course_id);
            valid_ids.push(course_id);
            previous_invalid = false;
            course_id += 1;
            continue;
        }

        if previous_invalid {
            break;
        }

        previous_invalid = true;
        course_id += 1;
    }
    println!(
        "Found {:#?} valid id with {:#?} being the maximum valid id",
        valid_ids.len(),
        valid_ids.last().unwrap()
    );
    let file_creation = File::create("output.txt");
    let mut file = file_creation.expect("Should not fail to create file");
    for id in valid_ids {
        writeln!(file, "{:?}", id);
    }

    Ok(())
}

async fn id_is_valid(course_id: i32) -> Result<bool, reqwest::Error> {
    let request_url = format!(
        "https://campus.tum.de/tumonline/ee/rest/slc.tm.cp/student/courses/{}",
        course_id
    );
    let res = reqwest::get(request_url).await?;
    let body = res.text().await?;
    let parsing_result: Result<Document<'_>, roxmltree::Error> = Document::parse(body.as_str());
    Ok(parsing_result.is_ok())
}
