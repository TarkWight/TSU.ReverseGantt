use crate::auth::AuthContext;
use crate::domain::enums::AssignRole;
use crate::infra::errors::{AppError, AppResult};
use crate::utils::Id;
use crate::state::AppState;

pub async fn ensure_project_access(
    auth: &AuthContext,
    project_id: Id,
    state: &AppState,
) -> AppResult<()> {
    if auth.is_teacher {
        return Ok(());
    }

    state
        .membership_service
        .get_by_project_and_user(project_id, auth.user_id)
        .await?;

    Ok(())
}

pub async fn ensure_project_leader_or_teacher(
    auth: &AuthContext,
    project_id: Id,
    state: &AppState,
) -> AppResult<()> {
    if auth.is_teacher {
        return Ok(());
    }

    let is_leader = state
        .membership_service
        .check_user_is_leader(project_id, auth.user_id)
        .await?;

    if !is_leader {
        return Err(AppError::Forbidden(
            "You must be the project leader to perform this operation".to_string(),
        ));
    }

    Ok(())
}

pub async fn ensure_can_manage_members(
    auth: &AuthContext,
    project_id: Id,
    state: &AppState,
) -> AppResult<()> {
    ensure_project_leader_or_teacher(auth, project_id, state).await
}

pub async fn ensure_can_create_task(
    auth: &AuthContext,
    project_id: Id,
    state: &AppState,
) -> AppResult<()> {
    ensure_project_access(auth, project_id, state).await
}

pub async fn ensure_can_edit_task(
    auth: &AuthContext,
    task_id: Id,
    state: &AppState,
) -> AppResult<()> {
    if auth.is_teacher {
        return Ok(());
    }

    let task = state.task_service.get_by_id_internal(task_id).await?;
    let project_id = task.project_id;

    let is_leader = state
        .membership_service
        .check_user_is_leader(project_id, auth.user_id)
        .await?;

    if is_leader {
        return Ok(());
    }

    let assignments = state.assignment_service.get_by_task(task_id).await?;
    let is_owner = assignments
        .iter()
        .any(|a| a.user_id == auth.user_id && a.role == AssignRole::Owner);

    if !is_owner {
        return Err(AppError::Forbidden(
            "You do not have permission to update this task".to_string(),
        ));
    }

    Ok(())
}

pub async fn ensure_can_delete_task(
    auth: &AuthContext,
    task_id: Id,
    state: &AppState,
) -> AppResult<()> {
    if auth.is_teacher {
        return Ok(());
    }

    let task = state.task_service.get_by_id_internal(task_id).await?;
    let project_id = task.project_id;

    let is_leader = state
        .membership_service
        .check_user_is_leader(project_id, auth.user_id)
        .await?;

    if !is_leader {
        return Err(AppError::Forbidden(
            "Only project leaders or teachers can delete tasks".to_string(),
        ));
    }

    Ok(())
}

pub async fn ensure_can_manage_dependencies(
    auth: &AuthContext,
    task_id: Id,
    state: &AppState,
) -> AppResult<()> {
    if auth.is_teacher {
        return Ok(());
    }

    let task = state.task_service.get_by_id_internal(task_id).await?;
    let project_id = task.project_id;

    let is_leader = state
        .membership_service
        .check_user_is_leader(project_id, auth.user_id)
        .await?;

    if is_leader {
        return Ok(());
    }

    let assignments = state.assignment_service.get_by_task(task_id).await?;
    let is_owner = assignments
        .iter()
        .any(|a| a.user_id == auth.user_id && a.role == AssignRole::Owner);

    if !is_owner {
        return Err(AppError::Forbidden(
            "You do not have permission to manage dependencies for this task".to_string(),
        ));
    }

    Ok(())
}

pub async fn ensure_teacher(auth: &AuthContext) -> AppResult<()> {
    if !auth.is_teacher {
        return Err(AppError::Forbidden(
            "This operation requires Teacher role".to_string(),
        ));
    }
    Ok(())
}
