use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::reading_notes;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = reading_notes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadingNote {
    pub id: i64,
    pub book_id: i64,
    pub title: Option<String>,
    pub content: String,
    pub note_type: Option<String>,
    pub page_reference: Option<String>,
    pub is_favorite: Option<bool>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = reading_notes)]
pub struct NewNote {
    pub book_id: i64,
    pub title: Option<String>,
    pub content: String,
    pub note_type: Option<String>,
    pub page_reference: Option<String>,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = reading_notes)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub note_type: Option<String>,
    pub page_reference: Option<String>,
    pub is_favorite: Option<bool>,
}