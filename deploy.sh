#!/bin/bash

# PNAR World API Production Deployment Script
# This script handles building, testing, and deploying the API

set -euo pipefail

# Configuration
readonly APP_NAME="pnar-world-api"
readonly IMAGE_NAME="pnar-world-api"
readonly VERSION="1.0.0"
readonly REGISTRY="${REGISTRY:-localhost:5000}"
readonly ENVIRONMENT="${ENVIRONMENT:-production}"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking deployment prerequisites..."
    
    # Check if required tools are installed
    local required_tools=("podman" "cargo" "git")
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            log_error "$tool is not installed or not in PATH"
            exit 1
        fi
    done
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || [[ ! -f "Dockerfile" ]]; then
        log_error "Must be run from the project root directory"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Run tests
run_tests() {
    log_info "Running test suite..."
    
    # Run unit tests
    if ! cargo test --release; then
        log_error "Unit tests failed"
        exit 1
    fi
    
    # Run clippy for linting
    if ! cargo clippy --release -- -D warnings; then
        log_error "Clippy linting failed"
        exit 1
    fi
    
    # Check formatting
    if ! cargo fmt --check; then
        log_error "Code formatting check failed"
        exit 1
    fi
    
    # Security audit
    if command -v cargo-audit &> /dev/null; then
        if ! cargo audit; then
            log_warning "Security audit found issues"
        fi
    else
        log_warning "cargo-audit not installed, skipping security audit"
    fi
    
    log_success "All tests passed"
}

# Build the application
build_application() {
    log_info "Building application..."
    
    # Clean previous builds
    cargo clean
    
    # Build in release mode
    if ! cargo build --release; then
        log_error "Application build failed"
        exit 1
    fi
    
    # Verify binary exists and is executable
    if [[ ! -x "target/release/$APP_NAME" ]]; then
        log_error "Built binary is not executable"
        exit 1
    fi
    
    log_success "Application built successfully"
}

# Build Docker image
build_image() {
    log_info "Building Docker image..."
    
    local image_tag="${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    local latest_tag="${REGISTRY}/${IMAGE_NAME}:latest"
    
    # Build the image
    if ! podman build \
        --tag "$image_tag" \
        --tag "$latest_tag" \
        --build-arg BUILD_TIMESTAMP="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
        --build-arg GIT_HASH="$(git rev-parse --short HEAD)" \
        --build-arg VERSION="$VERSION" \
        .; then
        log_error "Docker image build failed"
        exit 1
    fi
    
    log_success "Docker image built: $image_tag"
}

# Test the Docker image
test_image() {
    log_info "Testing Docker image..."
    
    local image_tag="${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    local container_name="${APP_NAME}-test"
    
    # Remove any existing test container
    podman rm -f "$container_name" 2>/dev/null || true
    
    # Run the container in test mode
    if ! podman run \
        --name "$container_name" \
        --detach \
        --publish 8001:8000 \
        --env APP_ENVIRONMENT=test \
        --env DATABASE_HOST=localhost \
        --env DATABASE_USERNAME=test \
        --env DATABASE_PASSWORD=test \
        --env DATABASE_NAME=test_db \
        --env JWT_SECRET=test-secret-key \
        "$image_tag"; then
        log_error "Failed to start test container"
        exit 1
    fi
    
    # Wait for container to be ready
    log_info "Waiting for container to be ready..."
    local max_attempts=30
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        if curl -s -f http://localhost:8001/api/v1/health/live >/dev/null 2>&1; then
            log_success "Container is responding to health checks"
            break
        fi
        
        attempt=$((attempt + 1))
        sleep 2
    done
    
    if [[ $attempt -eq $max_attempts ]]; then
        log_error "Container failed to become ready"
        podman logs "$container_name"
        podman rm -f "$container_name"
        exit 1
    fi
    
    # Run basic API tests
    log_info "Running API tests..."
    
    # Test liveness endpoint
    if ! curl -s -f http://localhost:8001/api/v1/health/live >/dev/null; then
        log_error "Liveness check failed"
        podman rm -f "$container_name"
        exit 1
    fi
    
    # Test metrics endpoint
    if ! curl -s -f http://localhost:8001/api/v1/metrics >/dev/null; then
        log_error "Metrics endpoint failed"
        podman rm -f "$container_name"
        exit 1
    fi
    
    # Cleanup test container
    podman rm -f "$container_name"
    
    log_success "Docker image tests passed"
}

# Push image to registry
push_image() {
    if [[ "$REGISTRY" == "localhost:5000" ]]; then
        log_warning "Skipping push to localhost registry"
        return 0
    fi
    
    log_info "Pushing image to registry..."
    
    local image_tag="${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    local latest_tag="${REGISTRY}/${IMAGE_NAME}:latest"
    
    # Push versioned tag
    if ! podman push "$image_tag"; then
        log_error "Failed to push versioned image"
        exit 1
    fi
    
    # Push latest tag
    if ! podman push "$latest_tag"; then
        log_error "Failed to push latest image"
        exit 1
    fi
    
    log_success "Images pushed to registry"
}

# Generate deployment manifests
generate_manifests() {
    log_info "Generating deployment manifests..."
    
    local manifests_dir="deploy"
    mkdir -p "$manifests_dir"
    
    # Generate Kubernetes deployment
    cat > "$manifests_dir/deployment.yaml" << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pnar-world-api
  labels:
    app: pnar-world-api
    version: ${VERSION}
spec:
  replicas: 3
  selector:
    matchLabels:
      app: pnar-world-api
  template:
    metadata:
      labels:
        app: pnar-world-api
        version: ${VERSION}
    spec:
      containers:
      - name: api
        image: ${REGISTRY}/${IMAGE_NAME}:${VERSION}
        ports:
        - containerPort: 8000
        env:
        - name: APP_ENVIRONMENT
          value: "production"
        - name: DATABASE_HOST
          valueFrom:
            secretKeyRef:
              name: pnar-world-secrets
              key: database-host
        - name: DATABASE_USERNAME
          valueFrom:
            secretKeyRef:
              name: pnar-world-secrets
              key: database-username
        - name: DATABASE_PASSWORD
          valueFrom:
            secretKeyRef:
              name: pnar-world-secrets
              key: database-password
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: pnar-world-secrets
              key: jwt-secret
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/v1/health/live
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/v1/health/ready
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 5
        securityContext:
          allowPrivilegeEscalation: false
          runAsNonRoot: true
          runAsUser: 1000
          capabilities:
            drop:
            - ALL
---
apiVersion: v1
kind: Service
metadata:
  name: pnar-world-api-service
spec:
  selector:
    app: pnar-world-api
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8000
  type: ClusterIP
EOF

    # Generate docker-compose for simple deployments
    cat > "$manifests_dir/docker-compose.yml" << EOF
version: '3.8'

services:
  api:
    image: ${REGISTRY}/${IMAGE_NAME}:${VERSION}
    ports:
      - "8000:8000"
    environment:
      - APP_ENVIRONMENT=production
      - DATABASE_HOST=\${DATABASE_HOST}
      - DATABASE_USERNAME=\${DATABASE_USERNAME}
      - DATABASE_PASSWORD=\${DATABASE_PASSWORD}
      - DATABASE_NAME=\${DATABASE_NAME}
      - JWT_SECRET=\${JWT_SECRET}
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/api/v1/health/live"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '0.5'
        reservations:
          memory: 256M
          cpus: '0.25'

  postgres:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=\${DATABASE_NAME}
      - POSTGRES_USER=\${DATABASE_USERNAME}
      - POSTGRES_PASSWORD=\${DATABASE_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U \${DATABASE_USERNAME}"]
      interval: 10s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
EOF

    log_success "Deployment manifests generated in $manifests_dir/"
}

# Create release notes
create_release_notes() {
    log_info "Creating release notes..."
    
    local release_notes_file="RELEASE_NOTES_${VERSION}.md"
    
    cat > "$release_notes_file" << EOF
# PNAR World API Release ${VERSION}

**Release Date:** $(date -u +'%Y-%m-%d %H:%M:%S UTC')
**Git Hash:** $(git rev-parse --short HEAD)

## 🚀 Features

- Production-ready configuration with environment-specific settings
- Comprehensive security middleware (CORS, rate limiting, security headers)
- Enhanced health checks and monitoring endpoints
- Improved database connection pooling and migration handling
- Fail-fast startup with proper error handling
- Docker multi-stage build for optimized production images

## 🔒 Security Improvements

- Rate limiting middleware
- Security headers (CSP, HSTS, etc.)
- Non-root container execution
- Input validation and sanitization
- JWT token security enhancements

## 📊 Monitoring & Observability

- Comprehensive health check endpoints (/health, /ready, /live)
- Metrics endpoint for monitoring
- Request ID tracking
- Structured logging with JSON format
- Performance monitoring capabilities

## 🐛 Bug Fixes

- Fixed database connection handling
- Improved error responses
- Better CORS configuration
- Enhanced middleware error handling

## 📦 Deployment

### Docker
\`\`\`bash
podman run -d \\
  --name pnar-world-api \\
  -p 8000:8000 \\
  -e APP_ENVIRONMENT=production \\
  -e DATABASE_HOST=your-db-host \\
  -e DATABASE_USERNAME=your-username \\
  -e DATABASE_PASSWORD=your-password \\
  -e JWT_SECRET=your-jwt-secret \\
  ${REGISTRY}/${IMAGE_NAME}:${VERSION}
\`\`\`

### Kubernetes
\`\`\`bash
kubectl apply -f deploy/deployment.yaml
\`\`\`

### Docker Compose
\`\`\`bash
cd deploy && docker-compose up -d
\`\`\`

## 🔧 Configuration

See \`configuration.production.yaml\` for production configuration options.

## 📋 Requirements

- PostgreSQL 15+
- Rust 1.89+
- Container runtime (Docker/Podman)

## 🔗 Links

- [API Documentation](http://localhost:8000/swagger-ui/index.html)
- [Health Check](http://localhost:8000/api/v1/health)
- [Metrics](http://localhost:8000/api/v1/metrics)
EOF

    log_success "Release notes created: $release_notes_file"
}

# Main deployment function
main() {
    echo "🚀 PNAR World API Production Deployment"
    echo "========================================"
    echo "Version: $VERSION"
    echo "Environment: $ENVIRONMENT"
    echo "Registry: $REGISTRY"
    echo ""
    
    check_prerequisites
    run_tests
    build_application
    build_image
    test_image
    push_image
    generate_manifests
    create_release_notes
    
    echo ""
    log_success "🎉 Deployment completed successfully!"
    echo ""
    echo "📋 Next steps:"
    echo "  1. Review the generated manifests in deploy/"
    echo "  2. Update your environment variables"
    echo "  3. Deploy using your preferred method:"
    echo "     - Kubernetes: kubectl apply -f deploy/deployment.yaml"
    echo "     - Docker Compose: cd deploy && docker-compose up -d"
    echo "     - Podman: podman run ${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    echo ""
    echo "🔗 Useful endpoints:"
    echo "  • Health: http://your-domain/api/v1/health"
    echo "  • API Docs: http://your-domain/swagger-ui/index.html"
    echo "  • Metrics: http://your-domain/api/v1/metrics"
    echo ""
}

# Handle script arguments
case "${1:-deploy}" in
    "test")
        check_prerequisites
        run_tests
        ;;
    "build")
        check_prerequisites
        build_application
        build_image
        ;;
    "deploy")
        main
        ;;
    "help"|"--help"|"-h")
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  deploy  - Full deployment pipeline (default)"
        echo "  test    - Run tests only"
        echo "  build   - Build application and image only"
        echo "  help    - Show this help message"
        echo ""
        echo "Environment variables:"
        echo "  REGISTRY    - Container registry (default: localhost:5000)"
        echo "  ENVIRONMENT - Deployment environment (default: production)"
        ;;
    *)
        log_error "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac