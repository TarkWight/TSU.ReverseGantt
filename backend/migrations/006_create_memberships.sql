CREATE TABLE IF NOT EXISTS memberships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('student', 'teacher')),
    is_leader BOOLEAN NOT NULL DEFAULT FALSE
);

ALTER TABLE memberships
    DROP CONSTRAINT IF EXISTS memberships_project_id_fkey;

ALTER TABLE memberships
    ADD CONSTRAINT memberships_project_id_fkey
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE;

ALTER TABLE memberships
    DROP CONSTRAINT IF EXISTS memberships_user_id_fkey;

ALTER TABLE memberships
    ADD CONSTRAINT memberships_user_id_fkey
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE memberships
    DROP CONSTRAINT IF EXISTS unique_project_user;

ALTER TABLE memberships
    ADD CONSTRAINT unique_project_user
        UNIQUE (project_id, user_id);

COMMENT ON TABLE memberships IS 'Project memberships - links users to projects with roles';
COMMENT ON COLUMN memberships.is_leader IS 'Whether the user is a project leader';