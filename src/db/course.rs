use crate::schema::course;
use diesel::{deserialize::Queryable, prelude::Insertable};

#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Eq)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: i32,
    pub subject: String,
    pub semester: String,
}
