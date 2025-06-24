use chrono::{NaiveDate, DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::books;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i64,
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub page_count: Option<i32>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub page_count: Option<i32>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset, Default)]
#[diesel(table_name = books)]
pub struct UpdateBook {
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<NaiveDate>,
    pub page_count: Option<i32>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
}

impl Book {
    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
    
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}