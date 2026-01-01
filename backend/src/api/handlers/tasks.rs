
pub async fn get_task(
    Path(id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
) -> AppResult<Json<TaskResponse>> {
    let task_id = parse_id(&id)?;
    let task = service.get_by_id(task_id).await?;
    Ok(Json(task.into()))
}

pub async fn get_tasks(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
) -> AppResult<Json<Vec<TaskResponse>>> {
    let pid = parse_id(&project_id)?;
    let tasks = service.get_by_project(pid).await?;
    Ok(Json(tasks.into_iter().map(Into::into).collect()))
}

pub async fn create_task(
    Path(project_id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<impl IntoResponse> {
    crate::utils::validate_task_name(&req.name)?;

    let task = Task {
        id: crate::utils::generate_id(),
        project_id: parse_id(&project_id)?,
        parent_task_id: req.parent_task_id.and_then(|s| parse_id(&s).ok()),
        name: req.name,
        description: req.description,
        task_type: req.task_type,
        status: TaskStatus::Planned,
        priority: req.priority,
        estimated_duration: req.estimated_duration,
        planned_start: None,
        planned_finish: None,
        actual_start: None,
        actual_finish: None,
        progress: 0,
        buffer: req.buffer.unwrap_or(0),
        hardness: req
            .hardness
            .unwrap_or(crate::domain::enums::Hardness::Soft),
        deadline: req.deadline,
        schedule: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = service.create(task).await?;
    Ok((StatusCode::CREATED, Json(TaskResponse::from(created))))
}

pub async fn update_task(
    Path(id): Path<String>,
    State(service): State<std::sync::Arc<dyn TaskService>>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<TaskResponse>> {
    let task_id = parse_id(&id)?;
    let existing = service.get_by_id(task_id).await?;

    let updated = Task {
        id: existing.id,
        project_id: existing.project_id,
        parent_task_id: existing.parent_task_id,
        name: req.name.unwrap_or(existing.name),
        description: req.description.or(existing.description),
        task_type: req.task_type.unwrap_or(existing.task_type),
        status: req.status.unwrap_or(existing.status),
        priority: req.priority.unwrap_or(existing.priority),
        estimated_duration: req.estimated_duration.or(existing.estimated_duration),
        planned_start: req.planned_start.or(existing.planned_start),
        planned_finish: req.planned_finish.or(existing.planned_finish),
        actual_start: req.actual_start.or(existing.actual_start),
        actual_finish: req.actual_finish.or(existing.actual_finish),
        progress: req.progress.unwrap_or(existing.progress),
        buffer: req.buffer.unwrap_or(existing.buffer),
        hardness: req.hardness.unwrap_or(existing.hardness),
        deadline: req.deadline.or(existing.deadline),
        schedule: existing.schedule,
        created_at: existing.created_at,
        updated_at: chrono::Utc::now(),
    };

    let task = service.update(task_id, updated).await?;
    Ok(Json(task.into()))
}
