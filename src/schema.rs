// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "processing_error"))]
    pub struct ProcessingError;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProcessingError;

    course (id) {
        id -> Varchar,
        course_type -> Varchar,
        sws -> Float8,
        name_en -> Varchar,
        name_de -> Varchar,
        semester -> Varchar,
        processing_error -> ProcessingError,
    }
}

diesel::table! {
    curriculum (id) {
        id -> Varchar,
        name_en -> Varchar,
        name_de -> Varchar,
        semester -> Varchar,
    }
}

diesel::table! {
    lecture (id, start_time, weekday, curriculum) {
        id -> Varchar,
        start_time -> Time,
        end_time -> Time,
        weekday -> Varchar,
        subject -> Varchar,
        course_type -> Varchar,
        name_en -> Varchar,
        name_de -> Varchar,
        semester -> Varchar,
        curriculum -> Varchar,
        description -> Varchar,
        organization -> Varchar,
        ects -> Float8,
    }
}

diesel::table! {
    organization (id) {
        id -> Varchar,
        name -> Varchar,
        parent -> Varchar,
        kind -> Varchar,
    }
}

diesel::joinable!(lecture -> curriculum (curriculum));
diesel::joinable!(lecture -> organization (organization));

diesel::allow_tables_to_appear_in_same_query!(
    course,
    curriculum,
    lecture,
    organization,
);
