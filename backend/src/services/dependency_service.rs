use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query;
use std::collections::{HashMap, HashSet};

use crate::domain::{Dependency, DepType};
use crate::utils::{AppError, AppResult, Id};

#[async_trait]
pub trait DependencyService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>>;
    async fn create(&self, dependency: Dependency) -> AppResult<Dependency>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}

pub struct DependencyServiceImpl {
    pool: PgPool,
}

impl DependencyServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn get_project_id_for_task(&self, task_id: Id) -> AppResult<Id> {
        let row = query!(
            "SELECT project_id FROM tasks WHERE id = $1",
            task_id
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", task_id)))?;

        Ok(row.project_id)
    }

    async fn check_for_cycles(&self, project_id: Id, new_dep: &Dependency) -> AppResult<()> {
        let tasks = query!(
            "SELECT id FROM tasks WHERE project_id = $1",
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let task_ids: Vec<Id> = tasks.iter().map(|t| t.id).collect();

        if task_ids.is_empty() {
            return Ok(());
        }

        let deps = query!(
            r#"
            SELECT from_task_id, to_task_id
            FROM dependencies
            WHERE from_task_id = ANY($1) AND to_task_id = ANY($1)
            "#,
            &task_ids[..]
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let mut graph: HashMap<Id, Vec<Id>> = HashMap::new();
        for dep in deps {
            graph
                .entry(dep.from_task_id)
                .or_insert_with(Vec::new)
                .push(dep.to_task_id);
        }

        graph
            .entry(new_dep.from_task_id)
            .or_insert_with(Vec::new)
            .push(new_dep.to_task_id);

        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for &task_id in &task_ids {
            if !visited.contains(&task_id) {
                if self.has_cycle(&graph, task_id, &mut visited, &mut rec_stack) {
                    return Err(AppError::Validation(
                        "Adding this dependency would create a cycle in the task graph".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn has_cycle(
        &self,
        graph: &HashMap<Id, Vec<Id>>,
        node: Id,
        visited: &mut HashSet<Id>,
        rec_stack: &mut HashSet<Id>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if self.has_cycle(graph, neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&neighbor) {
                    return true;
                }
            }
        }

        rec_stack.remove(&node);
        false
    }
}

#[async_trait]
impl DependencyService for DependencyServiceImpl {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>> {
        let rows = query!(
            r#"
            SELECT
                id,
                from_task_id,
                to_task_id,
                dep_type,
                min_gap
            FROM dependencies
            WHERE from_task_id = $1 OR to_task_id = $1
            "#,
            task_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let dependencies = rows
            .into_iter()
            .map(|row| Dependency {
                id: row.id,
                from_task_id: row.from_task_id,
                to_task_id: row.to_task_id,
                dep_type: row
                    .dep_type
                    .parse()
                    .unwrap_or(DepType::FS),
                min_gap: row.min_gap,
            })
            .collect();

        Ok(dependencies)
    }

    async fn create(&self, _dependency: Dependency) -> AppResult<Dependency> {
        todo!("create dependency not implemented yet");
    }

    async fn delete(&self, _id: Id) -> AppResult<()> {
        todo!("delete dependency not implemented yet");
    }
}

