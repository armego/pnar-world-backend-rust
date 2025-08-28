# PNAR World API - Development Guide 🚀

## Overview

This guide covers the local development setup for the PNAR World API, a modern Rust-based REST API for Pnar language dictionary and translation services.

## 🛠️ Quick Setup

### One-Command Development Environment

```bash
# Clone and setup everything
git clone <repository-url>
cd pnar-world-backend-rust

# Start full development environment
./dev.sh

# Access points:
# - API: http://localhost:8000
# - Adminer (Database UI): http://localhost:8080
# - API Docs: http://localhost:8000/docs (if enabled)
```

## 📋 Prerequisites

### Required Software

- **Rust**: 1.89+ (https://rustup.rs/)
- **PostgreSQL**: 15+ (local installation)
- **Git**: For version control
- **Optional**: Docker (for Adminer)

### macOS Setup

```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install required software
brew install rust postgresql git

# Start PostgreSQL service
brew services start postgresql
```

### Linux Setup

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib curl build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Start PostgreSQL
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

## 🚀 Development Workflow

### 1. Database Setup

The `./dev.sh` script automatically:

- ✅ Installs PostgreSQL (if missing)
- ✅ Creates `pnar_world` database
- ✅ Sets up `postgres` user with password `root`
- ✅ Runs database migrations
- ✅ Starts Adminer for database management

### 2. Environment Configuration

The application uses YAML configuration (`configuration.yaml`) for most settings. Environment variables are used for:

- `APP_ENVIRONMENT` - Set to "development" or "production"
- `RUST_LOG` - Logging level (e.g., "debug", "info")
- `JWT_SECRET` - JWT signing secret (change in production!)

Set environment variables as needed:

```bash
# Development mode (default)
export APP_ENVIRONMENT=development

# Debug logging
export RUST_LOG=debug

# Custom JWT secret
export JWT_SECRET=your-secure-secret-here
```

### 3. Running the Application

```bash
# Full development environment (recommended)
./dev.sh

# Or run API only (if database is already running)
cargo run
```

### 4. Database Management

**Using Adminer (Web Interface):**

- URL: http://localhost:8080
- Server: localhost
- Username: postgres
- Password: root
- Database: pnar_world

**Using Command Line:**

```bash
# Connect to database
psql -h localhost -U postgres -d pnar_world

# Useful commands
\dt                    # List tables
\dn                    # List schemas
\d table_name         # Describe table
\q                    # Quit
```

## 🔧 Available Scripts

| Script          | Description                                   |
| --------------- | --------------------------------------------- |
| `./dev.sh`      | 🚀 Start full development environment         |
| `./stop-dev.sh` | 🛑 Stop all development services              |
| `./reset-db.sh` | 💥 Reset database (WARNING: deletes all data) |
| `cargo run`     | ⚡ Run API only                               |
| `cargo test`    | 🧪 Run tests                                  |
| `cargo check`   | ✅ Check code without building                |
| `cargo fmt`     | 🎨 Format code                                |
| `cargo clippy`  | 🔍 Lint code                                  |

## 🗄️ Database Operations

### Running Migrations

```bash
# Automatic (via dev.sh)
./dev.sh

# Manual migration
DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run

# Create new migration
sqlx migrate add create_users_table
```

### Reset Database (Development Only)

```bash
# WARNING: This deletes ALL data
./reset-db.sh
```

### Database Schema

The database schema is defined in the `migrations/` directory. Key tables:

- `users` - User accounts and authentication
- `dictionaries` - Pnar dictionary entries
- `translations` - Translation requests and results
- `analytics` - Usage tracking and statistics

## 🧪 Testing

### API Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_user_registration

# Run with output
cargo test -- --nocapture
```

### Manual API Testing

Use the provided Postman collection:

- Import `postman/PNAR-API.postman_collection.json`
- Set environment to `postman/PNAR-API.postman_environment.json`
- Update base URL to `http://localhost:8000`

## 🔍 Debugging

### Logging

Set log levels in your `.env` file:

```bash
# Debug logging
RUST_LOG=debug

# Specific module logging
RUST_LOG=pnar_world=debug,actix_web=info
```

### Database Debugging

```bash
# View active connections
psql -h localhost -U postgres -d pnar_world -c "SELECT * FROM pg_stat_activity;"

# View table sizes
psql -h localhost -U postgres -d pnar_world -c "SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) FROM pg_tables ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;"
```

## 🚀 Deployment

### Local Production Testing

```bash
# Set production environment
export APP_ENVIRONMENT=production
export DATABASE_URL="postgresql://prod-user:prod-password@localhost:5432/pnar_world_prod"

# Run with release optimizations
cargo run --release
```

### Production Deployment

For production deployment, consider:

- **Supabase** (recommended for simplicity)
- **Railway** or **Render** (PaaS)
- **Docker** with orchestration
- **Cloud PostgreSQL** (AWS RDS, Google Cloud SQL, Azure Database)

## 🆘 Troubleshooting

### Common Issues

**PostgreSQL Connection Failed:**

```bash
# Check if PostgreSQL is running
brew services list | grep postgresql

# Start PostgreSQL
brew services start postgresql

# Reset database
./reset-db.sh
```

**Port Already in Use:**

```bash
# Find process using port 8000
lsof -i :8000

# Kill the process
kill -9 <PID>

# Or use different port
PORT=8001 cargo run
```

**Migration Errors:**

```bash
# Reset and re-run migrations
./reset-db.sh

# Check migration status
sqlx migrate info
```

**Adminer Not Accessible:**

```bash
# Check if Docker container is running
docker ps | grep adminer

# Restart Adminer
docker stop pnar-adminer
docker rm pnar-adminer
docker run -d --name pnar-adminer -p 8080:8080 adminer
```

### Getting Help

- Check existing issues on GitHub
- Review the main README.md
- Test with the provided Postman collection
- Check logs with `RUST_LOG=debug`

## 📚 Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Actix Web Documentation](https://actix.rs/docs/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Adminer Documentation](https://www.adminer.org/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

---

**Happy coding! 🎉**
