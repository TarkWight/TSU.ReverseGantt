use std::sync::Arc;

pub async fn get_projects(
    State(service): State<Arc<dyn ProjectService>>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    let projects = service.get_all().await?;
    Ok(Json(projects.into_iter().map(Into::into).collect()))
}
