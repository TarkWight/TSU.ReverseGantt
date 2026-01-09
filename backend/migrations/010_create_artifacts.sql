CREATE TABLE IF NOT EXISTS artifacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    uri TEXT NOT NULL,
    kind VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE artifacts
    DROP IF EXISTS artifacts_task_id_fkey;

ALTER TABLE artifacts
    ADD CONSTRAINT artifacts_task_id_fkey
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE;

DROP TRIGGER IF EXISTS update_artifacts_updated_at ON artifacts;
CREATE TRIGGER update_artifacts_updated_at
    BEFORE UPDATE ON artifacts
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE artifacts IS 'Task artifacts - files and documents associated with tasks';
COMMENT ON COLUMN artifacts.uri IS 'Artifact URI (can be file path, URL, storage key, etc.)';
COMMENT ON COLUMN artifacts.kind IS 'Artifact type (document, code, test-report, demo, etc.)';
