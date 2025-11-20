CREATE TABLE IF NOT EXISTS assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('owner', 'assignee'))
);

ALTER TABLE assignments
    DROP CONSTRAINT IF EXISTS assignments_task_id_fkey;

ALTER TABLE assignments
    ADD CONSTRAINT assignments_task_id_fkey
        FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE;

ALTER TABLE assignments
    DROP CONSTRAINT IF EXISTS assignments_user_id_fkey;

ALTER TABLE assignments
    ADD CONSTRAINT assignments_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE assignments
    DROP CONSTRAINT IF EXISTS unique_task_user_role;

ALTER TABLE assignments
    ADD CONSTRAINT unique_task_user_role
        UNIQUE (task_id, user_id, role);

COMMENT ON TABLE assignments IS 'Task assignments - links users to tasks with roles';
COMMENT ON COLUMN assignments.role IS 'Assignment role: owner (exactly one per task) or assignee (0..*)';