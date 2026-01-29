ALTER TABLE traces
ADD COLUMN IF NOT EXISTS environment TEXT NOT NULL DEFAULT 'default';

ALTER TABLE observations
ADD COLUMN IF NOT EXISTS environment TEXT NOT NULL DEFAULT 'default';

CREATE INDEX IF NOT EXISTS idx_traces_environment ON traces (environment);
CREATE INDEX IF NOT EXISTS idx_observations_environment ON observations (environment);
