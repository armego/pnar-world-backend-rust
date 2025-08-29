#!/bin/bash

# PNAR World API - macOS Development Setup Script
# This script sets up the development environment on macOS

set -e

echo "🚀 PNAR World API - macOS Development Setup"
echo "==========================================="

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This script is designed for macOS systems only"
    echo "   For Linux production, use setup-linux.sh"
    echo "   For other systems, follow the manual setup in README.md"
    exit 1
fi

# Check if Homebrew is installed
echo "📋 Checking Homebrew..."
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found. Please install Homebrew first:"
    echo "   /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    exit 1
fi

# Check if PostgreSQL is installed
echo "📋 Checking PostgreSQL..."
if ! command -v psql &> /dev/null; then
    echo "📦 Installing PostgreSQL..."
    brew install postgresql@14
else
    echo "✅ PostgreSQL is already installed"
fi

# Start PostgreSQL service
echo "📋 Checking PostgreSQL service..."
if pg_isready -q; then
    echo "✅ PostgreSQL is already running"
else
    echo "📋 Starting PostgreSQL service..."
    # Try brew services first
    if brew services start postgresql@14 2>/dev/null; then
        echo "✅ PostgreSQL started via brew services"
    else
        echo "⚠️  Brew services failed, trying manual start..."
        # Manual start as fallback
        if [ ! -d "/usr/local/var/postgresql@14" ]; then
            echo "📦 Initializing PostgreSQL database cluster..."
            initdb -D /usr/local/var/postgresql@14
        fi
        pg_ctl -D /usr/local/var/postgresql@14 -l /usr/local/var/postgresql@14/server.log start
        echo "✅ PostgreSQL started manually"
    fi
fi

# Wait a moment for PostgreSQL to start
sleep 3

# Check if database exists
echo "📋 Checking database..."
if ! psql -lqt | cut -d \| -f 1 | grep -qw pnar_world; then
    echo "📦 Creating database 'pnar_world'..."
    createdb pnar_world
    psql -d postgres -c "CREATE USER postgres WITH PASSWORD 'root';"
    psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;"
    echo "✅ Database created successfully"
else
    echo "✅ Database 'pnar_world' already exists"
fi

# Check if Rust is installed
echo "📋 Checking Rust..."
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Run database migrations
echo "📋 Running database migrations..."
if [ -z "$DATABASE_URL" ]; then
    export DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world"
fi

sqlx migrate run

# Build the application (development mode)
echo "📋 Building application..."
cargo build

echo ""
echo "✅ macOS Development Setup Complete!"
echo ""
echo "🚀 To start the API server:"
echo "   cargo run"
echo ""
echo "📖 API will be available at: http://localhost:8000"
echo "📖 Check README.md for API documentation and Postman collection"
echo ""
echo "🔄 For production deployment on Linux, use: setup-linux.sh"
