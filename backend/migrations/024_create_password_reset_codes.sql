-- Table for storing password reset codes (email-based and teacher-approved requests)
CREATE TABLE IF NOT EXISTS password_reset_codes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code_hash TEXT, -- NULL for teacher-approved requests (no email code needed)
    request_type TEXT NOT NULL CHECK (request_type IN ('email', 'teacher_request')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'used', 'expired', 'approved', 'rejected')),
    expires_at TIMESTAMPTZ, -- NULL for teacher_request (doesn't expire until reviewed)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL, -- teacher who approved/rejected
    reviewed_at TIMESTAMPTZ,
    notes TEXT -- optional notes from teacher
);

CREATE INDEX IF NOT EXISTS idx_password_reset_codes_user_id ON password_reset_codes(user_id);
CREATE INDEX IF NOT EXISTS idx_password_reset_codes_status ON password_reset_codes(status);
CREATE INDEX IF NOT EXISTS idx_password_reset_codes_expires_at ON password_reset_codes(expires_at);

COMMENT ON TABLE password_reset_codes IS 'Password reset codes for email-based reset and teacher-approved reset requests';
COMMENT ON COLUMN password_reset_codes.code_hash IS 'Hashed verification code (6-digit) for email-based reset, NULL for teacher requests';
COMMENT ON COLUMN password_reset_codes.request_type IS 'Type of reset: email (with code) or teacher_request (manual approval)';
COMMENT ON COLUMN password_reset_codes.status IS 'Status: pending (waiting), used (code verified/password changed), expired (TTL passed), approved (teacher approved), rejected (teacher rejected)';


