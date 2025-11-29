use async_trait::async_trait;
use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration, NaiveDate};
use std::collections::{HashMap, VecDeque};
use anyhow::Context;

use crate::domain::{TaskStatus, TaskType, Priority};
use crate::domain::enums::*;
use crate::domain::{Task, Schedule, Dependency, DepType};
use crate::utils::{AppError, AppResult, Id};
use sqlx::query;

#[async_trait]
pub trait ScheduleService: Send + Sync {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>>;
}

pub struct ScheduleServiceImpl {
    pool: PgPool,
}

impl ScheduleServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn get_tasks_with_dependencies(
        &self,
        project_id: Id,
    ) -> AppResult<(Vec<Task>, Vec<Dependency>)> {
        // Get all tasks for the project
        let tasks = query!(
            r#"
            SELECT
                id,
                project_id,
                parent_task_id,
                name,
                description,
                task_type,
                status,
                priority,
                estimated_duration,
                planned_start,
                planned_finish,
                actual_start,
                actual_finish,
                progress,
                buffer,
                hardness,
                deadline,
                schedule_ls,
                schedule_lf,
                schedule_slack,
                schedule_is_critical,
                created_at,
                updated_at
            FROM tasks
            WHERE project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .context(format!(
                "Failed to load tasks for project {} in reverse_schedule",
                project_id
            ))?;

        let task_ids: Vec<Id> = tasks.iter().map(|t| t.id).collect();

        // Get all dependencies for these tasks
        let deps = if !task_ids.is_empty() {
            query!(
                r#"
                SELECT
                    id,
                    from_task_id,
                    to_task_id,
                    dep_type,
                    min_gap
                FROM dependencies
                WHERE from_task_id = ANY($1) OR to_task_id = ANY($1)
                "#,
                &task_ids[..]
            )
                .fetch_all(&self.pool)
                .await
                .context(format!(
                    "Failed to load dependencies for project {} in reverse_schedule",
                    project_id
                ))?
        } else {
            vec![]
        };

        let domain_tasks: Vec<Task> = tasks
            .into_iter()
            .map(|row| {
                Task {
                    id: row.id,
                    project_id: row.project_id,
                    parent_task_id: row.parent_task_id,
                    name: row.name,
                    description: row.description,
                    task_type: row.task_type.parse().unwrap_or(TaskType::Task),
                    status: row.status.parse().unwrap_or(TaskStatus::Planned),
                    priority: row.priority.parse().unwrap_or(Priority::Normal),
                    estimated_duration: row.estimated_duration,
                    planned_start: row.planned_start,
                    planned_finish: row.planned_finish,
                    actual_start: row.actual_start,
                    actual_finish: row.actual_finish,
                    progress: row.progress.unwrap_or(0),
                    buffer: row.buffer.unwrap_or(0),
                    hardness: row.hardness.parse().unwrap_or(Hardness::Soft),
                    deadline: row.deadline,
                    schedule: Schedule {
                        ls: row.schedule_ls,
                        lf: row.schedule_lf,
                        slack: row.schedule_slack,
                        is_critical: row.schedule_is_critical,
                    },
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            })
            .collect();

        let domain_deps: Vec<Dependency> = deps
            .into_iter()
            .map(|row| Dependency {
                id: row.id,
                from_task_id: row.from_task_id,
                to_task_id: row.to_task_id,
                dep_type: row.dep_type.parse().unwrap_or(DepType::FS),
                min_gap: row.min_gap,
            })
            .collect();

        Ok((domain_tasks, domain_deps))
    }

    fn compute_reverse_schedule(
        &self,
        tasks: &mut [Task],
        dependencies: &[Dependency],
        project_due_date: NaiveDate,
    ) -> AppResult<()> {
        if tasks.is_empty() {
            return Ok(());
        }

        // Build dependency graph
        let mut successors: HashMap<Id, Vec<&Dependency>> = HashMap::new();
        let mut predecessors: HashMap<Id, Vec<&Dependency>> = HashMap::new();

        for dep in dependencies {
            successors.entry(dep.from_task_id).or_insert_with(Vec::new).push(dep);
            predecessors.entry(dep.to_task_id).or_insert_with(Vec::new).push(dep);
        }

        // Create task index map for quick lookup
        let mut task_indices: HashMap<Id, usize> = HashMap::new();
        for (idx, task) in tasks.iter().enumerate() {
            task_indices.insert(task.id, idx);
        }

        // Convert project due_date to DateTime<Utc> at end of day
        let deadline = project_due_date
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();

        // Initialize LF (Latest Finish) for all tasks
        // Start with tasks that have no successors (leaf nodes)
        let mut queue = VecDeque::new();
        for task in tasks.iter() {
            if !successors.contains_key(&task.id) {
                queue.push_back(task.id);
            }
        }

        // Set LF for leaf nodes to project deadline
        while let Some(task_id) = queue.pop_front() {
            if let Some(&task_idx) = task_indices.get(&task_id) {
                let task = &mut tasks[task_idx];

                if task.schedule.lf.is_none() {
                    task.schedule.lf = Some(deadline);

                    // Calculate LS (Latest Start) based on estimated duration
                    if let Some(duration) = task.estimated_duration {
                        let duration_delta = Duration::seconds(duration);
                        task.schedule.ls = task.schedule.lf.map(|lf| lf - duration_delta);
                    } else {
                        task.schedule.ls = task.schedule.lf;
                    }
                }

                // Process predecessors - collect updates first to avoid multiple borrows
                let mut updates: Vec<(Id, DateTime<Utc>)> = Vec::new();

                if let Some(preds) = predecessors.get(&task_id) {
                    let task_ls = task.schedule.ls;
                    let task_lf = task.schedule.lf;

                    for dep in preds {
                        if let Some(&pred_idx) = task_indices.get(&dep.from_task_id) {
                            let pred_task = &tasks[pred_idx];

                            // Calculate LF for predecessor based on dependency type
                            let pred_lf = match dep.dep_type {
                                DepType::FS => {
                                    // Finish-to-Start: predecessor must finish before successor starts
                                    task_ls.map(|ls| ls - Duration::seconds(dep.min_gap))
                                }
                                DepType::FF => {
                                    // Finish-to-Finish: predecessor must finish before successor finishes
                                    task_lf.map(|lf| lf - Duration::seconds(dep.min_gap))
                                }
                                DepType::SS => {
                                    // Start-to-Start: predecessor must start before successor starts
                                    let pred_duration = pred_task.estimated_duration.unwrap_or(0);
                                    task_ls.map(|ls| {
                                        ls - Duration::seconds(dep.min_gap + pred_duration)
                                    })
                                }
                                DepType::SF => {
                                    // Start-to-Finish: predecessor must start before successor finishes
                                    task_lf.map(|lf| lf - Duration::seconds(dep.min_gap))
                                }
                            };

                            if let Some(new_lf) = pred_lf {
                                let should_update = pred_task
                                    .schedule
                                    .lf
                                    .map(|current_lf| new_lf < current_lf)
                                    .unwrap_or(true);

                                if should_update {
                                    updates.push((dep.from_task_id, new_lf));
                                }
                            }
                        }
                    }
                }

                // Apply updates
                for (pred_id, new_lf) in updates {
                    if let Some(&pred_idx) = task_indices.get(&pred_id) {
                        let pred_task = &mut tasks[pred_idx];
                        pred_task.schedule.lf = Some(new_lf);
                        if let Some(duration) = pred_task.estimated_duration {
                            pred_task.schedule.ls = Some(new_lf - Duration::seconds(duration));
                        } else {
                            pred_task.schedule.ls = Some(new_lf);
                        }
                        queue.push_back(pred_id);
                    }
                }
            }
        }

        // Calculate slack for all tasks
        for task in tasks.iter_mut() {
            if let (Some(ls), Some(lf)) = (task.schedule.ls, task.schedule.lf) {
                if let Some(duration) = task.estimated_duration {
                    let es = ls; // Earliest Start = Latest Start (in reverse scheduling)
                    let ef = es + Duration::seconds(duration); // Earliest Finish
                    let slack_seconds = (lf - ef).num_seconds();
                    task.schedule.slack = Some(slack_seconds);
                    task.schedule.is_critical = slack_seconds == 0;
                }
            }
        }

        Ok(())
    }

    async fn save_schedules(&self, tasks: &[Task]) -> AppResult<()> {
        for task in tasks {
            query!(
                r#"
                UPDATE tasks
                SET
                    schedule_ls = $2,
                    schedule_lf = $3,
                    schedule_slack = $4,
                    schedule_is_critical = $5,
                    updated_at = $6
                WHERE id = $1
                "#,
                task.id,
                task.schedule.ls,
                task.schedule.lf,
                task.schedule.slack,
                task.schedule.is_critical,
                task.updated_at
            )
                .execute(&self.pool)
                .await
                .context(format!(
                    "Failed to update schedule for task {}",
                    task.id
                ))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ScheduleService for ScheduleServiceImpl {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>> {
        let project = query!(
            "SELECT due_date FROM projects WHERE id = $1",
            project_id
        )
            .fetch_optional(&self.pool)
            .await
            .context(format!(
                "Failed to load project {} for reverse_schedule",
                project_id
            ))?
            .ok_or_else(|| AppError::NotFound(format!("Project with id {} not found", project_id)))?;

        let (mut tasks, dependencies) = self
            .get_tasks_with_dependencies(project_id)
            .await?;

        self.compute_reverse_schedule(&mut tasks, &dependencies, project.due_date)?;

        let now = Utc::now();
        for task in &mut tasks {
            task.updated_at = now;
        }

        self.save_schedules(&tasks).await?;

        Ok(tasks)
    }

    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>> {
        // Пока просто возвращаем текущие schedule поля задач
        // TODO: Реализовать прямой расчет расписания
        Ok(tasks.iter().map(|t| t.schedule.clone()).collect())
    }
}