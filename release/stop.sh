#!/bin/bash

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}正在停止读书笔记管理系统...${NC}"

# Function to run docker-compose
run_compose() {
    if docker compose version &> /dev/null; then
        docker compose "$@"
    else
        docker-compose "$@"
    fi
}

# Stop services
if run_compose down; then
    echo -e "${GREEN}服务已成功停止${NC}"
else
    echo -e "${RED}停止服务时出错${NC}"
    exit 1
fi