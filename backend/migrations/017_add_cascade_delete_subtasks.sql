-- Add cascade delete for subtasks
-- When a task is deleted, all its subtasks (recursively) should be deleted

-- First, change the foreign key constraint to CASCADE
ALTER TABLE tasks
    DROP CONSTRAINT IF EXISTS tasks_parent_task_id_fkey;

ALTER TABLE tasks
    ADD CONSTRAINT tasks_parent_task_id_fkey
        FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE CASCADE;

-- Create a function to recursively delete all subtasks
-- This function will be called by a trigger before deleting a task
CREATE OR REPLACE FUNCTION delete_task_subtasks()
RETURNS TRIGGER AS $$
BEGIN
    -- Recursively delete all subtasks
    -- PostgreSQL will handle the cascade automatically with ON DELETE CASCADE
    -- But we need to ensure the order is correct
    WITH RECURSIVE subtask_tree AS (
        -- Start with direct children
        SELECT id FROM tasks WHERE parent_task_id = OLD.id
        UNION ALL
        -- Recursively get all descendants
        SELECT t.id 
        FROM tasks t
        INNER JOIN subtask_tree st ON t.parent_task_id = st.id
    )
    DELETE FROM tasks WHERE id IN (SELECT id FROM subtask_tree);
    
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

-- Create trigger that fires BEFORE DELETE to ensure proper cascade
DROP TRIGGER IF EXISTS trigger_delete_task_subtasks ON tasks;
CREATE TRIGGER trigger_delete_task_subtasks
    BEFORE DELETE ON tasks
    FOR EACH ROW
    EXECUTE FUNCTION delete_task_subtasks();

COMMENT ON FUNCTION delete_task_subtasks IS 'Recursively deletes all subtasks when a parent task is deleted';


