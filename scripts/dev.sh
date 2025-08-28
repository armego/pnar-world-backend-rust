#!/bin/bash

# PNAR World API - Local Development Setup
# This script sets up a complete local development environment

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DB_NAME="pnar_world"
DB_USER="postgres"
DB_PASSWORD="root"
DB_HOST="127.0.0.1"
DB_PORT="5432"
APP_PORT="8000"
ADMINER_PORT="8080"

echo -e "${BLUE}🚀 PNAR World API - Local Development Setup${NC}"
echo "=============================================="

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# 1. Check and install PostgreSQL
echo -e "\n${BLUE}1. Checking PostgreSQL installation...${NC}"

if command_exists psql; then
    print_status "PostgreSQL is already installed"
    POSTGRES_VERSION=$(psql --version | awk '{print $3}')
    print_info "Version: $POSTGRES_VERSION"
else
    print_warning "PostgreSQL not found. Installing..."

    if command_exists brew; then
        echo "Installing PostgreSQL via Homebrew..."
        brew install postgresql

        # Start PostgreSQL service
        brew services start postgresql
        print_status "PostgreSQL installed and started"
    else
        print_error "Homebrew not found. Please install Homebrew first:"
        echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
        exit 1
    fi
fi

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 3

# 2. Setup database and user
echo -e "\n${BLUE}2. Setting up database...${NC}"

# Check if we can connect to PostgreSQL
if ! pg_isready -h $DB_HOST -p $DB_PORT >/dev/null 2>&1; then
    print_error "Cannot connect to PostgreSQL at $DB_HOST:$DB_PORT"
    print_info "Make sure PostgreSQL is running:"
    echo "  brew services start postgresql"
    exit 1
fi

# Create database and user if they don't exist
psql -h $DB_HOST -p $DB_PORT -U $USER -d postgres -c "
DO \$\$
BEGIN
    -- Create user if not exists
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '$DB_USER') THEN
        CREATE ROLE $DB_USER LOGIN PASSWORD '$DB_PASSWORD';
        RAISE NOTICE 'User $DB_USER created';
    ELSE
        RAISE NOTICE 'User $DB_USER already exists';
    END IF;

    -- Create database if not exists
    IF NOT EXISTS (SELECT FROM pg_database WHERE datname = '$DB_NAME') THEN
        CREATE DATABASE $DB_NAME OWNER $DB_USER;
        RAISE NOTICE 'Database $DB_NAME created';
    ELSE
        RAISE NOTICE 'Database $DB_NAME already exists';
    END IF;

    -- Grant privileges
    GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;
    RAISE NOTICE 'Privileges granted to $DB_USER on $DB_NAME';
END
\$\$;"

print_status "Database setup completed"

# 3. Run database migrations
echo -e "\n${BLUE}3. Running database migrations...${NC}"

# Set database URL for sqlx
export DATABASE_URL="postgresql://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"

# Check if sqlx-cli is installed
if ! command_exists sqlx; then
    print_warning "sqlx-cli not found. Installing..."
    cargo install sqlx-cli --features postgres
fi

# Run migrations
if [ -d "migrations" ]; then
    sqlx migrate run
    print_status "Migrations completed"
else
    print_warning "No migrations directory found. Skipping migrations."
fi

# 4. Setup Adminer (Database Management)
echo -e "\n${BLUE}4. Setting up Adminer...${NC}"

# Check if Docker is available for Adminer
if command_exists docker; then
    print_info "Using Docker for Adminer"

    # Stop any existing Adminer container
    docker stop pnar-adminer 2>/dev/null || true
    docker rm pnar-adminer 2>/dev/null || true

    # Start Adminer container
    docker run -d \
        --name pnar-adminer \
        -p $ADMINER_PORT:8080 \
        -e ADMINER_DEFAULT_SERVER=$DB_HOST \
        adminer

    print_status "Adminer started at http://localhost:$ADMINER_PORT"
    print_info "Login credentials:"
    echo "  Server: $DB_HOST"
    echo "  Username: $DB_USER"
    echo "  Password: $DB_PASSWORD"
    echo "  Database: $DB_NAME"

elif command_exists brew; then
    print_info "Installing Adminer via Homebrew"

    # Install PHP (required for Adminer)
    if ! command_exists php; then
        brew install php
    fi

    # Install Adminer
    brew install adminer

    # Start Adminer (you might need to configure this based on your PHP setup)
    print_info "Adminer installed. You can start it manually or use Docker"
else
    print_warning "Neither Docker nor Homebrew found for Adminer"
    print_info "You can install Adminer manually or use a different database client"
fi

# 5. Start the application
echo -e "\n${BLUE}5. Starting PNAR World API...${NC}"

# Check if the application is already running
if lsof -Pi :$APP_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    print_warning "Port $APP_PORT is already in use"
    print_info "The application might already be running"
else
    print_info "Starting the application..."

    # Set environment variables
    export RUST_LOG=debug
    export APP_ENVIRONMENT=development

    # Start the application
    cargo run &
    APP_PID=$!

    # Wait a moment for the app to start
    sleep 3

    if kill -0 $APP_PID 2>/dev/null; then
        print_status "Application started successfully"
        print_info "API available at: http://localhost:$APP_PORT"
        print_info "API Documentation: http://localhost:$APP_PORT/docs (if enabled)"
    else
        print_error "Failed to start the application"
        exit 1
    fi
fi

# 6. Display summary
echo -e "\n${GREEN}🎉 Development environment is ready!${NC}"
echo "========================================"
echo -e "${BLUE}Services:${NC}"
echo "  📡 API: http://localhost:$APP_PORT"
if command_exists docker && docker ps | grep -q pnar-adminer; then
    echo "  🗄️  Adminer: http://localhost:$ADMINER_PORT"
fi
echo "  🗃️  Database: $DB_HOST:$DB_PORT/$DB_NAME"
echo ""
echo -e "${BLUE}Database Credentials:${NC}"
echo "  Username: $DB_USER"
echo "  Password: $DB_PASSWORD"
echo "  Database: $DB_NAME"
echo ""
echo -e "${YELLOW}Useful Commands:${NC}"
echo "  🛑 Stop services: ./stop-dev.sh"
echo "  🔄 Reset database: ./reset-db.sh"
echo "  📊 View logs: tail -f logs/app.log"
echo ""
echo -e "${GREEN}Happy coding! 🚀${NC}"

# Keep the script running to show logs
if [ ! -z "$APP_PID" ]; then
    echo -e "\n${BLUE}Application is running (PID: $APP_PID)${NC}"
    echo "Press Ctrl+C to stop all services"

    # Wait for the application process
    wait $APP_PID
fi
