use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use axum::{extract::State, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::utils::{generate_id, validate_project_name};
use crate::utils::parse_id;
use crate::services::ProjectService;
use crate::utils::AppResult;
use crate::domain::Project;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub due_date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub due_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub due_date: chrono::NaiveDate,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id.to_string(),
            name: project.name,
            description: project.description,
            start_date: project.start_date,
            due_date: project.due_date,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
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