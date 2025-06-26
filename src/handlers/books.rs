//! Book management HTTP handlers
//! 
//! Provides RESTful API endpoints for book CRUD operations

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::book::{Book, CreateBookRequest, UpdateBook, BookResponse, BookListResponse};

/// Query parameters for book listing
#[derive(Debug, Deserialize, IntoParams)]
pub struct BookListQuery {
    /// Page number (1-based, default: 1)
    #[param(example = 1)]
    pub page: Option<u32>,
    /// Items per page (default: 20, max: 100)
    #[param(example = 20)]
    pub per_page: Option<u32>,
    /// Search query for title/author
    #[param(example = "rust")]
    pub search: Option<String>,
}

/// Path parameters for book operations
#[derive(Debug, Deserialize, IntoParams)]
pub struct BookPath {
    /// Book ID
    #[param(example = 1)]
    pub id: i64,
}

/// Creates a new book
#[utoipa::path(
    post,
    path = "/api/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created successfully", body = BookResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn create_book(
    pool: web::Data<DbPool>,
    book_data: web::Json<CreateBookRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate required fields
    if book_data.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title is required".to_string()));
    }
    if book_data.author.trim().is_empty() {
        return Err(AppError::ValidationError("Author is required".to_string()));
    }

    let new_book = book_data.into_inner().into();
    let book = Book::create(&mut conn, new_book)?;
    let response = BookResponse::from(book);

    Ok(HttpResponse::Created().json(response))
}

/// Gets a book by ID
#[utoipa::path(
    get,
    path = "/api/books/{id}",
    params(BookPath),
    responses(
        (status = 200, description = "Book found", body = BookResponse),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn get_book(
    pool: web::Data<DbPool>,
    path: web::Path<BookPath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    let book = Book::find_by_id(&mut conn, path.id)?;
    let response = BookResponse::from(book);

    Ok(HttpResponse::Ok().json(response))
}

/// Lists books with pagination and optional search
#[utoipa::path(
    get,
    path = "/api/books",
    params(BookListQuery),
    responses(
        (status = 200, description = "Books retrieved successfully", body = BookListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn list_books(
    pool: web::Data<DbPool>,
    query: web::Query<BookListQuery>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate and set defaults for pagination
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (books, total) = if let Some(ref search_query) = query.search {
        if search_query.trim().is_empty() {
            Book::list_paginated(&mut conn, page, per_page)?
        } else {
            Book::search(&mut conn, search_query.trim(), page, per_page)?
        }
    } else {
        Book::list_paginated(&mut conn, page, per_page)?
    };

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    let book_responses: Vec<BookResponse> = books.into_iter().map(BookResponse::from).collect();

    let response = BookListResponse {
        books: book_responses,
        total,
        page,
        per_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Updates a book
#[utoipa::path(
    put,
    path = "/api/books/{id}",
    params(BookPath),
    request_body = UpdateBook,
    responses(
        (status = 200, description = "Book updated successfully", body = BookResponse),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn update_book(
    pool: web::Data<DbPool>,
    path: web::Path<BookPath>,
    update_data: web::Json<UpdateBook>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // Validate update data
    if let Some(ref title) = update_data.title {
        if title.trim().is_empty() {
            return Err(AppError::ValidationError("Title cannot be empty".to_string()));
        }
    }
    if let Some(ref author) = update_data.author {
        if author.trim().is_empty() {
            return Err(AppError::ValidationError("Author cannot be empty".to_string()));
        }
    }

    let book = Book::update(&mut conn, path.id, update_data.into_inner())?;
    let response = BookResponse::from(book);

    Ok(HttpResponse::Ok().json(response))
}

/// Soft deletes a book
#[utoipa::path(
    delete,
    path = "/api/books/{id}",
    params(BookPath),
    responses(
        (status = 204, description = "Book deleted successfully"),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn delete_book(
    pool: web::Data<DbPool>,
    path: web::Path<BookPath>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    Book::soft_delete(&mut conn, path.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    // Additional unit tests can be added here
    // Integration tests are located in tests/book_api_test.rs
}