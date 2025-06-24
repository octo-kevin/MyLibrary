# API 开发快速参考

## 🚀 快速开始新 API

### 1. 创建数据库迁移
```bash
diesel migration generate create_resources_table
```

```sql
-- up.sql
CREATE TABLE resources (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);

-- down.sql
DROP TABLE resources;
```

### 2. 运行迁移
```bash
diesel migration run
```

### 3. 创建模型文件
`src/models/resource.rs`

### 4. 创建处理器文件
`src/handlers/resources.rs`

### 5. 更新模块导出
- `src/models/mod.rs` - 添加 `pub mod resource;`
- `src/handlers/mod.rs` - 添加 `pub mod resources;`

### 6. 配置路由
在 `src/lib.rs` 中添加路由配置

### 7. 编写测试
`tests/resource_api_test.rs`

### 8. 更新文档
- `API.md` - 添加 API 端点说明
- OpenAPI 会自动生成

## 📋 标准 HTTP 状态码

| 操作 | 成功状态码 | 错误状态码 |
|------|-----------|-----------|
| CREATE | 201 Created | 422 Validation Error |
| GET (单个) | 200 OK | 404 Not Found |
| GET (列表) | 200 OK | 400 Bad Request |
| UPDATE | 200 OK | 404 Not Found, 422 Validation |
| DELETE | 204 No Content | 404 Not Found |

## 🔧 常用代码片段

### 获取数据库连接
```rust
let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
```

### 输入验证
```rust
if input.field.trim().is_empty() {
    return Err(AppError::ValidationError("Field is required".to_string()));
}
```

### 分页参数处理
```rust
let page = query.page.unwrap_or(1).max(1);
let per_page = query.per_page.unwrap_or(20).min(100).max(1);
```

### 软删除过滤
```rust
.filter(resources::deleted_at.is_null())
```

### 搜索模式
```rust
let search_pattern = format!("%{}%", query);
.filter(resources::name.ilike(&search_pattern))
```

## 🧪 测试模板

### 成功场景测试
```rust
#[actix_web::test]
async fn test_create_resource_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/api/resources")
        .set_json(&json!({ "name": "Test" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 201);
}
```

### 错误场景测试
```rust
#[actix_web::test]
async fn test_validation_error() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    // Act
    let req = test::TestRequest::post()
        .uri("/api/resources")
        .set_json(&json!({ "name": "" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 422);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "VALIDATION_ERROR");
}
```

## 📝 OpenAPI 注解模板

### 路径注解
```rust
#[utoipa::path(
    post,
    path = "/api/resources",
    request_body = CreateResourceRequest,
    responses(
        (status = 201, description = "Created", body = ResourceResponse),
        (status = 422, description = "Validation error", body = ErrorResponse)
    ),
    tag = "Resources"
)]
```

### Schema 注解
```rust
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateResourceRequest {
    #[schema(example = "Example name")]
    pub name: String,
    
    #[schema(example = "Example description")]
    pub description: Option<String>,
}
```

## 🛠️ 常用命令

```bash
# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 运行测试
cargo test

# 运行特定测试
cargo test test_create_resource

# 检查构建
cargo check

# 启动服务器
cargo run

# 查看 Swagger UI
open http://localhost:8080/docs/
```

## ⚡ 性能建议

1. **使用连接池** - 不要每次创建新连接
2. **分页查询** - 避免一次返回大量数据
3. **索引优化** - 为搜索字段添加索引
4. **N+1 查询** - 使用 joins 避免多次查询
5. **缓存策略** - 考虑缓存频繁访问的数据

## 🔒 安全检查

- [ ] 验证所有输入
- [ ] 使用参数化查询
- [ ] 不暴露内部错误详情
- [ ] 限制分页大小
- [ ] 实现访问控制（未来）
- [ ] 记录敏感操作

## 🐛 调试技巧

### 查看 SQL 查询
```bash
RUST_LOG=diesel=debug cargo run
```

### 查看 HTTP 请求
```bash
RUST_LOG=actix_web=debug cargo run
```

### 测试单个端点
```bash
curl -X POST http://localhost:8080/api/resources \
  -H "Content-Type: application/json" \
  -d '{"name": "Test"}'
```

---

💡 **提示**: 遇到问题时，参考 `books` API 的实现！