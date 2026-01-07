-- Add email notifications setting to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_notifications_enabled BOOLEAN NOT NULL DEFAULT true;

COMMENT ON COLUMN users.email_notifications_enabled IS 'Whether the user wants to receive email notifications about task updates';

