use async_trait::async_trait;
use chrono::{DateTime, Utc, Duration, NaiveDate};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use crate::domain::{Task, Schedule, Dependency, DepType};
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::ScheduleRepository;
use crate::utils::Id;

struct TmpSchedule {
    es: DateTime<Utc>,
    ef: DateTime<Utc>,
    ls: DateTime<Utc>,
    lf: DateTime<Utc>,
    slack: i64,
    is_critical: bool,
}

#[async_trait]
pub trait ScheduleService: Send + Sync {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>>;
    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>>;
}

pub struct ScheduleServiceImpl {
    schedule_repo: Arc<dyn ScheduleRepository>,
}

impl ScheduleServiceImpl {
    pub fn new(schedule_repo: Arc<dyn ScheduleRepository>) -> Self {
        Self { schedule_repo }
    }

    fn topological_sort(&self, tasks: &[Task], successors: &HashMap<Id, Vec<&Dependency>>) -> Vec<Id> {
        let mut in_degree: HashMap<Id, usize> = HashMap::new();

        for task in tasks {
            in_degree.insert(task.id, 0);
        }

        for task in tasks {
            if let Some(deps) = successors.get(&task.id) {
                for dep in deps {
                    *in_degree.entry(dep.to_task_id).or_insert(0) += 1;
                }
            }
        }

        let mut queue: VecDeque<Id> = VecDeque::new();
        for task in tasks {
            if in_degree.get(&task.id) == Some(&0) {
                queue.push_back(task.id);
            }
        }

        let mut result: Vec<Id> = Vec::new();
        while let Some(task_id) = queue.pop_front() {
            result.push(task_id);

            if let Some(deps) = successors.get(&task_id) {
                for dep in deps {
                    if let Some(deg) = in_degree.get_mut(&dep.to_task_id) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep.to_task_id);
                        }
                    }
                }
            }
        }

        result
    }

    fn compute_full_schedule(
        &self,
        tasks: &mut [Task],
        dependencies: &[Dependency],
        project_start_date: Option<NaiveDate>,
        project_due_date: NaiveDate,
    ) -> AppResult<()> {
        if tasks.is_empty() {
            return Ok(());
        }

        let mut successors: HashMap<Id, Vec<&Dependency>> = HashMap::new();
        let mut predecessors: HashMap<Id, Vec<&Dependency>> = HashMap::new();

        for dep in dependencies {
            successors.entry(dep.from_task_id).or_default().push(dep);
            predecessors.entry(dep.to_task_id).or_default().push(dep);
        }

        let mut task_by_id: HashMap<Id, usize> = HashMap::new();
        for (idx, task) in tasks.iter().enumerate() {
            task_by_id.insert(task.id, idx);
        }

        let project_start = project_start_date
            .unwrap_or(project_due_date)
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

        let deadline = project_due_date
            .and_hms_opt(23, 59, 59)
            .unwrap()
            .and_utc();

        let mut tmp: HashMap<Id, TmpSchedule> = HashMap::new();
        let topo_order = self.topological_sort(tasks, &successors);

        for task_id in &topo_order {
            if let Some(&task_idx) = task_by_id.get(task_id) {
                let task = &tasks[task_idx];
                let dur = Duration::seconds(task.estimated_duration.unwrap_or(0));

                let es = if let Some(pred_deps) = predecessors.get(task_id) {
                    let mut max_es = project_start;

                    for dep in pred_deps {
                        if let Some(pred_sched) = tmp.get(&dep.from_task_id) {
                            let gap = Duration::seconds(dep.min_gap);

                            let candidate_es = match dep.dep_type {
                                DepType::FS => pred_sched.ef + gap,
                                DepType::FF => pred_sched.ef + gap - dur,
                                DepType::SS => pred_sched.es + gap,
                                DepType::SF => pred_sched.es + gap - dur,
                            };

                            if candidate_es > max_es {
                                max_es = candidate_es;
                            }
                        }
                    }
                    max_es
                } else {
                    project_start
                };

                let ef = es + dur;

                tmp.insert(*task_id, TmpSchedule {
                    es,
                    ef,
                    ls: deadline,
                    lf: deadline,
                    slack: 0,
                    is_critical: false,
                });
            }
        }

        let reverse_topo: Vec<Id> = topo_order.iter().rev().cloned().collect();

        for task_id in &reverse_topo {
            if let Some(&task_idx) = task_by_id.get(task_id) {
                let task = &tasks[task_idx];
                let dur = Duration::seconds(task.estimated_duration.unwrap_or(0));

                let lf = if let Some(succ_deps) = successors.get(task_id) {
                    let mut min_lf = deadline;

                    for dep in succ_deps {
                        if let Some(succ_sched) = tmp.get(&dep.to_task_id) {
                            let gap = Duration::seconds(dep.min_gap);

                            let candidate_lf = match dep.dep_type {
                                DepType::FS => succ_sched.ls - gap,
                                DepType::FF => succ_sched.lf - gap,
                                DepType::SS => succ_sched.ls - gap + dur,
                                DepType::SF => succ_sched.lf - gap + dur,
                            };

                            if candidate_lf < min_lf {
                                min_lf = candidate_lf;
                            }
                        }
                    }
                    min_lf
                } else {
                    deadline
                };

                let ls = lf - dur;

                if let Some(sched) = tmp.get_mut(task_id) {
                    sched.ls = ls;
                    sched.lf = lf;
                }
            }
        }

        for task_id in &topo_order {
            if let Some(sched) = tmp.get_mut(task_id) {
                let slack_seconds = (sched.lf - sched.ef).num_seconds();
                sched.slack = slack_seconds.max(0);
                sched.is_critical = slack_seconds <= 0;
            }
        }

        for task in tasks.iter_mut() {
            if let Some(sched) = tmp.get(&task.id) {
                task.schedule.ls = Some(sched.ls);
                task.schedule.lf = Some(sched.lf);
                task.schedule.slack = Some(sched.slack);
                task.schedule.is_critical = sched.is_critical;
            }
        }

        self.aggregate_hierarchy(tasks);

        Ok(())
    }

    fn aggregate_hierarchy(&self, tasks: &mut [Task]) {
        if tasks.is_empty() {
            return;
        }

        let mut by_id: HashMap<Id, usize> = HashMap::new();
        for (idx, task) in tasks.iter().enumerate() {
            by_id.insert(task.id, idx);
        }

        let mut children: HashMap<Id, Vec<Id>> = HashMap::new();
        let mut roots: Vec<Id> = Vec::new();

        for task in tasks.iter() {
            if let Some(parent_id) = task.parent_task_id {
                if by_id.contains_key(&parent_id) {
                    children.entry(parent_id).or_default().push(task.id);
                } else {
                    roots.push(task.id);
                }
            } else {
                roots.push(task.id);
            }
        }

        for root_id in roots {
            let mut stack: Vec<(Id, usize)> = Vec::new();
            let mut visited: HashSet<Id> = HashSet::new();

            stack.push((root_id, 0));

            while let Some((task_id, children_processed)) = stack.last_mut() {
                let current_id = *task_id;

                if visited.contains(&current_id) {
                    stack.pop();
                    continue;
                }

                let child_ids = children.get(&current_id).map(|v| v.as_slice()).unwrap_or(&[]);

                if *children_processed < child_ids.len() {
                    let child_id = child_ids[*children_processed];
                    *children_processed += 1;

                    if by_id.contains_key(&child_id) {
                        stack.push((child_id, 0));
                    }
                } else {
                    stack.pop();
                    visited.insert(current_id);

                    if let Some(&task_idx) = by_id.get(&current_id) {
                        let child_ids = children.get(&current_id).map(|v| v.as_slice()).unwrap_or(&[]);

                        if !child_ids.is_empty() {
                            let mut child_ls_values: Vec<DateTime<Utc>> = Vec::new();
                            let mut child_lf_values: Vec<DateTime<Utc>> = Vec::new();
                            let mut child_slack_values: Vec<i64> = Vec::new();
                            let mut any_child_critical = false;

                            for &child_id in child_ids.iter() {
                                if let Some(&child_idx) = by_id.get(&child_id) {
                                    let child = &tasks[child_idx];
                                    if let Some(ls) = child.schedule.ls {
                                        child_ls_values.push(ls);
                                    }
                                    if let Some(lf) = child.schedule.lf {
                                        child_lf_values.push(lf);
                                    }
                                    if let Some(slack) = child.schedule.slack {
                                        child_slack_values.push(slack);
                                    }
                                    if child.schedule.is_critical {
                                        any_child_critical = true;
                                    }
                                }
                            }

                            let task = &mut tasks[task_idx];

                            if let Some(&min_child_ls) = child_ls_values.iter().min() {
                                task.schedule.ls = Some(
                                    task.schedule.ls
                                        .map(|parent_ls| parent_ls.min(min_child_ls))
                                        .unwrap_or(min_child_ls)
                                );
                            }

                            if let Some(&max_child_lf) = child_lf_values.iter().max() {
                                task.schedule.lf = Some(
                                    task.schedule.lf
                                        .map(|parent_lf| parent_lf.max(max_child_lf))
                                        .unwrap_or(max_child_lf)
                                );
                            }

                            if !child_slack_values.is_empty() {
                                let min_child_slack = *child_slack_values.iter().min().unwrap();
                                task.schedule.slack = Some(min_child_slack);
                                task.schedule.is_critical = min_child_slack == 0 || any_child_critical;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl ScheduleService for ScheduleServiceImpl {
    async fn reverse_schedule(&self, project_id: Id) -> AppResult<Vec<Task>> {
        let (start_date, due_date) = self.schedule_repo
            .get_project_dates(project_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("Project {} not found", project_id)))?;

        let mut tasks = self.schedule_repo
            .get_tasks_by_project(project_id)
            .await
            .map_err(AppError::Internal)?;

        let task_ids: Vec<Id> = tasks.iter().map(|t| t.id).collect();

        let dependencies = self.schedule_repo
            .get_dependencies_by_task_ids(&task_ids)
            .await
            .map_err(AppError::Internal)?;

        self.compute_full_schedule(&mut tasks, &dependencies, start_date, due_date)?;

        let now = Utc::now();
        for task in &mut tasks {
            task.updated_at = now;
            self.schedule_repo
                .update_task_schedule(task)
                .await
                .map_err(AppError::Internal)?;
        }

        Ok(tasks)
    }

    async fn compute_schedule(&self, tasks: &[Task]) -> AppResult<Vec<Schedule>> {
        Ok(tasks.iter().map(|t| t.schedule.clone()).collect())
    }
}
