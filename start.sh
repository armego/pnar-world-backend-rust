#!/bin/bash

# PNAR World Application Startup Script
# Complete setup with PostgreSQL + Adminer + Rust API

set -euo pipefail

# Configuration
readonly POD_NAME="pnar-app-pod"
readonly API_IMAGE="pnar-world-api:1.0.0"
readonly MAX_RETRIES=30
readonly RETRY_DELAY=2
readonly CLEANUP_DELAY=3

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
    log_info "Checking prerequisites..."
    
    if ! command -v podman &> /dev/null; then
        log_error "Podman is not installed or not in PATH"
        exit 1
    fi
    
    if [[ ! -f "pod.yaml" ]]; then
        log_error "pod.yaml not found in current directory"
        exit 1
    fi
    
    if [[ ! -f "Dockerfile" ]]; then
        log_error "Dockerfile not found in current directory"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Build API image with error handling
build_api_image() {
    log_info "Building API image: $API_IMAGE"
    
    if ! podman build -t "$API_IMAGE" . --quiet; then
        log_error "Failed to build API image"
        exit 1
    fi
    
    # Verify image was created
    if ! podman image exists "$API_IMAGE"; then
        log_error "API image was not created successfully"
        exit 1
    fi
    
    log_success "API image built successfully"
}

# Check pod status
check_pod_status() {
    if podman pod exists "$POD_NAME" 2>/dev/null; then
        local status
        status=$(podman pod inspect "$POD_NAME" --format "{{.State}}" 2>/dev/null || echo "unknown")
        echo "$status"
    else
        echo "not_found"
    fi
}

# Cleanup existing pod
cleanup_existing_pod() {
    local pod_status
    pod_status=$(check_pod_status)
    
    if [[ "$pod_status" != "not_found" ]]; then
        log_info "Found existing pod with status: $pod_status"
        
        if [[ "$pod_status" == "Running" ]]; then
            log_info "Gracefully stopping running pod..."
            podman pod stop "$POD_NAME" --timeout 10 2>/dev/null || {
                log_warning "Graceful stop failed, forcing stop..."
                podman pod kill "$POD_NAME" 2>/dev/null || true
            }
        fi
        
        log_info "Removing existing pod..."
        podman pod rm -f "$POD_NAME" 2>/dev/null || true
        
        log_info "Waiting for cleanup to complete..."
        sleep $CLEANUP_DELAY
        
        if podman pod exists "$POD_NAME" 2>/dev/null; then
            log_error "Failed to remove existing pod"
            exit 1
        fi
        
        log_success "Existing pod cleaned up successfully"
    fi
}

# Start the application pod
start_pod() {
    log_info "Starting application pod..."
    
    local retry_count=0
    while [[ $retry_count -lt 3 ]]; do
        if podman play kube pod.yaml --quiet; then
            log_success "Application pod started successfully"
            return 0
        else
            retry_count=$((retry_count + 1))
            log_warning "Pod start attempt $retry_count failed, retrying..."
            sleep $RETRY_DELAY
            
            podman pod rm -f "$POD_NAME" 2>/dev/null || true
            sleep 1
        fi
    done
    
    log_error "Failed to start pod after 3 attempts"
    exit 1
}

# Wait for container to be running
wait_for_container() {
    local container_name="$1"
    local max_wait="$2"
    local wait_count=0
    
    log_info "Waiting for $container_name container to be running..."
    
    while [[ $wait_count -lt $max_wait ]]; do
        local status
        status=$(podman container inspect "${POD_NAME}-${container_name}" --format "{{.State.Status}}" 2>/dev/null || echo "not_found")
        
        if [[ "$status" == "running" ]]; then
            log_success "$container_name container is running"
            return 0
        elif [[ "$status" == "exited" ]] || [[ "$status" == "stopped" ]]; then
            log_error "$container_name container has stopped unexpectedly"
            podman logs "${POD_NAME}-${container_name}" --tail 10 2>/dev/null || true
            return 1
        fi
        
        wait_count=$((wait_count + 1))
        sleep 1
    done
    
    log_error "$container_name container failed to start within ${max_wait}s"
    return 1
}

# Wait for PostgreSQL to be ready
wait_for_postgres() {
    log_info "Waiting for PostgreSQL to be ready..."
    
    if ! wait_for_container "postgres" 10; then
        log_error "PostgreSQL container failed to start"
        return 1
    fi
    
    local retry_count=0
    while [[ $retry_count -lt $MAX_RETRIES ]]; do
        if podman exec "${POD_NAME}-postgres" pg_isready -h 127.0.0.1 -U postgres -q 2>/dev/null; then
            if podman exec "${POD_NAME}-postgres" psql -h 127.0.0.1 -U postgres -d pnar_world -c "SELECT 1;" >/dev/null 2>&1; then
                log_success "PostgreSQL is ready and database 'pnar_world' is accessible"
                return 0
            fi
        fi
        
        retry_count=$((retry_count + 1))
        if [[ $((retry_count % 5)) -eq 0 ]]; then
            log_info "Still waiting for PostgreSQL... (attempt $retry_count/$MAX_RETRIES)"
        fi
        sleep $RETRY_DELAY
    done
    
    log_error "PostgreSQL failed to become ready"
    podman logs "${POD_NAME}-postgres" --tail 20 2>/dev/null || true
    return 1
}

# Wait for Adminer to be ready
wait_for_adminer() {
    log_info "Waiting for Adminer to be ready..."
    
    if ! wait_for_container "adminer" 10; then
        log_error "Adminer container failed to start"
        return 1
    fi
    
    local retry_count=0
    while [[ $retry_count -lt 15 ]]; do
        if curl -s -f http://localhost:8080/ >/dev/null 2>&1; then
            log_success "Adminer is ready and responding"
            return 0
        fi
        
        retry_count=$((retry_count + 1))
        sleep 2
    done
    
    log_warning "Adminer health check timeout, but container is running"
    return 0
}

# Wait for API to be ready
wait_for_api() {
    log_info "Waiting for API to be ready..."
    
    if ! wait_for_container "api" 15; then
        log_error "API container failed to start"
        return 1
    fi
    
    local retry_count=0
    while [[ $retry_count -lt $MAX_RETRIES ]]; do
        # Try different health check endpoints
        if curl -s -f http://localhost:8000/api/v1/actuator/health >/dev/null 2>&1 || 
           curl -s -f http://localhost:8000/health >/dev/null 2>&1 ||
           curl -s -f http://localhost:8000/ >/dev/null 2>&1; then
            log_success "API is ready and responding"
            return 0
        fi
        
        retry_count=$((retry_count + 1))
        if [[ $((retry_count % 5)) -eq 0 ]]; then
            log_info "Still waiting for API... (attempt $retry_count/$MAX_RETRIES)"
        fi
        sleep $RETRY_DELAY
    done
    
    log_warning "API health check failed, but container is running"
    log_info "API logs:"
    podman logs "${POD_NAME}-api" --tail 20 2>/dev/null || true
    return 0  # Don't fail the entire script for API health check
}

# Display connection information
show_connection_info() {
    echo ""
    echo "🎉 PNAR World Application Setup Complete!"
    echo "========================================="
    echo ""
    echo "🚀 PNAR World API:"
    echo "  • Base URL:     http://localhost:8000"
    echo "  • Health Check: http://localhost:8000/api/v1/actuator/health"
    echo "  • Swagger UI:   http://localhost:8000/swagger-ui/index.html"
    echo ""
    echo "🗄️  PostgreSQL Database:"
    echo "  • Host:     localhost"
    echo "  • Port:     5432"
    echo "  • Database: pnar_world"
    echo "  • Username: postgres"
    echo "  • Password: root"
    echo ""
    echo "🌐 Adminer Database Client:"
    echo "  • URL:      http://localhost:8080"
    echo "  • System:   PostgreSQL"
    echo "  • Server:   localhost:5432"
    echo "  • Username: postgres"
    echo "  • Password: root"
    echo "  • Database: pnar_world"
    echo ""
    echo "📝 Quick Start:"
    echo "  1. Open http://localhost:8080 to manage your database"
    echo "  2. Create your tables manually using the SQL interface"
    echo "  3. Use http://localhost:8000 for API calls from your JavaScript client"
    echo "  4. Check API documentation at http://localhost:8000/swagger-ui/index.html"
    echo ""
    echo "🔧 Management Commands:"
    echo "  • Stop:    podman pod stop $POD_NAME"
    echo "  • Start:   podman pod start $POD_NAME"
    echo "  • Logs:    podman pod logs $POD_NAME"
    echo "  • Remove:  podman pod rm -f $POD_NAME"
    echo ""
    echo "💡 Tips:"
    echo "  • Your data persists as long as the pod exists"
    echo "  • API is ready for JavaScript client connections"
    echo "  • Use Adminer for manual database operations"
    echo ""
}

# Cleanup on exit
cleanup_on_exit() {
    if [[ $? -ne 0 ]]; then
        log_error "Script failed. Cleaning up..."
        podman pod rm -f "$POD_NAME" 2>/dev/null || true
    fi
}

# Main execution
main() {
    trap cleanup_on_exit EXIT
    
    echo "🚀 Starting PNAR World Application Setup..."
    echo "==========================================="
    
    check_prerequisites
    cleanup_existing_pod
    build_api_image
    start_pod
    
    if wait_for_postgres; then
        wait_for_adminer
        wait_for_api
        show_connection_info
    else
        log_error "Application setup failed"
        exit 1
    fi
}

# Run main function
main "$@"