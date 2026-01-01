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