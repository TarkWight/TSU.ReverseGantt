use serde::{Deserialize, Serialize};

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

