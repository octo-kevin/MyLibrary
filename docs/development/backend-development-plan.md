# 后端开发路径规划

## 开发环境准备

### 必需工具
- Rust 1.87.0 (当前已安装版本)
- PostgreSQL 17.5 (最新稳定版)
- Docker & Docker Compose (用于数据库)
- cargo-watch (热重载开发)
- diesel_cli (数据库迁移工具)

### 安装命令
```bash
# 安装 cargo-watch
cargo install cargo-watch

# 安装 diesel_cli (只支持 PostgreSQL)
cargo install diesel_cli --no-default-features --features postgres

# 安装 cargo-edit (方便添加依赖)
cargo install cargo-edit
```

## 第一阶段：项目基础搭建（2-3天）

### 1.1 项目结构初始化
```
backend/
├── src/
│   ├── main.rs              # 程序入口
│   ├── lib.rs              # 库根文件
│   ├── config/             # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── db/                 # 数据库相关
│   │   ├── mod.rs
│   │   ├── connection.rs   # 连接池管理
│   │   └── schema.rs       # 数据库 schema
│   ├── models/             # 数据模型
│   │   ├── mod.rs
│   │   ├── book.rs
│   │   ├── category.rs
│   │   ├── tag.rs
│   │   └── note.rs
│   ├── handlers/           # API处理器
│   │   ├── mod.rs
│   │   ├── books.rs
│   │   ├── categories.rs
│   │   ├── tags.rs
│   │   └── notes.rs
│   ├── middleware/         # 中间件
│   │   ├── mod.rs
│   │   ├── cors.rs
│   │   └── logger.rs
│   ├── errors/            # 错误处理
│   │   ├── mod.rs
│   │   └── handlers.rs
│   └── utils/             # 工具函数
│       ├── mod.rs
│       └── pagination.rs
├── migrations/            # 数据库迁移文件
├── tests/                # 集成测试
├── Cargo.toml
├── .env.example
├── diesel.toml
└── README.md
```

### 1.2 核心依赖配置 (Cargo.toml)
```toml
[package]
name = "reading-notes-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web框架
actix-web = "4"
actix-cors = "0.7"

# 数据库
diesel = { version = "2.1", features = ["postgres", "chrono", "r2d2"] }
diesel-async = { version = "0.4", features = ["postgres"] }
r2d2 = "0.8"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 环境变量
dotenv = "0.15"

# 日志
env_logger = "0.11"
log = "0.4"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 验证
validator = { version = "0.16", features = ["derive"] }

[dev-dependencies]
actix-rt = "2"
```

### 1.3 环境配置文件 (.env)
```env
# 数据库配置
DATABASE_URL=postgres://username:password@localhost/reading_notes
DATABASE_POOL_SIZE=10

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# 日志级别
RUST_LOG=debug

# 分页默认值
DEFAULT_PAGE_SIZE=20
MAX_PAGE_SIZE=100
```

## 第二阶段：数据库层实现（3-4天）

### 2.1 数据库连接池设置
- 使用 r2d2 连接池管理 PostgreSQL 连接
- 实现连接池的依赖注入
- 配置连接池参数（最大连接数、超时等）

### 2.2 Diesel Schema 生成
```bash
# 运行迁移
diesel setup
diesel migration generate create_books
# 编辑迁移文件...
diesel migration run

# 生成 schema
diesel print-schema > src/db/schema.rs
```

### 2.3 数据模型实现
- 为每个表创建对应的 Rust 结构体
- 实现 Queryable 和 Insertable trait
- 创建 NewBook、UpdateBook 等 DTO 结构体
- 实现软删除 trait

### 2.4 Repository 层
- 实现通用的 CRUD trait
- 为每个模型实现具体的 Repository
- 实现复杂查询（分页、搜索、过滤）
- 实现事务支持

## 第三阶段：API层开发（4-5天）

### 3.1 路由设计
```rust
// 书籍相关路由
/api/books
    GET    /           # 获取书籍列表
    POST   /           # 创建书籍
    GET    /{id}       # 获取书籍详情
    PUT    /{id}       # 更新书籍
    DELETE /{id}       # 删除书籍（软删除）
    GET    /{id}/notes # 获取书籍的笔记

// 分类路由
/api/categories
    GET    /           # 获取分类列表（树形结构）
    POST   /           # 创建分类
    PUT    /{id}       # 更新分类
    DELETE /{id}       # 删除分类

// 标签路由
/api/tags
    GET    /           # 获取标签列表
    POST   /           # 创建标签
    GET    /popular    # 获取热门标签

// 笔记路由
/api/notes
    GET    /           # 获取笔记列表
    POST   /           # 创建笔记
    PUT    /{id}       # 更新笔记
    DELETE /{id}       # 删除笔记

// 搜索路由
/api/search
    GET    /books      # 搜索书籍
    GET    /notes      # 搜索笔记

// 统计路由
/api/stats
    GET    /summary    # 获取统计概览
    GET    /timeline   # 获取阅读时间线
```

### 3.2 请求/响应结构设计
- 统一的响应格式包装
- 分页响应结构
- 错误响应格式
- 请求验证

### 3.3 Handler 实现
- 每个资源一个 handler 模块
- 实现参数验证
- 调用 Repository 层
- 处理业务逻辑

## 第四阶段：中间件和公共功能（2-3天）

### 4.1 CORS 配置
- 配置允许的源、方法、头部
- 开发环境和生产环境差异化配置

### 4.2 日志中间件
- 请求日志记录
- 响应时间统计
- 错误日志记录

### 4.3 错误处理
- 自定义错误类型
- 统一错误响应格式
- 错误码设计

### 4.4 工具函数
- 分页参数提取
- 搜索参数处理
- 日期格式化

## 第五阶段：高级功能（3-4天）

### 5.1 全文搜索实现
- PostgreSQL 全文搜索配置
- 中文分词支持
- 搜索结果高亮

### 5.2 统计功能
- 阅读统计计算
- 分类统计
- 时间线数据生成

### 5.3 数据导入导出
- CSV 导入功能
- JSON 导出功能
- 批量操作优化

### 5.4 性能优化
- 查询优化
- 缓存策略（可选）
- 数据库索引优化

## 第六阶段：测试和文档（2-3天）

### 6.1 单元测试
- Repository 层测试
- 工具函数测试
- 模型验证测试

### 6.2 集成测试
- API 端到端测试
- 数据库事务测试
- 错误场景测试

### 6.3 API 文档
- OpenAPI 规范文档
- 使用 utoipa 生成 Swagger 文档
- 请求/响应示例

### 6.4 部署准备
- Dockerfile 编写
- docker-compose.yml 配置
- 环境变量管理
- 健康检查端点

## 开发顺序建议

1. **基础设施**（第1周）
   - 项目结构搭建
   - 数据库连接配置
   - 基础中间件

2. **核心功能**（第2周）
   - 书籍 CRUD
   - 分类和标签管理
   - 基础搜索

3. **笔记功能**（第3周）
   - 笔记 CRUD
   - 笔记搜索
   - 标签关联

4. **完善和优化**（第4周）
   - 统计功能
   - 性能优化
   - 测试完善

## 技术要点

### Diesel 使用技巧
1. 使用 `diesel::prelude::*` 导入常用 trait
2. 软删除使用 `filter(deleted_at.is_null())`
3. 分页使用 `limit()` 和 `offset()`
4. 使用 `diesel::result::QueryResult` 处理查询结果

### Actix-web 最佳实践
1. 使用 `web::Data` 共享应用状态
2. 使用 `web::Json` 自动序列化/反序列化
3. 使用 `actix_web::Result` 简化错误处理
4. 合理使用中间件顺序

### 性能考虑
1. 使用连接池避免频繁建立连接
2. 合理设置分页大小限制
3. 使用索引优化查询
4. 避免 N+1 查询问题

## Docker Compose 开发环境

### PostgreSQL 容器配置
```yaml
# docker-compose.yml
version: '3.8'
services:
  postgres:
    image: postgres:17.5
    environment:
      POSTGRES_DB: reading_notes
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./sql/init.sql:/docker-entrypoint-initdb.d/init.sql
    command: >
      postgres
      -c shared_preload_libraries=pg_stat_statements
      -c pg_stat_statements.track=all
      -c max_connections=200

volumes:
  postgres_data:
```

### 快速启动开发环境
```bash
# 启动数据库
docker-compose up -d postgres

# 验证连接
psql -h localhost -U postgres -d reading_notes -c "SELECT version();"

# 运行迁移
diesel migration run

# 启动开发服务器
cargo watch -x run
```

---

**文档版本**: 1.1  
**创建日期**: 2025-06-22  
**最后更新**: 2025-06-22  
**适用版本**: Rust 1.87.0, PostgreSQL 17.5, Actix-web 4.0+, Diesel 2.1+