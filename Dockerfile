# Build stage: Compile the Rust application
FROM rust:1.89.0 AS builder

WORKDIR /app

LABEL org.opencontainers.image.version="1.0.0" \
      org.opencontainers.image.authors="armego" \
      org.opencontainers.image.source="github.com/armego/pnar-world-backend-rust"

# Copy all files
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage: Create minimal runtime image
FROM debian:bookworm-slim

# Create non-root user and install minimal runtime dependencies
RUN useradd -r -s /bin/false appuser && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        openssl \
        postgresql-client && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy application binary and required files from builder
COPY --from=builder /app/target/release/pnar-world-api .
COPY --from=builder /app/configuration.yaml .

# Set ownership for security
RUN chown -R appuser:appuser /app

# Set environment variables
ENV APP_ENVIRONMENT=production \
    RUST_LOG=info

# Switch to non-root user
USER appuser

# Document the port
EXPOSE 8000

# Set healthcheck
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:8000/api/v1/actuator/health || exit 1

# Run the binary
CMD ["./pnar-world-api"]