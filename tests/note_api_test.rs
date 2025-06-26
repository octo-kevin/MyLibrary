//! Unit tests for reading notes API endpoints
//!
//! Tests the complete CRUD functionality for reading notes,
//! including tag associations and book relationships.

use actix_web::test;
use reading_notes_backend::{create_app, models::book::CreateBookRequest};
use serde_json::json;

mod common;

/// Test creating a new reading note
#[actix_web::test]
async fn test_create_note() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // First create a book to associate with the note
    let book_data = CreateBookRequest {
        title: "Rust编程语言".to_string(),
        author: "Steve Klabnik".to_string(),
        isbn: Some("9787115563439".to_string()),
        publisher: Some("人民邮电出版社".to_string()),
        publication_date: None,
        page_count: Some(520),
        cover_image: None,
        description: Some("Rust编程语言官方指南".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Now create a note
    let note_data = json!({
        "book_id": book_id,
        "note_type": "quote",
        "title": "Rust所有权概念",
        "content": "Rust的所有权系统是其内存安全的核心机制，每个值都有一个唯一的所有者。",
        "page_number": 42,
        "tags": ["Rust", "编程", "内存管理"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;

    // Verify response structure
    assert_eq!(response["book_id"], book_id);
    assert_eq!(response["title"], "Rust所有权概念");
    assert_eq!(response["note_type"], "quote");
    assert_eq!(
        response["content"],
        "Rust的所有权系统是其内存安全的核心机制，每个值都有一个唯一的所有者。"
    );

    // Check tags were created and associated
    let tags = response["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
    assert!(tags.contains(&json!("Rust")));
    assert!(tags.contains(&json!("编程")));
    assert!(tags.contains(&json!("内存管理")));
}

/// Test creating a note with invalid book ID
#[actix_web::test]
async fn test_create_note_invalid_book() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let note_data = json!({
        "book_id": 99999,
        "note_type": "quote",
        "title": "测试笔记",
        "content": "这是一个测试笔记",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"]
        .as_str()
        .unwrap()
        .contains("Book with id 99999 not found"));
}

/// Test creating a note with empty content
#[actix_web::test]
async fn test_create_note_empty_content() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let note_data = json!({
        "book_id": 1,
        "note_type": "quote",
        "title": "测试笔记",
        "content": "",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"]
        .as_str()
        .unwrap()
        .contains("Content is required"));
}

/// Test getting a specific note by ID
#[actix_web::test]
async fn test_get_note() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create a note
    let note_data = json!({
        "book_id": book_id,
        "note_type": "summary",
        "title": "章节总结",
        "content": "这一章主要讲述了Rust的基本概念。",
        "page_number": 10,
        "tags": ["Rust", "基础"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = create_response["id"].as_i64().unwrap();

    // Get the note
    let req = test::TestRequest::get()
        .uri(&format!("/api/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["id"], note_id);
    assert_eq!(response["title"], "章节总结");
    assert_eq!(response["note_type"], "summary");
    assert_eq!(response["content"], "这一章主要讲述了Rust的基本概念。");
}

/// Test getting a non-existent note
#[actix_web::test]
async fn test_get_note_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::get()
        .uri("/api/notes/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test listing notes with pagination
#[actix_web::test]
async fn test_list_notes() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create multiple notes
    for i in 1..=3 {
        let note_data = json!({
            "book_id": book_id,
            "note_type": "general",
            "title": format!("笔记 {}", i),
            "content": format!("这是第{}个测试笔记", i),
            "tags": []
        });

        let req = test::TestRequest::post()
            .uri("/api/notes")
            .set_json(&note_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // List notes
    let req = test::TestRequest::get().uri("/api/notes").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 3);
    assert_eq!(response["page"], 1);
    assert_eq!(response["per_page"], 20);

    let notes = response["notes"].as_array().unwrap();
    assert_eq!(notes.len(), 3);
}

/// Test listing notes with pagination parameters
#[actix_web::test]
async fn test_list_notes_pagination() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create 5 notes
    for i in 1..=5 {
        let note_data = json!({
            "book_id": book_id,
            "note_type": "general",
            "title": format!("笔记 {}", i),
            "content": format!("这是第{}个测试笔记", i),
            "tags": []
        });

        let req = test::TestRequest::post()
            .uri("/api/notes")
            .set_json(&note_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // Test pagination: page 1, 2 items per page
    let req = test::TestRequest::get()
        .uri("/api/notes?page=1&per_page=2")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 5);
    assert_eq!(response["page"], 1);
    assert_eq!(response["per_page"], 2);
    assert_eq!(response["total_pages"], 3);

    let notes = response["notes"].as_array().unwrap();
    assert_eq!(notes.len(), 2);
}

/// Test updating a note
#[actix_web::test]
async fn test_update_note() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create a note
    let note_data = json!({
        "book_id": book_id,
        "note_type": "thought",
        "title": "原始标题",
        "content": "原始内容",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = create_response["id"].as_i64().unwrap();

    // Update the note
    let update_data = json!({
        "title": "更新后的标题",
        "content": "更新后的内容",
        "note_type": "summary"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}", note_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["id"], note_id);
    assert_eq!(response["title"], "更新后的标题");
    assert_eq!(response["content"], "更新后的内容");
    assert_eq!(response["note_type"], "summary");
}

/// Test updating a non-existent note
#[actix_web::test]
async fn test_update_note_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let update_data = json!({
        "title": "更新标题"
    });

    let req = test::TestRequest::put()
        .uri("/api/notes/99999")
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test updating note with empty content
#[actix_web::test]
async fn test_update_note_empty_content() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book and note first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "测试笔记",
        "content": "原始内容",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = create_response["id"].as_i64().unwrap();

    // Try to update with empty content
    let update_data = json!({
        "content": ""
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}", note_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"]
        .as_str()
        .unwrap()
        .contains("Content cannot be empty"));
}

/// Test soft deleting a note
#[actix_web::test]
async fn test_delete_note() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create a note
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "要删除的笔记",
        "content": "这个笔记将被删除",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = create_response["id"].as_i64().unwrap();

    // Delete the note
    let req = test::TestRequest::delete()
        .uri(&format!("/api/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // Try to get the deleted note
    let req = test::TestRequest::get()
        .uri(&format!("/api/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test deleting a non-existent note
#[actix_web::test]
async fn test_delete_note_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::delete()
        .uri("/api/notes/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test getting notes for a specific book
#[actix_web::test]
async fn test_get_book_notes() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create two books
    let book1_data = CreateBookRequest {
        title: "书籍1".to_string(),
        author: "作者1".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let book2_data = CreateBookRequest {
        title: "书籍2".to_string(),
        author: "作者2".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book1_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let book1_response: serde_json::Value = test::read_body_json(resp).await;
    let book1_id = book1_response["id"].as_i64().unwrap();

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book2_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let book2_response: serde_json::Value = test::read_body_json(resp).await;
    let book2_id = book2_response["id"].as_i64().unwrap();

    // Create notes for both books
    for i in 1..=2 {
        let note_data = json!({
            "book_id": book1_id,
            "note_type": "general",
            "title": format!("书籍1-笔记{}", i),
            "content": format!("书籍1的第{}个笔记", i),
            "tags": []
        });

        let req = test::TestRequest::post()
            .uri("/api/notes")
            .set_json(&note_data)
            .to_request();
        test::call_service(&app, req).await;
    }

    let note_data = json!({
        "book_id": book2_id,
        "note_type": "general",
        "title": "书籍2-笔记1",
        "content": "书籍2的笔记",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();
    test::call_service(&app, req).await;

    // Get notes for book1
    let req = test::TestRequest::get()
        .uri(&format!("/api/books/{}/notes", book1_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 2);

    let notes = response["notes"].as_array().unwrap();
    assert_eq!(notes.len(), 2);

    // Verify all notes belong to book1
    for note in notes {
        assert_eq!(note["book_id"], book1_id);
    }
}

/// Test getting notes for a non-existent book
#[actix_web::test]
async fn test_get_book_notes_book_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::get()
        .uri("/api/books/99999/notes")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test updating note tags
#[actix_web::test]
async fn test_update_note_tags() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book first
    let book_data = CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: None,
        publisher: None,
        publication_date: None,
        page_count: None,
        cover_image: None,
        description: None,
    };

    let req = test::TestRequest::post()
        .uri("/api/books")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let book_response: serde_json::Value = test::read_body_json(resp).await;
    let book_id = book_response["id"].as_i64().unwrap();

    // Create a note with initial tags
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "测试标签笔记",
        "content": "用于测试标签功能的笔记",
        "tags": ["初始标签1", "初始标签2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = create_response["id"].as_i64().unwrap();

    // Update the note tags
    let new_tags = json!(["Rust", "编程", "内存管理", "系统编程"]);

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}/tags", note_id))
        .set_json(&new_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response: serde_json::Value = test::read_body_json(resp).await;

    // Verify the tags were updated
    let tags = response["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 4);
    assert!(tags.contains(&json!("Rust")));
    assert!(tags.contains(&json!("编程")));
    assert!(tags.contains(&json!("内存管理")));
    assert!(tags.contains(&json!("系统编程")));

    // Verify old tags are no longer present
    assert!(!tags.contains(&json!("初始标签1")));
    assert!(!tags.contains(&json!("初始标签2")));
}

/// Test updating tags for a non-existent note
#[actix_web::test]
async fn test_update_note_tags_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let new_tags = json!(["测试标签"]);

    let req = test::TestRequest::put()
        .uri("/api/notes/99999/tags")
        .set_json(&new_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
