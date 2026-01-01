use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::api::models::ProjectResponse;
use crate::api::requests::{
    CreateProjectRequest,
    UpdateProjectRequest,
};
use crate::state::AppState;
use crate::auth::AuthContext;
use crate::domain::Project;
use crate::infra::errors::{
    AppError,
    AppResult,
};
use crate::utils::{
    generate_id,
    parse_id,
    validate_project_name,
    validate_project_time,
};
use crate::utils::validation::validate_project_optioanl_time;

pub async fn get_projects(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ProjectResponse>>> {
    let projects = state.project_service.get_all_for_user(&auth).await?;
    Ok(Json(projects.into_iter().map(Into::into).collect()))
}

pub async fn get_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<ProjectResponse>> {
    let project_id = parse_id(&id)?;
    let project = state.project_service.get_by_id(&auth, project_id).await?;
    Ok(Json(project.into()))
}

pub async fn create_project(
    auth: AuthContext,
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<impl IntoResponse> {
    validate_project_name(&req.name)?;
    validate_project_time(&req.start_date, &req.due_date)?;

    let leader_id = if auth.is_teacher {
        let leader_id_str = req.leader_id.ok_or_else(|| {
            AppError::Validation(
                "Teacher must specify leader_id when creating a project"
                    .to_string()
            )
        })?;
        parse_id(&leader_id_str)?
    } else {
        auth.user_id
    };

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

    let created = state.project_service.create(&auth, project, leader_id).await?;

    Ok((StatusCode::CREATED, Json(ProjectResponse::from(created))))
}

pub async fn update_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<ProjectResponse>> {
    let project_id = parse_id(&id)?;
    validate_project_optioanl_time(&req.start_date, &req.due_date)?;

    let project = state.project_service.update(
        &auth, project_id,
        req.name,
        req.description,
        req.start_date,
        req.due_date
    ).await?;
    Ok(Json(project.into()))
}

pub async fn delete_project(
    auth: AuthContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let project_id = parse_id(&id)?;
    state.project_service.delete(&auth, project_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
