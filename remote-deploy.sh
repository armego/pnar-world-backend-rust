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
BIN_PATH="/opt/pnar/${BIN_NAME}"
BACKUP_DIR="/opt/pnar/backups"
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"

echo "Deploy: $(date -u) Service=${SERVICE} RunMigrations=${RUN_MIGRATIONS}"

if [ ! -f "$ARCHIVE" ]; then
  echo "ERROR: archive not found: $ARCHIVE"
  exit 2
fi

mkdir -p "$BACKUP_DIR"
sudo systemctl stop "$SERVICE" || true

if [ -f "$BIN_PATH" ]; then
  echo "Backing up existing binary..."
  mkdir -p "$BACKUP_DIR/$TIMESTAMP"
  cp -a "$BIN_PATH" "$BACKUP_DIR/$TIMESTAMP/"
fi

tar -xzf "$ARCHIVE" -C "$APP_DIR"
mv -f "$APP_DIR/${BIN_NAME}" "$BIN_PATH"
chown pnar:pnar "$BIN_PATH" || true
chmod 750 "$BIN_PATH" || true

# Ensure /etc/pnar exists and configure SERVER_HOST to bind to localhost so API is local-only
ETC_ENV_DIR="/etc/pnar"
ETC_ENV_FILE="$ETC_ENV_DIR/pnar.env"
sudo mkdir -p "$ETC_ENV_DIR"
sudo chown root:root "$ETC_ENV_DIR"
sudo chmod 700 "$ETC_ENV_DIR"

if [ -f "$ETC_ENV_FILE" ]; then
  # Replace or add SERVER_HOST=127.0.0.1
  if sudo grep -q '^SERVER_HOST=' "$ETC_ENV_FILE" 2>/dev/null; then
    sudo sed -i 's/^SERVER_HOST=.*/SERVER_HOST=127.0.0.1/' "$ETC_ENV_FILE"
  else
    echo 'SERVER_HOST=127.0.0.1' | sudo tee -a "$ETC_ENV_FILE" >/dev/null
  fi
else
  # Create a minimal env file if missing (do not overwrite secrets)
  sudo tee "$ETC_ENV_FILE" > /dev/null <<'EOF'
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
RUST_LOG=info
EOF
  sudo chmod 600 "$ETC_ENV_FILE"
fi

# Optional: configure UFW to only allow SSH and nginx, and block direct API port exposure
if [ "$ENABLE_FIREWALL" = "true" ]; then
  echo "Configuring UFW firewall..."
  # Install ufw if missing (best-effort)
  if ! command -v ufw >/dev/null 2>&1; then
    echo "ufw not found; attempting to install..."
    sudo apt-get update && sudo apt-get install -y ufw || true
  fi

  # Allow SSH
  sudo ufw allow OpenSSH || true
  # Allow http/https (nginx)
  if sudo ufw status verbose | grep -q "Nginx Full"; then
    sudo ufw allow 'Nginx Full' || true
  else
    sudo ufw allow 80/tcp || true
    sudo ufw allow 443/tcp || true
  fi

  # Deny external access to API port
  sudo ufw deny proto tcp from any to any port 8000 || true

  # Enable ufw (non-interactively)
  sudo ufw --force enable || true
  sudo ufw status verbose || true
fi

if [ "$RUN_MIGRATIONS" = "true" ]; then
  if [ -n "$DATABASE_URL" ]; then
    echo "Running migrations..."
    if "$BIN_PATH" --migrate --database-url="$DATABASE_URL"; then
      echo "Migrations via binary completed."
    else
      echo "Binary does not support migrations flag or failed, attempting sqlx-cli..."
      if command -v sqlx >/dev/null 2>&1; then
        sudo -u pnar DATABASE_URL="$DATABASE_URL" sqlx migrate run
      else
        echo "sqlx not found; skipping migrations."
      fi
    fi
  else
    echo "DATABASE_URL is empty; skipping migrations."
  fi
fi

sudo systemctl daemon-reload
sudo systemctl start "$SERVICE"
sleep 1
sudo systemctl status "$SERVICE" --no-pager
echo "Deployment finished at $(date -u)"
