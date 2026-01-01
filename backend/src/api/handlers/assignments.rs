pub async fn create_assignment(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateAssignmentRequest>,
) -> AppResult<impl IntoResponse> {
    let task_id = parse_id(&task_id)?;
    let task = state.task_service.get_by_id(&auth, task_id).await?;
    ensure_can_manage_members(&auth, task.project_id, &state).await?;

    let user_id = parse_id(&req.user_id)?;

    if req.role == AssignRole::Owner {
        let owner_user = state.user_service.get_by_id(user_id).await?;
        if owner_user.global_role != GlobalRole::Student {
            return Err(AppError::Validation(
                "Task owner must be a Student, not a Teacher".to_string(),
            ));
        }

        state.membership_service
            .get_by_project_and_user(task.project_id, user_id)
            .await?;

        let existing_assignments = state.assignment_service.get_by_task(task_id).await?;
        for assignment in existing_assignments {
            if assignment.role == AssignRole::Owner {
                state.assignment_service.delete(assignment.id).await?;
            }
        }
    }

    let assignment = Assignment {
        id: generate_id(),
        task_id,
        user_id,
        role: req.role,
    };

    let created = state.assignment_service.create(assignment).await?;
    Ok((StatusCode::CREATED, Json(AssignmentResponse::from(created))))
}

pub async fn delete_assignment(
    auth: AuthContext,
    Path(assignment_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let assignment_id = parse_id(&assignment_id)?;
    let assignment = state.assignment_service.get_by_id(assignment_id).await?;
    let task = state.task_service.get_by_id(&auth, assignment.task_id).await?;
    ensure_can_manage_members(&auth, task.project_id, &state).await?;

    state.assignment_service.delete(assignment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_task_assignments(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<AssignmentResponse>>> {
    let task_id = parse_id(&task_id)?;
    let task = state.task_service.get_by_id(&auth, task_id).await?;
    ensure_project_access(&auth, task.project_id, &state).await?;

    let assignments = state.assignment_service.get_by_task(task_id).await?;
    Ok(Json(assignments.into_iter().map(Into::into).collect()))
}