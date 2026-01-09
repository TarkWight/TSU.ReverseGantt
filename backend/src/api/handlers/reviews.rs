use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::domain::{
    Review,
    ReviewDecision,
    Notification,
    NotificationType,
};
use crate::infra::errors::AppResult;
use crate::utils::{
    parse_id,
    generate_id,
};
use crate::auth::AuthContext;
use crate::auth::permissions::{
    ensure_project_access,
    ensure_project_leader_or_teacher,
};
use crate::state::AppState;
use crate::domain::enums::AssignRole;
use crate::api::requests::CreateReviewRequest;
use crate::api::models::ReviewResponse;

pub async fn get_review(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Option<ReviewResponse>>> {
    let tid = parse_id(&task_id)?;
    let task = state.task_service.get_by_id_internal(tid).await?;
    ensure_project_access(&auth, task.project_id, &state).await?;

    let review = state.review_service.get_by_task(tid).await?;
    Ok(Json(review.map(Into::into)))
}

pub async fn create_review(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateReviewRequest>,
) -> AppResult<impl IntoResponse> {
    let tid = parse_id(&task_id)?;
    let task = state.task_service.get_by_id_internal(tid).await?;
    ensure_project_leader_or_teacher(&auth, task.project_id, &state).await?;

    let reviewer_id = parse_id(&req.reviewer_id)?;

    let review = Review {
        id: generate_id(),
        task_id: tid,
        reviewer_id,
        decision: req.decision,
        comment: req.comment,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = state.review_service.create(review.clone()).await?;

    if let Some(decision) = &created.decision {
        tracing::info!("Review has decision {:?}, looking for task owner...", decision);

        let assignments = state.assignment_service.get_by_task(tid).await?;

        if let Some(owner_assignment) = assignments.iter().find(|a| a.role == AssignRole::Owner) {
            let owner = state.user_service.get_by_id(owner_assignment.user_id).await?;

            let (notification_type, title, status_text) = match decision {
                ReviewDecision::Accepted => (
                    NotificationType::TaskAccepted,
                    format!("✅ Задача «{}» принята!", task.name),
                    "Done"
                ),
                ReviewDecision::Rejected => (
                    NotificationType::TaskRejected,
                    format!("❌ Задача «{}» отклонена", task.name),
                    "InProgress"
                ),
            };

            let notification = Notification {
                id: generate_id(),
                user_id: owner.id,
                notification_type,
                title: title.clone(),
                message: created.comment.clone(),
                task_id: Some(tid),
                project_id: Some(task.project_id),
                is_read: false,
                created_at: chrono::Utc::now(),
            };

            if let Err(e) = state.notification_service.create(notification).await {
                tracing::error!("Failed to create in-app notification: {}", e);
            }

            if owner.email_notifications_enabled {
                let email_service = state.email_service.clone();
                let task_name = task.name.clone();
                let owner_email = owner.email.clone();
                let owner_name = owner.name.clone();
                let review_comment = created.comment.clone();
                let is_approved = matches!(decision, ReviewDecision::Accepted);
                let is_rejected = matches!(decision, ReviewDecision::Rejected);

                tokio::spawn(async move {
                    if let Err(e) = email_service.send_task_notification(
                        &owner_email,
                        &owner_name,
                        &task_name,
                        Some("NeedsReview"),
                        status_text,
                        review_comment.as_deref(),
                        is_approved,
                        is_rejected,
                    ).await {
                        tracing::error!("Failed to send email: {}", e);
                    }
                });
            }
        } else {
            tracing::warn!("No owner assignment found for task {}", tid);
        }
    }

    Ok((StatusCode::CREATED, Json(ReviewResponse::from(created))))
}

