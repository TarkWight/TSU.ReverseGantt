pub async fn reverse_schedule(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn ScheduleService>>,
) -> AppResult<Json<ReverseScheduleResponse>> {
    let pid = parse_id(&project_id)?;
    let tasks = service.reverse_schedule(pid).await?;
    Ok(Json(ReverseScheduleResponse {
        tasks: tasks.into_iter().map(Into::into).collect(),
    }))
}