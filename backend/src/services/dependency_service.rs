use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::domain::Dependency;
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::DependencyRepository;
use crate::utils::Id;

#[async_trait]
pub trait DependencyService: Send + Sync {
    async fn get_by_task(&self, task_id: Id) -> AppResult<Vec<Dependency>>;
    async fn get_by_id(&self, id: Id) -> AppResult<Dependency>;
    async fn create(&self, dependency: Dependency) -> AppResult<Dependency>;
    async fn delete(&self, id: Id) -> AppResult<()>;
}

pub struct DependencyServiceImpl {
    dependency_repo: Arc<dyn DependencyRepository>,
}

impl DependencyServiceImpl {
    pub fn new(dependency_repo: Arc<dyn DependencyRepository>) -> Self {
        Self { dependency_repo }
    }

    async fn check_for_cycles(&self, project_id: Id, new_dep: &Dependency) -> AppResult<()> {
        let task_ids = self.dependency_repo
            .get_project_task_ids(project_id)
            .await
            .map_err(AppError::Internal)?;

        if task_ids.is_empty() {
            return Ok(());
        }

        let deps = self.dependency_repo
            .find_by_project(project_id)
            .await
            .map_err(AppError::Internal)?;

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
                if Self::has_cycle(&graph, task_id, &mut visited, &mut rec_stack) {
                    return Err(AppError::Validation(
                        "Adding this dependency would create a cycle in the task graph".into(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn has_cycle(
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
                    if Self::has_cycle(graph, neighbor, visited, rec_stack) {
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
        self.dependency_repo
            .find_by_task(task_id)
            .await
            .map_err(AppError::Internal)
    }

    async fn get_by_id(&self, id: Id) -> AppResult<Dependency> {
        self.dependency_repo
            .find_by_id(id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Dependency {} not found", id)))
    }

    async fn create(&self, dependency: Dependency) -> AppResult<Dependency> {
        let project_id = self.dependency_repo
            .get_task_project_id(dependency.from_task_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Task {} not found", dependency.from_task_id)))?;

        self.check_for_cycles(project_id, &dependency).await?;

        self.dependency_repo
            .insert(&dependency)
            .await
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("unique_dependency") {
                    AppError::Validation("A dependency with the same parameters already exists".into())
                } else {
                    AppError::Internal(e)
                }
            })?;

        Ok(dependency)
    }

    async fn delete(&self, id: Id) -> AppResult<()> {
        let deleted = self.dependency_repo
            .delete(id)
            .await
            .map_err(AppError::Internal)?;

        if !deleted {
            return Err(AppError::NotFound(format!("Dependency {} not found", id)));
        }

        Ok(())
    }
}
