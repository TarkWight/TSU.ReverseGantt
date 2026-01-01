use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskType {
    Task,
    Feature,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Task => write!(f, "Task"),
            TaskType::Feature => write!(f, "Feature"),
        }
    }
}

impl std::str::FromStr for TaskType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Task" => Ok(TaskType::Task),
            "Feature" => Ok(TaskType::Feature),
            _ => Err(format!("Invalid TaskType: {}", s)),
        }
    }
}

