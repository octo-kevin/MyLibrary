use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::db::schema::{reading_notes, note_tags};
use crate::errors::{AppError, Result};

/// Note type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum NoteType {
    Quote,      // 摘录
    Summary,    // 总结
    Thought,    // 感想
    General,    // 一般笔记
}

impl From<String> for NoteType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "quote" => NoteType::Quote,
            "summary" => NoteType::Summary,
            "thought" => NoteType::Thought,
            _ => NoteType::General,
        }
    }
}

impl From<NoteType> for String {
    fn from(note_type: NoteType) -> Self {
        match note_type {
            NoteType::Quote => "quote".to_string(),
            NoteType::Summary => "summary".to_string(),
            NoteType::Thought => "thought".to_string(),
            NoteType::General => "general".to_string(),
        }
    }
}

/// Reading note database model
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
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

/// New reading note for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = reading_notes)]
pub struct NewReadingNote {
    pub book_id: i64,
    pub title: Option<String>,
    pub content: String,
    pub note_type: Option<String>,
    pub page_reference: Option<String>,
    pub is_favorite: Option<bool>,
}

/// Update reading note structure
#[derive(Debug, Deserialize, AsChangeset, Default, ToSchema)]
#[diesel(table_name = reading_notes)]
pub struct UpdateReadingNote {
    #[schema(example = "Updated Note Title")]
    pub title: Option<String>,
    
    #[schema(example = "Updated note content in Markdown")]
    pub content: Option<String>,
    
    #[schema(example = "summary")]
    pub note_type: Option<String>,
    
    #[schema(example = "Pages 10-20")]
    pub page_reference: Option<String>,
    
    #[schema(example = true)]
    pub is_favorite: Option<bool>,
}

/// Request structure for creating a new reading note
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateNoteRequest {
    #[schema(example = 1)]
    pub book_id: i64,
    
    #[schema(example = "Chapter 1 Summary")]
    pub title: Option<String>,
    
    #[schema(example = "This chapter introduces the main concepts...")]
    pub content: String,
    
    #[schema(example = "summary")]
    pub note_type: Option<NoteType>,
    
    #[schema(example = "Pages 1-15")]
    pub page_reference: Option<String>,
    
    #[schema(example = false)]
    pub is_favorite: Option<bool>,
    
    #[schema(example = json!(["important", "chapter1"]))]
    pub tags: Option<Vec<String>>,
}

/// Response structure for reading note
#[derive(Debug, Serialize, ToSchema)]
pub struct NoteResponse {
    #[schema(example = 1)]
    pub id: i64,
    
    #[schema(example = 1)]
    pub book_id: i64,
    
    #[schema(example = "Chapter 1 Summary")]
    pub title: Option<String>,
    
    #[schema(example = "This chapter introduces the main concepts...")]
    pub content: String,
    
    #[schema(example = "summary")]
    pub note_type: Option<NoteType>,
    
    #[schema(example = "Pages 1-15")]
    pub page_reference: Option<String>,
    
    #[schema(example = false)]
    pub is_favorite: bool,
    
    #[schema(example = json!(["important", "chapter1"]))]
    pub tags: Vec<String>,
    
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,
    
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Paginated note list response
#[derive(Debug, Serialize, ToSchema)]
pub struct NoteListResponse {
    pub notes: Vec<NoteResponse>,
    
    #[schema(example = 50)]
    pub total: i64,
    
    #[schema(example = 1)]
    pub page: u32,
    
    #[schema(example = 20)]
    pub per_page: u32,
    
    #[schema(example = 3)]
    pub total_pages: u32,
}

impl From<CreateNoteRequest> for NewReadingNote {
    fn from(req: CreateNoteRequest) -> Self {
        Self {
            book_id: req.book_id,
            title: req.title,
            content: req.content,
            note_type: req.note_type.map(|nt| nt.into()),
            page_reference: req.page_reference,
            is_favorite: req.is_favorite,
        }
    }
}

impl ReadingNote {
    /// Creates a new reading note
    pub fn create(conn: &mut PgConnection, new_note: NewReadingNote) -> Result<ReadingNote> {
        use crate::db::schema::books;
        
        // Verify book exists and is not deleted
        let book_exists = books::table
            .filter(books::id.eq(new_note.book_id))
            .filter(books::deleted_at.is_null())
            .select(books::id)
            .first::<i64>(conn)
            .optional()?;
            
        if book_exists.is_none() {
            return Err(AppError::NotFound(format!("Book with id {} not found", new_note.book_id)));
        }
        
        diesel::insert_into(reading_notes::table)
            .values(&new_note)
            .returning(ReadingNote::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    /// Finds a note by ID (excluding soft deleted)
    pub fn find_by_id(conn: &mut PgConnection, note_id: i64) -> Result<ReadingNote> {
        reading_notes::table
            .filter(reading_notes::id.eq(note_id))
            .filter(reading_notes::deleted_at.is_null())
            .first(conn)
            .map_err(|_| AppError::NotFound(format!("Note with id {} not found", note_id)))
    }

    /// Finds all notes for a specific book
    pub fn find_by_book_id(
        conn: &mut PgConnection,
        book_id: i64,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ReadingNote>, i64)> {
        let offset = ((page.saturating_sub(1)) * per_page) as i64;
        
        let notes = reading_notes::table
            .filter(reading_notes::book_id.eq(book_id))
            .filter(reading_notes::deleted_at.is_null())
            .order(reading_notes::created_at.desc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<ReadingNote>(conn)?;

        let total = reading_notes::table
            .filter(reading_notes::book_id.eq(book_id))
            .filter(reading_notes::deleted_at.is_null())
            .count()
            .get_result::<i64>(conn)?;

        Ok((notes, total))
    }

    /// Lists all notes with pagination
    pub fn list_paginated(
        conn: &mut PgConnection,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ReadingNote>, i64)> {
        let offset = ((page.saturating_sub(1)) * per_page) as i64;
        
        let notes = reading_notes::table
            .filter(reading_notes::deleted_at.is_null())
            .order(reading_notes::created_at.desc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<ReadingNote>(conn)?;

        let total = reading_notes::table
            .filter(reading_notes::deleted_at.is_null())
            .count()
            .get_result::<i64>(conn)?;

        Ok((notes, total))
    }

    /// Searches notes by title or content
    pub fn search(
        conn: &mut PgConnection,
        query: &str,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ReadingNote>, i64)> {
        let search_pattern = format!("%{}%", query);
        let offset = ((page.saturating_sub(1)) * per_page) as i64;
        
        let notes = reading_notes::table
            .filter(reading_notes::deleted_at.is_null())
            .filter(
                reading_notes::title.ilike(&search_pattern)
                    .or(reading_notes::content.ilike(&search_pattern))
            )
            .order(reading_notes::created_at.desc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<ReadingNote>(conn)?;

        let total = reading_notes::table
            .filter(reading_notes::deleted_at.is_null())
            .filter(
                reading_notes::title.ilike(&search_pattern)
                    .or(reading_notes::content.ilike(&search_pattern))
            )
            .count()
            .get_result::<i64>(conn)?;

        Ok((notes, total))
    }

    /// Updates a note
    pub fn update(
        conn: &mut PgConnection,
        note_id: i64,
        update_data: UpdateReadingNote,
    ) -> Result<ReadingNote> {
        diesel::update(reading_notes::table.find(note_id))
            .filter(reading_notes::deleted_at.is_null())
            .set((
                &update_data,
                reading_notes::updated_at.eq(Some(Utc::now())),
            ))
            .returning(ReadingNote::as_returning())
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Note with id {} not found", note_id))
                }
                _ => AppError::from(e),
            })
    }

    /// Soft deletes a note
    pub fn soft_delete(conn: &mut PgConnection, note_id: i64) -> Result<()> {
        let affected = diesel::update(reading_notes::table.find(note_id))
            .filter(reading_notes::deleted_at.is_null())
            .set(reading_notes::deleted_at.eq(Some(Utc::now())))
            .execute(conn)?;

        if affected == 0 {
            return Err(AppError::NotFound(format!("Note with id {} not found", note_id)));
        }

        // Also remove tag associations
        diesel::delete(note_tags::table.filter(note_tags::note_id.eq(note_id)))
            .execute(conn)?;

        Ok(())
    }

    /// Gets tags associated with this note
    pub fn get_tags(&self, conn: &mut PgConnection) -> Result<Vec<String>> {
        use crate::db::schema::tags;
        
        let tag_names = note_tags::table
            .inner_join(tags::table)
            .filter(note_tags::note_id.eq(self.id))
            .filter(tags::deleted_at.is_null())
            .select(tags::name)
            .load::<String>(conn)?;

        Ok(tag_names)
    }

    /// Associates tags with this note
    pub fn set_tags(&self, conn: &mut PgConnection, tag_names: Vec<String>) -> Result<()> {
        use crate::models::tag::Tag;
        
        // Start a transaction
        conn.transaction(|conn| {
            // Remove existing associations
            diesel::delete(note_tags::table.filter(note_tags::note_id.eq(self.id)))
                .execute(conn)?;

            if tag_names.is_empty() {
                return Ok(());
            }

            // Get or create tags
            let mut tag_ids = Vec::new();
            for tag_name in tag_names {
                let tag = Tag::find_or_create(conn, tag_name)?;
                tag_ids.push(tag.id);
            }

            // Create new associations
            let new_associations: Vec<_> = tag_ids
                .into_iter()
                .map(|tag_id| (note_tags::note_id.eq(self.id), note_tags::tag_id.eq(tag_id)))
                .collect();

            diesel::insert_into(note_tags::table)
                .values(&new_associations)
                .execute(conn)?;

            Ok(())
        })
    }
}

/// Converts ReadingNote to NoteResponse with tags
impl ReadingNote {
    pub fn to_response(&self, conn: &mut PgConnection) -> Result<NoteResponse> {
        let tags = self.get_tags(conn).unwrap_or_default();
        
        Ok(NoteResponse {
            id: self.id,
            book_id: self.book_id,
            title: self.title.clone(),
            content: self.content.clone(),
            note_type: self.note_type.as_ref().map(|t| NoteType::from(t.clone())),
            page_reference: self.page_reference.clone(),
            is_favorite: self.is_favorite.unwrap_or(false),
            tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_type_conversion() {
        assert_eq!(String::from(NoteType::Quote), "quote");
        assert_eq!(String::from(NoteType::Summary), "summary");
        assert_eq!(String::from(NoteType::Thought), "thought");
        assert_eq!(String::from(NoteType::General), "general");
        
        assert_eq!(NoteType::from("quote".to_string()), NoteType::Quote);
        assert_eq!(NoteType::from("invalid".to_string()), NoteType::General);
    }
}