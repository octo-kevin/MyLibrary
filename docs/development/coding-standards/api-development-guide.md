# API 开发规范指南

本文档定义了个人读书记录系统的 API 开发标准和最佳实践。所有新的 API 端点都应遵循这些规范。

## 目录

1. [项目结构](#项目结构)
2. [API 设计原则](#api-设计原则)
3. [数据模型规范](#数据模型规范)
4. [错误处理规范](#错误处理规范)
5. [数据库操作规范](#数据库操作规范)
6. [API 处理器规范](#api-处理器规范)
7. [测试规范](#测试规范)
8. [文档规范](#文档规范)
9. [开发流程](#开发流程)
10. [代码示例](#代码示例)

## 项目结构

### 目录组织
```
src/
├── handlers/       # HTTP 请求处理器（控制器）
│   ├── mod.rs     # 处理器模块导出
│   └── books.rs   # 书籍相关处理器
├── models/        # 数据模型和数据库操作
│   ├── mod.rs     # 模型模块导出
│   └── book.rs    # 书籍模型和方法
├── errors/        # 错误处理
│   └── mod.rs     # 统一错误类型
├── db/           # 数据库配置
│   ├── mod.rs    # 连接池配置
│   └── schema.rs # Diesel schema
└── lib.rs        # 应用配置和路由

tests/
├── common/       # 测试公共模块
├── book_api_test.rs    # 书籍 API 测试
└── error_handling_test.rs # 错误处理测试
```

### 模块职责
- **handlers**: 仅处理 HTTP 请求/响应，不包含业务逻辑
- **models**: 包含数据结构定义和所有数据库操作
- **errors**: 定义应用错误类型和 HTTP 响应转换
- **db**: 数据库连接和 schema 定义

## API 设计原则

### RESTful 规范
1. 使用标准 HTTP 方法：
   - `GET` - 获取资源
   - `POST` - 创建资源
   - `PUT` - 更新资源（完整或部分）
   - `DELETE` - 删除资源

2. URL 设计：
   ```
   GET    /api/resources          # 获取资源列表
   GET    /api/resources/{id}     # 获取单个资源
   POST   /api/resources          # 创建资源
   PUT    /api/resources/{id}     # 更新资源
   DELETE /api/resources/{id}     # 删除资源
   ```

3. HTTP 状态码：
   - `200 OK` - 成功获取/更新
   - `201 Created` - 成功创建
   - `204 No Content` - 成功删除
   - `400 Bad Request` - 请求格式错误
   - `404 Not Found` - 资源不存在
   - `422 Unprocessable Entity` - 验证失败
   - `500 Internal Server Error` - 服务器错误

### 请求/响应格式
- 统一使用 JSON 格式
- 时间使用 ISO 8601 格式（UTC）
- 空值使用 `null`，不省略字段

## 数据模型规范

### 模型定义结构
```rust
// 1. 数据库模型
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = resources)]
pub struct Resource {
    pub id: i64,
    pub field: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// 2. 创建请求 DTO
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateResourceRequest {
    #[schema(example = "example value")]
    pub field: String,
}

// 3. 更新请求 DTO（所有字段可选）
#[derive(Debug, Deserialize, AsChangeset, ToSchema)]
#[diesel(table_name = resources)]
pub struct UpdateResource {
    pub field: Option<String>,
}

// 4. 响应 DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct ResourceResponse {
    pub id: i64,
    pub field: String,
    pub created_at: Option<DateTime<Utc>>,
}

// 5. 列表响应 DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct ResourceListResponse {
    pub resources: Vec<ResourceResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}
```

### 类型转换
```rust
impl From<CreateResourceRequest> for NewResource {
    fn from(req: CreateResourceRequest) -> Self {
        Self { field: req.field }
    }
}

impl From<Resource> for ResourceResponse {
    fn from(resource: Resource) -> Self {
        Self {
            id: resource.id,
            field: resource.field,
            created_at: resource.created_at,
        }
    }
}
```

## 错误处理规范

### 错误类型定义
```rust
use crate::errors::{AppError, Result};

// 在模型方法中使用
pub fn find_by_id(conn: &mut PgConnection, id: i64) -> Result<Resource> {
    resources::table
        .filter(resources::id.eq(id))
        .filter(resources::deleted_at.is_null())
        .first(conn)
        .map_err(|_| AppError::NotFound(
            format!("Resource with id {} not found", id)
        ))
}
```

### 错误响应格式
```json
{
    "error": "VALIDATION_ERROR",
    "message": "Field is required"
}
```

### 标准错误类型
- `VALIDATION_ERROR` (422) - 输入验证失败
- `NOT_FOUND` (404) - 资源不存在
- `BAD_REQUEST` (400) - 请求格式错误
- `DATABASE_ERROR` (500) - 数据库错误
- `INTERNAL_ERROR` (500) - 内部错误

## 数据库操作规范

### 标准方法命名
```rust
impl Resource {
    // 创建
    pub fn create(conn: &mut PgConnection, new_resource: NewResource) -> Result<Resource>
    
    // 查询单个
    pub fn find_by_id(conn: &mut PgConnection, id: i64) -> Result<Resource>
    
    // 查询列表（带分页）
    pub fn list_paginated(
        conn: &mut PgConnection, 
        page: u32, 
        per_page: u32
    ) -> Result<(Vec<Resource>, i64)>
    
    // 搜索（带分页）
    pub fn search(
        conn: &mut PgConnection,
        query: &str,
        page: u32,
        per_page: u32
    ) -> Result<(Vec<Resource>, i64)>
    
    // 更新
    pub fn update(
        conn: &mut PgConnection,
        id: i64,
        update_data: UpdateResource
    ) -> Result<Resource>
    
    // 软删除
    pub fn soft_delete(conn: &mut PgConnection, id: i64) -> Result<()>
}
```

### 分页实现
```rust
pub fn list_paginated(
    conn: &mut PgConnection,
    page: u32,
    per_page: u32,
) -> Result<(Vec<Resource>, i64)> {
    let offset = ((page.saturating_sub(1)) * per_page) as i64;
    
    let resources = resources::table
        .filter(resources::deleted_at.is_null())
        .order(resources::created_at.desc())
        .limit(per_page as i64)
        .offset(offset)
        .load::<Resource>(conn)?;

    let total = resources::table
        .filter(resources::deleted_at.is_null())
        .count()
        .get_result::<i64>(conn)?;

    Ok((resources, total))
}
```

## API 处理器规范

### 处理器结构
```rust
#[utoipa::path(
    post,
    path = "/api/resources",
    request_body = CreateResourceRequest,
    responses(
        (status = 201, description = "Resource created", body = ResourceResponse),
        (status = 422, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    tag = "Resources"
)]
pub async fn create_resource(
    pool: web::Data<DbPool>,
    resource_data: web::Json<CreateResourceRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    // 验证输入
    if resource_data.field.trim().is_empty() {
        return Err(AppError::ValidationError("Field is required".to_string()));
    }

    // 执行业务逻辑
    let new_resource = resource_data.into_inner().into();
    let resource = Resource::create(&mut conn, new_resource)?;
    let response = ResourceResponse::from(resource);

    Ok(HttpResponse::Created().json(response))
}
```

### 查询参数处理
```rust
#[derive(Debug, Deserialize, IntoParams)]
pub struct ResourceListQuery {
    #[param(example = 1)]
    pub page: Option<u32>,
    #[param(example = 20)]
    pub per_page: Option<u32>,
    #[param(example = "search term")]
    pub search: Option<String>,
}
```

## 测试规范

### 测试文件组织
```
tests/
├── common/
│   └── mod.rs          # 测试工具函数
├── resource_api_test.rs    # 资源 API 测试
└── error_handling_test.rs  # 错误处理测试
```

### 测试结构
```rust
#[actix_web::test]
async fn test_create_resource_success() {
    // Arrange
    let test_db = common::setup_test_db();
    let app = test::init_service(create_app(test_db.pool.clone())).await;

    let new_resource = CreateResourceRequest {
        field: "test value".to_string(),
    };

    // Act
    let req = test::TestRequest::post()
        .uri("/api/resources")
        .set_json(&new_resource)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status(), 201);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["field"], "test value");
}
```

### 测试覆盖要求
1. **正常流程**：每个端点的成功场景
2. **错误处理**：验证失败、资源不存在等
3. **边界情况**：空值、超长输入、特殊字符
4. **业务逻辑**：分页、搜索、软删除

## 文档规范

### 代码文档
```rust
//! 资源管理模块
//! 
//! 提供资源的 CRUD 操作和搜索功能

/// 创建新资源
/// 
/// # 参数
/// - `conn`: 数据库连接
/// - `new_resource`: 新资源数据
/// 
/// # 返回
/// - `Ok(Resource)`: 创建成功的资源
/// - `Err(AppError)`: 创建失败的错误
pub fn create(conn: &mut PgConnection, new_resource: NewResource) -> Result<Resource> {
    // 实现...
}
```

### API 文档要求
1. 在 `API.md` 中添加端点说明
2. 包含请求/响应示例
3. 列出所有可能的错误情况
4. 提供 curl 命令示例

### OpenAPI 注解
```rust
/// 资源请求结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateResourceRequest {
    /// 资源字段（必填）
    #[schema(example = "示例值")]
    pub field: String,
}
```

## 开发流程

### 1. 数据库设计
- 创建迁移文件
- 定义表结构（包含软删除字段）
- 运行迁移

### 2. 模型开发
- 定义数据结构
- 实现数据库操作方法
- 添加必要的验证

### 3. API 处理器
- 实现 HTTP 处理函数
- 添加 OpenAPI 注解
- 配置路由

### 4. 测试编写
- 编写集成测试
- 覆盖正常和异常场景
- 确保测试隔离

### 5. 文档更新
- 更新 API.md
- 添加使用示例
- 更新 SWAGGER.md（如需要）

## 代码示例

### 完整的资源模块示例

#### 1. 模型定义 (`src/models/resource.rs`)
```rust
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::db::schema::resources;
use crate::errors::{AppError, Result};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = resources)]
pub struct Resource {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = resources)]
pub struct NewResource {
    pub name: String,
    pub description: Option<String>,
}

impl Resource {
    pub fn create(conn: &mut PgConnection, new_resource: NewResource) -> Result<Resource> {
        diesel::insert_into(resources::table)
            .values(&new_resource)
            .returning(Resource::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(conn: &mut PgConnection, resource_id: i64) -> Result<Resource> {
        resources::table
            .filter(resources::id.eq(resource_id))
            .filter(resources::deleted_at.is_null())
            .first(conn)
            .map_err(|_| AppError::NotFound(
                format!("Resource with id {} not found", resource_id)
            ))
    }
}
```

#### 2. 处理器实现 (`src/handlers/resources.rs`)
```rust
use actix_web::{web, HttpResponse, Result};
use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::resource::{Resource, CreateResourceRequest, ResourceResponse};

#[utoipa::path(
    post,
    path = "/api/resources",
    request_body = CreateResourceRequest,
    responses(
        (status = 201, description = "Resource created", body = ResourceResponse),
        (status = 422, description = "Validation error", body = ErrorResponse)
    ),
    tag = "Resources"
)]
pub async fn create_resource(
    pool: web::Data<DbPool>,
    resource_data: web::Json<CreateResourceRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::InternalError)?;
    
    if resource_data.name.trim().is_empty() {
        return Err(AppError::ValidationError("Name is required".to_string()));
    }

    let new_resource = resource_data.into_inner().into();
    let resource = Resource::create(&mut conn, new_resource)?;
    let response = ResourceResponse::from(resource);

    Ok(HttpResponse::Created().json(response))
}
```

#### 3. 路由配置 (`src/lib.rs`)
```rust
fn configure_resource_routes() -> actix_web::Scope {
    web::scope("/resources")
        .route("", web::post().to(handlers::resources::create_resource))
        .route("", web::get().to(handlers::resources::list_resources))
        .route("/{id}", web::get().to(handlers::resources::get_resource))
        .route("/{id}", web::put().to(handlers::resources::update_resource))
        .route("/{id}", web::delete().to(handlers::resources::delete_resource))
}
```

## 检查清单

开发新 API 时，请确保：

- [ ] 遵循 RESTful 设计原则
- [ ] 使用标准 HTTP 状态码
- [ ] 实现统一的错误处理
- [ ] 包含完整的输入验证
- [ ] 支持软删除（如适用）
- [ ] 提供分页功能（列表端点）
- [ ] 编写完整的测试用例
- [ ] 添加 OpenAPI 注解
- [ ] 更新 API 文档
- [ ] 代码通过 `cargo fmt` 和 `cargo clippy`
- [ ] 所有测试通过

## 版本管理

- API 版本通过 URL 路径管理：`/api/v1/resources`
- 重大变更需要新版本
- 保持向后兼容性
- 在文档中明确版本差异

---

遵循这些规范将确保 API 的一致性、可维护性和高质量。如有疑问，请参考现有的 books API 实现作为示例。