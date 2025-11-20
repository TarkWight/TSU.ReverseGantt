CREATE TABLE IF NOT EXISTS dependencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    from_task_id UUID NOT NULL,
    to_task_id UUID NOT NULL,
    dep_type VARCHAR(5) NOT NULL CHECK (dep_type IN ('FS', 'FF', 'SS', 'SF')),
    min_gap BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT no_self_dependency CHECK (from_task_id != to_task_id),
    CONSTRAINT unique_dependency UNIQUE (from_task_id, to_task_id, dep_type, min_gap)
);

ALTER TABLE dependencies
    DROP CONSTRAINT IF EXISTS dependencies_from_task_id_fkey;

ALTER TABLE dependencies
    ADD CONSTRAINT dependencies_from_task_id_fkey
    FOREIGN KEY (from_task_id) REFERENCES tasks(id) ON DELETE CASCADE;

ALTER TABLE dependencies
    DROP CONSTRAINT IF EXISTS dependencies_to_task_id_fkey;

ALTER TABLE dependencies
    ADD CONSTRAINT dependencies_to_task_id_fkey
    FOREIGN KEY (to_task_id) REFERENCES tasks(id) ON DELETE CASCADE;

COMMENT ON TABLE dependencies IS 'Task dependencies - defines relationships between tasks';
COMMENT ON COLUMN dependencies.dep_type IS 'Dependency type: FS=Finish-to-Start, FF=Finish-to-Finish, SS=Start-to-Start, SF=Start-to-Finish';
COMMENT ON COLUMN dependencies.min_gap IS 'Minimum time gap between dependent tasks in seconds';
