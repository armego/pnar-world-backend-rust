#!/usr/bin/env bash
# remote-deploy.sh <systemd-service> <run-migrations:true|false> <database-url>
set -euo pipefail

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
systemctl --user stop "$SERVICE" 2>/dev/null || echo "Failed to stop service $SERVICE (may not exist or user session not available)"

if [ -f "$BIN_PATH" ]; then
  echo "Backing up existing binary..."
  mkdir -p "$BACKUP_DIR/$TIMESTAMP"
  # If BIN_PATH equals the location the archive will extract to, avoid copying the same file
  SRC_PATH="$APP_DIR/${BIN_NAME}"
  # Resolve canonical paths when possible to avoid false negatives
  CANON_BIN_PATH="$(readlink -f "$BIN_PATH" 2>/dev/null || echo "$BIN_PATH")"
  CANON_SRC_PATH="$(readlink -f "$SRC_PATH" 2>/dev/null || echo "$SRC_PATH")"
  echo "Existing binary canonical: $CANON_BIN_PATH"
  echo "Source binary canonical:   $CANON_SRC_PATH"
  if [ "$CANON_BIN_PATH" = "$CANON_SRC_PATH" ]; then
    echo "Binary source and target are identical; creating backup copy instead of move."
    cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
  else
    cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/" || true
  fi
fi

tar -xzf "$ARCHIVE" -C "$APP_DIR"
# Move the binary into place. Resolve canonical paths to detect identical files/symlinks.
SRC_PATH="$APP_DIR/${BIN_NAME}"
CANON_BIN_PATH="$(readlink -f "$BIN_PATH" 2>/dev/null || echo "$BIN_PATH")"
CANON_SRC_PATH="$(readlink -f "$SRC_PATH" 2>/dev/null || echo "$SRC_PATH")"
echo "Post-extract source canonical: $CANON_SRC_PATH"
if [ "$CANON_BIN_PATH" = "$CANON_SRC_PATH" ]; then
  echo "Source and target refer to the same file; skipping move."
else
  mv -f "$SRC_PATH" "$BIN_PATH"
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
  echo "Configuring UFW firewall..."
  # We must avoid interactive sudo in CI. Only attempt firewall changes if running as root.
  # Skip if not root to avoid sudo prompts.
  SUDO_CMD=""
  if [ "$(id -u)" -eq 0 ]; then
    # Running as root, no need for sudo
    SUDO_CMD=""
  else
    echo "Not running as root; skipping firewall configuration to avoid sudo prompts."
    SUDO_CMD=""
  fi

  if [ -n "$SUDO_CMD" ] || [ "$(id -u)" -eq 0 ]; then
    # Install ufw if missing (best-effort)
    if ! command -v ufw >/dev/null 2>&1; then
      echo "ufw not found; attempting to install..."
      ${SUDO_CMD:-} apt-get update && ${SUDO_CMD:-} apt-get install -y ufw || true
    fi

    # Allow SSH
    ${SUDO_CMD:-} ufw allow OpenSSH || true
    # Allow http/https (nginx)
    if ${SUDO_CMD:-} ufw status verbose | grep -q "Nginx Full"; then
      ${SUDO_CMD:-} ufw allow 'Nginx Full' || true
    else
      ${SUDO_CMD:-} ufw allow 80/tcp || true
      ${SUDO_CMD:-} ufw allow 443/tcp || true
    fi

    # Deny external access to API port
    ${SUDO_CMD:-} ufw deny proto tcp from any to any port 8000 || true

    # Enable ufw (non-interactively)
    ${SUDO_CMD:-} ufw --force enable || true
    ${SUDO_CMD:-} ufw status verbose || true
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
