#!/usr/bin/env bash
# remote-# ALWAYS deploy into the user-writable app dir in CI
BIN_PATH="$APP_DIR/${BIN_NAME}"
BACKUP_DIR="$APP_DIR/backups"
echo "Using local deployment path: BIN_PATH=$BIN_PATH"
echo "Using local backups path: BACKUP_DIR=$BACKUP_DIR"h <systemd-service> <run-migrations:true|false> <database-url>
set -euo pipefail

# IMMEDIATE DEBUG: Print what script this is and where it's running
echo "=== DEPLOY SCRIPT DEBUG START ==="
echo "Script: $0"
echo "PWD: $(pwd)"
echo "User: $(whoami)"
echo "UID: $(id -u)"
echo "Arguments: $*"
echo "Bash version: $BASH_VERSION"
echo "Environment: $(env | grep -v DATABASE_URL | grep -v KEY)"
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
OPT_BIN_DIR="/opt/pnar"
# Prefer installing into /opt/pnar when writable; otherwise deploy into the user-writable app dir
if [ -d "$OPT_BIN_DIR" ] && [ -w "$OPT_BIN_DIR" ]; then
  BIN_PATH="$OPT_BIN_DIR/${BIN_NAME}"
  BACKUP_DIR="$OPT_BIN_DIR/backups"
else
  BIN_PATH="$APP_DIR/${BIN_NAME}"
  BACKUP_DIR="$APP_DIR/backups"
fi
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"

# This script runs as the deploy user (pnar). It performs non-privileged
# deployment into the same directory where the archive is uploaded.
# To run a persistent systemd --user service for this user, enable linger:
#   sudo loginctl enable-linger pnar
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


mkdir -p "$BACKUP_DIR"
# Stop the user-level service (non-privileged)
# Ensure XDG_RUNTIME_DIR is set for systemctl --user to work properly
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"
echo "Using XDG_RUNTIME_DIR: $XDG_RUNTIME_DIR"
echo "About to call systemctl --user stop $SERVICE"
echo "Current user: $(whoami), UID: $(id -u)"
systemctl --user stop "$SERVICE" 2>/dev/null || echo "Failed to stop service $SERVICE (may not exist or user session not available)"

if [ -f "$BIN_PATH" ]; then
  echo "Backing up existing binary..."
  mkdir -p "$BACKUP_DIR/$TIMESTAMP"
  # If BIN_PATH equals the location the archive will extract to, avoid copying the same file
  SRC_PATH="$APP_DIR/${BIN_NAME}"
  # Prefer a robust same-file check using -ef (inode equivalence) when both files exist
  if [ -e "$BIN_PATH" ] && [ -e "$SRC_PATH" ] && [ "$BIN_PATH" -ef "$SRC_PATH" ]; then
    echo "Binary source and target are the same file (inode); creating backup copy instead of move."
    cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
  else
    cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
  fi
fi

tar -xzf "$ARCHIVE" -C "$APP_DIR"
# Move the binary into place. Resolve canonical paths to detect identical files/symlinks.
SRC_PATH="$APP_DIR/${BIN_NAME}"
echo "Post-extract source: $SRC_PATH"
# If both files exist and are the same inode, skip moving; otherwise perform move
if [ -e "$BIN_PATH" ] && [ -e "$SRC_PATH" ] && [ "$BIN_PATH" -ef "$SRC_PATH" ]; then
  echo "Source and target are the same file (inode); skipping move."
else
  # Fallback: also skip if the path strings are identical
  if [ "$SRC_PATH" != "$BIN_PATH" ]; then
    mv -f "$SRC_PATH" "$BIN_PATH"
  else
    echo "Source and destination paths identical; skipping mv."
  fi
fi
echo "Set perms on $BIN_PATH"
chmod 750 "$BIN_PATH" || true
chmod 750 "$BIN_PATH" || true

# Create/update a local env file in the deploy directory (non-privileged)
LOCAL_ENV_FILE="$APP_DIR/pnar.env"
if [ -f "$LOCAL_ENV_FILE" ]; then
  if grep -q '^SERVER_HOST=' "$LOCAL_ENV_FILE" 2>/dev/null; then
    sed -i 's/^SERVER_HOST=.*/SERVER_HOST=127.0.0.1/' "$LOCAL_ENV_FILE"
  else
    echo 'SERVER_HOST=127.0.0.1' >> "$LOCAL_ENV_FILE"
  fi
else
  cat > "$LOCAL_ENV_FILE" <<'EOF'
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
RUST_LOG=info
EOF
  chmod 600 "$LOCAL_ENV_FILE"
fi

# Note: If you want a persistent systemd --user service, enable linger for the user once:
# sudo loginctl enable-linger pnar

# Optional: configure UFW to only allow SSH and nginx, and block direct API port exposure
if [ "$ENABLE_FIREWALL" = "true" ]; then
  if [ "$(id -u)" -ne 0 ]; then
    echo "Firewall requested but not running as root; skipping firewall configuration to avoid sudo prompts."
  else
    echo "Configuring UFW firewall (running as root)..."
    if ! command -v ufw >/dev/null 2>&1; then
      echo "ufw not found; installing..."
      apt-get update && apt-get install -y ufw || true
    fi
    ufw allow OpenSSH || true
    if ufw status verbose | grep -q "Nginx Full"; then
      ufw allow 'Nginx Full' || true
    else
      ufw allow 80/tcp || true
      ufw allow 443/tcp || true
    fi
    ufw deny proto tcp from any to any port 8000 || true
    ufw --force enable || true
    ufw status verbose || true
  fi
fi

if [ "$RUN_MIGRATIONS" = "true" ]; then
  if [ -n "$DATABASE_URL" ]; then
    echo "Running migrations..."
    if "$BIN_PATH" --migrate --database-url="$DATABASE_URL"; then
      echo "Migrations via binary completed."
    else
      echo "Binary does not support migrations flag or failed, attempting sqlx-cli..."
      if command -v sqlx >/dev/null 2>&1; then
    # Run migrations as this user
    DATABASE_URL="$DATABASE_URL" sqlx migrate run
      else
        echo "sqlx not found; skipping migrations."
      fi
    fi
  else
    echo "DATABASE_URL is empty; skipping migrations."
  fi
fi
  # Reload and start the user-level service
  echo "Reloading and starting user systemd service..."
  systemctl --user daemon-reload 2>/dev/null || echo "Failed to reload systemd --user daemon"
  systemctl --user start "$SERVICE" 2>/dev/null || echo "Failed to start service $SERVICE"
  sleep 1
  systemctl --user status "$SERVICE" --no-pager 2>/dev/null || echo "Failed to get status of service $SERVICE"
echo "Deployment finished at $(date -u)"
