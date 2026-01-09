-- Add global_role column to users table
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS global_role TEXT NOT NULL DEFAULT 'student';

-- Add check constraint to ensure only valid roles
ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_global_role_check;

ALTER TABLE users
    ADD CONSTRAINT users_global_role_check
        CHECK (global_role IN ('student', 'teacher'));

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_global_role ON users(global_role);

COMMENT ON COLUMN users.global_role IS 'Global user role: student or teacher (system-wide, not per-project)';

