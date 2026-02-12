CREATE TABLE IF NOT EXISTS metrics (
  id BIGSERIAL PRIMARY KEY,
  project_id TEXT NOT NULL,
  environment TEXT NOT NULL DEFAULT 'default',
  name TEXT NOT NULL,
  labels JSONB NOT NULL DEFAULT '{}',
  value DOUBLE PRECISION NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_metrics_name_ts ON metrics (project_id, name, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_labels_gin ON metrics USING GIN (labels);
CREATE INDEX IF NOT EXISTS idx_metrics_ts ON metrics (timestamp DESC);
