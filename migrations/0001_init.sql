CREATE TABLE IF NOT EXISTS traces (
  id UUID PRIMARY KEY,
  project_id TEXT NOT NULL,
  "timestamp" TIMESTAMPTZ NOT NULL,

  name TEXT NULL,
  input JSONB NULL,
  output JSONB NULL,
  session_id TEXT NULL,
  release TEXT NULL,
  version TEXT NULL,
  user_id TEXT NULL,
  metadata JSONB NULL,
  tags TEXT[] NOT NULL DEFAULT '{}',
  public BOOLEAN NOT NULL DEFAULT FALSE,
  external_id TEXT NULL,
  bookmarked BOOLEAN NOT NULL DEFAULT FALSE,
  latency DOUBLE PRECISION NULL,
  total_cost DOUBLE PRECISION NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_traces_project_timestamp ON traces (project_id, "timestamp" DESC);
CREATE INDEX IF NOT EXISTS idx_traces_user_id ON traces (user_id);
CREATE INDEX IF NOT EXISTS idx_traces_session_id ON traces (session_id);
CREATE INDEX IF NOT EXISTS idx_traces_tags_gin ON traces USING GIN (tags);

CREATE TABLE IF NOT EXISTS observations (
  id UUID PRIMARY KEY,
  trace_id UUID NOT NULL REFERENCES traces(id) ON DELETE CASCADE,

  type TEXT NOT NULL,
  name TEXT NULL,

  start_time TIMESTAMPTZ NULL,
  end_time TIMESTAMPTZ NULL,
  completion_start_time TIMESTAMPTZ NULL,

  model TEXT NULL,
  model_parameters JSONB NULL,

  input JSONB NULL,
  output JSONB NULL,

  usage JSONB NULL,

  level TEXT NULL,
  status_message TEXT NULL,
  parent_observation_id UUID NULL,

  prompt_id TEXT NULL,
  prompt_name TEXT NULL,
  prompt_version TEXT NULL,

  model_id TEXT NULL,

  input_price DOUBLE PRECISION NULL,
  output_price DOUBLE PRECISION NULL,
  total_price DOUBLE PRECISION NULL,

  calculated_input_cost DOUBLE PRECISION NULL,
  calculated_output_cost DOUBLE PRECISION NULL,
  calculated_total_cost DOUBLE PRECISION NULL,

  latency DOUBLE PRECISION NULL,
  time_to_first_token DOUBLE PRECISION NULL,

  completion_tokens BIGINT NULL,
  prompt_tokens BIGINT NULL,
  total_tokens BIGINT NULL,
  unit TEXT NULL,

  metadata JSONB NULL,
  project_id TEXT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_observations_trace_id_start_time ON observations (trace_id, start_time);
CREATE INDEX IF NOT EXISTS idx_observations_project_id ON observations (project_id);
