#!/bin/bash

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== 读书笔记管理系统启动脚本 ===${NC}"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo -e "${YELLOW}未找到 .env 文件，正在从模板创建...${NC}"
    cp .env.example .env
    echo -e "${GREEN}已创建 .env 文件，请编辑配置后重新运行此脚本${NC}"
    echo -e "${YELLOW}编辑命令: nano .env 或 vim .env${NC}"
    exit 1
fi

# Load environment variables
source .env

# Check Docker and Docker Compose
echo -e "${YELLOW}检查 Docker 环境...${NC}"
if ! command -v docker &> /dev/null; then
    echo -e "${RED}错误: Docker 未安装${NC}"
    echo "请访问 https://docs.docker.com/get-docker/ 安装 Docker"
    exit 1
fi

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo -e "${RED}错误: Docker Compose 未安装${NC}"
    echo "请访问 https://docs.docker.com/compose/install/ 安装 Docker Compose"
    exit 1
fi

echo -e "${GREEN}Docker 环境检查通过${NC}"
echo ""

# Function to run docker-compose
run_compose() {
    if docker compose version &> /dev/null; then
        docker compose "$@"
    else
        docker-compose "$@"
    fi
}

# Pull latest images
echo -e "${YELLOW}拉取最新镜像...${NC}"
run_compose pull

# Start services
echo -e "${YELLOW}启动服务...${NC}"
run_compose up -d

# Wait for services to be ready
echo -e "${YELLOW}等待服务启动...${NC}"
sleep 5

# Check service status
if run_compose ps | grep -q "Up"; then
    echo -e "${GREEN}服务启动成功！${NC}"
    echo ""
    echo -e "${GREEN}访问地址: http://localhost:${APP_PORT:-8080}${NC}"
    echo -e "${GREEN}API文档: http://localhost:${APP_PORT:-8080}/docs${NC}"
    echo ""
    echo -e "${YELLOW}常用命令:${NC}"
    echo "  查看日志: docker-compose logs -f"
    echo "  停止服务: docker-compose down"
    echo "  重启服务: docker-compose restart"
    echo "  查看状态: docker-compose ps"
else
    echo -e "${RED}服务启动失败，请查看日志${NC}"
    echo "运行命令: docker-compose logs"
    exit 1
fi