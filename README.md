# PNAR World API 🌍

A modern, production-ready REST API for the Pnar language dictionary and translation service. Built with Rust, Actix-web, and PostgreSQL.

[![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)

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
                                               └─��───────────────┘
```

## 🛠️ Technology Stack

- **Language**: Rust 1.89+
- **Web Framework**: Actix-web 4.9
- **Database**: PostgreSQL 15+ with SQLx
- **Authentication**: JWT with Argon2 password hashing
- **Documentation**: OpenAPI 3.0 with Swagger UI
- **Containerization**: Docker/Podman with multi-stage builds
- **Monitoring**: Built-in health checks and metrics

## 📋 Prerequisites

- **Rust**: 1.89 or later
- **PostgreSQL**: 15 or later
- **Container Runtime**: Docker or Podman
- **System**: Linux, macOS, or Windows

## 🚀 Quick Start

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/armego/pnar-world-backend-rust.git
   cd pnar-world-backend-rust
   ```

2. **Start the development environment**
   ```bash
   ./start.sh
   ```

3. **Access the API**
   - API: http://localhost:8000
   - Documentation: http://localhost:8000/swagger-ui/index.html
   - Database Admin: http://localhost:8080

### Production Deployment

1. **Build and deploy**
   ```bash
   ./deploy.sh
   ```

2. **Or use Docker Compose**
   ```bash
   cd deploy
   docker-compose up -d
   ```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `APP_ENVIRONMENT` | Environment (development/production) | development | No |
| `DATABASE_HOST` | PostgreSQL host | 127.0.0.1 | Yes |
| `DATABASE_USERNAME` | Database username | postgres | Yes |
| `DATABASE_PASSWORD` | Database password | - | Yes |
| `DATABASE_NAME` | Database name | pnar_world | Yes |
| `JWT_SECRET` | JWT signing secret | - | Yes |
| `RUST_LOG` | Log level | info | No |

### Configuration Files

- `configuration.yaml` - Development configuration
- `configuration.production.yaml` - Production configuration

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

| Role | Permissions |
|------|-------------|
| `superadmin` | Full system access |
| `admin` | User and content management |
| `moderator` | Content moderation and verification |
| `translator` | Translation and dictionary management |
| `contributor` | Content creation |
| `user` | Basic API access |

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

## 🐳 Docker Deployment

### Using Docker Compose

```yaml
version: '3.8'
services:
  api:
    image: pnar-world-api:1.0.0
    ports:
      - "8000:8000"
    environment:
      - APP_ENVIRONMENT=production
      - DATABASE_HOST=postgres
      - DATABASE_USERNAME=postgres
      - DATABASE_PASSWORD=your-password
      - JWT_SECRET=your-jwt-secret
    depends_on:
      - postgres
    
  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=pnar_world
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=your-password
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

### Using Kubernetes

```bash
kubectl apply -f deploy/deployment.yaml
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

## 🧪 Testing

### Run Tests
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Linting
cargo clippy

# Security audit
cargo audit
```

### Load Testing
```bash
# Using Apache Bench
ab -n 1000 -c 10 http://localhost:8000/api/v1/health

# Using wrk
wrk -t12 -c400 -d30s http://localhost:8000/api/v1/health
```

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
# Check database is running
podman ps | grep postgres

# Check connection
psql -h localhost -U postgres -d pnar_world
```

**Migration Errors**
```bash
# Reset database
./start.sh  # This will recreate the database

# Manual migration
sqlx migrate run --database-url postgresql://postgres:root@localhost/pnar_world
```

**Container Won't Start**
```bash
# Check logs
podman logs pnar-app-pod-api

# Check health
curl http://localhost:8000/api/v1/health
```

### Performance Tuning

**Database**
- Adjust connection pool size in configuration
- Monitor slow queries
- Add database indexes for frequently queried fields

**Application**
- Tune worker count based on CPU cores
- Adjust request timeout settings
- Enable compression for large responses

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

- **Stavros Grigoriou** - *Initial work* - [unix121@protonmail.com](mailto:unix121@protonmail.com)

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