use diesel::prelude::{Insertable, Queryable};

use crate::schema::*;

#[derive(Queryable)]
pub struct DbCapture {
    id: i32,
    uuid: uuid::Uuid,
    url: String,
    time: chrono::DateTime<chrono::Utc>,
    owner: i32,
    public: bool,
}

#[derive(Insertable)]
#[diesel(table_name = captures)]
pub struct InsCapture {
    uuid: uuid::Uuid,
    url: String,
    time: chrono::NaiveDateTime,
    owner: i32,
    public: bool,
}

#[derive(Queryable)]
pub struct DbUser {
    id: i32,
    name: String,
    passhash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsUser {
    name: String,
    passhash: String,
}
