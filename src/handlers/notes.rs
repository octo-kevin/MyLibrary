//! Reading note management HTTP handlers
//! 
//! Provides RESTful API endpoints for note CRUD operations

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::note::{ReadingNote, CreateNoteRequest, UpdateReadingNote, NoteListResponse};

/// Query parameters for note listing
#[derive(Debug, Deserialize, IntoParams)]
pub struct NoteListQuery {
    /// Page number (1-based, default: 1)
    #[param(example = 1)]
    pub page: Option<u32>,
    /// Items per page (default: 20, max: 100)
    #[param(example = 20)]
    pub per_page: Option<u32>,
    /// Search query for title/content
    #[param(example = "important")]
    pub search: Option<String>,
    /// Filter by note type (quote, summary, thought, general)
    #[param(example = "quote")]
    pub note_type: Option<String>,
}

/// Path parameters for note operations
#[derive(Debug, Deserialize, IntoParams)]
pub struct NotePath {
    /// Note ID
    #[param(example = 1)]
    pub id: i64,
}

/// Path parameters for book's notes
#[derive(Debug, Deserialize, IntoParams)]
pub struct BookNotesPath {
    /// Book ID
    #[param(example = 1)]
    pub book_id: i64,
}

/// Creates a new reading note
#[utoipa::path(
    post,
    path = "/api/notes",
    request_body = CreateNoteRequest,
    responses(
        (status = 201, description = "Note created successfully", body = NoteResponse),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn create_note(
    pool: web::Data<DbPool>,
    note_data: web::Json<CreateNoteRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate required fields
    if note_data.content.trim().is_empty() {
        return Err(AppError::ValidationError("Content is required".to_string()));
    }

    let new_note = note_data.into_inner();
    let tags = new_note.tags.clone();
    let note = ReadingNote::create(&mut conn, new_note.into())?;
    
    // Set tags if provided
    if let Some(tag_names) = tags {
        note.set_tags(&mut conn, tag_names)?;
    }
    
    let response = note.to_response(&mut conn)?;

    Ok(HttpResponse::Created().json(response))
}

/// Gets a note by ID
#[utoipa::path(
    get,
    path = "/api/notes/{id}",
    params(NotePath),
    responses(
        (status = 200, description = "Note found", body = NoteResponse),
        (status = 404, description = "Note not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn get_note(
    pool: web::Data<DbPool>,
    path: web::Path<NotePath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    let note = ReadingNote::find_by_id(&mut conn, path.id)?;
    let response = note.to_response(&mut conn)?;

    Ok(HttpResponse::Ok().json(response))
}

/// Lists notes with pagination and optional search
#[utoipa::path(
    get,
    path = "/api/notes",
    params(NoteListQuery),
    responses(
        (status = 200, description = "Notes retrieved successfully", body = NoteListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn list_notes(
    pool: web::Data<DbPool>,
    query: web::Query<NoteListQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate and set defaults for pagination
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100).max(1);

    let (notes, total) = ReadingNote::list_with_filters(
        &mut conn,
        query.search.as_deref(),
        query.note_type.as_deref(),
        page,
        per_page,
    )?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    let mut note_responses = Vec::new();
    
    for note in notes {
        note_responses.push(note.to_response(&mut conn)?);
    }

    let response = NoteListResponse {
        notes: note_responses,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Gets all notes for a specific book
#[utoipa::path(
    get,
    path = "/api/books/{book_id}/notes",
    params(BookNotesPath, NoteListQuery),
    responses(
        (status = 200, description = "Notes retrieved successfully", body = NoteListResponse),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn get_book_notes(
    pool: web::Data<DbPool>,
    path: web::Path<BookNotesPath>,
    query: web::Query<NoteListQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Verify book exists
    use crate::models::book::Book;
    Book::find_by_id(&mut conn, path.book_id)?;
    
    // Get notes for the book
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100).max(1);
    
    let (notes, total) = ReadingNote::find_by_book_id(&mut conn, path.book_id, page, per_page)?;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let mut note_responses = Vec::new();
    for note in notes {
        note_responses.push(note.to_response(&mut conn)?);
    }

    let response = NoteListResponse {
        notes: note_responses,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Updates a note
#[utoipa::path(
    put,
    path = "/api/notes/{id}",
    params(NotePath),
    request_body = UpdateReadingNote,
    responses(
        (status = 200, description = "Note updated successfully", body = NoteResponse),
        (status = 404, description = "Note not found", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn update_note(
    pool: web::Data<DbPool>,
    path: web::Path<NotePath>,
    update_data: web::Json<UpdateReadingNote>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate update data
    if let Some(ref content) = update_data.content {
        if content.trim().is_empty() {
            return Err(AppError::ValidationError("Content cannot be empty".to_string()));
        }
    }

    let note = ReadingNote::update(&mut conn, path.id, update_data.into_inner())?;
    let response = note.to_response(&mut conn)?;

    Ok(HttpResponse::Ok().json(response))
}

/// Updates tags for a note
#[utoipa::path(
    put,
    path = "/api/notes/{id}/tags",
    params(NotePath),
    request_body = Vec<String>,
    responses(
        (status = 200, description = "Tags updated successfully", body = NoteResponse),
        (status = 404, description = "Note not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn update_note_tags(
    pool: web::Data<DbPool>,
    path: web::Path<NotePath>,
    tags: web::Json<Vec<String>>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    let note = ReadingNote::find_by_id(&mut conn, path.id)?;
    note.set_tags(&mut conn, tags.into_inner())?;
    
    let response = note.to_response(&mut conn)?;

    Ok(HttpResponse::Ok().json(response))
}

/// Soft deletes a note
#[utoipa::path(
    delete,
    path = "/api/notes/{id}",
    params(NotePath),
    responses(
        (status = 204, description = "Note deleted successfully"),
        (status = 404, description = "Note not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Notes"
)]
pub async fn delete_note(
    pool: web::Data<DbPool>,
    path: web::Path<NotePath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    ReadingNote::soft_delete(&mut conn, path.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    // Unit tests can be added here
}