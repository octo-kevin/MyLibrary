# 后端开发问题报告

## 待修复问题

### 1. 笔记API筛选功能缺失

**问题描述：**
前端在请求 `/api/notes` 时传递了 `note_type` 参数进行笔记类型筛选，但后端没有处理该参数，导致筛选功能无效。

**具体表现：**
- 前端URL: `GET /api/notes?page=1&per_page=20&note_type=quote`
- 后端返回所有笔记，未按 `note_type` 筛选

**期望行为：**
- 当传递 `note_type=quote` 时，只返回类型为 `quote` 的笔记
- 当传递 `note_type=summary` 时，只返回类型为 `summary` 的笔记
- 当传递 `note_type=thought` 时，只返回类型为 `thought` 的笔记
- 当传递 `note_type=general` 时，只返回类型为 `general` 的笔记

**涉及文件：**
- `src/handlers/notes.rs` - 需要在 `get_notes` 函数中添加 `note_type` 参数处理
- 可能需要修改查询逻辑以支持按类型筛选

**建议修复方案：**
1. 在 `get_notes` 函数的查询参数中添加 `note_type` 的处理
2. 在数据库查询中添加对应的 WHERE 条件
3. 确保参数验证（只接受 `quote`、`summary`、`thought`、`general` 四个有效值）

**优先级：** 中等
**影响范围：** 笔记管理功能的用户体验

**测试方法：**
```bash
# 测试不同类型的筛选
curl "http://localhost:8080/api/notes?note_type=quote"
curl "http://localhost:8080/api/notes?note_type=summary"
curl "http://localhost:8080/api/notes?note_type=thought"
curl "http://localhost:8080/api/notes?note_type=general"
```

**报告时间：** 2025-06-25
**报告人：** 前端开发团队

---

### 2. 标签搜索功能未实现

**问题描述：**
后端标签API (`/api/tags`) 似乎没有实现搜索功能，前端传递 `search` 参数时无法按标签名称进行搜索。

**具体表现：**
- 前端URL: `GET /api/tags?page=1&per_page=20&search=编程`
- 后端返回所有标签，未按搜索关键词筛选

**期望行为：**
- 当传递 `search=编程` 时，返回标签名称包含"编程"的标签
- 支持模糊搜索（部分匹配）
- 搜索应该不区分大小写

**涉及文件：**
- `src/handlers/tags.rs` - 需要在 `get_tags` 函数中添加 `search` 参数处理
- 可能需要修改查询逻辑以支持按名称搜索

**建议修复方案：**
1. 在 `get_tags` 函数的查询参数中添加 `search` 的处理
2. 在数据库查询中添加对应的 WHERE 条件 (如 `name ILIKE %search%`)
3. 确保搜索支持中文和英文

**优先级：** 中等
**影响范围：** 标签管理功能的用户体验

**测试方法：**
```bash
# 测试标签搜索功能
curl "http://localhost:8080/api/tags?search=编程"
curl "http://localhost:8080/api/tags?search=test"
curl "http://localhost:8080/api/tags?search=技术"
```

**报告时间：** 2025-06-25
**报告人：** 前端开发团队