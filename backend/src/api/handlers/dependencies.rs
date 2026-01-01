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

pub async fn get_dependencies(
    Path(task_id): Path<String>,
    State(service): State<std::sync::Arc<dyn DependencyService>>,
) -> AppResult<Json<Vec<DependencyResponse>>> {
    let tid = parse_id(&task_id)?;
    let deps = service.get_by_task(tid).await?;
    Ok(Json(deps.into_iter().map(Into::into).collect()))
}

pub async fn delete_dependency(
    auth: AuthContext,
    Path((task_id, dep_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let task_id = parse_id(&task_id)?;
    let dep_id = parse_id(&dep_id)?;

    let dependency = state.dependency_service.get_by_id(dep_id).await?;

    if dependency.from_task_id != task_id && dependency.to_task_id != task_id {
        return Err(AppError::BadRequest("Dependency does not belong to this task".to_string()));
    }

    let mut has_permission = ensure_can_manage_dependencies(&auth, dependency.from_task_id, &state).await.is_ok();
    if !has_permission {
        has_permission = ensure_can_manage_dependencies(&auth, dependency.to_task_id, &state).await.is_ok();
    }

    if !has_permission {
        return Err(AppError::BadRequest("You do not have permission to delete this dependency".to_string()));
    }

    state.dependency_service.delete(dep_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
