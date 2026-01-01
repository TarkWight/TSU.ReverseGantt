use crate::utils::AppError;

// Project
pub fn validate_project_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation("Project name cannot be empty".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("Project name too long".into()));
    }
    Ok(())
}

pub fn validate_project_name_optional(name: &Option<String>) -> Result<(), AppError> {
    if let Some(name) = name {
        validate_project_name(name)?;
    }
    Ok(())
}

pub fn validate_project_time(
    start: &Option<chrono::NaiveDate>,
    end: &chrono::NaiveDate,
) -> Result<(), AppError> {
    if let Some(start_date) = start {
        if start_date > end {
            return Err(AppError::Validation(
                "Project start date must not be later than due date".into(),
            ));
        }
    }
    Ok(())
}

// Task

pub fn validate_task_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation("Task name cannot be empty".into()));
    }
    if name.len() > 255 {
        return Err(AppError::Validation("Task name too long".into()));
    }
    Ok(())
}