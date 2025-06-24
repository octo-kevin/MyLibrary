use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::tags;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub usage_count: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
    pub slug: String,
}