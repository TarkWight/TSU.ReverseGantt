CREATE TABLE IF NOT EXISTS risks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    score INT NOT NULL CHECK (score >= 0),
    reason TEXT NOT NULL,
    mitigation TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE risks
    DROP IF EXISTS risks_task_id_fkey;

ALTER TABLE risks
    ADD CONSTRAINT risks_task_id_fkey
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE;

DROP TRIGGER IF EXISTS update_risks_updated_at ON risks;
CREATE TRIGGER update_risks_updated_at
    BEFORE UPDATE ON risks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE risks IS 'Task risks - risk logging and mitigation tracking';
COMMENT ON COLUMN risks.score IS 'Numeric risk value (for sorting and aggregation)';
COMMENT ON COLUMN risks.reason IS 'Reason/description of the risk';
