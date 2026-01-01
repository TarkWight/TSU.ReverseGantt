pub async fn get_tasks(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    let pid = parse_id(&project_id)?;
    let tasks = service.get_by_project(pid).await?;
    Ok(Json(tasks.into_iter().map(Into::into).collect()))
}
