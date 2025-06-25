//! Unit tests for tag management API endpoints
//! 
//! Tests the complete CRUD functionality for tags,
//! including popular tags and usage statistics.

use actix_web::test;
use serde_json::json;
use reading_notes_backend::{create_app, models::book::CreateBookRequest};

mod common;

/// Test creating a new tag
#[actix_web::test]
async fn test_create_tag() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let tag_data = json!({
        "name": "Rust编程",
        "description": "Rust编程语言相关内容"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    
    // Verify response structure
    assert!(response["id"].is_number());
    assert_eq!(response["name"], "Rust编程");
    assert_eq!(response["slug"], "rust编程");
    assert_eq!(response["book_count"], 0);
    assert_eq!(response["note_count"], 0);
    assert_eq!(response["usage_count"], 0);
    assert!(response["created_at"].is_string());
}

/// Test creating a tag with English name
#[actix_web::test]
async fn test_create_tag_english() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let tag_data = json!({
        "name": "Programming",
        "description": "General programming topics"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    
    assert_eq!(response["name"], "Programming");
    assert_eq!(response["slug"], "programming");
}

/// Test creating a tag with empty name
#[actix_web::test]
async fn test_create_tag_empty_name() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let tag_data = json!({
        "name": "",
        "description": "Empty name test"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"].as_str().unwrap().contains("Tag name is required"));
}

/// Test creating duplicate tags
#[actix_web::test]
async fn test_create_duplicate_tag() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let tag_data = json!({
        "name": "重复标签",
        "description": "第一个标签"
    });

    // Create first tag
    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Try to create second tag with same name
    let duplicate_tag_data = json!({
        "name": "重复标签",
        "description": "第二个标签"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&duplicate_tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"].as_str().unwrap().contains("already exists"));
}

/// Test getting a specific tag by ID
#[actix_web::test]
async fn test_get_tag() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a tag first
    let tag_data = json!({
        "name": "测试标签",
        "description": "用于测试的标签"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let tag_id = create_response["id"].as_i64().unwrap();

    // Get the tag
    let req = test::TestRequest::get()
        .uri(&format!("/api/tags/{}", tag_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["id"], tag_id);
    assert_eq!(response["name"], "测试标签");
    assert_eq!(response["slug"], "测试标签");
}

/// Test getting a non-existent tag
#[actix_web::test]
async fn test_get_tag_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::get()
        .uri("/api/tags/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test listing tags with pagination
#[actix_web::test]
async fn test_list_tags() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create multiple tags
    for i in 1..=3 {
        let tag_data = json!({
            "name": format!("标签{}", i),
            "description": format!("第{}个标签", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // List tags
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 3);
    assert_eq!(response["page"], 1);
    assert_eq!(response["per_page"], 20);
    
    let tags = response["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
}

/// Test listing tags with pagination parameters
#[actix_web::test]
async fn test_list_tags_pagination() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create 5 tags
    for i in 1..=5 {
        let tag_data = json!({
            "name": format!("分页测试标签{}", i),
            "description": format!("第{}个分页测试标签", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // Test pagination: page 1, 2 items per page
    let req = test::TestRequest::get()
        .uri("/api/tags?page=1&per_page=2")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 5);
    assert_eq!(response["page"], 1);
    assert_eq!(response["per_page"], 2);
    assert_eq!(response["total_pages"], 3);
    
    let tags = response["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 2);
}

/// Test updating a tag
#[actix_web::test]
async fn test_update_tag() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a tag first
    let tag_data = json!({
        "name": "原始标签名",
        "description": "原始描述"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let tag_id = create_response["id"].as_i64().unwrap();

    // Update the tag
    let update_data = json!({
        "name": "更新后的标签名",
        "description": "更新后的描述"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/tags/{}", tag_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["id"], tag_id);
    assert_eq!(response["name"], "更新后的标签名");
    assert_eq!(response["slug"], "更新后的标签名");
}

/// Test updating a tag with duplicate name
#[actix_web::test]
async fn test_update_tag_duplicate_name() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create two tags
    let tag1_data = json!({
        "name": "标签1",
        "description": "第一个标签"
    });

    let tag2_data = json!({
        "name": "标签2",
        "description": "第二个标签"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag1_data)
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag2_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let tag2_id = create_response["id"].as_i64().unwrap();

    // Try to update tag2 to have the same name as tag1
    let update_data = json!({
        "name": "标签1"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/tags/{}", tag2_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"].as_str().unwrap().contains("already exists"));
}

/// Test updating a non-existent tag
#[actix_web::test]
async fn test_update_tag_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let update_data = json!({
        "name": "不存在的标签"
    });

    let req = test::TestRequest::put()
        .uri("/api/tags/99999")
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test updating a tag with empty name
#[actix_web::test]
async fn test_update_tag_empty_name() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a tag first
    let tag_data = json!({
        "name": "原始标签",
        "description": "原始描述"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let tag_id = create_response["id"].as_i64().unwrap();

    // Try to update with empty name
    let update_data = json!({
        "name": ""
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/tags/{}", tag_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert!(response["message"].as_str().unwrap().contains("cannot be empty"));
}

/// Test soft deleting a tag
#[actix_web::test]
async fn test_delete_tag() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a tag first
    let tag_data = json!({
        "name": "要删除的标签",
        "description": "这个标签将被删除"
    });

    let req = test::TestRequest::post()
        .uri("/api/tags")
        .set_json(&tag_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let create_response: serde_json::Value = test::read_body_json(resp).await;
    let tag_id = create_response["id"].as_i64().unwrap();

    // Delete the tag
    let req = test::TestRequest::delete()
        .uri(&format!("/api/tags/{}", tag_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // Try to get the deleted tag
    let req = test::TestRequest::get()
        .uri(&format!("/api/tags/{}", tag_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test deleting a non-existent tag
#[actix_web::test]
async fn test_delete_tag_not_found() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let req = test::TestRequest::delete()
        .uri("/api/tags/99999")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

/// Test getting popular tags
#[actix_web::test]
async fn test_get_popular_tags() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create some tags
    let tag_names = vec!["Rust", "编程", "内存管理"];
    let mut tag_ids = Vec::new();

    for name in &tag_names {
        let tag_data = json!({
            "name": name,
            "description": format!("{}相关内容", name)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        let response: serde_json::Value = test::read_body_json(resp).await;
        tag_ids.push(response["id"].as_i64().unwrap());
    }

    // Create a book and notes to use tags
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

    // Create notes with different tag usage patterns
    // Note 1: uses "Rust" and "编程"
    let note1_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "笔记1",
        "content": "使用Rust和编程标签",
        "tags": ["Rust", "编程"]
    });

    // Note 2: uses "Rust" only
    let note2_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "笔记2",
        "content": "只使用Rust标签",
        "tags": ["Rust"]
    });

    for note_data in [note1_data, note2_data] {
        let req = test::TestRequest::post()
            .uri("/api/notes")
            .set_json(&note_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // Get popular tags
    let req = test::TestRequest::get()
        .uri("/api/tags/popular")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    let popular_tags = response.as_array().unwrap();
    
    // Should have at least the tags we used
    assert!(popular_tags.len() >= 2);
    
    // Find Rust tag (should have usage_count = 2)
    let rust_tag = popular_tags.iter()
        .find(|tag| tag["name"] == "Rust")
        .expect("Rust tag should be in popular tags");
    assert_eq!(rust_tag["usage_count"], 2);
    
    // Find 编程 tag (should have usage_count = 1)
    let programming_tag = popular_tags.iter()
        .find(|tag| tag["name"] == "编程")
        .expect("编程 tag should be in popular tags");
    assert_eq!(programming_tag["usage_count"], 1);
}

/// Test getting popular tags with limit
#[actix_web::test]
async fn test_get_popular_tags_with_limit() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create multiple tags
    for i in 1..=5 {
        let tag_data = json!({
            "name": format!("标签{}", i),
            "description": format!("第{}个标签", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();
        test::call_service(&app, req).await;
    }

    // Get popular tags with limit=3
    let req = test::TestRequest::get()
        .uri("/api/tags/popular?limit=3")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    let popular_tags = response.as_array().unwrap();
    
    // Should return at most 3 tags
    assert!(popular_tags.len() <= 3);
}

/// Test tag usage count integration with notes
#[actix_web::test]
async fn test_tag_usage_count_with_notes() {
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

    // Create a note with tags
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "测试笔记",
        "content": "测试标签使用计数",
        "tags": ["计数测试", "标签统计"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Check tag usage count
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();

    // Find the tags we created
    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        if tag_name == "计数测试" || tag_name == "标签统计" {
            assert_eq!(tag["note_count"], 1);
            assert_eq!(tag["usage_count"], 1);
        }
    }
}

/// Test tag creation through note creation
#[actix_web::test]
async fn test_tag_auto_creation_via_notes() {
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

    // Verify no tags exist initially
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 0);

    // Create a note with new tags
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "自动创建标签测试",
        "content": "这个笔记会自动创建标签",
        "tags": ["自动创建1", "自动创建2", "自动创建3"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify tags were automatically created
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 3);

    let tags = response["tags"].as_array().unwrap();
    let tag_names: Vec<&str> = tags.iter()
        .map(|tag| tag["name"].as_str().unwrap())
        .collect();

    assert!(tag_names.contains(&"自动创建1"));
    assert!(tag_names.contains(&"自动创建2"));
    assert!(tag_names.contains(&"自动创建3"));
}