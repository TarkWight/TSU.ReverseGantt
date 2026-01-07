-- Enforce exactly one leader per project using unique partial index
-- This ensures database-level constraint that exactly one membership per project has is_leader = true

-- First, check if there are any projects with multiple leaders (should not happen, but safety check)
DO $$
DECLARE
    project_with_multiple_leaders UUID;
BEGIN
    SELECT project_id INTO project_with_multiple_leaders
    FROM memberships
    WHERE is_leader = true
    GROUP BY project_id
    HAVING COUNT(*) > 1
    LIMIT 1;
    
    IF project_with_multiple_leaders IS NOT NULL THEN
        RAISE EXCEPTION 'Cannot migrate: project % has multiple leaders. Please fix data first.', project_with_multiple_leaders;
    END IF;
END $$;

-- Create unique partial index to enforce exactly one leader per project
CREATE UNIQUE INDEX IF NOT EXISTS idx_memberships_unique_leader_per_project
    ON memberships(project_id)
    WHERE is_leader = true;

COMMENT ON INDEX idx_memberships_unique_leader_per_project IS 'Ensures exactly one leader per project at database level';


