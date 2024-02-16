use crate::schema::course;
use chrono::NaiveTime;
use diesel::{deserialize::Queryable, prelude::Insertable};

#[derive(Debug, Clone, Insertable, Queryable, PartialEq, Eq)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub weekday: String,
    pub subject: String,
    pub course_type: String,
    pub semester: String,
}
