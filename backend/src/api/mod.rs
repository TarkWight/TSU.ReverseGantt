pub mod projects;

use std::sync::Arc;
use crate::services::ProjectService;

#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<dyn ProjectService>,
}