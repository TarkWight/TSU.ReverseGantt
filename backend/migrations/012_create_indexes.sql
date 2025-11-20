-- Tasks indexes
CREATE INDEX IF NOT EXISTS idx_tasks_project_id       ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_parent_task_id   ON tasks(parent_task_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status           ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_priority         ON tasks(priority);
CREATE INDEX IF NOT EXISTS idx_tasks_deadline         ON tasks(deadline);
CREATE INDEX IF NOT EXISTS idx_tasks_planned_start    ON tasks(planned_start);
CREATE INDEX IF NOT EXISTS idx_tasks_actual_start     ON tasks(actual_start);

-- Dependencies indexes
CREATE INDEX IF NOT EXISTS idx_dependencies_from_task_id ON dependencies(from_task_id);
CREATE INDEX IF NOT EXISTS idx_dependencies_to_task_id   ON dependencies(to_task_id);
CREATE INDEX IF NOT EXISTS idx_dependencies_type         ON dependencies(dep_type);

-- Memberships indexes
CREATE INDEX IF NOT EXISTS idx_memberships_project_id ON memberships(project_id);
CREATE INDEX IF NOT EXISTS idx_memberships_user_id    ON memberships(user_id);
CREATE INDEX IF NOT EXISTS idx_memberships_role       ON memberships(role);

-- Assignments indexes
CREATE INDEX IF NOT EXISTS idx_assignments_task_id ON assignments(task_id);
CREATE INDEX IF NOT EXISTS idx_assignments_user_id ON assignments(user_id);

-- Reviews indexes
CREATE INDEX IF NOT EXISTS idx_reviews_task_id      ON reviews(task_id);
CREATE INDEX IF NOT EXISTS idx_reviews_reviewer_id  ON reviews(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_reviews_decision     ON reviews(decision);

-- Artifacts indexes
CREATE INDEX IF NOT EXISTS idx_artifacts_task_id ON artifacts(task_id);
CREATE INDEX IF NOT EXISTS idx_artifacts_kind    ON artifacts(kind);

-- Risks indexes
CREATE INDEX IF NOT EXISTS idx_risks_task_id ON risks(task_id);
CREATE INDEX IF NOT EXISTS idx_risks_score   ON risks(score);
