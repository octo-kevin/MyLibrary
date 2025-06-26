mod common;

use actix_web::test;
use chrono::NaiveDate;
use reading_notes_backend::{create_app, models::book::CreateBookRequest};
use serde_json::Value;

#[actix_web::test]
async fn test_create_book_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let new_book = CreateBookRequest {
        isbn: Some("978-0134685991".to_string()),
        title: "Effective Java".to_string(),
        author: "Joshua Bloch".to_string(),
        publisher: Some("Addison-Wesley".to_string()),
        publication_date: Some(NaiveDate::from_ymd_opt(2017, 12, 27).unwrap()),
        page_count: Some(416),
        cover_image: None,
        description: Some("Best practices for the Java platform".to_string()),
    };

    // Act
    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&new_book)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "Effective Java");
    assert_eq!(body["author"], "Joshua Bloch");
    assert_eq!(body["isbn"], "978-0134685991");
    assert!(body["id"].as_i64().unwrap() > 0);
}

#[actix_web::test]
async fn test_create_book_validation_error() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let invalid_book = CreateBookRequest {
        isbn: None,
        title: "".to_string(), // Empty title should fail validation
        author: "Some Author".to_string(),
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    // Act
    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&invalid_book)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 422); // Unprocessable Entity
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_get_book_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // First create a book
    let new_book = CreateBookRequest {
        isbn: Some("978-0134685991".to_string()),
        title: "Effective Java".to_string(),
        author: "Joshua Bloch".to_string(),
        publisher: Some("Addison-Wesley".to_string()),
        publication_date: Some(NaiveDate::from_ymd_opt(2017, 12, 27).unwrap()),
        page_count: Some(416),
        cover_image: None,
        description: Some("Best practices for the Java platform".to_string()),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&new_book)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body: Value = test::read_body_json(create_resp).await;
    let book_id = create_body["id"].as_i64().unwrap();

    // Act - Get the book
    let get_req = test::TestRequest::get()
        .uri(&format!("/api/books/{}", book_id))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;

    // Assert
    assert!(get_resp.status().is_success());
    let body: Value = test::read_body_json(get_resp).await;
    assert_eq!(body["id"], book_id);
    assert_eq!(body["title"], "Effective Java");
    assert_eq!(body["author"], "Joshua Bloch");
}

#[actix_web::test]
async fn test_get_book_not_found() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act
    let req = test::TestRequest::get()
        .uri("/api/books/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 404);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "NOT_FOUND");
}

#[actix_web::test]
async fn test_list_books_empty() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act
    let req = test::TestRequest::get().uri("/api/books").to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["books"].as_array().unwrap().len(), 0);
    assert_eq!(body["total"], 0);
    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 20);
    assert_eq!(body["total_pages"], 0);
}

#[actix_web::test]
async fn test_list_books_with_pagination() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create multiple books
    for i in 1..=5 {
        let book = CreateBookRequest {
            isbn: Some(format!("978-123456789{}", i)),
            title: format!("Test Book {}", i),
            author: format!("Author {}", i),
            publisher: None,
            publication_date: None,
            page_count: None,
            cover_image: None,
            description: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/books")
            .set_json(&book)
            .to_request();

        test::call_service(&app, req).await;
    }

    // Act - Request with pagination
    let req = test::TestRequest::get()
        .uri("/api/books?page=1&per_page=3")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["books"].as_array().unwrap().len(), 3);
    assert_eq!(body["total"], 5);
    assert_eq!(body["page"], 1);
    assert_eq!(body["per_page"], 3);
    assert_eq!(body["total_pages"], 2);
}

#[actix_web::test]
async fn test_search_books() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create test books
    let books = vec![
        CreateBookRequest {
            title: "Effective Java".to_string(),
            author: "Joshua Bloch".to_string(),
            isbn: None,
            publisher: None,
            publication_date: None,
            page_count: None,
            cover_image: None,
            description: None,
        },
        CreateBookRequest {
            title: "Clean Code".to_string(),
            author: "Robert Martin".to_string(),
            isbn: None,
            publisher: None,
            publication_date: None,
            page_count: None,
            cover_image: None,
            description: None,
        },
        CreateBookRequest {
            title: "Design Patterns".to_string(),
            author: "Gang of Four".to_string(),
            isbn: None,
            publisher: None,
            publication_date: None,
            page_count: None,
            cover_image: None,
            description: None,
        },
    ];

    for book in books {
        let req = test::TestRequest::post()
            .uri("/api/books")
            .set_json(&book)
            .to_request();
        test::call_service(&app, req).await;
    }

    // Act - Search by title
    let req = test::TestRequest::get()
        .uri("/api/books?search=Java")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["books"].as_array().unwrap().len(), 1);
    assert_eq!(body["total"], 1);
    assert_eq!(body["books"][0]["title"], "Effective Java");
}

#[actix_web::test]
async fn test_update_book_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let new_book = CreateBookRequest {
        title: "Original Title".to_string(),
        author: "Original Author".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let create_req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&new_book)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body: Value = test::read_body_json(create_resp).await;
    let book_id = create_body["id"].as_i64().unwrap();

    // Act - Update the book
    let update_data = serde_json::json!({
        "title": "Updated Title",
        "description": "Updated description"
    });

    let update_req = test::TestRequest::put()
        .uri(&format!("/api/books/{}", book_id))
        .set_json(&update_data)
        .to_request();

    let update_resp = test::call_service(&app, update_req).await;

    // Assert
    assert!(update_resp.status().is_success());
    let body: Value = test::read_body_json(update_resp).await;
    assert_eq!(body["id"], book_id);
    assert_eq!(body["title"], "Updated Title");
    assert_eq!(body["author"], "Original Author"); // Should remain unchanged
    assert_eq!(body["description"], "Updated description");
}

#[actix_web::test]
async fn test_delete_book_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let new_book = CreateBookRequest {
        title: "Book to Delete".to_string(),
        author: "Some Author".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let create_req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&new_book)
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body: Value = test::read_body_json(create_resp).await;
    let book_id = create_body["id"].as_i64().unwrap();

    // Act - Delete the book
    let delete_req = test::TestRequest::delete()
        .uri(&format!("/api/books/{}", book_id))
        .to_request();

    let delete_resp = test::call_service(&app, delete_req).await;

    // Assert
    assert_eq!(delete_resp.status(), 204); // No Content

    // Verify book is no longer accessible
    let get_req = test::TestRequest::get()
        .uri(&format!("/api/books/{}", book_id))
        .to_request();

    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), 404); // Not Found
}
