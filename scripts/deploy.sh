#!/usr/bin/env bash
set -Eeuo pipefail
on_err() {
  local code="$?"
  local line="${BASH_LINENO[0]:-${LINENO}}"
  local cmd="${BASH_COMMAND:-}"
  if [ "${#cmd}" -gt 200 ]; then
    cmd="${cmd:0:200}..."
  fi
  echo "deploy failed: exit=${code} line=${line} cmd=${cmd}" >&2
  exit "$code"
}
trap on_err ERR
: # Default deploy user
DEPLOY_USER="${DEPLOY_USER:-ubuntu}"
: "${DEPLOY_SSH_KEY:?}"
DEPLOY_HOST="${DEPLOY_HOST:-52.83.216.97}"
DEPLOY_PORT="${DEPLOY_PORT:-22}"
ARTIFACT_FILE="${ARTIFACT_FILE:-xtrace.tar.gz}"
APP_DIR_REMOTE="${APP_DIR_REMOTE:-}"
DEPLOY_SSH_VERBOSE="${DEPLOY_SSH_VERBOSE:-0}"

if [ -z "${DEPLOY_HOST}" ]; then
  echo "DEPLOY_HOST is empty" >&2
  exit 1
fi
if [ -z "${DEPLOY_USER}" ]; then
  echo "DEPLOY_USER is empty" >&2
  exit 1
fi
if [ -z "${DEPLOY_PORT}" ] || ! [[ "${DEPLOY_PORT}" =~ ^[0-9]+$ ]]; then
  DEPLOY_PORT="22"
fi
if [ ! -f "${ARTIFACT_FILE}" ]; then
  echo "Artifact not found: ${ARTIFACT_FILE}" >&2
  exit 1
fi
if [ ! -s "${ARTIFACT_FILE}" ]; then
  echo "Artifact is empty: ${ARTIFACT_FILE}" >&2
  exit 1
fi

echo "Deploying artifact=${ARTIFACT_FILE} to ${DEPLOY_USER}@${DEPLOY_HOST}:${DEPLOY_PORT} app_dir=${APP_DIR_REMOTE:-\$HOME/xtrace}"

KEY_FILE="$(mktemp)"
trap 'rm -f "$KEY_FILE"' EXIT
printf "%s\n" "$DEPLOY_SSH_KEY" > "$KEY_FILE"
chmod 600 "$KEY_FILE"
SSH_ARGS=(-p "$DEPLOY_PORT" -i "$KEY_FILE" -o StrictHostKeyChecking=no -o BatchMode=yes -o ConnectTimeout=15 -o ServerAliveInterval=15 -o ServerAliveCountMax=3)
SCP_ARGS=(-P "$DEPLOY_PORT" -i "$KEY_FILE" -o StrictHostKeyChecking=no -o BatchMode=yes -o ConnectTimeout=15)
if [ "${DEPLOY_SSH_VERBOSE}" != "0" ]; then
  SSH_ARGS=(-vv "${SSH_ARGS[@]}")
  SCP_ARGS=(-vv "${SCP_ARGS[@]}")
fi

set +e
scp_out="$(scp "${SCP_ARGS[@]}" "$ARTIFACT_FILE" "$DEPLOY_USER@$DEPLOY_HOST:/tmp/xtrace.tar.gz" 2>&1)"
scp_status="$?"
set -e
if [ "$scp_status" -ne 0 ]; then
  echo "scp failed (exit=$scp_status) output:" >&2
  echo "$scp_out" >&2
  exit "$scp_status"
fi

set +e
ssh_out="$(ssh "${SSH_ARGS[@]}" "$DEPLOY_USER@$DEPLOY_HOST" bash -s -- "$APP_DIR_REMOTE" "$DEPLOY_SSH_VERBOSE" 2>&1 <<'REMOTE'
set -Eeuo pipefail
on_err_remote() {
  local code="$?"
  local line="${BASH_LINENO[0]:-${LINENO}}"
  local cmd="${BASH_COMMAND:-}"
  cmd="${cmd//$'\n'/ ; }"
  if [ "${#cmd}" -gt 240 ]; then
    cmd="${cmd:0:240}..."
  fi
  echo "deploy remote failed: exit=${code} line=${line} cmd=${cmd}" >&2
  exit "$code"
}
trap on_err_remote ERR

APP_DIR_REMOTE_ARG="${1:-}"
DEPLOY_SSH_VERBOSE_REMOTE="${2:-0}"
APP_DIR="${APP_DIR_REMOTE_ARG:-$HOME/xtrace}"
if [ "${DEPLOY_SSH_VERBOSE_REMOTE}" != "0" ]; then
  set -x
fi
mkdir -p "$APP_DIR"
tar -xzf /tmp/xtrace.tar.gz -C "$APP_DIR"
chmod +x "$APP_DIR/xtrace"
if ! command -v pm2 >/dev/null 2>&1; then
  NVM_DIR="$HOME/.nvm"
  if [ -s "$NVM_DIR/nvm.sh" ]; then . "$NVM_DIR/nvm.sh"; fi
  if command -v nvm >/dev/null 2>&1; then nvm use --lts >/dev/null 2>&1 || nvm use default >/dev/null 2>&1; fi
fi
if ! command -v pm2 >/dev/null 2>&1; then
  if command -v npm >/dev/null 2>&1; then
    npm i -g pm2
  fi
fi
if ! command -v pm2 >/dev/null 2>&1; then
  echo "pm2 not found in PATH" >&2
  exit 1
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
export BIND_ADDR="${BIND_ADDR:-0.0.0.0:8742}"
pid="$(pm2 pid xtrace || true)"
if [ "${pid:-0}" = "0" ] || [ -z "${pid:-}" ]; then
  pm2 start "$SCRIPT_PATH" --name xtrace --interpreter none --cwd "$APP_DIR" --update-env
else
  pm2 restart xtrace --update-env
fi
pm2 save

pid="$(pm2 pid xtrace || true)"
if [ "${pid:-0}" = "0" ] || [ -z "${pid:-}" ]; then
  echo "pm2 started but xtrace has no pid; dumping last logs" >&2
  pm2 logs xtrace --lines 120 >&2 || true
  exit 1
fi

if command -v curl >/dev/null 2>&1; then
  for _ in 1 2 3 4 5; do
    if curl -fsS "http://127.0.0.1:8742/healthz" >/dev/null; then
      exit 0
    fi
    sleep 1
  done
  echo "healthz check failed on 127.0.0.1:8742; dumping last logs" >&2
  pm2 logs xtrace --lines 120 >&2 || true
  exit 1
fi
REMOTE
)"; ssh_status="$?"
set -e
if [ "$ssh_status" -ne 0 ]; then
  echo "ssh failed (exit=$ssh_status) output:" >&2
  echo "$ssh_out" >&2
  exit "$ssh_status"
fi

if [ "${DEPLOY_SSH_VERBOSE}" != "0" ] && [ -n "${ssh_out}" ]; then
  echo "$ssh_out"
fi
