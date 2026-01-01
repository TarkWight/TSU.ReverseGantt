use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReviewDecision {
    Accepted,
    Rejected,
}

impl std::fmt::Display for ReviewDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewDecision::Accepted => write!(f, "Accepted"),
            ReviewDecision::Rejected => write!(f, "Rejected"),
        }
    }
}

impl std::str::FromStr for ReviewDecision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Accepted" => Ok(ReviewDecision::Accepted),
            "Rejected" => Ok(ReviewDecision::Rejected),
            _ => Err(format!("Invalid ReviewDecision: {}", s)),
        }
    }
}

