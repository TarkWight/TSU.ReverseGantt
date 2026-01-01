use serde::Serialize;
use super::task::TaskResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseScheduleResponse {
    pub tasks: Vec<TaskResponse>,
}
