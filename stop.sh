#!/bin/bash

# PNAR World Application Pod Stop Script

set -euo pipefail

readonly POD_NAME="pnar-app-pod"
readonly GREEN='\033[0;32m'
readonly BLUE='\033[0;34m'
readonly RED='\033[0;31m'
readonly NC='\033[0m'

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

echo "🛑 Stopping PNAR World Application Pod..."
echo "========================================="

if podman pod exists "$POD_NAME" 2>/dev/null; then
    log_info "Stopping application pod gracefully..."
    podman pod stop "$POD_NAME" --timeout 10 2>/dev/null || {
        log_info "Graceful stop failed, forcing stop..."
        podman pod kill "$POD_NAME" 2>/dev/null || true
    }
    
    log_success "Application pod stopped successfully"
    echo ""
    echo "💡 To start again: ./start.sh"
    echo "💡 To remove completely: podman pod rm -f $POD_NAME"
else
    log_error "Pod '$POD_NAME' not found"
    exit 1
fi