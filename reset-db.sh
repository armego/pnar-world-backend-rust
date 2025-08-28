#!/bin/bash

# PNAR World API - Reset Database
# This script resets the development database (WARNING: Destroys all data!)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
DB_NAME="pnar_world"
DB_USER="postgres"
DB_PASSWORD="root"
DB_HOST="127.0.0.1"
DB_PORT="5432"

echo -e "${RED}💥 PNAR World Database Reset${NC}"
echo "================================"
echo -e "${RED}⚠️  WARNING: This will DELETE ALL DATA in the database!${NC}"
echo ""

# Ask for confirmation
read -p "Are you sure you want to reset the database? (type 'yes' to confirm): " confirm

if [[ "$confirm" != "yes" ]]; then
    echo -e "${YELLOW}❌ Database reset cancelled${NC}"
    exit 0
fi

echo -e "\n${BLUE}1. Checking database connection...${NC}"

# Check if PostgreSQL is running
if ! pg_isready -h $DB_HOST -p $DB_PORT >/dev/null 2>&1; then
    echo -e "${RED}❌ Cannot connect to PostgreSQL at $DB_HOST:$DB_PORT${NC}"
    echo -e "${YELLOW}Make sure PostgreSQL is running:${NC}"
    echo "  brew services start postgresql"
    exit 1
fi

echo -e "${GREEN}✅ Database connection OK${NC}"

# Set database URL for sqlx
export DATABASE_URL="postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

echo -e "\n${BLUE}2. Resetting database...${NC}"

# Drop and recreate the database
psql -h $DB_HOST -p $DB_PORT -U $USER -d postgres -c "
DROP DATABASE IF EXISTS $DB_NAME;
CREATE DATABASE $DB_NAME OWNER $DB_USER;
GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

echo -e "${GREEN}✅ Database reset completed${NC}"

echo -e "\n${BLUE}3. Running migrations...${NC}"

# Check if sqlx-cli is available
if ! command -v sqlx >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  sqlx-cli not found. Installing...${NC}"
    cargo install sqlx-cli --features postgres
fi

# Run migrations
if [ -d "migrations" ]; then
    sqlx migrate run
    echo -e "${GREEN}✅ Migrations completed${NC}"
else
    echo -e "${YELLOW}⚠️  No migrations directory found${NC}"
fi

echo -e "\n${GREEN}🎉 Database reset successful!${NC}"
echo "================================="
echo -e "${BLUE}Database has been reset and migrations applied.${NC}"
echo -e "${BLUE}You can now restart your application.${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Restart your app: ./dev.sh"
echo "  2. Or run manually: cargo run"
