use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatsResponse {
    pub project_id: String,
    pub project_name: String,

    // Tasks stats
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub in_progress_tasks: i64,
    pub needs_review_tasks: i64,
    pub completion_percent: f64,

    // Team
    pub leader: Option<MemberInfo>,
    pub members_count: i64,
    pub members: Vec<MemberInfo>,

    // Time stats
    pub due_date: String,
    pub days_remaining: i64,
    pub is_overdue: bool,
    pub slack_days: Option<i64>,
    pub critical_tasks_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_leader: bool,
    pub tags: Vec<String>,
}

