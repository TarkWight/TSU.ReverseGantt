use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DepType {
    FS,
    FF,
    SS,
    SF,
}

impl std::fmt::Display for DepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepType::FS => write!(f, "FS"),
            DepType::FF => write!(f, "FF"),
            DepType::SS => write!(f, "SS"),
            DepType::SF => write!(f, "SF"),
        }
    }
}

impl std::str::FromStr for DepType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FS" => Ok(DepType::FS),
            "FF" => Ok(DepType::FF),
            "SS" => Ok(DepType::SS),
            "SF" => Ok(DepType::SF),
            _ => Err(format!("Invalid DepType: {}", s)),
        }
    }
}

