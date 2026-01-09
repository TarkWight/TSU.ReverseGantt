use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::utils::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResetRequestType {
    Email,
    TeacherRequest,
}

impl std::fmt::Display for ResetRequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetRequestType::Email => write!(f, "email"),
            ResetRequestType::TeacherRequest => write!(f, "teacher_request"),
        }
    }
}

impl std::str::FromStr for ResetRequestType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "email" => Ok(ResetRequestType::Email),
            "teacher_request" => Ok(ResetRequestType::TeacherRequest),
            _ => Err(format!("Unknown reset request type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResetRequestStatus {
    Pending,
    Used,
    Expired,
    Approved,
    Rejected,
}

impl std::fmt::Display for ResetRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResetRequestStatus::Pending => write!(f, "pending"),
            ResetRequestStatus::Used => write!(f, "used"),
            ResetRequestStatus::Expired => write!(f, "expired"),
            ResetRequestStatus::Approved => write!(f, "approved"),
            ResetRequestStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl std::str::FromStr for ResetRequestStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(ResetRequestStatus::Pending),
            "used" => Ok(ResetRequestStatus::Used),
            "expired" => Ok(ResetRequestStatus::Expired),
            "approved" => Ok(ResetRequestStatus::Approved),
            "rejected" => Ok(ResetRequestStatus::Rejected),
            _ => Err(format!("Unknown reset request status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetCode {
    pub id: Id,
    pub user_id: Id,
    pub code_hash: Option<String>,
    pub request_type: ResetRequestType,
    pub status: ResetRequestStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub reviewed_by: Option<Id>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}


