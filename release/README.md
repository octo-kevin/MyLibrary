# 读书笔记管理系统 - 快速部署指南

## 系统要求

- Docker 20.10+
- Docker Compose 2.0+ 或 Docker Compose V1
- 最少 1GB 可用内存
- 端口 8080（可配置）

## 快速开始

### 1. 配置系统

编辑 `.env` 文件，设置数据库密码和其他配置：

```bash
# 编辑配置文件
cp .env.example .env
nano .env  # 或使用 vim .env
```

**重要配置项：**
- `DB_PASSWORD`: 数据库密码（请修改默认值）
- `APP_PORT`: 应用端口（默认 8080）

### 2. 启动服务

```bash
./start.sh
```

启动脚本会自动：
- 检查 Docker 环境
- 拉取最新镜像
- 启动数据库和应用服务
- 等待服务就绪

### 3. 访问系统

- 主页: http://localhost:8080
- API文档: http://localhost:8080/docs

## 常用操作

### 查看服务状态
```bash
docker-compose ps
```

### 查看日志
```bash
# 查看所有日志
docker-compose logs

# 实时查看日志
docker-compose logs -f

# 只看应用日志
docker-compose logs -f app
```

### 停止服务
```bash
./stop.sh
# 或
docker-compose down
```

### 重启服务
```bash
docker-compose restart
```

### 备份数据库
```bash
docker-compose exec postgres pg_dump -U reading_notes reading_notes > backup.sql
```

### 恢复数据库
```bash
docker-compose exec -T postgres psql -U reading_notes reading_notes < backup.sql
```

## 高级配置

### 使用外部数据库

如果您想使用外部 PostgreSQL 数据库：

1. 编辑 `.env` 文件，添加：
   ```
   EXTERNAL_DATABASE_URL=postgres://user:password@host:port/database
   ```

2. 修改 `docker-compose.yml`，注释掉 postgres 服务，并修改 app 服务的环境变量：
   ```yaml
   environment:
     DATABASE_URL: ${EXTERNAL_DATABASE_URL}
   ```

### 使用 HTTPS

建议使用反向代理（如 Nginx）来提供 HTTPS 支持：

```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 故障排除

### 服务无法启动

1. 检查端口是否被占用：
   ```bash
   sudo lsof -i :8080
   ```

2. 查看详细日志：
   ```bash
   docker-compose logs --tail=50
   ```

### 数据库连接失败

1. 确保数据库服务正在运行：
   ```bash
   docker-compose ps postgres
   ```

2. 检查数据库日志：
   ```bash
   docker-compose logs postgres
   ```

### 性能问题

1. 增加内存限制，在 `docker-compose.yml` 中添加：
   ```yaml
   services:
     app:
       deploy:
         resources:
           limits:
             memory: 2G
   ```

## 支持

- 项目主页: https://github.com/your-username/reading-notes
- 问题反馈: https://github.com/your-username/reading-notes/issues
- 文档: https://github.com/your-username/reading-notes/wiki