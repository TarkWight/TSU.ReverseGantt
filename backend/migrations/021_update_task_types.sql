-- Update task_type constraint: remove Final and Milestone, add Feature
-- First, update existing Final and Milestone tasks to Task
UPDATE tasks SET task_type = 'Task' WHERE task_type IN ('Final', 'Milestone');

-- Drop old constraint
ALTER TABLE tasks DROP CONSTRAINT IF EXISTS tasks_task_type_check;

-- Add new constraint with Task and Feature only
ALTER TABLE tasks ADD CONSTRAINT tasks_task_type_check 
    CHECK (task_type IN ('Task', 'Feature'));


