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