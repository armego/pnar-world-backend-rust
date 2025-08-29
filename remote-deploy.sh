#!/usr/bin/env bash
# remote-deploy.sh <systemd-service> <run-migrations:true|false> <database-url>
set -euo pipefail

SERVICE="${1:-pnar.service}"
RUN_MIGRATIONS="${2:-false}"
DATABASE_URL="${3:-}"

APP_DIR="$(cd "$(dirname "$0")" && pwd)"
ARCHIVE="$APP_DIR/pnar-release.tar.gz"
BIN_PATH="/opt/pnar/pnar"
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
mv -f "$APP_DIR/pnar" "$BIN_PATH"
chown pnar:pnar "$BIN_PATH" || true
chmod 750 "$BIN_PATH" || true

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
