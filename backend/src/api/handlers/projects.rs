use std::sync::Arc;

pub async fn get_projects(
    State(service): State<Arc<dyn ProjectService>>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    let projects = service.get_all().await?;
    Ok(Json(projects.into_iter().map(Into::into).collect()))
}

pub async fn get_project(
    Path(id): Path<String>,
    State(service): State<Arc<dyn ProjectService>>,
) -> AppResult<Json<ProjectResponse>> {
    let project_id = parse_id(&id)?;
    let project = service.get_by_id(project_id).await?;
    Ok(Json(project.into()))
}

pub async fn create_project(
    State(service): State<Arc<dyn ProjectService>>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<impl IntoResponse> {
    validate_project_name(&req.name)?;

    let now = chrono::Utc::now();
    let project = Project {
        id: generate_id(),
        name: req.name,
        description: req.description,
        start_date: req.start_date,
        due_date: req.due_date,
        created_at: now,
        updated_at: now,
    };

    let created = service.create(project).await?;
    Ok((StatusCode::CREATED, Json(ProjectResponse::from(created))))
}

pub async fn update_project(
    Path(id): Path<String>,
    State(service): State<Arc<dyn ProjectService>>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    let project_id = parse_id(&id)?;
    let existing = service.get_by_id(project_id).await?;

    let updated = Project {
        id: existing.id,
        name: req.name.unwrap_or(existing.name),
        description: req.description.or(existing.description),
        start_date: req.start_date.or(existing.start_date),
        due_date: req.due_date.unwrap_or(existing.due_date),
        created_at: existing.created_at,
        updated_at: chrono::Utc::now(),
    };

    let project = service.update(project_id, updated).await?;
    Ok(Json(project.into()))
}
pub async fn delete_project(
    Path(id): Path<String>,
    State(service): State<Arc<dyn ProjectService>>,
) -> AppResult<StatusCode> {
    let project_id = parse_id(&id)?;
    service.delete(project_id).await?;
    Ok(StatusCode::NO_CONTENT)
}