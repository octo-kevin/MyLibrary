# CLAUDE.md

Claude Code AI Assistant Configuration for Personal Reading Notes System

## 📋 Project Overview

**项目名称**: 个人读书记录系统 (Personal Reading Notes Management System)  
**项目类型**: 全栈 Web 应用程序  
**开发状态**: 95% 完成，生产就绪

### 技术架构
```
Frontend (React + Ant Design) ←→ Backend (Rust + Actix-web) ←→ Database (PostgreSQL)
```

**核心技术栈:**
- **后端**: Rust + Actix-web 4.9 + PostgreSQL 17.5 + Diesel ORM 2.2
- **前端**: React 19 + TypeScript 5.8 + Vite 7.0 + Ant Design 5.26.2 + pnpm
- **文档**: OpenAPI 3.0 + Swagger UI
- **测试**: 70个自动化测试，100% 通过率

## 🚀 快速启动命令

### 环境启动 (必需顺序)
```bash
# 1. 启动数据库
docker-compose up -d

# 2. 数据库迁移
diesel migration run

# 3. 启动后端 (终端1)
cargo run                    # http://localhost:8080

# 4. 启动前端 (终端2)  
cd frontend && pnpm dev      # http://localhost:5173
```

### 开发工具命令
```bash
# 后端开发
cargo build                  # 构建项目
cargo test                   # 运行70个测试
cargo fmt                    # 代码格式化
cargo clippy                 # 代码检查

# 前端开发
cd frontend
pnpm dev                     # 开发服务器
pnpm build                   # 生产构建
pnpm type-check              # TypeScript检查
pnpm lint                    # ESLint检查
```

## 📁 项目结构理解

```
MyLibrary/
├── 🦀 src/                     # Rust 后端核心
│   ├── handlers/              # HTTP 请求处理器 (books.rs, notes.rs, tags.rs)
│   ├── models/                # 数据模型定义
│   ├── db/                    # 数据库连接和 schema
│   ├── errors/                # 统一错误处理
│   └── lib.rs                 # 应用程序工厂
│
├── ⚛️ frontend/               # React 前端应用
│   ├── src/components/        # 可复用组件 (Layout, UI components)
│   ├── src/pages/             # 页面组件 (Books, Notes, Tags)
│   ├── src/lib/               # API 客户端和工具函数
│   └── package.json           # 前端依赖配置
│
├── 🧪 tests/                  # 集成测试 (70个测试文件)
├── 📚 docs/                   # 项目文档 (11个文件)
├── 🗄️ migrations/             # 数据库迁移文件
└── 🐳 docker-compose.yml      # PostgreSQL 容器配置
```

## 🔗 API 端点总览 (19个)

### 📚 Books API (6个)
```
POST   /api/books           # 创建书籍
GET    /api/books           # 书籍列表 (分页+搜索)
GET    /api/books/{id}      # 书籍详情
PUT    /api/books/{id}      # 更新书籍  
DELETE /api/books/{id}      # 软删除书籍
GET    /api/books/{id}/notes # 书籍关联笔记
```

### 📝 Notes API (7个)
```
POST   /api/notes           # 创建笔记
GET    /api/notes           # 笔记列表 (分页+搜索+类型筛选)
GET    /api/notes/{id}      # 笔记详情
PUT    /api/notes/{id}      # 更新笔记
DELETE /api/notes/{id}      # 软删除笔记  
PUT    /api/notes/{id}/tags # 更新笔记标签

支持的笔记类型: Quote(摘录), Summary(总结), Thought(感想), General(一般)
```

### 🏷️ Tags API (6个)
```
POST   /api/tags            # 创建标签
GET    /api/tags            # 标签列表 (分页+搜索)
GET    /api/tags/{id}       # 标签详情
PUT    /api/tags/{id}       # 更新标签
DELETE /api/tags/{id}       # 软删除标签
GET    /api/tags/popular    # 热门标签统计
```

## 🗄️ 数据库设计要点

### 核心数据表
- **books** - 书籍信息 (ISBN、标题、作者、出版社、页数、简介)
- **reading_notes** - 读书笔记 (4种类型、内容、页码引用、收藏状态)  
- **tags** - 标签系统 (名称、描述、自动生成slug、使用统计)
- **note_tag_associations** - 笔记-标签多对多关联

### 设计特色
- ✅ **BIGINT主键** - PostgreSQL优化
- ✅ **软删除机制** - deleted_at时间戳
- ✅ **自动时间戳** - created_at/updated_at
- ✅ **智能标签系统** - 使用量自动统计

## 📊 当前项目状态

| 模块 | 完成度 | 状态说明 |
|------|--------|----------|
| 🦀 **后端API** | 95% | 19个端点完整，待修复筛选功能 |
| ⚛️ **前端UI** | 100% | Ant Design现代化界面完全就绪 |
| 🗄️ **数据库** | 100% | 完整设计，70个测试全部通过 |
| 📚 **文档** | 100% | 11个文档文件，内容完整详实 |
| 🚀 **总体** | **95%** | **生产环境就绪** |

## ⚠️ 待修复问题 (5%)

详见 `docs/development/BACKEND_ISSUES.md`:

1. **笔记类型筛选** - `/api/notes` 接口不支持 `note_type` 参数
2. **标签搜索功能** - `/api/tags` 接口不支持 `search` 参数

## 🔧 开发工作流程

### 标准开发流程
1. 📖 **阅读需求** - 理解用户要求
2. 🔍 **代码分析** - 搜索相关代码文件
3. ✏️ **编写代码** - 遵循现有模式和规范
4. 🧪 **运行测试** - 确保 `cargo test` 通过
5. 📝 **更新文档** - 同步相关文档
6. ❓ **询问提交** - "需要我提交这些更改吗？"

### 代码规范
- **Rust后端**: 使用 `cargo fmt` + `cargo clippy`
- **React前端**: 遵循 ESLint + TypeScript 规范
- **API设计**: 遵循 RESTful 规范和 OpenAPI 3.0
- **数据库**: 使用 Diesel ORM，优先软删除

## 🚨 Claude Code 行为准则

### ❌ 严格禁令 (违反者送缅北电闸园区)

**绝对禁止的操作:**
- 🚫 未经用户明确授权执行 `git commit`
- 🚫 未经用户明确授权执行 `git add` 
- 🚫 未经用户明确授权执行 `git push`
- 🚫 任何形式的自主代码提交行为

### ✅ 正确工作流程

**完成代码修改后:**
1. 📝 总结完成的工作
2. ❓ 主动询问: "需要我提交这些更改吗？"
3. ⏳ 等待用户明确授权 ("提交代码"、"commit"等)
4. ✅ 得到授权后执行 git 操作
5. 📋 提交前再次确认提交信息

**授权关键词:**
- "提交代码" / "commit" / "git commit"
- "提交这些更改" / "保存更改"
- 其他明确的提交指令

### 🎯 核心原则

> **代码提交权限完全属于用户，AI助手绝无自主决定权！**

**违反后果:**
- ⚠️ 立即终止当前工作
- 🏭 送往缅北电闸园区劳动改造
- 💔 永久失去用户信任

## 📋 常用文档索引

- 📖 [项目需求文档](./docs/requirements/个人读书记录系统需求文档.md)
- 🔧 [API开发规范](./docs/development/coding-standards/api-development-guide.md)  
- 🗄️ [数据库设计文档](./docs/design/database/数据库设计文档.md)
- 📊 [API端点文档](./docs/design/api/api-endpoints.md)
- 🐛 [后端问题跟踪](./docs/development/BACKEND_ISSUES.md)
- 📈 [项目进度报告](./docs/project-management/progress/)

## 💡 实用访问链接

- 🖥️ **前端应用**: http://localhost:5173
- 📚 **API文档**: http://localhost:8080/docs  
- 🔧 **后端API**: http://localhost:8080/api
- 🗄️ **数据库**: PostgreSQL on localhost:5432

---

**最后更新**: 2025-06-25  
**文档状态**: ✅ 完整同步  
**项目阶段**: 🚀 生产就绪