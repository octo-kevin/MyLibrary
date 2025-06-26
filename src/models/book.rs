use crate::db::schema::books;
use crate::errors::{AppError, Result};
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

#[derive(Debug, Deserialize, AsChangeset, Default, ToSchema)]
#[diesel(table_name = books)]
pub struct UpdateBook {
    /// ISBN number (optional)
    #[schema(example = "978-0134685991")]
    pub isbn: Option<String>,

    /// Book title (optional)
    #[schema(example = "Updated Title")]
    pub title: Option<String>,

    /// Author name (optional)
    #[schema(example = "Updated Author")]
    pub author: Option<String>,

    /// Publisher name (optional)
    #[schema(example = "Updated Publisher")]
    pub publisher: Option<String>,

    /// Publication date (optional)
    #[schema(example = "2024-01-01")]
    pub publication_date: Option<NaiveDate>,

    /// Number of pages (optional)
    #[schema(example = 500)]
    pub page_count: Option<i32>,

    /// Cover image URL (optional)
    #[schema(example = "https://example.com/new-cover.jpg")]
    pub cover_image: Option<String>,

    /// Book description (optional)
    #[schema(example = "Updated description")]
    pub description: Option<String>,
}

/// Request structure for creating a new book
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateBookRequest {
    /// ISBN number (optional)
    #[schema(example = "978-0134685991")]
    pub isbn: Option<String>,

    /// Book title (required)
    #[schema(example = "Effective Java")]
    pub title: String,

    /// Author name (required)
    #[schema(example = "Joshua Bloch")]
    pub author: String,

    /// Publisher name (optional)
    #[schema(example = "Addison-Wesley")]
    pub publisher: Option<String>,

    /// Publication date (optional, YYYY-MM-DD format)
    #[schema(example = "2017-12-27")]
    pub publication_date: Option<NaiveDate>,

    /// Number of pages (optional)
    #[schema(example = 416)]
    pub page_count: Option<i32>,

    /// Cover image URL (optional)
    #[schema(example = "https://example.com/cover.jpg")]
    pub cover_image: Option<String>,

    /// Book description (optional)
    #[schema(example = "Best practices for the Java platform")]
    pub description: Option<String>,
}

/// Response structure for book operations
#[derive(Debug, Serialize, ToSchema)]
pub struct BookResponse {
    /// Unique book identifier
    #[schema(example = 1)]
    pub id: i64,

    /// ISBN number
    #[schema(example = "978-0134685991")]
    pub isbn: Option<String>,

    /// Book title
    #[schema(example = "Effective Java")]
    pub title: String,

    /// Author name
    #[schema(example = "Joshua Bloch")]
    pub author: String,

    /// Publisher name
    #[schema(example = "Addison-Wesley")]
    pub publisher: Option<String>,

    /// Publication date
    #[schema(example = "2017-12-27")]
    pub publication_date: Option<NaiveDate>,

    /// Number of pages
    #[schema(example = 416)]
    pub page_count: Option<i32>,

    /// Cover image URL
    #[schema(example = "https://example.com/cover.jpg")]
    pub cover_image: Option<String>,

    /// Book description
    #[schema(example = "Best practices for the Java platform")]
    pub description: Option<String>,

    /// Creation timestamp
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: Option<DateTime<Utc>>,

    /// Last update timestamp
    #[schema(example = "2024-01-01T12:00:00Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// Paginated book list response
#[derive(Debug, Serialize, ToSchema)]
pub struct BookListResponse {
    /// List of books
    pub books: Vec<BookResponse>,

    /// Total number of books
    #[schema(example = 50)]
    pub total: i64,

    /// Current page number
    #[schema(example = 1)]
    pub page: u32,

    /// Number of items per page
    #[schema(example = 20)]
    pub per_page: u32,

    /// Total number of pages
    #[schema(example = 3)]
    pub total_pages: u32,
}

impl From<CreateBookRequest> for NewBook {
    fn from(req: CreateBookRequest) -> Self {
        Self {
            isbn: req.isbn,
            title: req.title,
            author: req.author,
            publisher: req.publisher,
            publication_date: req.publication_date,
            page_count: req.page_count,
            cover_image: req.cover_image,
            description: req.description,
        }
    }
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        Self {
            id: book.id,
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publisher: book.publisher,
            publication_date: book.publication_date,
            page_count: book.page_count,
            cover_image: book.cover_image,
            description: book.description,
            created_at: book.created_at,
            updated_at: book.updated_at,
        }
    }
}

impl Book {
    /// Creates a new book in the database
    pub fn create(conn: &mut PgConnection, new_book: NewBook) -> Result<Book> {
        diesel::insert_into(books::table)
            .values(&new_book)
            .returning(Book::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    /// Finds a book by ID (excluding soft deleted)
    pub fn find_by_id(conn: &mut PgConnection, book_id: i64) -> Result<Book> {
        books::table
            .filter(books::id.eq(book_id))
            .filter(books::deleted_at.is_null())
            .first(conn)
            .map_err(|_| AppError::NotFound(format!("Book with id {} not found", book_id)))
    }

    /// Lists all active books with pagination
    pub fn list_paginated(
        conn: &mut PgConnection,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Book>, i64)> {
        let offset = ((page.saturating_sub(1)) * per_page) as i64;

        let books = books::table
            .filter(books::deleted_at.is_null())
            .order(books::created_at.desc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<Book>(conn)?;

        let total = books::table
            .filter(books::deleted_at.is_null())
            .count()
            .get_result::<i64>(conn)?;

        Ok((books, total))
    }

    /// Searches books by title or author
    pub fn search(
        conn: &mut PgConnection,
        query: &str,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<Book>, i64)> {
        let search_pattern = format!("%{}%", query);
        let offset = ((page.saturating_sub(1)) * per_page) as i64;

        let books = books::table
            .filter(books::deleted_at.is_null())
            .filter(
                books::title
                    .ilike(&search_pattern)
                    .or(books::author.ilike(&search_pattern)),
            )
            .order(books::created_at.desc())
            .limit(per_page as i64)
            .offset(offset)
            .load::<Book>(conn)?;

        let total = books::table
            .filter(books::deleted_at.is_null())
            .filter(
                books::title
                    .ilike(&search_pattern)
                    .or(books::author.ilike(&search_pattern)),
            )
            .count()
            .get_result::<i64>(conn)?;

        Ok((books, total))
    }

    /// Updates a book
    pub fn update(conn: &mut PgConnection, book_id: i64, update_data: UpdateBook) -> Result<Book> {
        diesel::update(books::table.find(book_id))
            .filter(books::deleted_at.is_null())
            .set((
                &update_data,
                books::updated_at.eq(Some(Utc::now().naive_utc())),
            ))
            .returning(Book::as_returning())
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Book with id {} not found", book_id))
                }
                _ => AppError::from(e),
            })
    }

    /// Soft deletes a book
    pub fn soft_delete(conn: &mut PgConnection, book_id: i64) -> Result<()> {
        let affected = diesel::update(books::table.find(book_id))
            .filter(books::deleted_at.is_null())
            .set(books::deleted_at.eq(Some(Utc::now().naive_utc())))
            .execute(conn)?;

        if affected == 0 {
            return Err(AppError::NotFound(format!(
                "Book with id {} not found",
                book_id
            )));
        }

        Ok(())
    }

    /// Checks if the book is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
