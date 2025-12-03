ALTER TABLE users
    ADD COLUMN password_hash TEXT DEFAULT '';

UPDATE users
SET password_hash = ''
WHERE password_hash IS NULL;

ALTER TABLE users
    ALTER COLUMN password_hash SET NOT NULL;

ALTER TABLE users
    ALTER COLUMN password_hash DROP DEFAULT;