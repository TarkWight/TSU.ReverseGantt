use async_trait::async_trait;
use sqlx::query;
use chrono::{Duration};

use crate::utils::{AppResult, AppError, Id};
use crate::domain::{
    Task,
    Dependency,
    DepType,
    TaskType,
    TaskStatus,
    Priority,
    Schedule
};

#[async_trait]
pub trait ScheduleService: Send + Sync {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>>;
}

use sqlx::PgPool;

pub struct ScheduleServiceImpl {
    pool: PgPool,
}

impl ScheduleServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    async fn get_tasks_with_dependencies(&self, project_id: Id)
                                         -> AppResult<(Vec<Task>, Vec<Dependency>)>
    {
        let tasks = query!(
            r#"
            SELECT
                id, project_id, parent_task_id, name, description,
                task_type, status, priority, estimated_duration,
                planned_start, planned_finish, actual_start, actual_finish,
                progress, buffer, hardness, deadline,
                schedule_ls, schedule_lf, schedule_slack, schedule_is_critical,
                created_at, updated_at
            FROM tasks
            WHERE project_id = $1
            "#,
            project_id
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;

        let task_ids: Vec<Id> = tasks.iter().map(|t| t.id).collect();

        let deps = if !task_ids.is_empty() {
            query!(
                r#"
                SELECT id, from_task_id, to_task_id, dep_type, min_gap
                FROM dependencies
                WHERE from_task_id = ANY($1) OR to_task_id = ANY($1)
                "#,
                &task_ids[..]
            )
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?
        } else {
            vec![]
        };

        let domain_tasks = tasks.into_iter().map(|row| {
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
                hardness: row.hardness.parse().unwrap_or(crate::domain::enums::Hardness::Soft),
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
        }).collect();

        let domain_deps = deps.into_iter().map(|row| Dependency {
            id: row.id,
            from_task_id: row.from_task_id,
            to_task_id: row.to_task_id,
            dep_type: row.dep_type.parse().unwrap_or(DepType::FS),
            min_gap: row.min_gap,
        }).collect();

        Ok((domain_tasks, domain_deps))
    }

    fn compute_reverse_schedule(
        &self,
        tasks: &mut [Task],
        dependencies: &[Dependency],
        project_due_date: chrono::NaiveDate,
    ) -> AppResult<()> {
        if tasks.is_empty() {
            return Ok(());
        }

        use std::collections::{VecDeque, HashMap};

        let mut successors: HashMap<Id, Vec<&Dependency>> = HashMap::new();
        let mut predecessors: HashMap<Id, Vec<&Dependency>> = HashMap::new();

        for dep in dependencies {
            successors.entry(dep.from_task_id).or_default().push(dep);
            predecessors.entry(dep.to_task_id).or_default().push(dep);
        }

        let mut index: HashMap<Id, usize> = HashMap::new();
        for (i, t) in tasks.iter().enumerate() {
            index.insert(t.id, i);
        }

        let deadline = project_due_date.and_hms_opt(23, 59, 59).unwrap().and_utc();

        let mut queue = VecDeque::new();
        for t in tasks.iter() {
            if !successors.contains_key(&t.id) {
                queue.push_back(t.id);
            }
        }

        while let Some(task_id) = queue.pop_front() {
            let idx = index[&task_id];
            let task = &mut tasks[idx];

            if task.schedule.lf.is_none() {
                task.schedule.lf = Some(deadline);

                if let Some(dur) = task.estimated_duration {
                    task.schedule.ls = task.schedule.lf.map(|lf| lf - Duration::seconds(dur));
                } else {
                    task.schedule.ls = task.schedule.lf;
                }
            }

            let mut to_update = vec![];

            if let Some(preds) = predecessors.get(&task_id) {
                let ls = task.schedule.ls;
                let lf = task.schedule.lf;

                for dep in preds {
                    let pred_idx = index[&dep.from_task_id];
                    let pred_task = &tasks[pred_idx];

                    let candidate_lf = match dep.dep_type {
                        DepType::FS => ls.map(|ls| ls - Duration::seconds(dep.min_gap)),
                        DepType::FF => lf.map(|lf| lf - Duration::seconds(dep.min_gap)),
                        DepType::SS => ls.map(|ls| {
                            let pred_dur = pred_task.estimated_duration.unwrap_or(0);
                            ls - Duration::seconds(pred_dur + dep.min_gap)
                        }),
                        DepType::SF => lf.map(|lf| lf - Duration::seconds(dep.min_gap)),
                    };

                    if let Some(new_lf) = candidate_lf {
                        let should =
                            pred_task.schedule.lf.map(|old| new_lf < old).unwrap_or(true);
                        if should {
                            to_update.push((dep.from_task_id, new_lf));
                        }
                    }
                }
            }

            for (pid, new_lf) in to_update {
                let idx = index[&pid];
                let pred = &mut tasks[idx];
                pred.schedule.lf = Some(new_lf);

                if let Some(dur) = pred.estimated_duration {
                    pred.schedule.ls = Some(new_lf - Duration::seconds(dur));
                } else {
                    pred.schedule.ls = Some(new_lf);
                }

                queue.push_back(pid);
            }
        }

        for t in tasks.iter_mut() {
            if let (Some(ls), Some(lf)) = (t.schedule.ls, t.schedule.lf) {
                if let Some(dur) = t.estimated_duration {
                    let es = ls;
                    let ef = es + Duration::seconds(dur);
                    let slack = (lf - ef).num_seconds();
                    t.schedule.slack = Some(slack);
                    t.schedule.is_critical = slack == 0;
                }
            }
        }

        Ok(())
    }

    async fn save_schedules(&self, tasks: &[Task]) -> AppResult<()> {
        for t in tasks {
            query!(
                r#"
                UPDATE tasks
                SET schedule_ls = $2,
                    schedule_lf = $3,
                    schedule_slack = $4,
                    schedule_is_critical = $5,
                    updated_at = $6
                WHERE id = $1
                "#,
                t.id,
                t.schedule.ls,
                t.schedule.lf,
                t.schedule.slack,
                t.schedule.is_critical,
                t.updated_at
            )
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Database error: {}", e)))?;
        }
        Ok(())
    }
}