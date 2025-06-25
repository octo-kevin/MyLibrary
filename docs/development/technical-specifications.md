# 技术规范文档

> **文档版本**: v1.0  
> **最后更新**: 2025年6月25日  
> **维护者**: 项目开发团队

## 📋 概述

本文档详细描述了个人读书记录系统后端的技术规范、架构设计、编码标准和最佳实践。

## 🏗️ 架构设计

### 总体架构
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   前端应用       │    │   API网关        │    │   数据库         │
│  (React/Vue)    │◄──►│  (Actix-web)    │◄──►│  (PostgreSQL)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                               │
                       ┌───────┴───────┐
                       │   中间件层     │
                       │ (CORS, Auth)   │
                       └───────────────┘
```

### 分层架构
```rust
// 请求流处理层次
HTTP Request
    ↓
Middleware (CORS, Logging)
    ↓
Handler (业务逻辑入口)
    ↓
Model (数据访问层)
    ↓
Database (PostgreSQL)
```

## 🛠️ 技术栈规范

### 核心依赖版本
```toml
[dependencies]
actix-web = "4.9"          # Web框架
diesel = "2.2"             # ORM
serde = "1.0"              # 序列化
tokio = "1.0"              # 异步运行时
chrono = "0.4"             # 时间处理
uuid = "1.0"               # UUID生成
env_logger = "0.11"        # 日志
dotenv = "0.15"            # 环境变量
```

### 开发工具
```toml
[dev-dependencies]
actix-rt = "2.0"           # 测试运行时
serde_json = "1.0"         # JSON测试数据
```

## 📊 数据库规范

### 命名约定
- **表名**: 复数形式，蛇形命名 (`books`, `reading_notes`)
- **字段名**: 蛇形命名 (`created_at`, `book_id`)
- **外键**: `{表名}_id` 格式 (`book_id`, `note_id`)
- **索引**: `idx_{表名}_{字段名}` 格式

### 标准字段
所有业务表必须包含以下字段：
```sql
-- 主键
id BIGSERIAL PRIMARY KEY,

-- 时间戳
created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

-- 软删除
deleted_at TIMESTAMP WITH TIME ZONE
```

### 数据类型标准
```sql
-- 文本类型
title VARCHAR(500)           -- 短文本（标题）
content TEXT                 -- 长文本（内容）
slug VARCHAR(100)           -- URL标识符

-- 数值类型  
id BIGINT                   -- 主键和外键
page_count INTEGER          -- 页数等计数
progress_percent REAL       -- 百分比 (0.0-1.0)

-- 时间类型
created_at TIMESTAMP WITH TIME ZONE
date_only DATE              -- 仅日期

-- 布尔类型
is_favorite BOOLEAN DEFAULT FALSE
```

## 🔧 代码规范

### Rust编码标准

#### 1. 模块结构
```rust
// 标准模块声明顺序
use std::collections::HashMap;           // 标准库
use actix_web::{web, HttpResponse};      // 外部库  
use crate::models::Book;                 // 项目内部

pub mod handlers;                        // 子模块声明
```

#### 2. 错误处理
```rust
// 统一错误类型
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(String),
    InternalError,
}

// 错误传播
fn some_function() -> Result<Data, AppError> {
    let data = database_call()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(data)
}
```

#### 3. 数据结构定义
```rust
// Request DTO
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBookRequest {
    #[schema(example = "Rust程序设计语言")]
    pub title: String,
    
    #[schema(example = "Steve Klabnik")]
    pub author: String,
    
    #[schema(example = "9787115563439")]
    pub isbn: Option<String>,
}

// Response DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct BookResponse {
    pub id: i64,
    pub title: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

#### 4. Handler设计模式
```rust
#[utoipa::path(
    post,
    path = "/api/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created", body = BookResponse),
        (status = 422, description = "Validation error", body = ErrorResponse)
    ),
    tag = "Books"
)]
pub async fn create_book(
    pool: web::Data<DbPool>,
    book_data: web::Json<CreateBookRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // 数据验证
    if book_data.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title is required".to_string()));
    }
    
    // 业务逻辑
    let book = Book::create(&mut conn, book_data.into_inner().into())?;
    
    Ok(HttpResponse::Created().json(book.to_response()))
}
```

### API设计规范

#### 1. RESTful端点设计
```
资源操作映射：
POST   /api/{resource}           # 创建资源
GET    /api/{resource}           # 列出资源（支持分页）
GET    /api/{resource}/{id}      # 获取单个资源
PUT    /api/{resource}/{id}      # 更新资源
DELETE /api/{resource}/{id}      # 删除资源（软删除）

关联资源：
GET    /api/{resource}/{id}/{sub_resource}  # 获取关联资源
PUT    /api/{resource}/{id}/{sub_resource}  # 更新关联
```

#### 2. HTTP状态码标准
```
200 OK           - 成功获取资源
201 Created      - 成功创建资源  
204 No Content   - 成功删除资源
400 Bad Request  - 请求参数错误
404 Not Found    - 资源不存在
422 Unprocessable Entity - 数据验证失败
500 Internal Server Error - 服务器内部错误
```

#### 3. 响应格式标准
```json
// 成功响应 - 单个资源
{
  "id": 1,
  "title": "Rust程序设计语言",
  "created_at": "2025-06-25T10:30:00Z"
}

// 成功响应 - 资源列表
{
  "data": [...],
  "total": 100,
  "page": 1,
  "per_page": 20,
  "total_pages": 5
}

// 错误响应
{
  "error": "VALIDATION_ERROR",
  "message": "Title is required",
  "timestamp": "2025-06-25T10:30:00Z"
}
```

#### 4. 分页参数标准
```
查询参数：
?page=1          # 页码（从1开始）
?per_page=20     # 每页数量（默认20，最大100）
?search=keyword  # 搜索关键词
?sort=created_at # 排序字段
?order=desc      # 排序方向（asc/desc）
```

## 🧪 测试规范

### 测试分类和命名
```rust
// 单元测试 - 测试单个函数
#[test]
fn test_slugify() {
    assert_eq!(slugify("Hello World"), "hello-world");
}

// 集成测试 - 测试HTTP API
#[actix_web::test]
async fn test_create_book_success() {
    // 测试成功场景
}

#[actix_web::test]
async fn test_create_book_validation_error() {
    // 测试验证错误场景
}
```

### 测试数据管理
```rust
// 测试辅助函数
mod common;

fn create_test_book() -> CreateBookRequest {
    CreateBookRequest {
        title: "测试书籍".to_string(),
        author: "测试作者".to_string(),
        isbn: Some("1234567890123".to_string()),
        // ...
    }
}

// 数据库隔离
#[actix_web::test]
async fn test_something() {
    let test_db = common::setup_test_db();  // 独立数据库
    let app = test::init_service(create_app(test_db.pool.clone())).await;
    // 测试逻辑...
}
```

### 测试覆盖要求
- ✅ **成功场景**: 每个API端点的正常使用
- ✅ **错误场景**: 验证错误、资源不存在、权限错误
- ✅ **边界条件**: 空值、极限值、特殊字符
- ✅ **业务逻辑**: 复杂的关联和计算逻辑

## 📚 文档规范

### OpenAPI文档标准
```rust
// Handler文档注解
#[utoipa::path(
    post,                                    // HTTP方法
    path = "/api/books",                     // 路径
    request_body = CreateBookRequest,        // 请求体
    responses(                               // 响应定义
        (status = 201, description = "Book created successfully", body = BookResponse),
        (status = 422, description = "Validation error", body = ErrorResponse)
    ),
    tag = "Books"                           // 分组标签
)]
```

### 代码注释标准
```rust
/// 创建新书籍
/// 
/// 根据提供的书籍信息创建新的书籍记录。书籍标题为必填项，
/// ISBN如果提供则会进行格式验证。
/// 
/// # Arguments
/// * `pool` - 数据库连接池
/// * `book_data` - 书籍创建请求数据
/// 
/// # Returns
/// 创建成功返回201状态码和书籍信息，验证失败返回422错误
/// 
/// # Errors
/// - `ValidationError` - 当必填字段为空或格式错误时
/// - `DatabaseError` - 当数据库操作失败时
pub async fn create_book(...) -> Result<HttpResponse, AppError> {
```

## 🔒 安全规范

### 输入验证
```rust
// 字符串验证
fn validate_title(title: &str) -> Result<(), AppError> {
    if title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    if title.len() > 500 {
        return Err(AppError::ValidationError("Title too long".to_string()));
    }
    Ok(())
}

// SQL注入防护 - 使用参数化查询
diesel::update(books::table.find(book_id))
    .set(title.eq(&new_title))  // 自动转义
    .execute(conn)
```

### 数据库安全
```rust
// 软删除查询
books::table
    .filter(books::deleted_at.is_null())  // 总是过滤已删除记录
    .load::<Book>(conn)

// 事务处理
conn.transaction(|conn| {
    // 原子操作
    let book = create_book(conn, data)?;
    update_statistics(conn, book.id)?;
    Ok(book)
})
```

## ⚡ 性能规范

### 数据库性能
```sql
-- 必须的索引
CREATE INDEX idx_books_title ON books USING gin(to_tsvector('english', title));
CREATE INDEX idx_books_deleted_at ON books(deleted_at);
CREATE INDEX idx_reading_notes_book_id ON reading_notes(book_id);

-- 分页查询优化
SELECT * FROM books 
WHERE deleted_at IS NULL 
ORDER BY created_at DESC 
LIMIT 20 OFFSET $1;
```

### 连接池配置
```rust
// 生产环境连接池设置
r2d2::Pool::builder()
    .max_size(20)                    // 最大连接数
    .min_idle(Some(5))              // 最小空闲连接
    .connection_timeout(Duration::from_secs(30))
    .idle_timeout(Some(Duration::from_secs(600)))
    .build(manager)
```

## 🚀 部署规范

### 环境变量
```bash
# 数据库配置
DATABASE_URL=postgresql://user:pass@localhost/reading_notes
DATABASE_MAX_CONNECTIONS=20

# 服务器配置  
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# 日志配置
RUST_LOG=info
LOG_LEVEL=info
```

### Docker配置
```dockerfile
# 多阶段构建
FROM rust:1.82 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5
COPY --from=builder /app/target/release/reading-notes-backend /usr/local/bin/
EXPOSE 8080
CMD ["reading-notes-backend"]
```

## 📊 监控规范

### 日志标准
```rust
use log::{info, warn, error};

// 请求日志
info!("Creating book: title={}", book_data.title);

// 错误日志
error!("Database connection failed: {}", e);

// 性能日志
let start = Instant::now();
let result = some_operation();
info!("Operation completed in {:?}", start.elapsed());
```

### 健康检查
```rust
// 健康检查端点
#[get("/health")]
async fn health_check(pool: web::Data<DbPool>) -> HttpResponse {
    match pool.get() {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "healthy",
            "database": "connected",
            "timestamp": Utc::now()
        })),
        Err(_) => HttpResponse::ServiceUnavailable().json(json!({
            "status": "unhealthy", 
            "database": "disconnected"
        }))
    }
}
```

## 📋 版本控制规范

### Git提交消息格式
```
<type>(<scope>): <description>

[optional body]

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>

类型说明：
feat: 新功能
fix: 错误修复  
docs: 文档更新
style: 代码格式
refactor: 重构
test: 测试
chore: 构建/工具配置
```

### 分支管理
```
main        - 主分支，生产就绪代码
develop     - 开发分支，集成新功能
feature/*   - 功能分支
hotfix/*    - 热修复分支
release/*   - 发布分支
```

---

**文档维护**: 随着项目演进持续更新  
**遵循原则**: 简单、安全、高效、可维护  
**技术债务**: 定期评估和清理