use diesel::{result::Error, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use futures::executor::block_on;
use meilisearch_sdk::{client::Client, errors, settings::Settings};

use lazy_static::lazy_static;

use crate::{db_setup::connection, scraper::lecture::Lecture};

lazy_static! {
    static ref CLIENT: Client = Client::new("http://localhost:7700", Some("masterKey")).unwrap();
}

// const MEILISEARCH_URL: &str = "http://localhost:7700";
// const MEILISEARCH_MASTER_KEY: &str = "Pr0lOM2R4OPuOA0ueFLZazNbIKd08go3ujB3ipSkH9U";

fn get_unique_representative_for_each_subject(
    conn: &mut PgConnection,
) -> Result<Vec<Lecture>, Error> {
    use crate::schema::lecture::dsl::*;
    lecture
        .order_by((subject.asc(), course_type.desc()))
        .distinct_on(subject)
        .load(conn)
}

pub async fn init_melisearch() -> Result<(), errors::Error> {
    block_on(async move {
        let conn = &mut connection().expect("should be able to establish connection");
        let lecture_representatives = get_unique_representative_for_each_subject(conn)
            .expect("should be able to get all courses");

        let settings = Settings::new().with_searchable_attributes(&["name_de", "name_en"]);

        // add the settings to the index
        CLIENT
            .index("lectures")
            .set_settings(&settings)
            .await
            .unwrap()
            .wait_for_completion(&CLIENT, None, None)
            .await
            .unwrap();

        CLIENT
            .index("lectures")
            .add_or_update(&lecture_representatives, Some("id"))
            .await
            .unwrap()
            .wait_for_completion(&CLIENT, None, None)
            .await
            .unwrap();
    });
    Ok(())
}
