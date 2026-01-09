-- Enforce exactly one owner per task in assignments table
-- Add unique partial index for owner role

-- First, ensure we don't have multiple owners per task
-- This will fail if there are duplicates, which is expected
DO $$
BEGIN
    IF EXISTS (
        SELECT task_id
        FROM assignments
        WHERE role = 'owner'
        GROUP BY task_id
        HAVING COUNT(*) > 1
    ) THEN
        RAISE EXCEPTION 'Cannot migrate: multiple owners found for some tasks. Please fix data first.';
    END IF;
END $$;

-- Create unique partial index to enforce exactly one owner per task
CREATE UNIQUE INDEX IF NOT EXISTS idx_assignments_unique_owner_per_task
    ON assignments(task_id)
    WHERE role = 'owner';

COMMENT ON INDEX idx_assignments_unique_owner_per_task IS 'Ensures exactly one owner per task';

