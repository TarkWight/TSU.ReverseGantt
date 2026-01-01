use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Planned,
    InProgress,
    NeedsReview,
    Accepted,
    Rejected,
    Blocked,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Planned => write!(f, "Planned"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::NeedsReview => write!(f, "NeedsReview"),
            TaskStatus::Accepted => write!(f, "Accepted"),
            TaskStatus::Rejected => write!(f, "Rejected"),
            TaskStatus::Blocked => write!(f, "Blocked"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Planned" => Ok(TaskStatus::Planned),
            "InProgress" => Ok(TaskStatus::InProgress),
            "NeedsReview" => Ok(TaskStatus::NeedsReview),
            "Accepted" => Ok(TaskStatus::Accepted),
            "Rejected" => Ok(TaskStatus::Rejected),
            "Blocked" => Ok(TaskStatus::Blocked),
            "Done" => Ok(TaskStatus::Done),
            _ => Err(format!("Invalid TaskStatus: {}", s)),
        }
    }
}

