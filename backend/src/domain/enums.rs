use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    Final,
    Milestone,
    Task,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Final => write!(formatter, "Final"),
            TaskType::Milestone => write!(formatter, "Milestone"),
            TaskType::Task => write!(formatter, "Task"),
        }
    }
}