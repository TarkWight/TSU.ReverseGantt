-- Remove role column from memberships (roles are now global, not per-project)
-- Add tags column for arbitrary text tags (metadata only, no permissions)

-- First, add tags column
ALTER TABLE memberships
    ADD COLUMN IF NOT EXISTS tags TEXT[] DEFAULT ARRAY[]::TEXT[];

-- Remove the role column (after ensuring we have tags)
ALTER TABLE memberships
    DROP COLUMN IF EXISTS role;

-- Update indexes
DROP INDEX IF EXISTS idx_memberships_role;

-- Add index for tags if needed (GIN index for array searches)
CREATE INDEX IF NOT EXISTS idx_memberships_tags ON memberships USING GIN(tags);

COMMENT ON COLUMN memberships.tags IS 'Arbitrary text tags for metadata (e.g., "backend", "frontend"). Tags do not grant permissions.';
COMMENT ON COLUMN memberships.is_leader IS 'Whether the user is the project leader (exactly one per project)';

