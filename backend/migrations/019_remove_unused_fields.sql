-- Remove unused fields from users table
ALTER TABLE users DROP COLUMN IF EXISTS created_at;

-- Remove unused fields from tasks table
ALTER TABLE tasks DROP COLUMN IF EXISTS planned_start;
ALTER TABLE tasks DROP COLUMN IF EXISTS planned_finish;
ALTER TABLE tasks DROP COLUMN IF EXISTS actual_start;
ALTER TABLE tasks DROP COLUMN IF EXISTS actual_finish;

-- Remove deadline column from tasks table
-- The only deadline should be Project.due_date
ALTER TABLE tasks DROP COLUMN IF EXISTS deadline;

-- Remove indexes on deleted columns
DROP INDEX IF EXISTS idx_tasks_planned_start;
DROP INDEX IF EXISTS idx_tasks_actual_start;
DROP INDEX IF EXISTS idx_tasks_deadline;

