use diesel::{Insertable, Queryable};

use super::schema::*;

#[derive(Queryable)]
pub struct DbCapture {
    pub id: i32,
    pub uuid: uuid::Uuid,
    pub url: url::Url,
    pub time: chrono::DateTime<chrono::Utc>,
    pub owner: i32,
    pub public: bool,
}

#[derive(Insertable)]
#[diesel(table_name = captures)]
pub struct InsCapture {
    pub uuid: uuid::Uuid,
    pub url: String,
    pub time: chrono::NaiveDateTime,
    pub owner: i32,
    pub public: bool,
}

#[derive(Queryable)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub passhash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsUser {
    pub name: String,
    pub passhash: String,
}
