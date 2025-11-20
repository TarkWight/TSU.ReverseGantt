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

impl std::str::FromStr for TaskType {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "Final" => Ok(TaskType::Final),
            "Milestone" => Ok(TaskType::Milestone),
            "Task" => Ok(TaskType::Task),
            _ => Err(format!("Invalid TaskType: {}", string)),
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Normal => write!(f, "Normal"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(Priority::Low),
            "Normal" => Ok(Priority::Normal),
            "High" => Ok(Priority::High),
            "Critical" => Ok(Priority::Critical),
            _ => Err(format!("Invalid Priority: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Hardness {
    Hard,
    Soft,
}

impl std::fmt::Display for Hardness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hardness::Hard => write!(f, "Hard"),
            Hardness::Soft => write!(f, "Soft"),
        }
    }
}

impl std::str::FromStr for Hardness {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Hard" => Ok(Hardness::Hard),
            "Soft" => Ok(Hardness::Soft),
            _ => Err(format!("Invalid Hardness: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DepType {
    FS, // Finish-to-Start
    FF, // Finish-to-Finish
    SS, // Start-to-Start
    SF, // Start-to-Finish
}