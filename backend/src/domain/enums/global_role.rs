use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GlobalRole {
    Student,
    Teacher,
}

impl std::fmt::Display for GlobalRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalRole::Student => write!(f, "student"),
            GlobalRole::Teacher => write!(f, "teacher"),
        }
    }
}

impl std::str::FromStr for GlobalRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "student" => Ok(GlobalRole::Student),
            "teacher" => Ok(GlobalRole::Teacher),
            _ => Err(format!("Invalid GlobalRole: {}", s)),
        }
    }
}

