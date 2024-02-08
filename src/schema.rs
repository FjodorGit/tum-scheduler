// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        id -> Int4,
        subject -> Varchar,
        #[max_length = 255]
        semester -> Varchar,
    }
}
