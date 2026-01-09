use serde::Serialize;
use crate::domain::Artifact;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactResponse {
    pub id: String,
    pub task_id: String,
    pub name: String,
    pub uri: String,
    pub kind: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Artifact> for ArtifactResponse {
    fn from(a: Artifact) -> Self {
        Self {
            id: a.id.to_string(),
            task_id: a.task_id.to_string(),
            name: a.name,
            uri: a.uri,
            kind: a.kind,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

