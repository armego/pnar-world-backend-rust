#!/bin/bash

# PNAR World API - Linux Production Deployment Script
# This script sets up the application for production on Linux systems

set -e

echo "🚀 PNAR World API - Linux Production Setup"
echo "=========================================="

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "❌ This script is designed for Linux systems only"
    echo "   For other systems, follow the manual setup in README.md"
    exit 1
fi

# Check if PostgreSQL is installed and running
echo "📋 Checking PostgreSQL..."
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL not found. Please install PostgreSQL first:"
    echo "   Ubuntu/Debian: sudo apt install postgresql postgresql-contrib"
    exit 1
fi

if ! sudo systemctl is-active --quiet postgresql; then
    echo "⚠️  PostgreSQL service not running. Starting..."
    sudo systemctl start postgresql
    sudo systemctl enable postgresql
fi

# Check if database exists
echo "📋 Checking database..."
if ! sudo -u postgres psql -lqt | cut -d \| -f 1 | grep -qw pnar_world; then
    echo "📦 Creating database 'pnar_world'..."
    sudo -u postgres createdb pnar_world
    sudo -u postgres psql -c "CREATE USER postgres WITH PASSWORD 'root';"
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;"
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

# Build the application
echo "📋 Building application..."
cargo build --release

echo ""
echo "✅ Setup complete!"
echo ""
echo "🚀 To start the API server:"
echo "   cargo run --release"
echo ""
echo "📖 API will be available at: http://localhost:8000"
echo "📖 Check README.md for API documentation and Postman collection"
