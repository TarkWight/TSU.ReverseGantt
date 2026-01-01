pub async fn create_dependency(
    Path(from_task_id): Path<String>,
    State(service): State<std::sync::Arc<dyn DependencyService>>,
    Json(req): Json<CreateDependencyRequest>,
) -> AppResult<impl IntoResponse> {
    let from_id = parse_id(&from_task_id)?;
    let to_id = parse_id(&req.to_task_id)?;

    if from_id == to_id {
        return Err(crate::utils::AppError::Validation(
            "Task cannot depend on itself".into(),
        ));
    }

    let dependency = Dependency {
        id: crate::utils::generate_id(),
        from_task_id: from_id,
        to_task_id: to_id,
        dep_type: req.dep_type,
        min_gap: req.min_gap,
    };

    let created = service.create(dependency).await?;
    Ok((StatusCode::CREATED, Json(DependencyResponse::from(created))))
}