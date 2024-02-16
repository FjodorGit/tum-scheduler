// @generated automatically by Diesel CLI.

diesel::table! {
    course (id, start_time) {
        id -> Varchar,
        start_time -> Time,
        end_time -> Time,
        weekday -> Varchar,
        subject -> Varchar,
        course_type -> Varchar,
        semester -> Varchar,
        curriculum -> Varchar,
    }
}

diesel::table! {
    curriculum (id) {
        id -> Varchar,
        de -> Varchar,
        en -> Varchar,
    }
}

diesel::joinable!(course -> curriculum (curriculum));

diesel::allow_tables_to_appear_in_same_query!(
    course,
    curriculum,
);
