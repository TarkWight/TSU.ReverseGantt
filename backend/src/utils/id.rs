use uuid::Uuid;
use crate::utils::AppError;

pub type Id = Uuid;

pub fn generate_id() -> Id {
    Uuid::new_v4()
}

pub fn parse_id(s: &str) -> Result<Id, AppError> {
    Ok(Uuid::parse_str(s)?)
}