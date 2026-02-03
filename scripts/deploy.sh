#!/usr/bin/env bash
set -euo pipefail
: # 默认部署用户
DEPLOY_USER="${DEPLOY_USER:-ubuntu}"
: "${DEPLOY_SSH_KEY:?}"
DEPLOY_HOST="${DEPLOY_HOST:-52.83.216.97}"
DEPLOY_PORT="${DEPLOY_PORT:-22}"
ARTIFACT_FILE="${ARTIFACT_FILE:-xtrace.tar.gz}"
APP_DIR_REMOTE="${APP_DIR_REMOTE:-}"
KEY_FILE="$(mktemp)"
trap 'rm -f "$KEY_FILE"' EXIT
printf "%s\n" "$DEPLOY_SSH_KEY" > "$KEY_FILE"
chmod 600 "$KEY_FILE"
scp -P "$DEPLOY_PORT" -i "$KEY_FILE" -o StrictHostKeyChecking=no "$ARTIFACT_FILE" "$DEPLOY_USER@$DEPLOY_HOST:/tmp/xtrace.tar.gz"
ssh -p "$DEPLOY_PORT" -i "$KEY_FILE" -o StrictHostKeyChecking=no "$DEPLOY_USER@$DEPLOY_HOST" bash -s <<'REMOTE'
set -euo pipefail
APP_DIR="${APP_DIR_REMOTE:-$HOME/xtrace}"
mkdir -p "$APP_DIR"
tar -xzf /tmp/xtrace.tar.gz -C "$APP_DIR"
chmod +x "$APP_DIR/xtrace"
if ! command -v pm2 >/dev/null 2>&1; then
  npm i -g pm2
fi
SCRIPT_PATH="./xtrace"
if [ ! -x "$APP_DIR/xtrace" ]; then
  if [ -x "$APP_DIR/target/release/xtrace" ]; then
    SCRIPT_PATH="./target/release/xtrace"
  else
    echo "xtrace binary not found in $APP_DIR (./xtrace) or ./target/release/xtrace" >&2
    exit 1
  fi
fi
cat > "$APP_DIR/ecosystem.config.js" <<EOF
module.exports = {
  apps: [{
    name: "xtrace",
    script: "$SCRIPT_PATH",
    interpreter: "none",
    cwd: process.env.APP_DIR || ".",
    env: {
      DATABASE_URL: process.env.DATABASE_URL,
      API_BEARER_TOKEN: process.env.API_BEARER_TOKEN,
      BIND_ADDR: "0.0.0.0:8742"
    },
    instances: 1,
    autorestart: true,
    max_restarts: 10
  }]
}
EOF
export APP_DIR="$APP_DIR"
cd "$APP_DIR"
ENV_FILE="$APP_DIR/.env"
if [ -f "$ENV_FILE" ]; then
  set -a
  . "$ENV_FILE"
  set +a
else
  echo ".env not found at $ENV_FILE" >&2
  exit 1
fi
if [ -z "${DATABASE_URL:-}" ] || [ -z "${API_BEARER_TOKEN:-}" ]; then
  echo "Missing DATABASE_URL or API_BEARER_TOKEN in $ENV_FILE" >&2
  exit 1
fi
export DATABASE_URL
export API_BEARER_TOKEN
pid="$(pm2 pid xtrace || true)"
if [ "${pid:-0}" = "0" ] || [ -z "${pid:-}" ]; then
  pm2 start ecosystem.config.js --only xtrace --update-env
else
  pm2 restart xtrace --update-env
fi
pm2 save
REMOTE
