// @generated automatically by Diesel CLI.

diesel::table! {
    curriculum (id) {
        id -> Varchar,
        name_en -> Varchar,
        name_de -> Varchar,
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
    }
}

diesel::joinable!(lecture -> curriculum (curriculum));

diesel::allow_tables_to_appear_in_same_query!(
    curriculum,
    lecture,
);
