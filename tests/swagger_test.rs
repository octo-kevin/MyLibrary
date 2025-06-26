mod common;

use actix_web::test;
use reading_notes_backend::create_app;
use serde_json::Value;

#[actix_web::test]
async fn test_swagger_ui_accessible() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Access Swagger UI
    let req = test::TestRequest::get().uri("/docs/").to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_openapi_json_accessible() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act - Access OpenAPI JSON spec
    let req = test::TestRequest::get()
        .uri("/api-docs/openapi.json")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());

    // Verify it's valid JSON
    let body: Value = test::read_body_json(resp).await;

    // Basic OpenAPI structure validation
    assert_eq!(body["openapi"], "3.0.3");
    assert_eq!(body["info"]["title"], "Personal Reading Notes API");
    assert_eq!(body["info"]["version"], "1.0.0");

    // Verify paths exist
    assert!(body["paths"]["/api/books"].is_object());
    assert!(body["paths"]["/api/books/{id}"].is_object());

    // Verify components/schemas exist
    assert!(body["components"]["schemas"]["CreateBookRequest"].is_object());
    assert!(body["components"]["schemas"]["BookResponse"].is_object());
    assert!(body["components"]["schemas"]["BookListResponse"].is_object());
    assert!(body["components"]["schemas"]["UpdateBook"].is_object());
    assert!(body["components"]["schemas"]["ErrorResponse"].is_object());
}

#[actix_web::test]
async fn test_openapi_spec_completeness() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act
    let req = test::TestRequest::get()
        .uri("/api-docs/openapi.json")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let spec: Value = test::read_body_json(resp).await;

    // Assert - Check all CRUD operations are documented
    let books_path = &spec["paths"]["/api/books"];
    assert!(
        books_path["get"].is_object(),
        "GET /api/books should be documented"
    );
    assert!(
        books_path["post"].is_object(),
        "POST /api/books should be documented"
    );

    let book_by_id_path = &spec["paths"]["/api/books/{id}"];
    assert!(
        book_by_id_path["get"].is_object(),
        "GET /api/books/id should be documented"
    );
    assert!(
        book_by_id_path["put"].is_object(),
        "PUT /api/books/id should be documented"
    );
    assert!(
        book_by_id_path["delete"].is_object(),
        "DELETE /api/books/id should be documented"
    );

    // Check that all endpoints have proper tags
    assert_eq!(books_path["get"]["tags"][0], "Books");
    assert_eq!(books_path["post"]["tags"][0], "Books");
    assert_eq!(book_by_id_path["get"]["tags"][0], "Books");
    assert_eq!(book_by_id_path["put"]["tags"][0], "Books");
    assert_eq!(book_by_id_path["delete"]["tags"][0], "Books");
}
