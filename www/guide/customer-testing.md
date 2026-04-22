# Customer Testing

Use this page when you want to give a customer a hosted xtrace endpoint for a quick validation without asking them to self-host first.

## Hosted Endpoint

- Documentation site: `https://xtrace.sh`
- Hosted API endpoint: `https://api.xtrace.sh`

## 1. Smoke Check The Service

These checks require no credentials:

```bash
curl -sS https://api.xtrace.sh/healthz
curl -sS https://api.xtrace.sh/readyz
```

Expected result:

- `/healthz` returns `200 OK`
- `/readyz` returns `{"status":"ready"}`

## 2. Configure Xinference

Send the trial key pair to the customer over a secure channel, then ask them to point Xinference at the hosted API:

```bash
export LANGFUSE_HOST=https://api.xtrace.sh
export LANGFUSE_PUBLIC_KEY=pk-...
export LANGFUSE_SECRET_KEY=sk-...
```

If the customer configures Langfuse through the Xinference settings UI or API, use the same values there.

## 3. Validate The Read Path

After the customer receives the trial key pair, they can verify the Langfuse-compatible public API directly:

```bash
curl -sS -u "$LANGFUSE_PUBLIC_KEY:$LANGFUSE_SECRET_KEY" \
  https://api.xtrace.sh/api/public/projects

curl -sS -u "$LANGFUSE_PUBLIC_KEY:$LANGFUSE_SECRET_KEY" \
  "https://api.xtrace.sh/api/public/traces?page=1&limit=10"

curl -sS -u "$LANGFUSE_PUBLIC_KEY:$LANGFUSE_SECRET_KEY" \
  https://api.xtrace.sh/api/public/metrics/daily
```

## 4. Recommended Customer Test Script

1. Confirm `https://api.xtrace.sh/readyz` is healthy.
2. Configure `LANGFUSE_HOST=https://api.xtrace.sh` in Xinference.
3. Apply the provided `LANGFUSE_PUBLIC_KEY` and `LANGFUSE_SECRET_KEY`.
4. Trigger one real inference request from the customer environment.
5. Open the Xinference trace list or call `/api/public/traces` directly.
6. Confirm the latest trace and daily metrics are visible.

## 5. What To Send To Customers

Provide the customer with:

- The hosted endpoint: `https://api.xtrace.sh`
- One trial `LANGFUSE_PUBLIC_KEY`
- One matching `LANGFUSE_SECRET_KEY`
- The smoke-check commands from this page
- The direct read-back commands from this page

Do not publish the trial keys in public documentation, screenshots, or tickets.