# Multi-stage build for production-ready PNAR World API
# Build stage: Compile the Rust application
FROM rust:1.89.0-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create app user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY configuration.yaml ./

# Build the application with optimizations
RUN cargo build --release --locked

# Verify the binary was created
RUN ls -la target/release/

# Runtime stage: Create minimal runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    libpq5 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /tmp/* \
    && rm -rf /var/tmp/*

# Create non-root user
RUN groupadd -r appuser && useradd -r -g appuser -d /app -s /bin/false appuser

WORKDIR /app

# Copy application binary and configuration
COPY --from=builder /app/target/release/pnar-world-api ./
COPY --from=builder /app/configuration.yaml ./
COPY --from=builder /app/migrations ./migrations

# Create necessary directories
RUN mkdir -p /var/log && \
    chown -R appuser:appuser /app /var/log

# Set environment variables
ENV APP_ENVIRONMENT=production \
    RUST_LOG=info \
    RUST_BACKTRACE=1

# Security: Switch to non-root user
USER appuser

# Document the port
EXPOSE 8000

# Add labels for better container management
LABEL org.opencontainers.image.title="PNAR World API" \
      org.opencontainers.image.description="A modern web service for Pnar language translation" \
      org.opencontainers.image.version="1.0.0" \
      org.opencontainers.image.authors="Stavros Grigoriou <unix121@protonmail.com>" \
      org.opencontainers.image.source="https://github.com/armego/pnar-world-backend-rust" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.created="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
      maintainer="Stavros Grigoriou <unix121@protonmail.com>"

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/api/v1/health/live || exit 1

# Run the binary
CMD ["./pnar-world-api"]