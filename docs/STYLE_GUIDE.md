# 📋 Documentation Style Guide

> **文档格式规范指南**  
> Version: v1.0  
> Last Updated: 2025-06-25  
> Purpose: 统一项目文档格式标准

## 🎯 Overview

本指南定义了项目文档的统一格式标准，确保所有文档具有一致的结构、样式和可读性。

## 📁 File Naming Convention

### 命名规则
```
格式: kebab-case-description.md
日期格式: YYYY-MM-DD-description.md (仅限进度报告)
```

### 推荐命名
```
✅ Good Examples:
- api-endpoints.md
- database-design.md
- system-requirements.md
- 2025-06-25-progress-report.md
- backend-development-plan.md

❌ Bad Examples:
- API_Endpoints.md
- 数据库设计文档.md
- api.endpoints.md
- BackendPlan.md
```

## 📖 Document Structure

### 标准模板
```markdown
# Document Title

> **Document Info**  
> Version: v1.0  
> Last Updated: YYYY-MM-DD  
> Author: Team Name  
> Status: Draft/Review/Final

## Table of Contents
- [Section 1](#section-1)
- [Section 2](#section-2)

## Section 1

### Subsection 1.1

#### Detail Section 1.1.1

Content here...

## Section 2

Content here...

---

**Document End**  
Next Review: YYYY-MM-DD
```

### Header Hierarchy Rules
```markdown
# Document Title (h1) - Only ONE per document
## Main Section (h2) - Primary divisions
### Subsection (h3) - Secondary divisions  
#### Detail Section (h4) - Specific topics
##### Minor Detail (h5) - Rarely used
```

## 🎨 Emoji Usage Guidelines

### 核心原则
- **最小化使用**: 仅在重要章节标题使用
- **保持一致**: 相同类型内容使用相同emoji
- **语义明确**: emoji必须与内容相关

### 标准化emoji集合
```markdown
📋 规划/文档类
🛠️ 开发/技术类
📊 进度/分析类
✅ 完成/成功类
❌ 问题/错误类
🔧 配置/设置类
📚 内容/知识类
🚀 部署/启动类
⚠️ 警告/注意类
💡 技巧/想法类
```

### 使用示例
```markdown
✅ Good:
## 📊 Project Progress
## 🛠️ Development Setup
## ✅ Completed Features

❌ Bad:
## 🎉🚀📊 Project Progress 🎯✨
## Development 🔥 Setup 💪
## ✅🎊 Completed 🏆 Features 🎉
```

## 📝 Content Formatting

### 文本样式
```markdown
**粗体**: 重要概念、关键词
*斜体*: 强调、引用
`代码`: 命令、文件名、变量名
```

### 列表格式
```markdown
无序列表 (使用 -)：
- 第一项
- 第二项
  - 子项目
  - 另一个子项目

有序列表：
1. 第一步
2. 第二步
3. 第三步

任务列表：
- [x] 已完成任务
- [ ] 待完成任务
```

### 表格格式
```markdown
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Data 1   | Data 2   | Data 3   |
| Data 4   | Data 5   | Data 6   |

对齐方式：
| Left | Center | Right |
|:-----|:------:|------:|
| 左对齐 | 居中对齐 | 右对齐 |
```

## 💻 Code Block Standards

### 代码块语言标签
```markdown
# 总是指定语言
```rust
fn main() {
    println!("Hello, world!");
}
```

```sql
SELECT * FROM books WHERE deleted_at IS NULL;
```

```bash
cargo run --release
```

```json
{
  "name": "example",
  "version": "1.0.0"
}
```

```typescript
interface User {
  id: number;
  name: string;
}
```
```

### 行内代码
```markdown
使用 `cargo test` 运行测试
配置文件位于 `src/config.rs`
环境变量 `DATABASE_URL` 必须设置
```

## 🔗 Link Formatting

### 内部链接
```markdown
相对路径链接：
[API 端点文档](../design/api/api-endpoints.md)
[数据库设计](./database/database-design.md)

锚点链接：
[跳转到章节](#section-name)
```

### 外部链接
```markdown
[Rust 官方文档](https://doc.rust-lang.org/)
[Actix Web](https://actix.rs/)
```

## 📊 Special Elements

### 引用块
```markdown
> **重要提示**  
> 这是一个重要的信息提示

> **注意**  
> 执行此操作前请备份数据
```

### 状态指示器
```markdown
状态标签：
- ✅ 已完成
- 🚧 进行中  
- ❌ 已取消
- ⏳ 等待中
- 🔄 需更新
```

### 优先级标记
```markdown
优先级：
- 🔥 高优先级
- 📋 中优先级  
- 📝 低优先级
```

## 📁 Directory-Specific Guidelines

### API 文档 (`docs/design/api/`)
```markdown
# API Name

## Endpoint Information
- **Method**: GET/POST/PUT/DELETE
- **URL**: `/api/resource`
- **Auth Required**: Yes/No

## Request
```json
{
  "field": "value"
}
```

## Response
```json
{
  "status": "success",
  "data": {}
}
```
```

### 开发文档 (`docs/development/`)
```markdown
# Development Topic

## Prerequisites
- Tool 1 version X.X
- Tool 2 version Y.Y

## Setup Steps
1. First step
2. Second step
3. Third step

## Code Examples
```language
example code
```
```

### 进度报告 (`docs/project-management/progress/`)
```markdown
# YYYY-MM-DD Progress Report

## Summary
Brief overview of progress

## Completed
- [x] Task 1
- [x] Task 2

## In Progress
- [ ] Task 3
- [ ] Task 4

## Next Steps
1. Priority 1 task
2. Priority 2 task
```

## ✅ Quality Checklist

完成文档前请检查：

### 结构检查
- [ ] 只有一个 h1 标题
- [ ] Header 层级正确 (h1 > h2 > h3 > h4)
- [ ] 包含目录 (长文档)
- [ ] 包含文档信息块

### 格式检查
- [ ] 代码块有语言标签
- [ ] 链接格式正确
- [ ] 表格对齐正确
- [ ] Emoji 使用适度且一致

### 内容检查
- [ ] 信息准确完整
- [ ] 示例代码可运行
- [ ] 链接可访问
- [ ] 语法正确

## 🔧 Tools and Automation

### 推荐工具
- **Markdown Linter**: markdownlint
- **Spell Check**: cSpell
- **Link Checker**: markdown-link-check

### 自动化检查
```bash
# Lint markdown files
markdownlint docs/**/*.md

# Check spelling
cspell "docs/**/*.md"

# Verify links
markdown-link-check docs/**/*.md
```

## 📚 References

- [CommonMark Spec](https://commonmark.org/)
- [GitHub Flavored Markdown](https://github.github.com/gfm/)
- [Markdown Guide](https://www.markdownguide.org/)

---

**Style Guide Version**: v1.0  
**Next Review**: 2025-07-25  
**Maintained by**: Development Team