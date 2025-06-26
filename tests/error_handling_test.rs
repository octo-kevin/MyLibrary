mod common;

use actix_web::test;
use reading_notes_backend::create_app;
use serde_json::Value;

#[actix_web::test]
async fn test_validation_error_format() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Send empty title (validation error)
    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&serde_json::json!({
            "title": "",
            "author": "Test Author"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 422);
    let body: Value = test::read_body_json(resp).await;

    // Check error format
    assert_eq!(body["error"], "VALIDATION_ERROR");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Title is required"));
}

#[actix_web::test]
async fn test_not_found_error_format() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Request non-existent book
    let req = test::TestRequest::get()
        .uri("/api/books/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 404);
    let body: Value = test::read_body_json(resp).await;

    // Check error format
    assert_eq!(body["error"], "NOT_FOUND");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Book with id 99999 not found"));
}

#[actix_web::test]
async fn test_update_not_found_error_format() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Update non-existent book
    let req = test::TestRequest::put()
        .uri("/api/books/99999")
        .set_json(&serde_json::json!({
            "title": "Updated Title"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 404);
    let body: Value = test::read_body_json(resp).await;

    // Check error format
    assert_eq!(body["error"], "NOT_FOUND");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Book with id 99999 not found"));
}

#[actix_web::test]
async fn test_delete_not_found_error_format() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Delete non-existent book
    let req = test::TestRequest::delete()
        .uri("/api/books/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 404);
    let body: Value = test::read_body_json(resp).await;

    // Check error format
    assert_eq!(body["error"], "NOT_FOUND");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Book with id 99999 not found"));
}

#[actix_web::test]
async fn test_validation_error_empty_author() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Send empty author (validation error)
    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&serde_json::json!({
            "title": "Valid Title",
            "author": ""
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 422);
    let body: Value = test::read_body_json(resp).await;

    // Check error format
    assert_eq!(body["error"], "VALIDATION_ERROR");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Author is required"));
}

#[actix_web::test]
async fn test_update_validation_error() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // First create a book
    let create_req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&serde_json::json!({
            "title": "Original Title",
            "author": "Original Author"
        }))
        .to_request();

    let create_resp = test::call_service(&app, create_req).await;
    let create_body: Value = test::read_body_json(create_resp).await;
    let book_id = create_body["id"].as_i64().unwrap();

    // Act - Update with empty title
    let update_req = test::TestRequest::put()
        .uri(&format!("/api/books/{}", book_id))
        .set_json(&serde_json::json!({
            "title": ""
        }))
        .to_request();

    let update_resp = test::call_service(&app, update_req).await;

    // Assert
    assert_eq!(update_resp.status(), 422);
    let body: Value = test::read_body_json(update_resp).await;

    // Check error format
    assert_eq!(body["error"], "VALIDATION_ERROR");
    assert!(body["message"]
        .as_str()
        .unwrap()
        .contains("Title cannot be empty"));
}
