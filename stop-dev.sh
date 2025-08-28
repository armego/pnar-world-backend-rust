#!/bin/bash

# PNAR World API - Stop Development Environment
# This script stops all development services

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🛑 Stopping PNAR World Development Environment${NC}"
echo "==============================================="

# Configuration
APP_PORT="8000"
ADMINER_PORT="8080"

# Stop the application
echo -e "\n${BLUE}1. Stopping application...${NC}"
if lsof -Pi :$APP_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    APP_PID=$(lsof -Pi :$APP_PORT -sTCP:LISTEN -t)
    kill $APP_PID 2>/dev/null || true
    echo -e "${GREEN}✅ Application stopped (PID: $APP_PID)${NC}"
else
    echo -e "${YELLOW}⚠️  No application running on port $APP_PORT${NC}"
fi

# Stop Adminer container
echo -e "\n${BLUE}2. Stopping Adminer...${NC}"
if command -v docker >/dev/null 2>&1 && docker ps | grep -q pnar-adminer; then
    docker stop pnar-adminer >/dev/null 2>&1
    docker rm pnar-adminer >/dev/null 2>&1
    echo -e "${GREEN}✅ Adminer stopped${NC}"
else
    echo -e "${YELLOW}⚠️  Adminer not running${NC}"
fi

# Stop PostgreSQL (optional - comment out if you want to keep it running)
echo -e "\n${BLUE}3. PostgreSQL service...${NC}"
if command -v brew >/dev/null 2>&1; then
    if brew services list | grep postgresql | grep started >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  PostgreSQL is still running${NC}"
        echo "   Run 'brew services stop postgresql' to stop it completely"
    else
        echo -e "${GREEN}✅ PostgreSQL is not running${NC}"
    fi
fi

echo -e "\n${GREEN}🎉 All services stopped successfully!${NC}"
echo "========================================"
echo -e "${BLUE}To restart development environment:${NC}"
echo "  ./dev.sh"
