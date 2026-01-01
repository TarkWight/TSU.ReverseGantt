use serde::Deserialize;
use crate::domain::ReviewDecision;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReviewRequest {
    pub reviewer_id: String,
    pub decision: Option<ReviewDecision>,
    pub comment: Option<String>,
}