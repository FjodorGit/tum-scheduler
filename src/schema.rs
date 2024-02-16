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
    }
}
