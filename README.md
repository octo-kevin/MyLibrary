# 个人读书记录系统 (Personal Reading Notes System)

一个现代化的全栈个人读书记录管理系统，基于 Rust + React 构建，用于记录阅读过的书籍和管理读书笔记。

## 🌟 项目特色

- 🚀 **现代化技术栈**：Rust + Actix-web + PostgreSQL + React 19 + Ant Design
- 📱 **响应式设计**：完美适配桌面端和移动端
- 🎨 **优雅 UI**：基于 Ant Design 的现代化用户界面
- 🔒 **类型安全**：TypeScript + Rust 双重类型保障
- 📊 **完整功能**：书籍管理、笔记记录、标签分类、搜索筛选
- 🧪 **高测试覆盖率**：70个自动化测试，100% 通过率

## 🚀 快速开始

### 环境要求
- **Rust** 1.82.0+
- **Node.js** 18+ (推荐使用 pnpm)
- **PostgreSQL** 17.5
- **Docker & Docker Compose**

### 一键启动

1. **克隆仓库**
   ```bash
   git clone <repository-url>
   cd MyLibrary
   ```

2. **启动完整环境**
   ```bash
   # 启动数据库
   docker-compose up -d
   
   # 运行数据库迁移
   diesel migration run
   
   # 启动后端 (终端1)
   cargo run
   
   # 启动前端 (终端2)
   cd frontend && pnpm dev
   ```

3. **访问应用**
   - 🖥️ **前端界面**: http://localhost:5173
   - 📚 **API 文档**: http://localhost:8080/docs
   - 🔧 **后端 API**: http://localhost:8080/api

## 🛠️ 技术栈

### 后端 (Rust)
- **框架**: Actix-web 4.9
- **数据库**: PostgreSQL 17.5 + Diesel ORM 2.2
- **文档**: OpenAPI 3.0 + Swagger UI
- **测试**: 70个集成测试 (100% 通过率)

### 前端 (React)
- **框架**: React 19 + TypeScript 5.8
- **构建**: Vite 7.0 (快速开发和构建)
- **UI 库**: Ant Design 5.26.2 + Icons
- **状态管理**: @tanstack/react-query
- **路由**: React Router v7
- **包管理**: pnpm

## 📦 核心功能

### ✅ 已完成功能 (95% 完成度)

#### 📚 书籍管理
- ✅ 书籍 CRUD 操作 (创建、查看、更新、删除)
- ✅ 分页查询和全文搜索
- ✅ 书籍详情页面和编辑表单
- ✅ 关联笔记展示

#### 📝 读书笔记
- ✅ 多类型笔记 (摘录、总结、感想、一般)
- ✅ 富文本内容编辑
- ✅ 笔记与书籍关联
- ✅ 页码引用支持
- ✅ 收藏功能
- 🔧 类型筛选 (待后端修复)

#### 🏷️ 标签系统
- ✅ 标签 CRUD 操作
- ✅ 智能标签关联
- ✅ 热门标签统计
- ✅ 标签使用量追踪
- 🔧 标签搜索 (待后端修复)

#### 🎨 用户界面
- ✅ 现代化 Ant Design 设计语言
- ✅ 响应式布局 (桌面端/移动端)
- ✅ 可折叠侧边栏导航
- ✅ 优雅的加载和错误状态
- ✅ 一致的交互体验

### 🔧 待完成功能 (5%)
- 📊 阅读状态跟踪模块
- 🔍 笔记类型筛选功能 (后端)
- 🔍 标签搜索功能 (后端)

## 📊 API 接口 (19个端点)

### 书籍 API (6个)
```
POST   /api/books           创建书籍
GET    /api/books           书籍列表 (分页+搜索)
GET    /api/books/{id}      书籍详情
PUT    /api/books/{id}      更新书籍
DELETE /api/books/{id}      删除书籍
GET    /api/books/{id}/notes 书籍笔记
```

### 笔记 API (7个)
```
POST   /api/notes           创建笔记
GET    /api/notes           笔记列表 (分页+搜索+筛选)
GET    /api/notes/{id}      笔记详情
PUT    /api/notes/{id}      更新笔记
DELETE /api/notes/{id}      删除笔记
PUT    /api/notes/{id}/tags 更新笔记标签
```

### 标签 API (6个)
```
POST   /api/tags            创建标签
GET    /api/tags            标签列表 (分页+搜索)
GET    /api/tags/{id}       标签详情
PUT    /api/tags/{id}       更新标签
DELETE /api/tags/{id}       删除标签
GET    /api/tags/popular    热门标签
```

## 🗄️ 数据库设计

### 核心表结构
- **books** - 书籍信息 (ISBN、标题、作者、出版社等)
- **reading_notes** - 读书笔记 (4种类型，支持页码引用)
- **tags** - 标签系统 (自动生成slug，使用量统计)
- **note_tag_associations** - 笔记标签关联 (多对多)

### 设计特色
- 🗃️ **BIGINT 主键** - 针对 PostgreSQL 优化
- 🔄 **软删除** - 使用 `deleted_at` 时间戳
- ⏰ **自动时间戳** - `created_at` 和 `updated_at`
- 🏷️ **智能标签** - 自动创建和使用量追踪

## 🧪 测试与质量

### 后端测试
```bash
# 运行所有测试 (70个测试)
cargo test

# 特定模块测试
cargo test --test note_api_test
cargo test --test tag_api_test

# 代码质量检查
cargo fmt && cargo clippy
```

### 前端开发
```bash
cd frontend

# 开发服务器
pnpm dev

# 类型检查
pnpm type-check

# 代码检查
pnpm lint

# 生产构建
pnpm build
```

## 📁 项目结构

```
MyLibrary/
├── src/                    # Rust 后端源码
│   ├── handlers/          # API 处理器
│   ├── models/            # 数据模型
│   ├── db/                # 数据库配置
│   └── lib.rs             # 应用入口
├── frontend/               # React 前端
│   ├── src/
│   │   ├── components/    # React 组件
│   │   ├── pages/         # 页面组件
│   │   ├── lib/           # API 客户端
│   │   └── main.tsx       # 前端入口
│   └── package.json
├── tests/                  # 集成测试 (70个)
├── docs/                   # 项目文档 (11个文件)
├── migrations/             # 数据库迁移
└── docker-compose.yml      # Docker 配置
```

## 📚 文档目录

### 📖 核心文档
- [CLAUDE.md](./CLAUDE.md) - AI 助手配置和项目指南
- [API 端点文档](./docs/design/api/api-endpoints.md) - 完整 API 说明
- [数据库设计](./docs/design/database/数据库设计文档.md) - 数据架构
- [需求文档](./docs/requirements/个人读书记录系统需求文档.md) - 系统需求

### 🛠️ 开发文档
- [API 开发规范](./docs/development/coding-standards/api-development-guide.md)
- [后端开发计划](./docs/development/backend-development-plan.md)
- [前端开发计划](./docs/development/frontend-development-plan.md)
- [后端问题跟踪](./docs/development/BACKEND_ISSUES.md)

### 📊 进度报告
- [项目进度总结](./docs/project-management/progress/2025-06-25-frontend-ant-design-migration-completion.md)
- [文档索引](./docs/README.md) - 完整文档目录

## 🚀 部署指南

### 开发环境
```bash
# 完整启动流程
docker-compose up -d          # 数据库
diesel migration run          # 迁移
cargo run                     # 后端 :8080
cd frontend && pnpm dev       # 前端 :5173
```

### 生产环境
```bash
# 后端构建
cargo build --release

# 前端构建
cd frontend && pnpm build

# Docker 部署
docker build -t reading-notes .
```

## 🤝 贡献指南

1. Fork 项目并创建功能分支
2. 遵循现有代码规范和测试要求
3. 更新相关文档
4. 确保所有测试通过
5. 提交 Pull Request

### 开发规范
- **后端**: 使用 `cargo fmt` 和 `cargo clippy`
- **前端**: 使用 ESLint 和 TypeScript
- **提交**: 遵循 Conventional Commits 规范

## 📈 当前状态

| 模块 | 进度 | 状态 |
|------|------|------|
| 后端 API | 95% | ✅ 功能完整，待修复筛选功能 |
| 前端界面 | 100% | ✅ 完全就绪，现代化设计 |
| 数据库 | 100% | ✅ 完整设计，70个测试通过 |
| 文档 | 100% | ✅ 11个文档，3983行内容 |
| **总进度** | **95%** | 🚀 **即将完成** |

## 📞 支持与反馈

- 📝 **问题反馈**: GitHub Issues
- 📧 **技术交流**: 开发者邮箱
- 📋 **功能建议**: Pull Request 欢迎

---

⭐ **如果这个项目对你有帮助，请给个 Star！**