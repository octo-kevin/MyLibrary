# 个人读书记录系统 API 文档

## 概述

本文档描述了个人读书记录系统的RESTful API端点。API基于JSON格式，支持标准的HTTP方法。

## 基础信息

- **Base URL**: `http://localhost:8080/api`
- **Content-Type**: `application/json`
- **响应格式**: JSON

## 书籍管理 API

### 1. 创建书籍

**请求**
```
POST /api/books
Content-Type: application/json

{
  "isbn": "978-0134685991",           // 可选，ISBN号
  "title": "Effective Java",          // 必填，书名
  "author": "Joshua Bloch",           // 必填，作者
  "publisher": "Addison-Wesley",      // 可选，出版社
  "publication_date": "2017-12-27",   // 可选，出版日期 (YYYY-MM-DD)
  "page_count": 416,                  // 可选，页数
  "cover_image": "http://...",        // 可选，封面图片URL
  "description": "Best practices..."  // 可选，描述
}
```

**响应**
```json
{
  "id": 1,
  "isbn": "978-0134685991",
  "title": "Effective Java",
  "author": "Joshua Bloch",
  "publisher": "Addison-Wesley",
  "publication_date": "2017-12-27",
  "page_count": 416,
  "cover_image": "http://...",
  "description": "Best practices for the Java platform",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

**状态码**
- `201 Created`: 创建成功
- `422 Unprocessable Entity`: 验证失败

### 2. 获取书籍详情

**请求**
```
GET /api/books/{id}
```

**响应**
```json
{
  "id": 1,
  "isbn": "978-0134685991",
  "title": "Effective Java",
  "author": "Joshua Bloch",
  "publisher": "Addison-Wesley",
  "publication_date": "2017-12-27",
  "page_count": 416,
  "cover_image": "http://...",
  "description": "Best practices for the Java platform",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

**状态码**
- `200 OK`: 获取成功
- `404 Not Found`: 书籍不存在

### 3. 获取书籍列表

**请求**
```
GET /api/books?page=1&per_page=20&search=java
```

**查询参数**
- `page`: 页码（默认: 1）
- `per_page`: 每页数量（默认: 20，最大: 100）
- `search`: 搜索关键词（搜索标题和作者）

**响应**
```json
{
  "books": [
    {
      "id": 1,
      "isbn": "978-0134685991",
      "title": "Effective Java",
      "author": "Joshua Bloch",
      "publisher": "Addison-Wesley",
      "publication_date": "2017-12-27",
      "page_count": 416,
      "cover_image": "http://...",
      "description": "Best practices for the Java platform",
      "created_at": "2024-01-01T12:00:00Z",
      "updated_at": "2024-01-01T12:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "per_page": 20,
  "total_pages": 1
}
```

**状态码**
- `200 OK`: 获取成功

### 4. 更新书籍

**请求**
```
PUT /api/books/{id}
Content-Type: application/json

{
  "title": "Updated Title",        // 可选，更新标题
  "description": "New description" // 可选，更新描述
  // 只需要包含要更新的字段
}
```

**响应**
```json
{
  "id": 1,
  "isbn": "978-0134685991",
  "title": "Updated Title",
  "author": "Joshua Bloch",
  "publisher": "Addison-Wesley",
  "publication_date": "2017-12-27",
  "page_count": 416,
  "cover_image": "http://...",
  "description": "New description",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T13:00:00Z"
}
```

**状态码**
- `200 OK`: 更新成功
- `404 Not Found`: 书籍不存在
- `422 Unprocessable Entity`: 验证失败

### 5. 删除书籍（软删除）

**请求**
```
DELETE /api/books/{id}
```

**响应**
无响应体

**状态码**
- `204 No Content`: 删除成功
- `404 Not Found`: 书籍不存在

## 通用API

### 健康检查

**请求**
```
GET /api/health
```

**响应**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

## 错误响应格式

所有错误响应都遵循统一格式：

```json
{
  "error": "ERROR_TYPE",
  "message": "Human readable error message"
}
```

**错误类型**
- `VALIDATION_ERROR`: 输入验证失败（如必填字段为空）
- `NOT_FOUND`: 资源不存在（如书籍ID不存在）
- `BAD_REQUEST`: 请求格式错误（如无效参数）
- `JSON_ERROR`: JSON格式错误（如语法错误）
- `DATABASE_ERROR`: 数据库操作失败（如连接失败）
- `INTERNAL_ERROR`: 服务器内部错误（如未预期的程序错误）
- `CONFIGURATION_ERROR`: 配置错误（如环境变量缺失）

**特殊说明**
- JSON解析错误由框架处理，错误格式可能略有不同
- 路径参数验证由框架自动处理（如非数字的book ID）

## 使用示例

### 创建并获取书籍
```bash
# 创建书籍
curl -X POST http://localhost:8080/api/books \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Clean Code",
    "author": "Robert C. Martin",
    "isbn": "978-0132350884",
    "publisher": "Prentice Hall"
  }'

# 获取书籍列表
curl http://localhost:8080/api/books

# 搜索书籍
curl "http://localhost:8080/api/books?search=clean&page=1&per_page=10"

# 获取特定书籍
curl http://localhost:8080/api/books/1

# 更新书籍
curl -X PUT http://localhost:8080/api/books/1 \
  -H "Content-Type: application/json" \
  -d '{"description": "A handbook of agile software craftsmanship"}'

# 删除书籍
curl -X DELETE http://localhost:8080/api/books/1
```

## 数据验证规则

### 书籍字段验证
- `title`: 必填，不能为空字符串
- `author`: 必填，不能为空字符串
- `isbn`: 可选，建议使用标准ISBN格式
- `page_count`: 可选，必须为正整数
- `publication_date`: 可选，格式为YYYY-MM-DD

### 分页参数验证
- `page`: 最小值为1
- `per_page`: 范围为1-100

## 注意事项

1. **软删除**: 删除操作使用软删除，数据在数据库中保留但标记为已删除
2. **分页**: 所有列表接口都支持分页，建议使用合理的页面大小
3. **搜索**: 搜索功能支持模糊匹配，不区分大小写
4. **时间格式**: 所有时间字段使用ISO 8601格式（UTC时间）