use axum::{
    extract::{
        Path,
        State,
    },
    Json,
};
use chrono::Utc;

use crate::state::AppState;
use crate::infra::errors::AppResult;
use crate::utils::parse_id;
use crate::auth::AuthContext;
use crate::auth::permissions::ensure_project_access;
use crate::api::models::{
    ProjectStatsResponse,
    MemberInfo,
};
use crate::domain::TaskStatus;

pub async fn get_project_stats(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectStatsResponse>> {
    let pid = parse_id(&project_id)?;
    ensure_project_access(&auth, pid, &state).await?;

    let project = state.project_service
        .get_by_id(&auth, pid).await?;

    let tasks = state.task_service
        .get_by_project(&auth, pid).await?;

    let total_tasks = tasks.len() as i64;
    let completed_tasks = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::Done)
        .count() as i64;
    let in_progress_tasks = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::InProgress)
        .count() as i64;
    let needs_review_tasks = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::NeedsReview)
        .count() as i64;

    let completion_percent = if total_tasks > 0 {
        (completed_tasks as f64 / total_tasks as f64) * 100.0
    } else {
        0.0
    };

    let critical_tasks_count = tasks
        .iter()
        .filter(|t| t.schedule.is_critical)
        .count() as i64;

    let min_slack_seconds = tasks
        .iter()
        .filter_map(|t| t.schedule.slack)
        .min()
        .unwrap_or(0);
    let slack_days = Some(min_slack_seconds / 86400);

    let memberships = state.membership_service
        .get_by_project(pid).await?;
    let mut members: Vec<MemberInfo> = Vec::new();
    let mut leader: Option<MemberInfo> = None;

    for membership in &memberships {
        if let Ok(user) = state.user_service.get_by_id(membership.user_id).await {
            let member_info = MemberInfo {
                id: user.id.to_string(),
                name: user.name.clone(),
                email: user.email.clone(),
                is_leader: membership.is_leader,
                tags: membership.tags.clone(),
            };

            if membership.is_leader {
                leader = Some(member_info.clone());
            }
            members.push(member_info);
        }
    }

    let members_count = members.len() as i64;

    let today = Utc::now().date_naive();
    let due_date = project.due_date;
    let days_remaining = (due_date - today).num_days();
    let is_overdue = days_remaining < 0;

    Ok(Json(ProjectStatsResponse {
        project_id: project.id.to_string(),
        project_name: project.name,
        total_tasks,
        completed_tasks,
        in_progress_tasks,
        needs_review_tasks,
        completion_percent,
        leader,
        members_count,
        members,
        due_date: due_date.to_string(),
        days_remaining,
        is_overdue,
        slack_days,
        critical_tasks_count,
    }))
}

