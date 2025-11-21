use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use axum::{extract::State, Json};
use axum::extract::Path;

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