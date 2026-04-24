# 在 GCP 上部署 xtrace（供 Xinference 联调）

本文说明如何在本机用 `gcloud` 创建一台 VM、安装依赖并运行 xtrace，域名示例为 **`xtrace.sh`**（需你在 DNS 服务商处把该域名 **A 记录** 指到 VM 公网 IP）。

**注意**：仓库无法替你执行 GCP 操作，需在你已登录的 `gcloud` 环境里自行运行下列命令。

## 1. 前置条件

- 已安装 [Google Cloud SDK](https://cloud.google.com/sdk)，并 `gcloud auth login`、`gcloud config set project YOUR_PROJECT_ID`
- 已启用 **Compute Engine API**
- 数据库：生产建议 **Cloud SQL for PostgreSQL**；最小联调也可在 **同一台 VM 上装 PostgreSQL**（下文含简易步骤）

## 2. 创建 VM（示例：香港 `asia-east2`，机型 e2-small）

```bash
export GCP_ZONE=asia-east2-a
export VM_NAME=xtrace-vm
export MACHINE_TYPE=e2-small

gcloud compute instances create "${VM_NAME}" \
  --zone="${GCP_ZONE}" \
  --machine-type="${MACHINE_TYPE}" \
  --image-family=ubuntu-2204-lts \
  --image-project=ubuntu-os-cloud \
  --boot-disk-size=20GB \
  --tags=http-server,https-server,xtrace \
  --metadata=enable-oslogin=TRUE
```

## 3. 防火墙（HTTP/HTTPS/SSH）

若使用默认 VPC，通常需允许 80/443：

```bash
gcloud compute firewall-rules create allow-http-https \
  --allow=tcp:80,tcp:443 \
  --target-tags=http-server,https-server \
  --description="HTTP HTTPS" || true
```

SSH 默认已有；仅内网访问 xtrace 端口时勿对公网开放 `8742`，应通过 **反向代理 + TLS** 对外。

## 4. 查看公网 IP（做 DNS A 记录）

```bash
gcloud compute instances describe "${VM_NAME}" --zone="${GCP_ZONE}" \
  --format='get(networkInterfaces[0].accessConfigs[0].natIP)'
```

将 `xtrace.sh`（或子域如 `api.xtrace.sh`）的 **A 记录** 指向上述 IP。

## 5. VM 内安装（SSH 登录后）

```bash
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib curl

# PostgreSQL：创建库与用户（示例）
sudo -u postgres psql -c "CREATE USER xtrace WITH PASSWORD 'CHANGE_ME_STRONG';"
sudo -u postgres psql -c "CREATE DATABASE xtrace OWNER xtrace;"

# 安装二进制：从本机 cargo build --release 后 scp，或用 GitHub Release 资产
# 假设二进制在 /opt/xtrace/xtrace
sudo mkdir -p /opt/xtrace
```

将 `deploy/systemd/xtrace.service` 复制为 `/etc/systemd/system/xtrace.service`（修改 `Environment=` 与 `ExecStart`），然后：

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now xtrace
```

环境变量至少包括：

| 变量 | 说明 |
|------|------|
| `DATABASE_URL` | `postgresql://xtrace:密码@127.0.0.1:5432/xtrace` |
| `API_BEARER_TOKEN` | 随机长串 |
| `XTRACE_PUBLIC_KEY` / `XTRACE_SECRET_KEY` | 与 Xinference 中 `LANGFUSE_PUBLIC_KEY` / `LANGFUSE_SECRET_KEY` **一致** |
| `BIND_ADDR` | `127.0.0.1:8742`（仅本机，由反代对外） |

## 6. TLS 与反代（推荐 Caddy 自动证书）

```bash
sudo apt-get install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt-get update && sudo apt-get install -y caddy
```

`/etc/caddy/Caddyfile` 示例（将域名换成你的）：

```caddy
xtrace.sh {
    reverse_proxy 127.0.0.1:8742
}
```

```bash
sudo systemctl reload caddy
```

Xinference 侧配置：`LANGFUSE_HOST=https://xtrace.sh`（或你的子域），**必须使用 https**  unless 内网 HTTP。

## 7. 与仓库 `scripts/deploy.sh` 的关系

若你已在本地打好 `xtrace` 发布包（`xtrace.tar.gz`），可设置 `DEPLOY_HOST` 为 VM IP、`DEPLOY_SSH_KEY` 为私钥路径，用现有 `scripts/deploy.sh` 上传并解压到目标目录（需与 systemd 中路径一致）。

## 8. 联调前自检

在 VM 或跳板机：

```bash
curl -fsS https://xtrace.sh/readyz
```

本机有 Python 时，用 **`scripts/full_integration_test.py`** / **`xinference_chain_smoke_test.py`** 把 `XTRACE_BASE_URL` 设为 `https://xtrace.sh`（注意证书与 Bearer/Basic 与线上一致）。
