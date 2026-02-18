# Deployment

## Building for Production

```bash
cargo build --release
# Binary at target/release/xtrace
```

The binary is self-contained — it embeds database migrations and runs them on startup.

## Systemd Service

```ini
[Unit]
Description=xtrace AI Observability Service
After=postgresql.service

[Service]
Type=simple
Environment=DATABASE_URL=postgresql://user:pass@localhost:5432/xtrace
Environment=API_BEARER_TOKEN=your-secret-token
Environment=BIND_ADDR=0.0.0.0:8742
ExecStart=/usr/local/bin/xtrace
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

## Health Check

```bash
curl http://127.0.0.1:8742/healthz
```

Use this endpoint for load balancer health checks. It does not require authentication.

## Reverse Proxy (Nginx)

```nginx
upstream xtrace {
    server 127.0.0.1:8742;
}

server {
    listen 443 ssl;
    server_name xtrace.example.com;

    location / {
        proxy_pass http://xtrace;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## PostgreSQL Recommendations

- Use PostgreSQL 14+ for best `JSONB` and `GIN` index performance
- Minimum 2 GB RAM for typical workloads
- Enable `pg_stat_statements` for query performance monitoring
- Connection pool: xtrace uses up to 20 connections by default

## Data Retention

::: warning
Data retention is not yet automated. Plan manual cleanup or cron jobs:
:::

```sql
-- Delete metrics older than 30 days
DELETE FROM metrics WHERE timestamp < NOW() - INTERVAL '30 days';

-- Delete traces older than 90 days
DELETE FROM observations WHERE created_at < NOW() - INTERVAL '90 days';
DELETE FROM traces WHERE created_at < NOW() - INTERVAL '90 days';
```
