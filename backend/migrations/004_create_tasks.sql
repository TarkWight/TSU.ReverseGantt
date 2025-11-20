CREATE TABLE IF NOT EXISTS tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL,
    parent_task_id UUID,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    task_type VARCHAR(20) NOT NULL CHECK (task_type IN ('Final', 'Milestone', 'Task')),
    status VARCHAR(20) NOT NULL CHECK (status IN ('Planned', 'InProgress', 'NeedsReview', 'Accepted', 'Rejected', 'Blocked', 'Done')),
    priority VARCHAR(20) NOT NULL CHECK (priority IN ('Low', 'Normal', 'High', 'Critical')),
    -- Planning fields
    estimated_duration BIGINT,
    planned_start TIMESTAMPTZ,
    planned_finish TIMESTAMPTZ,
    -- Actual execution fields
    actual_start TIMESTAMPTZ,
    actual_finish TIMESTAMPTZ,
    -- Progress tracking
    progress INT DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
    -- Buffer and hardness
    buffer BIGINT DEFAULT 0,
    hardness VARCHAR(10) NOT NULL DEFAULT 'Soft' CHECK (hardness IN ('Hard', 'Soft')),
    -- Deadline
    deadline TIMESTAMPTZ,
    -- Schedule fields (computed by reverse scheduling)
    schedule_ls TIMESTAMPTZ,
    schedule_lf TIMESTAMPTZ,
    schedule_slack BIGINT,
    schedule_is_critical BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE tasks
    DROP CONSTRAINT IF EXISTS tasks_project_id_fkey;

ALTER TABLE tasks
    ADD CONSTRAINT tasks_project_id_fkey
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;


ALTER TABLE tasks
    DROP CONSTRAINT IF EXISTS tasks_parent_task_id_fkey;

ALTER TABLE tasks
    ADD CONSTRAINT tasks_parent_task_id_fkey
        FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE SET NULL;

DROP TRIGGER IF EXISTS update_tasks_updated_at ON tasks;
CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE tasks IS 'Tasks table - individual work items with scheduling information';
COMMENT ON COLUMN tasks.planned_start IS 'Planned start time (computed by scheduler)';
COMMENT ON COLUMN tasks.planned_finish IS 'Planned finish time (computed by scheduler)';
COMMENT ON COLUMN tasks.actual_start IS 'Actual start time (when student started)';
COMMENT ON COLUMN tasks.actual_finish IS 'Actual finish time (when student finished)';
COMMENT ON COLUMN tasks.progress IS 'Task progress percentage (0-100)';
COMMENT ON COLUMN tasks.buffer IS 'Time buffer in seconds';
COMMENT ON COLUMN tasks.hardness IS 'Task hardness: Hard (strict) or Soft (flexible)';
COMMENT ON COLUMN tasks.schedule_ls IS 'Latest Start time (computed by reverse scheduling)';
COMMENT ON COLUMN tasks.schedule_lf IS 'Latest Finish time (computed by reverse scheduling)';
COMMENT ON COLUMN tasks.schedule_slack IS 'Slack time - difference between latest and earliest finish';
COMMENT ON COLUMN tasks.schedule_is_critical IS 'Whether this task is on the critical path';
