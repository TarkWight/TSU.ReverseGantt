use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::domain::{Review, ReviewDecision};
use crate::services::ReviewService;
use crate::utils::{AppResult, parse_id};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReviewRequest {
    pub reviewer_id: String,
    pub decision: Option<ReviewDecision>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReviewRequest {
    pub decision: Option<ReviewDecision>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    pub id: String,
    pub task_id: String,
    pub reviewer_id: String,
    pub decision: Option<ReviewDecision>,
    pub comment: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Review> for ReviewResponse {
    fn from(review: Review) -> Self {
        Self {
            id: review.id.to_string(),
            task_id: review.task_id.to_string(),
            reviewer_id: review.reviewer_id.to_string(),
            decision: review.decision,
            comment: review.comment,
            created_at: review.created_at,
            updated_at: review.updated_at,
        }
    }
}

pub async fn get_review(
    Path(task_id): Path<String>,
    State(service): State<std::sync::Arc<dyn ReviewService>>,
) -> AppResult<Json<Option<ReviewResponse>>> {
    let tid = parse_id(&task_id)?;
    let review = service.get_by_task(tid).await?;
    Ok(Json(review.map(Into::into)))
}

pub async fn create_review(
    Path(task_id): Path<String>,
    State(service): State<std::sync::Arc<dyn ReviewService>>,
    Json(req): Json<CreateReviewRequest>,
) -> AppResult<impl IntoResponse> {
    let tid = parse_id(&task_id)?;
    let reviewer_id = parse_id(&req.reviewer_id)?;

    let review = Review {
        id: crate::utils::generate_id(),
        task_id: tid,
        reviewer_id,
        decision: req.decision,
        comment: req.comment,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = service.create(review).await?;
    Ok((StatusCode::CREATED, Json(ReviewResponse::from(created))))
}