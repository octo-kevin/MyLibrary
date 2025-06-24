# 🚀 Swagger API 文档使用指南

## 概述

本项目已集成 Swagger UI，提供交互式的 API 文档界面，让您可以直接在浏览器中测试所有的书籍管理 API。

## 🌐 访问 Swagger UI

### 1. 启动服务器
```bash
# 确保 PostgreSQL 容器运行
docker-compose up -d

# 启动 API 服务器
cargo run
```

### 2. 打开 Swagger UI
在浏览器中访问：
```
http://localhost:8080/docs/
```

### 3. 访问 OpenAPI 规范
如果您需要原始的 OpenAPI JSON 规范：
```
http://localhost:8080/api-docs/openapi.json
```

## 📚 API 文档功能

### 🔍 浏览 API 端点
- **书籍管理 (Books)**: 完整的 CRUD 操作
  - `POST /api/books` - 创建新书籍
  - `GET /api/books` - 获取书籍列表（支持分页和搜索）
  - `GET /api/books/{id}` - 获取特定书籍详情
  - `PUT /api/books/{id}` - 更新书籍信息
  - `DELETE /api/books/{id}` - 删除书籍（软删除）

### 🧪 交互式测试
1. **点击任意端点**展开详细信息
2. **点击 "Try it out"** 按钮
3. **填写参数**（如需要）
4. **点击 "Execute"** 执行请求
5. **查看响应**结果

### 📖 查看详细信息
每个端点都包含：
- **请求参数**说明和示例
- **请求体**结构（JSON Schema）
- **响应格式**说明
- **HTTP 状态码**说明
- **错误响应**格式

## 🔧 API 使用示例

### 创建书籍
```http
POST /api/books
Content-Type: application/json

{
  "title": "Rust程序设计语言",
  "author": "Steve Klabnik",
  "isbn": "978-7121323683",
  "publisher": "人民邮电出版社",
  "publication_date": "2018-01-01",
  "page_count": 600,
  "description": "系统学习Rust编程语言"
}
```

### 搜索书籍
```http
GET /api/books?search=rust&page=1&per_page=20
```

### 更新书籍
```http
PUT /api/books/1
Content-Type: application/json

{
  "description": "更新后的描述"
}
```

## 📋 数据模型说明

### CreateBookRequest
创建书籍的请求结构：
- `title` (必填): 书名
- `author` (必填): 作者
- `isbn` (可选): ISBN 号
- `publisher` (可选): 出版社
- `publication_date` (可选): 出版日期
- `page_count` (可选): 页数
- `cover_image` (可选): 封面图片 URL
- `description` (可选): 描述

### BookResponse
书籍响应结构：
- `id`: 唯一标识符
- 包含所有书籍字段
- `created_at`: 创建时间
- `updated_at`: 更新时间

### BookListResponse
分页书籍列表响应：
- `books`: 书籍数组
- `total`: 总书籍数
- `page`: 当前页
- `per_page`: 每页数量
- `total_pages`: 总页数

### ErrorResponse
错误响应结构：
- `error`: 错误类型代码
- `message`: 错误详细信息

## 🔒 错误代码说明

| 错误代码 | HTTP状态码 | 说明 |
|----------|------------|------|
| `VALIDATION_ERROR` | 422 | 输入验证失败 |
| `NOT_FOUND` | 404 | 资源不存在 |
| `BAD_REQUEST` | 400 | 请求格式错误 |
| `DATABASE_ERROR` | 500 | 数据库操作失败 |
| `INTERNAL_ERROR` | 500 | 服务器内部错误 |
| `CONFIGURATION_ERROR` | 500 | 配置错误 |

## 🛠️ 开发者工具

### 导出 OpenAPI 规范
您可以将 OpenAPI 规范用于：
- **代码生成**: 生成客户端 SDK
- **API 测试**: 集成到自动化测试
- **文档生成**: 生成其他格式的文档

### Postman 集合
可以从 OpenAPI 规范导入到 Postman：
1. 打开 Postman
2. 选择 "Import"
3. 输入 URL: `http://localhost:8080/api-docs/openapi.json`

### 其他工具集成
- **Insomnia**: 支持 OpenAPI 导入
- **VS Code**: 使用 REST Client 扩展
- **curl**: 通过 Swagger UI 生成 curl 命令

## 🚀 生产环境部署

### 环境配置
在生产环境中，您可能需要：
1. **禁用 Swagger UI**（出于安全考虑）
2. **配置 CORS**设置
3. **设置认证**机制

### 安全注意事项
- Swagger UI 暴露了 API 结构，在生产环境中考虑访问控制
- 确保敏感信息不出现在示例中
- 定期更新文档与实际 API 保持同步

## 📞 技术支持

如果您在使用 Swagger 文档时遇到问题：
1. 检查服务器是否正常运行
2. 确认数据库连接正常
3. 查看服务器日志获取详细错误信息
4. 在 GitHub Issues 中报告问题

## 🔄 自动化集成

### CI/CD 集成
可以在 CI/CD 流水线中：
- 验证 OpenAPI 规范的有效性
- 生成客户端代码
- 运行 API 兼容性测试

### 文档同步
每次 API 更改时，Swagger 文档会自动更新，确保文档与代码同步。

---

享受使用交互式 API 文档！🎉