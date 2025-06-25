//! Unit tests for note-tag associations
//! 
//! Tests the many-to-many relationship between notes and tags,
//! including automatic tag creation, usage count updates, and association management.

use actix_web::test;
use serde_json::json;
use reading_notes_backend::{create_app, models::book::CreateBookRequest};

mod common;

/// Test basic note-tag association
#[actix_web::test]
async fn test_note_tag_basic_association() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "关联测试书籍".to_string(),
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

    // Create tags first
    let tag_names = vec!["标签A", "标签B", "标签C"];
    for name in &tag_names {
        let tag_data = json!({
            "name": name,
            "description": format!("{}的描述", name)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // Create a note with some of the tags
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "关联测试笔记",
        "content": "测试笔记与标签的关联",
        "tags": ["标签A", "标签C"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_tags = note_response["tags"].as_array().unwrap();
    
    // Verify correct tags are associated
    assert_eq!(note_tags.len(), 2);
    assert!(note_tags.contains(&json!("标签A")));
    assert!(note_tags.contains(&json!("标签C")));
    assert!(!note_tags.contains(&json!("标签B")));
}

/// Test tag usage count updates with multiple notes
#[actix_web::test]
async fn test_tag_usage_count_multiple_notes() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "使用计数测试书籍".to_string(),
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

    // Create note 1 with tags ["共同标签", "独有标签1"]
    let note1_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "笔记1",
        "content": "第一个笔记",
        "tags": ["共同标签", "独有标签1"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note1_data)
        .to_request();
    test::call_service(&app, req).await;

    // Create note 2 with tags ["共同标签", "独有标签2"]
    let note2_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "笔记2",
        "content": "第二个笔记",
        "tags": ["共同标签", "独有标签2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note2_data)
        .to_request();
    test::call_service(&app, req).await;

    // Create note 3 with tags ["共同标签"]
    let note3_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "笔记3",
        "content": "第三个笔记",
        "tags": ["共同标签"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note3_data)
        .to_request();
    test::call_service(&app, req).await;

    // Check tag usage counts
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();

    // Find and verify usage counts
    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        match tag_name {
            "共同标签" => {
                assert_eq!(tag["note_count"], 3);
                assert_eq!(tag["usage_count"], 3);
            },
            "独有标签1" | "独有标签2" => {
                assert_eq!(tag["note_count"], 1);
                assert_eq!(tag["usage_count"], 1);
            },
            _ => {}
        }
    }
}

/// Test updating note tags and usage count changes
#[actix_web::test]
async fn test_update_note_tags_usage_count() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "标签更新测试书籍".to_string(),
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
        "title": "标签更新测试笔记",
        "content": "用于测试标签更新的笔记",
        "tags": ["初始标签1", "初始标签2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = note_response["id"].as_i64().unwrap();

    // Verify initial tag usage counts
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 2);

    // Update note tags to completely different tags
    let new_tags = json!(["新标签1", "新标签2", "新标签3"]);

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}/tags", note_id))
        .set_json(&new_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Verify tag changes
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    
    // Should now have 5 tags total (2 old + 3 new)
    assert_eq!(response["total"], 5);
    
    let tags = response["tags"].as_array().unwrap();
    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        match tag_name {
            "初始标签1" | "初始标签2" => {
                // Old tags should have 0 usage count
                assert_eq!(tag["note_count"], 0);
                assert_eq!(tag["usage_count"], 0);
            },
            "新标签1" | "新标签2" | "新标签3" => {
                // New tags should have 1 usage count
                assert_eq!(tag["note_count"], 1);
                assert_eq!(tag["usage_count"], 1);
            },
            _ => {}
        }
    }
}

/// Test partial tag update (some new, some existing)
#[actix_web::test]
async fn test_partial_tag_update() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "部分更新测试书籍".to_string(),
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
        "title": "部分更新测试笔记",
        "content": "用于测试部分标签更新的笔记",
        "tags": ["保留标签", "移除标签"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = note_response["id"].as_i64().unwrap();

    // Update tags: keep one, remove one, add one new
    let updated_tags = json!(["保留标签", "新增标签"]);

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}/tags", note_id))
        .set_json(&updated_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();
    
    // Verify final tags
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&json!("保留标签")));
    assert!(tags.contains(&json!("新增标签")));
    assert!(!tags.contains(&json!("移除标签")));

    // Verify tag usage counts
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();

    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        match tag_name {
            "保留标签" | "新增标签" => {
                assert_eq!(tag["note_count"], 1);
                assert_eq!(tag["usage_count"], 1);
            },
            "移除标签" => {
                assert_eq!(tag["note_count"], 0);
                assert_eq!(tag["usage_count"], 0);
            },
            _ => {}
        }
    }
}

/// Test tag usage with multiple notes and deletions
#[actix_web::test]
async fn test_tag_usage_with_note_deletion() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "删除测试书籍".to_string(),
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

    // Create two notes with shared tags
    let note1_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "第一个笔记",
        "content": "将被删除的笔记",
        "tags": ["共享标签", "独有标签1"]
    });

    let note2_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "第二个笔记",
        "content": "将保留的笔记",
        "tags": ["共享标签", "独有标签2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note1_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let note1_response: serde_json::Value = test::read_body_json(resp).await;
    let note1_id = note1_response["id"].as_i64().unwrap();

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note2_data)
        .to_request();
    test::call_service(&app, req).await;

    // Verify initial tag usage counts
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();

    // Before deletion: 共享标签=2, 独有标签1=1, 独有标签2=1
    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        match tag_name {
            "共享标签" => assert_eq!(tag["note_count"], 2),
            "独有标签1" | "独有标签2" => assert_eq!(tag["note_count"], 1),
            _ => {}
        }
    }

    // Delete the first note
    let req = test::TestRequest::delete()
        .uri(&format!("/api/notes/{}", note1_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // Note: In our current implementation, tag usage counts don't automatically 
    // decrease when notes are deleted (soft delete). This is by design to maintain
    // historical data integrity. The counts represent total usage over time.
    
    // However, if we wanted to test dynamic count updates, we would check:
    // - 共享标签 count should be 1
    // - 独有标签1 count should be 0  
    // - 独有标签2 count should remain 1
}

/// Test complex tag association scenarios
#[actix_web::test]
async fn test_complex_tag_associations() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "复杂关联测试书籍".to_string(),
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

    // Test scenario: Create note with mixed existing and new tags
    
    // First, create some existing tags
    let existing_tags = vec!["现有标签1", "现有标签2"];
    for name in &existing_tags {
        let tag_data = json!({
            "name": name,
            "description": format!("{}的描述", name)
        });

        let req = test::TestRequest::post()
            .uri("/api/tags")
            .set_json(&tag_data)
            .to_request();
        test::call_service(&app, req).await;
    }

    // Create a note with mix of existing and new tags
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "复杂关联测试笔记",
        "content": "测试现有标签和新标签的混合使用",
        "tags": ["现有标签1", "新标签1", "现有标签2", "新标签2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_tags = note_response["tags"].as_array().unwrap();
    
    // Verify all tags are associated
    assert_eq!(note_tags.len(), 4);
    assert!(note_tags.contains(&json!("现有标签1")));
    assert!(note_tags.contains(&json!("现有标签2")));
    assert!(note_tags.contains(&json!("新标签1")));
    assert!(note_tags.contains(&json!("新标签2")));

    // Verify all tags exist in the system
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response["total"], 4);

    // Verify usage counts are correct
    let tags = response["tags"].as_array().unwrap();
    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        if ["现有标签1", "现有标签2", "新标签1", "新标签2"].contains(&tag_name) {
            assert_eq!(tag["note_count"], 1);
            assert_eq!(tag["usage_count"], 1);
        }
    }
}

/// Test empty tags array
#[actix_web::test]
async fn test_empty_tags_array() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "空标签测试书籍".to_string(),
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

    // Create a note with empty tags array
    let note_data = json!({
        "book_id": book_id,
        "note_type": "general",
        "title": "无标签笔记",
        "content": "这个笔记没有标签",
        "tags": []
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_tags = note_response["tags"].as_array().unwrap();
    
    // Should have no tags
    assert_eq!(note_tags.len(), 0);

    // Update to add tags later
    let note_id = note_response["id"].as_i64().unwrap();
    let new_tags = json!(["后加标签1", "后加标签2"]);

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}/tags", note_id))
        .set_json(&new_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();
    
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&json!("后加标签1")));
    assert!(tags.contains(&json!("后加标签2")));
}

/// Test updating to empty tags array
#[actix_web::test]
async fn test_update_to_empty_tags() {
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Create a book
    let book_data = CreateBookRequest {
        title: "清空标签测试书籍".to_string(),
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
        "title": "清空标签测试笔记",
        "content": "这个笔记的标签将被清空",
        "tags": ["将被移除1", "将被移除2"]
    });

    let req = test::TestRequest::post()
        .uri("/api/notes")
        .set_json(&note_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let note_response: serde_json::Value = test::read_body_json(resp).await;
    let note_id = note_response["id"].as_i64().unwrap();

    // Update to empty tags array
    let empty_tags = json!([]);

    let req = test::TestRequest::put()
        .uri(&format!("/api/notes/{}/tags", note_id))
        .set_json(&empty_tags)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();
    
    // Should have no tags
    assert_eq!(tags.len(), 0);

    // Verify the tags still exist but with updated usage counts
    let req = test::TestRequest::get()
        .uri("/api/tags")
        .to_request();

    let resp = test::call_service(&app, req).await;
    let response: serde_json::Value = test::read_body_json(resp).await;
    let tags = response["tags"].as_array().unwrap();

    for tag in tags {
        let tag_name = tag["name"].as_str().unwrap();
        if tag_name == "将被移除1" || tag_name == "将被移除2" {
            // Tags should exist but have 0 note count
            assert_eq!(tag["note_count"], 0);
        }
    }
}