use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssignRole {
    Owner,
    Assignee,
}

impl std::fmt::Display for AssignRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignRole::Owner => write!(f, "owner"),
            AssignRole::Assignee => write!(f, "assignee"),
        }
    }
}

impl std::str::FromStr for AssignRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(AssignRole::Owner),
            "assignee" => Ok(AssignRole::Assignee),
            _ => Err(format!("Invalid AssignRole: {}", s)),
        }
    }
}

