mod common;

use actix_web::test;
use reading_notes_backend::create_app;
use serde_json::Value;

#[actix_web::test]
async fn test_health_check() {
    // Setup test database
    let test_db = common::setup_test_db();

    // Create test app
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Test health check endpoint
    let req = test::TestRequest::get().uri("/api/health").to_request();

    let resp = test::call_service(&app, req).await;

    // Verify response
    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["version"], "0.1.0");
}

#[actix_web::test]
async fn test_cors_headers() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Test preflight request
    let req = test::TestRequest::default()
        .method(actix_web::http::Method::OPTIONS)
        .uri("/api/health")
        .insert_header(("Origin", "http://localhost:3000"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Should have CORS headers
    assert!(resp.headers().contains_key("access-control-allow-origin"));
}

#[actix_web::test]
async fn test_404_for_unknown_route() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::get().uri("/api/unknown").to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}
