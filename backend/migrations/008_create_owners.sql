CREATE UNIQUE INDEX IF NOT EXISTS unique_owner_per_task
    ON assignments(task_id)
    WHERE role = 'owner';