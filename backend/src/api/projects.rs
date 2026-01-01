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







