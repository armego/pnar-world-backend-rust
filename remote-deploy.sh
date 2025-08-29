#!/usr/bin/env bash
# remote-deploy.sh <systemd-service> <run-migrations:true|false> <database-url>
set -euo pipefail

echo "=============================================="
echo "RUNNING CI-FRIENDLY DEPLOY SCRIPT (v3)"
echo "=============================================="
echo "Script: $0"
echo "PWD: $(pwd)"
echo "User: $(whoami) (UID: $(id -u))"
echo "Arguments: $*"
echo "CI env var: ${CI:-not set}"
echo "GITHUB_ACTIONS env var: ${GITHUB_ACTIONS:-not set}"
echo "=============================================="

# CI COMPATIBILITY MODE - Completely avoid any sudo, assume user-level deployment only
# If you need to install into /opt/pnar, run this script once as root

# IMMEDIATE DEBUG: Print what script this is and where it's running
echo "=== DEPLOY SCRIPT DEBUG START ==="
echo "Script: $0"
echo "PWD: $(pwd)"
echo "User: $(whoami)"
echo "UID: $(id -u))"
echo "Arguments: $*"
echo "Bash version: $BASH_VERSION"
echo "=== DEPLOY SCRIPT DEBUG END ==="

# Enable tracing to see exactly what's being executed
set -x

SERVICE="${1:-pnar.service}"
RUN_MIGRATIONS="${2:-false}"
DATABASE_URL="${3:-}"
# Optional fourth argument enables basic firewall hardening (true/false)
ENABLE_FIREWALL="${4:-false}"

APP_DIR="$(cd "$(dirname "$0")" && pwd)"
ARCHIVE="$APP_DIR/pnar-release.tar.gz"
# Preserve the binary name used in Cargo.toml
BIN_NAME="pnar-world-api"

# ALWAYS use local paths for CI - no /opt/pnar to avoid permission issues
BIN_PATH="$APP_DIR/${BIN_NAME}"
BACKUP_DIR="$APP_DIR/backups"

echo "Using local deployment path: BIN_PATH=$BIN_PATH"
echo "Using local backups path: BACKUP_DIR=$BACKUP_DIR"
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"

# This script runs as the deploy user (pnar). It performs non-privileged
# deployment into the same directory where the archive is uploaded.
# To run a persistent systemd --user service for this user, you may need to enable linger.
echo "Deploy: $(date -u) Service=${SERVICE} RunMigrations=${RUN_MIGRATIONS}"

# Check if running as root. If so, exit with a warning for CI.
if [ "$(id -u)" -eq 0 ]; then
    echo "WARNING: This script is running as root. This is not recommended for CI."
fi

# Debug: print which script is running and key paths for CI logs
SCRIPT_PATH="$(readlink -f "$0" 2>/dev/null || realpath "$0" 2>/dev/null || echo "$0")"
echo "Script path: $SCRIPT_PATH"
echo "APP_DIR: $APP_DIR"
echo "BIN_NAME: $BIN_NAME"
echo "BIN_PATH: $BIN_PATH"
echo "ARCHIVE: $ARCHIVE"

if [ ! -f "$ARCHIVE" ]; then
  echo "ERROR: archive not found: $ARCHIVE"
  exit 2
fi


# Create backup directory
mkdir -p "$BACKUP_DIR"

# Skip systemd operations in CI to avoid sudo prompts
if [ -z "${CI:-}" ] && [ -z "${GITHUB_ACTIONS:-}" ]; then
    echo "Attempting to stop service with systemctl --user (non-CI)..."
    systemctl --user stop "$SERVICE" || echo "Service stop failed or not running. Continuing..."
else
    echo "Skipping systemctl stop (CI compatibility mode)"
fi

# Backup if binary exists
if [ -f "$BIN_PATH" ]; then
  echo "Backing up existing binary from $BIN_PATH"
  mkdir -p "$BACKUP_DIR/$TIMESTAMP"
  cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
fi

# Extract the archive
echo "Extracting $ARCHIVE to $APP_DIR"
tar -xzf "$ARCHIVE" -C "$APP_DIR"

# Determine source and target paths and avoid moving the same file
SRC_PATH="$APP_DIR/${BIN_NAME}"
echo "Source: $SRC_PATH"
echo "Target: $BIN_PATH"

# Ensure we actually have the extracted binary
if [ ! -e "$SRC_PATH" ]; then
  echo "ERROR: extracted binary not found at $SRC_PATH"
  exit 3
fi

# If the target exists, check if source and target refer to the same file.
# Use inode comparison (-ef) when available and fall back to a path string compare.
if [ -e "$BIN_PATH" ]; then
  echo "Existing target found; checking identity"
  # Print inode info for debugging
  if command -v ls >/dev/null 2>&1; then
    echo "Source inode: $(ls -i "$SRC_PATH" 2>/dev/null | awk '{print $1}' || echo '?')"
    echo "Target inode: $(ls -i "$BIN_PATH" 2>/dev/null | awk '{print $1}' || echo '?')"
  fi

  same=false
  # Try inode comparison first
  if [ "$SRC_PATH" -ef "$BIN_PATH" ] 2>/dev/null; then
    same=true
  fi
  # Fall back to exact path equality if inode compare isn't supported
  if [ "$same" = false ] && [ "$(realpath "$SRC_PATH")" = "$(realpath "$BIN_PATH")" ]; then
    same=true
  fi

  if [ "$same" = true ]; then
    echo "Source and destination are the same file (by inode or path). Skipping move; updating perms."
    chmod 750 "$BIN_PATH" || true
  else
    echo "Moving extracted binary to destination"
    mv -f "$SRC_PATH" "$BIN_PATH"
    chmod 750 "$BIN_PATH" || true
  fi
else
  echo "No existing target; placing binary in destination"
  mv -f "$SRC_PATH" "$BIN_PATH"
  chmod 750 "$BIN_PATH" || true
fi

# Create/update environment file
LOCAL_ENV_FILE="$APP_DIR/pnar.env"
echo "Creating/updating environment file at $LOCAL_ENV_FILE"
cat > "$LOCAL_ENV_FILE" <<'EOF'
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
RUST_LOG=info
EOF
chmod 600 "$LOCAL_ENV_FILE"

# Skip firewall in CI mode
if [ "$ENABLE_FIREWALL" = "true" ]; then
    if [ "$(id -u)" -eq 0 ]; then
        echo "Configuring firewall as root..."
        ufw allow OpenSSH
        ufw allow 80/tcp
        ufw allow 443/tcp
        ufw deny 8000/tcp
        ufw --force enable
    else
        echo "Firewall configuration requested but skipped (not running as root)."
    fi
fi

# Run migrations if requested
if [ "$RUN_MIGRATIONS" = "true" ]; then
  if [ -n "$DATABASE_URL" ]; then
    echo "Running migrations..."
    if [ -x "$BIN_PATH" ] && "$BIN_PATH" --migrate --database-url="$DATABASE_URL" 2>/dev/null; then
      echo "Migrations via binary completed"
    else
      echo "Binary does not support migrations flag or failed, attempting sqlx-cli..."
      if command -v sqlx >/dev/null 2>&1; then
        DATABASE_URL="$DATABASE_URL" sqlx migrate run
      else
        echo "sqlx not found; skipping migrations."
      fi
    fi
  else
    echo "DATABASE_URL is empty; skipping migrations."
  fi
fi

# Skip systemd operations in CI to avoid sudo prompts
if [ -z "${CI:-}" ] && [ -z "${GITHUB_ACTIONS:-}" ]; then
    echo "Attempting to start service with systemctl --user (non-CI)..."
    systemctl --user daemon-reload || echo "systemd daemon-reload failed (non-critical)"
    systemctl --user start "$SERVICE" || echo "systemd start failed (service may not exist yet)"
    echo "To check status: systemctl --user status $SERVICE"
else
    echo "Skipping systemctl start (CI compatibility mode)"
fi
echo "Deployment finished at $(date -u)"
echo "Deploy: $(date -u) Service=${SERVICE} RunMigrations=${RUN_MIGRATIONS}"

# Debug: print which script is running and key paths for CI logs
SCRIPT_PATH="$(readlink -f "$0" 2>/dev/null || realpath "$0" 2>/dev/null || echo "$0")"
echo "Script path: $SCRIPT_PATH"
echo "APP_DIR: $APP_DIR"
echo "BIN_NAME: $BIN_NAME"
echo "BIN_PATH: $BIN_PATH"
echo "ARCHIVE: $ARCHIVE"

if [ ! -f "$ARCHIVE" ]; then
  echo "ERROR: archive not found: $ARCHIVE"
  exit 2
fi


# Create backup directory
mkdir -p "$BACKUP_DIR"

# Skip systemd operations in CI to avoid sudo prompts
echo "Skipping systemctl stop (CI compatibility mode)"

# Backup if binary exists
if [ -f "$BIN_PATH" ]; then
  echo "Backing up existing binary..."
  mkdir -p "$BACKUP_DIR/$TIMESTAMP"
  cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
fi

# Extract the archive
echo "Extracting $ARCHIVE to $APP_DIR"
tar -xzf "$ARCHIVE" -C "$APP_DIR"

# Determine source and target paths and avoid moving the same file
SRC_PATH="$APP_DIR/${BIN_NAME}"
echo "Source: $SRC_PATH"
echo "Target: $BIN_PATH"

# Ensure we actually have the extracted binary
if [ ! -e "$SRC_PATH" ]; then
  echo "ERROR: extracted binary not found at $SRC_PATH"
  exit 3
fi

# If the target exists, check if source and target refer to the same file.
# Use inode comparison (-ef) when available and fall back to a path string compare.
if [ -e "$BIN_PATH" ]; then
  echo "Existing target found; checking identity"
  # Print inode info for debugging
  if command -v ls >/dev/null 2>&1; then
    echo "Source inode: $(ls -i "$SRC_PATH" 2>/dev/null | awk '{print $1}' || echo '?')"
    echo "Target inode: $(ls -i "$BIN_PATH" 2>/dev/null | awk '{print $1}' || echo '?')"
  fi

  same=false
  # Try inode comparison first
  if [ "$SRC_PATH" -ef "$BIN_PATH" ] 2>/dev/null; then
    same=true
  fi
  # Fall back to exact path equality if inode compare isn't supported
  if [ "$same" = false ] && [ "$SRC_PATH" = "$BIN_PATH" ]; then
    same=true
  fi

  if [ "$same" = true ]; then
    echo "Source and destination are the same file (by inode or path). Skipping move; updating perms."
    chmod 750 "$BIN_PATH" || true
  else
    echo "Moving extracted binary to destination"
    mv -f "$SRC_PATH" "$BIN_PATH"
    chmod 750 "$BIN_PATH" || true
  fi
else
  echo "No existing target; placing binary in destination"
  mv -f "$SRC_PATH" "$BIN_PATH"
  chmod 750 "$BIN_PATH" || true
fi

# Create/update environment file
LOCAL_ENV_FILE="$APP_DIR/pnar.env"
echo "Creating/updating environment file at $LOCAL_ENV_FILE"
cat > "$LOCAL_ENV_FILE" <<'EOF'
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
RUST_LOG=info
EOF
chmod 600 "$LOCAL_ENV_FILE"

# Skip firewall in CI mode
if [ "$ENABLE_FIREWALL" = "true" ]; then
  echo "Firewall configuration requested but skipped in CI compatibility mode"
  echo "To configure firewall, run this script as root or use: ufw allow OpenSSH; ufw allow 80,443/tcp; ufw deny 8000/tcp"
fi

# Run migrations if requested
if [ "$RUN_MIGRATIONS" = "true" ]; then
  if [ -n "$DATABASE_URL" ]; then
    echo "Running migrations..."
    if [ -x "$BIN_PATH" ] && "$BIN_PATH" --migrate --database-url="$DATABASE_URL" 2>/dev/null; then
      echo "Migrations via binary completed"
    else
      echo "Binary does not support migrations flag or failed, attempting sqlx-cli..."
      if command -v sqlx >/dev/null 2>&1; then
        DATABASE_URL="$DATABASE_URL" sqlx migrate run
      else
        echo "sqlx not found; skipping migrations."
      fi
    fi
  else
    echo "DATABASE_URL is empty; skipping migrations."
  fi
fi

# Skip systemd operations in CI to avoid sudo prompts
echo "Skipping systemctl operations (CI compatibility mode)"
echo "To start the service manually:"
echo "  systemctl --user start $SERVICE"
echo "Deployment finished at $(date -u)"
