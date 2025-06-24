use chrono::{NaiveDate, DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use crate::db::schema::reading_status;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = reading_status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReadingStatus {
    pub id: i64,
    pub book_id: i64,
    pub status: String,
    pub rating: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub finish_date: Option<NaiveDate>,
    pub current_page: Option<i32>,
    pub reading_progress: Option<BigDecimal>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = reading_status)]
pub struct NewReadingStatus {
    pub book_id: i64,
    pub status: String,
    pub rating: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub finish_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = reading_status)]
pub struct UpdateReadingStatus {
    pub status: Option<String>,
    pub rating: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub finish_date: Option<NaiveDate>,
    pub current_page: Option<i32>,
    pub reading_progress: Option<BigDecimal>,
}