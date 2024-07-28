use diesel::prelude::{Insertable, Queryable};

use crate::schema::*;

#[derive(Debug, Queryable)]
pub struct DbCapture {
    pub id: i32,
    pub uuid: uuid::Uuid,
    pub url: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub owner: i32,
    pub public: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = captures)]
pub struct InsCapture {
    pub uuid: uuid::Uuid,
    pub url: String,
    pub time: chrono::NaiveDateTime,
    pub owner: i32,
    pub public: bool,
}

#[derive(Debug, Queryable)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub passhash: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct InsUser {
    pub name: String,
    pub passhash: String,
}
