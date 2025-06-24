# 个人读书记录系统 (Personal Reading Notes System)

一个基于 Rust 和 PostgreSQL 的个人读书记录管理系统，用于记录阅读过的书籍和管理读书笔记。

## 🚀 快速开始

### 环境要求
- Rust 1.82.0+
- PostgreSQL 17.5
- Docker & Docker Compose

### 安装步骤

1. **克隆仓库**
   ```bash
   git clone <repository-url>
   cd MyLibrary
   ```

2. **启动数据库**
   ```bash
   docker-compose up -d
   ```

3. **设置环境变量**
   ```bash
   cp .env.example .env
   # 编辑 .env 文件，设置数据库连接
   ```

4. **运行数据库迁移**
   ```bash
   diesel migration run
   ```

5. **启动应用**
   ```bash
   cargo run
   ```

6. **访问 API 文档**
   打开浏览器访问 http://localhost:8080/docs/

## 📚 文档目录

### 开发文档
- [API 开发规范指南](./API_DEVELOPMENT_GUIDE.md) - 详细的 API 开发标准和最佳实践
- [API 快速参考](./API_QUICK_REFERENCE.md) - 开发时的快速查阅手册
- [API 文档](./API.md) - 所有 API 端点的详细说明
- [Swagger 使用指南](./SWAGGER.md) - 交互式 API 文档使用说明

### 项目文档
- [需求文档](./docs/requirements/个人读书记录系统需求文档.md) - 系统需求说明
- [数据库设计](./docs/design/database/数据库设计文档.md) - 数据库架构设计
- [后端开发计划](./docs/development/backend-development-plan.md) - 开发路线图

### 其他文档
- [CLAUDE.md](./CLAUDE.md) - Claude Code AI 助手配置

## 🛠️ 技术栈

- **后端框架**: Actix-web 4.9
- **数据库**: PostgreSQL 17.5
- **ORM**: Diesel 2.2
- **API 文档**: OpenAPI 3.0 + Swagger UI
- **测试**: Actix-web test framework

## 📦 主要功能

### 已实现
- ✅ 书籍管理 (CRUD)
  - 创建、查看、更新、删除书籍
  - 分页查询和搜索
  - 软删除支持
- ✅ API 文档
  - OpenAPI 规范
  - Swagger UI 交互式文档
- ✅ 错误处理
  - 统一的错误响应格式
  - 详细的错误类型

### 计划中
- 📝 读书笔记管理
- 🏷️ 标签系统
- 📊 阅读状态跟踪
- 👤 用户认证
- 🌐 前端界面 (React)

## 🧪 测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test test_create_book
```

查看测试覆盖率：
```bash
cargo tarpaulin
```

## 🔧 开发指南

### 代码规范
```bash
# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 检查代码
cargo check
```

### 新增 API 端点
1. 参考 [API 开发规范指南](./API_DEVELOPMENT_GUIDE.md)
2. 使用 [API 快速参考](./API_QUICK_REFERENCE.md) 获取代码模板
3. 查看 `src/handlers/books.rs` 作为示例

### 项目结构
```
src/
├── handlers/     # HTTP 请求处理器
├── models/       # 数据模型
├── errors/       # 错误处理
├── db/          # 数据库配置
└── lib.rs       # 应用配置

tests/           # 集成测试
docs/           # 项目文档
```

## 🚀 部署

### Docker 部署
```bash
docker build -t reading-notes-backend .
docker run -p 8080:8080 reading-notes-backend
```

### 生产环境配置
- 设置环境变量 `RUST_ENV=production`
- 配置数据库连接池大小
- 启用 HTTPS
- 配置 CORS 策略

## 🤝 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📝 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 📞 联系方式

- 项目维护者：[Your Name]
- Email: your.email@example.com
- Issues: [GitHub Issues](https://github.com/yourusername/MyLibrary/issues)

---

⭐ 如果这个项目对你有帮助，请给个 Star！