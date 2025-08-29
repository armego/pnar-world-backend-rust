# PNAR World API 🌍

A modern, production-ready REST API for the Pnar language dictionary and translation service. Built with Rust, Actix-web, and PostgreSQL.

[![Rust](https:/## 📖## 📖 Additional Resources

- **API Documentation**: `postman/README.md`
- **Postman Collection**: `postman/PNAR-API.postman_collection.json`
- **Database Schema**: `migrations/` directory
- **Configuration**: `configuration.yaml`

---

**Made with ❤️ for the Pnar language community**al Resources

- **API Documentation**: `postman/README.md`
- **Postman Collection**: `postman/PNAR-API.postman_collection.json`
- **Database Schema**: `migrations/` directory
- **Configuration**: `configuration.yaml`lds.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🚀 Features

### Core Functionality

- **Dictionary Management**: Create, read, update, and delete Pnar dictionary entries
- **Translation Services**: Request and manage translations between Pnar and English
- **User Management**: Role-based authentication and authorization
- **Analytics**: Track word usage and translation patterns
- **Alphabet Conversion**: Convert between traditional and keyboard-friendly Pnar text

### Production-Ready Features

- **Security**: Rate limiting, CORS, security headers, JWT authentication
- **Monitoring**: Health checks, metrics, structured logging
- **Performance**: Connection pooling, optimized queries, caching
- **Reliability**: Fail-fast startup, graceful shutdown, error handling
- **Observability**: Request tracing, performance metrics, database monitoring

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client Apps   │    │   Load Balancer │    │   PNAR World    │
│                 │◄──►│                 │◄──►│      API        │
│ Web/Mobile/CLI  │    │  (nginx/traefik)│    │   (Rust/Actix)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
                                               ┌─────────────────┐
                                               │   PostgreSQL    │
                                               │    Database     │
                                               └─────────────────┘
```

## 🛠️ Technology Stack

- **Language**: Rust 1.89+
- **Web Framework**: Actix-web 4.9
- **Database**: PostgreSQL 15+ with SQLx
- **Authentication**: JWT with Argon2 password hashing
- **API Testing**: Postman collections with comprehensive examples
- **Development**: Local PostgreSQL with Adminer for database management
- **Monitoring**: Built-in health checks and metrics

## 📋 Prerequisites

- **Rust**: 1.89 or later
- **PostgreSQL**: 15 or later (must be installed and running)
- **Development**: macOS (with Homebrew)
- **Production**: Linux (target environment)

### macOS Development Setup

```bash
# Install PostgreSQL using Homebrew
brew install postgresql
brew services start postgresql

# Create database and user
createdb pnar_world
psql -d postgres -c "CREATE USER postgres WITH PASSWORD 'root';"
psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;"

# Verify connection
psql -h localhost -U postgres -d pnar_world
# Type: \q to exit
```

### Linux Production Setup

```bash
# Ubuntu/Debian (production target)
sudo apt update
sudo apt install postgresql postgresql-contrib

# Start and enable PostgreSQL service
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Switch to postgres user and setup database
sudo -u postgres psql

# In PostgreSQL shell:
CREATE DATABASE pnar_world;
CREATE USER postgres WITH PASSWORD 'root';
GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;
\q

# Verify connection
psql -h localhost -U postgres -d pnar_world
# Type: \q to exit
```

### PostgreSQL Setup (Windows - Development)

```bash
# Download from: https://www.postgresql.org/download/windows/
# Run installer and follow setup wizard
# Set password for 'postgres' user to 'root'
# Create database 'pnar_world' during setup or manually
```

## 🚀 Quick Start

### Automated Setup Scripts

#### macOS Development (Your Current Environment)

```bash
# Quick automated setup for macOS
chmod +x setup-macos.sh
./setup-macos.sh
```

#### Linux Production (Your Target Environment)

```bash
# Automated production setup for Linux
chmod +x setup-linux.sh
./setup-linux.sh
```

### Manual Setup

#### macOS Development Setup

```bash
# 1. Ensure PostgreSQL is running
brew services list | grep postgresql

# 2. Run database migrations
DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run

# 3. Start the API
cargo run

# API will be available at: http://localhost:8000
```

#### Linux Production Setup

```bash
# 1. Ensure PostgreSQL is running
sudo systemctl status postgresql

# 2. Run database migrations
DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run

# 3. Start the API
cargo run

# API will be available at: http://localhost:8000
```

### Cross-Platform Setup

The same setup works on all platforms - just adjust the PostgreSQL installation method for your OS.

### Database Management

Choose your preferred PostgreSQL client:

- **Command Line**: `psql -h localhost -U postgres -d pnar_world`
- **pgAdmin**: Web-based PostgreSQL administration
- **DBeaver**: Universal database tool
- **TablePlus**: Modern database client
- **Any PostgreSQL GUI client** of your choice

# 2. Setup database

createdb pnar_world
psql -d postgres -c "CREATE USER postgres WITH PASSWORD 'root';"
psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;"

# 3. Run migrations

DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run

# 4. Start the API

cargo run

# API will be available at: http://localhost:8000

````

### Environment Variables

Set these environment variables as needed:

```bash
# Development mode (default)
export APP_ENVIRONMENT=development

# Debug logging
export RUST_LOG=debug

# Custom JWT secret (optional)
export JWT_SECRET=your-secure-secret-here
````

### Database Management

Choose your preferred PostgreSQL client:

- **Command Line**: `psql -h localhost -U postgres -d pnar_world`
- **pgAdmin**: Web-based PostgreSQL administration
- **DBeaver**: Universal database tool
- **TablePlus**: Modern database client
- **Any PostgreSQL GUI client** of your choice

## 🧪 Testing

### API Testing with Postman

1. **Import collection**: `postman/PNAR-API.postman_collection.json`
2. **Import environment**: `postman/PNAR-API.postman_environment.json`
3. **Select environment**: "PNAR API Environment"
4. **Test endpoints**: Start with Health Check → Authentication → Core features

### Manual Testing

```bash
# Health check
curl http://localhost:8000/health

# List all endpoints in Postman collection
cat postman/README.md
```

## 📖 Additional Resources

- **API Documentation**: `postman/README.md`
- **Postman Collection**: `postman/PNAR-API.postman_collection.json`
- **Database Schema**: `migrations/` directory
- **Configuration**: `configuration.yaml`
  | `./stop-dev.sh` | 🛑 **Stop all services** | Stop API, Adminer, and PostgreSQL |
  | `./reset-db.sh` | � **Reset database** | Drop & recreate DB, run migrations |
  | `cargo run` | ⚡ **Run API only** | Start Rust API (DB must be running) |

### Script Usage Examples

```bash
# Development workflow
./scripts/dev.sh                     # Start everything automatically
# API available at http://localhost:8000
# Adminer at http://localhost:8080

# Reset database (WARNING: deletes all data)
./scripts/reset-db.sh

# Stop everything
./scripts/stop-dev.sh

# Run API only (if DB is already running)
cargo run
```

### Database Management

**Using Adminer (Web UI):**

- Open: http://localhost:8080
- Server: localhost
- Username: postgres
- Password: root
- Database: pnar_world

**Using psql (Command Line):**

```bash
# Connect to database
psql -h localhost -U postgres -d pnar_world

# List tables
\dt

# Exit
\q
```

### Environment Variables

| Variable          | Description                  | Default                                                | Required |
| ----------------- | ---------------------------- | ------------------------------------------------------ | -------- |
| `DATABASE_URL`    | PostgreSQL connection string | `postgresql://postgres:root@localhost:5432/pnar_world` | Yes      |
| `RUST_LOG`        | Log level                    | `info`                                                 | No       |
| `APP_ENVIRONMENT` | Environment name             | `development`                                          | No       |

### Configuration Files

- `configuration.yaml` - Development configuration
- `configuration.production.yaml` - Production configuration (if needed)

## 🔒 Security

### Authentication & Authorization

- JWT-based authentication
- Role-based access control (RBAC)
- Argon2 password hashing
- Session management

### Security Measures

- Rate limiting (60 requests/minute by default)
- CORS configuration
- Security headers (CSP, HSTS, etc.)
- Input validation and sanitization
- SQL injection prevention

### Roles & Permissions

| Role          | Permissions                           |
| ------------- | ------------------------------------- |
| `superadmin`  | Full system access                    |
| `admin`       | User and content management           |
| `moderator`   | Content moderation and verification   |
| `translator`  | Translation and dictionary management |
| `contributor` | Content creation                      |
| `user`        | Basic API access                      |

## 📊 API Endpoints

### Authentication

- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/logout` - User logout
- `GET /api/v1/auth/profile` - Get user profile

### Dictionary

- `GET /api/v1/dictionary` - List dictionary entries
- `POST /api/v1/dictionary` - Create dictionary entry
- `GET /api/v1/dictionary/{id}` - Get dictionary entry
- `PUT /api/v1/dictionary/{id}` - Update dictionary entry
- `DELETE /api/v1/dictionary/{id}` - Delete dictionary entry

### Health & Monitoring

- `GET /api/v1/health` - Comprehensive health check
- `GET /api/v1/health/live` - Liveness probe
- `GET /api/v1/health/ready` - Readiness probe
- `GET /api/v1/metrics` - Application metrics

### Documentation

- `GET /swagger-ui/index.html` - Interactive API documentation
- `GET /api-doc/openapi.json` - OpenAPI specification

## � Production Deployment (Linux)

### Automated Linux Setup

For quick Linux deployment, use the provided setup script:

```bash
# Make script executable and run
chmod +x setup-linux.sh
./setup-linux.sh
```

The script will:

- ✅ Verify PostgreSQL installation and service
- 🗄️ Create database and user if needed
- 🔄 Run database migrations
- 📦 Build the application in release mode
- 🚀 Provide startup instructions

### Manual Production Setup

```bash
# 1. Ensure PostgreSQL is running
sudo systemctl status postgresql

# 2. Set production environment
export APP_ENVIRONMENT=production
export DATABASE_URL="postgresql://postgres:your_secure_password@localhost:5432/pnar_world"
export JWT_SECRET="your-secure-jwt-secret-here"

# 3. Run migrations
sqlx migrate run

# 4. Build and run in production mode
cargo build --release
./target/release/pnar-world-backend-rust
```

### Production Checklist

- [ ] PostgreSQL running with proper credentials
- [ ] Environment variables set (APP_ENVIRONMENT=production)
- [ ] Database migrations applied
- [ ] JWT secret configured securely
- [ ] SSL enabled in configuration.yaml
- [ ] CORS origins restricted
- [ ] Logging level set to 'warn'
- [ ] Release build optimizations enabled

### Systemd Service (Optional)

Create a systemd service for automatic startup:

```bash
# Create service file
sudo nano /etc/systemd/system/pnar-world-api.service
```

```ini
[Unit]
Description=PNAR World API
After=network.target postgresql.service

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/your/app
Environment=APP_ENVIRONMENT=production
Environment=DATABASE_URL=postgresql://postgres:password@localhost:5432/pnar_world
Environment=JWT_SECRET=your-secure-secret
ExecStart=/path/to/your/app/target/release/pnar-world-backend-rust
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable pnar-world-api
sudo systemctl start pnar-world-api
```

## 📈 Monitoring & Observability

### Health Checks

- **Liveness**: `/api/v1/health/live` - Basic service availability
- **Readiness**: `/api/v1/health/ready` - Service ready to handle requests
- **Health**: `/api/v1/health` - Comprehensive system health

### Metrics

- **Application**: Request counts, response times, error rates
- **Database**: Connection pool stats, query performance
- **System**: Memory usage, CPU utilization

### Logging

- Structured JSON logging
- Request tracing with correlation IDs
- Error tracking and alerting
- Performance monitoring

## 🧪 API Testing

### Automated Testing

Use the comprehensive API testing script to verify all endpoints:

```bash
# Test all APIs automatically
./test-apis.sh

# Test with custom configuration
API_BASE_URL=http://localhost:8080 ./test-apis.sh
TIMEOUT=30 ./test-apis.sh
```

### What the Test Script Does:

#### ✅ **Health & Monitoring**

- Health check endpoints (`/health`, `/health/live`, `/health/ready`)
- Metrics endpoint (`/metrics`)

#### 🔐 **Authentication**

- User registration and login
- Profile management
- Token validation

#### 👥 **User Management**

- User CRUD operations
- Password updates
- Points awarding
- Email verification

#### 📚 **Dictionary**

- Public dictionary access
- Protected dictionary management
- Search functionality

#### 🌐 **Translations**

- Translation requests
- Translation management

#### 🤝 **Contributions**

- User contributions
- Contribution management

#### 📊 **Analytics**

- Usage analytics
- Anonymous analytics

#### 🔤 **Alphabet**

- Character mappings
- Text conversion

#### 📖 **Books**

- Book management
- Public/private books

#### 🔔 **Notifications**

- Notification management
- Read/unread status

#### 👮 **Roles**

- Role information
- Permission management

### Test Results

The script provides:

- **✅ Pass/Fail status** for each endpoint
- **📊 Summary report** with totals
- **🔍 Detailed error messages** for failures
- **🚀 Automatic application startup** if needed

### Manual Testing

You can also test individual endpoints using Swagger UI:

- **Swagger UI:** `http://localhost:8000/swagger-ui/index.html`
- **Interactive testing** with try-it-out functionality
- **Request/response examples** for all endpoints

## 🔧 Development

### Project Structure

```
src/
├── config.rs          # Configuration management
├── database.rs        # Database connection and migrations
├── error.rs           # Error handling
├── handlers/          # HTTP request handlers
├── middleware/        # Custom middleware
├── models/            # Data models
├── services/          # Business logic
├── utils/             # Utility functions
└── main.rs           # Application entry point

migrations/            # Database migrations
deploy/               # Deployment configurations
```

### Adding New Features

1. **Create a new handler**

   ```rust
   // src/handlers/my_feature.rs
   use actix_web::{web, HttpResponse};

   pub async fn my_endpoint() -> Result<HttpResponse, AppError> {
       Ok(HttpResponse::Ok().json("Hello, World!"))
   }
   ```

2. **Add to routing**

   ```rust
   // src/startup.rs
   .route("/my-feature", web::get().to(handlers::my_feature::my_endpoint))
   ```

3. **Add tests**

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[tokio::test]
       async fn test_my_endpoint() {
           // Test implementation
       }
   }
   ```

## 🚨 Troubleshooting

### Common Issues

**Database Connection Failed**

```bash
# Check PostgreSQL service status
sudo systemctl status postgresql

# Check if PostgreSQL is listening
sudo netstat -tlnp | grep 5432

# Test database connection
psql -h localhost -U postgres -d pnar_world
```

**Migration Errors**

```bash
# Check DATABASE_URL environment variable
echo $DATABASE_URL

# Manual migration with explicit URL
DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run

# Reset database (CAUTION: This will delete all data)
sudo -u postgres dropdb pnar_world
sudo -u postgres createdb pnar_world
DATABASE_URL="postgresql://postgres:root@localhost:5432/pnar_world" sqlx migrate run
```

**Application Won't Start**

```bash
# Check if port 8000 is available
sudo netstat -tlnp | grep 8000

# Check application logs (run in another terminal)
RUST_LOG=debug cargo run

# Test health endpoint
curl http://localhost:8000/api/v1/health
```

**Permission Issues (Linux)**

```bash
# Check PostgreSQL user permissions
sudo -u postgres psql -c "\du"

# Grant necessary permissions
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE pnar_world TO postgres;"

# Check file permissions for the application
ls -la target/release/pnar-world-backend-rust
```

### Performance Tuning

**Database**

- Adjust connection pool size in `configuration.yaml`
- Monitor slow queries with `pg_stat_statements`
- Add database indexes for frequently queried fields
- Tune PostgreSQL configuration for your hardware

**Application**

- Tune worker count based on CPU cores in `configuration.yaml`
- Adjust request timeout settings
- Enable compression for large responses
- Use release build for production: `cargo build --release`

## 📚 Documentation

- [API Documentation](http://localhost:8000/swagger-ui/index.html) - Interactive API docs
- [Database Schema](./docs/database-schema.md) - Database structure
- [Deployment Guide](./docs/deployment.md) - Detailed deployment instructions
- [Contributing Guide](./CONTRIBUTING.md) - How to contribute

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- **Stavros Grigoriou** - _Initial work_ - [unix121@protonmail.com](mailto:unix121@protonmail.com)

## 🙏 Acknowledgments

- The Rust community for excellent tooling and libraries
- The Pnar language community for cultural guidance
- Contributors and testers who helped improve the API

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/armego/pnar-world-backend-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/armego/pnar-world-backend-rust/discussions)
- **Email**: [unix121@protonmail.com](mailto:unix121@protonmail.com)

---

**Made with ❤️ for the Pnar language community**
