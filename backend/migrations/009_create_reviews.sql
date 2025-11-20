CREATE TABLE IF NOT EXISTS reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    reviewer_id UUID NOT NULL,
    decision VARCHAR(20) CHECK (decision IN ('Accepted', 'Rejected')),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE reviews
    DROP CONSTRAINT IF EXISTS reviews_task_id_fkey;

ALTER TABLE reviews
    ADD CONSTRAINT reviews_task_id_fkey
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE;

ALTER TABLE reviews
    DROP CONSTRAINT IF EXISTS reviews_reviewer_id_fkey;

ALTER TABLE reviews
    ADD CONSTRAINT reviews_reviewer_id_fkey
    FOREIGN KEY (reviewer_id) REFERENCES users(id) ON DELETE CASCADE;

DROP TRIGGER IF EXISTS update_reviews_updated_at ON reviews;
CREATE TRIGGER update_reviews_updated_at
    BEFORE UPDATE ON reviews
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE reviews IS 'Task reviews - review workflow for tasks';
