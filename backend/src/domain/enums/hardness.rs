use serde::{Deserialize, Serialize};

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

