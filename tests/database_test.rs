mod common;

use diesel::prelude::*;
use reading_notes_backend::models::*;
use chrono::NaiveDate;

#[tokio::test]
async fn test_database_connection() {
    // Arrange: Setup test database
    let test_db = common::setup_test_db();
    let mut conn = test_db.pool
        .get()
        .expect("Failed to get connection from pool");
    
    // Act: Execute a simple query to verify connection
    let result: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("1"))
        .first(&mut conn)
        .expect("Failed to execute test query");
    
    // Assert: Verify the connection works correctly    
    assert_eq!(result, 1);
}

#[tokio::test]
async fn test_book_crud_operations() {
    let test_db = common::setup_test_db();
    let mut conn = test_db.pool.get().expect("Failed to get connection");
    
    // Test creating a book
    let new_book = NewBook {
        isbn: Some("978-1234567890".to_string()),
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        publisher: Some("Test Publisher".to_string()),
        publication_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        page_count: Some(300),
        cover_image: None,
        description: Some("A test book".to_string()),
    };
    
    use reading_notes_backend::db::schema::books;
    
    let inserted_book: Book = diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert book");
    
    // Verify book was inserted
    assert_eq!(inserted_book.title, "Test Book");
    assert_eq!(inserted_book.author, "Test Author");
    assert_eq!(inserted_book.isbn, Some("978-1234567890".to_string()));
    assert!(inserted_book.id > 0);
    
    // Test reading the book
    let found_book: Book = books::table
        .filter(books::id.eq(inserted_book.id))
        .first(&mut conn)
        .expect("Failed to find book");
    
    assert_eq!(found_book.id, inserted_book.id);
    assert_eq!(found_book.title, "Test Book");
    
    // Test updating the book
    let update_book = UpdateBook {
        title: Some("Updated Test Book".to_string()),
        ..Default::default()
    };
    
    // AsChangeset is already imported via diesel::prelude::*
    
    let updated_book: Book = diesel::update(books::table.filter(books::id.eq(inserted_book.id)))
        .set(&update_book)
        .returning(Book::as_returning())
        .get_result(&mut conn)
        .expect("Failed to update book");
    
    assert_eq!(updated_book.title, "Updated Test Book");
    
    // Test soft delete
    use chrono::Utc;
    let deleted_book: Book = diesel::update(books::table.filter(books::id.eq(inserted_book.id)))
        .set(books::deleted_at.eq(Some(Utc::now())))
        .returning(Book::as_returning())
        .get_result(&mut conn)
        .expect("Failed to soft delete book");
    
    assert!(deleted_book.deleted_at.is_some());
    
    // Test that soft deleted book is not returned in normal queries
    let active_books: Vec<Book> = books::table
        .filter(books::deleted_at.is_null())
        .load(&mut conn)
        .expect("Failed to load active books");
    
    assert!(active_books.is_empty());
}

#[tokio::test]
async fn test_reading_status_operations() {
    let test_db = common::setup_test_db();
    let mut conn = test_db.pool.get().expect("Failed to get connection");
    
    // First create a book
    let new_book = NewBook {
        title: "Test Book for Status".to_string(),
        author: "Test Author".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: Some(200),
        cover_image: None,
        description: None,
    };
    
    use reading_notes_backend::db::schema::{books, reading_status};
    
    let book: Book = diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert book");
    
    // Create reading status
    let new_status = NewReadingStatus {
        book_id: book.id,
        status: "reading".to_string(),
        rating: None,
        start_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        finish_date: None,
    };
    
    let status: ReadingStatus = diesel::insert_into(reading_status::table)
        .values(&new_status)
        .returning(ReadingStatus::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert reading status");
    
    assert_eq!(status.book_id, book.id);
    assert_eq!(status.status, "reading");
    
    // Test unique constraint - should not be able to insert another status for same book
    let duplicate_status = NewReadingStatus {
        book_id: book.id,
        status: "completed".to_string(),
        rating: Some(5),
        start_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
        finish_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
    };
    
    let result = diesel::insert_into(reading_status::table)
        .values(&duplicate_status)
        .returning(ReadingStatus::as_returning())
        .get_result::<ReadingStatus>(&mut conn);
    
    // Should fail due to unique constraint
    assert!(result.is_err());
}

#[tokio::test]
async fn test_notes_operations() {
    let test_db = common::setup_test_db();
    let mut conn = test_db.pool.get().expect("Failed to get connection");
    
    // Create a book first
    let new_book = NewBook {
        title: "Test Book for Notes".to_string(),
        author: "Test Author".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: Some(150),
        cover_image: None,
        description: None,
    };
    
    use reading_notes_backend::db::schema::{books, reading_notes};
    
    let book: Book = diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert book");
    
    // Create a note
    let new_note = NewReadingNote {
        book_id: book.id,
        title: Some("Chapter 1 Notes".to_string()),
        content: "This is a test note with **markdown** formatting.".to_string(),
        note_type: Some("summary".to_string()),
        page_reference: Some("1-20".to_string()),
        is_favorite: Some(true),
    };
    
    let note: ReadingNote = diesel::insert_into(reading_notes::table)
        .values(&new_note)
        .returning(ReadingNote::as_returning())
        .get_result(&mut conn)
        .expect("Failed to insert note");
    
    assert_eq!(note.book_id, book.id);
    assert_eq!(note.title, Some("Chapter 1 Notes".to_string()));
    assert_eq!(note.is_favorite, Some(true));
    
    // Test loading notes for a book
    let book_notes: Vec<ReadingNote> = reading_notes::table
        .filter(reading_notes::book_id.eq(book.id))
        .filter(reading_notes::deleted_at.is_null())
        .load(&mut conn)
        .expect("Failed to load notes");
    
    assert_eq!(book_notes.len(), 1);
    assert_eq!(book_notes[0].id, note.id);
}