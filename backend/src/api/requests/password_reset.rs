use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmPasswordResetRequest {
    pub email: String,
    pub code: String,
    pub new_password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_password_reset_request_deserialization() {
        let json = r#"{"email":"test@test.com","code":"123456","newPassword":"Test123!"}"#;
        let result: Result<ConfirmPasswordResetRequest, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.email, "test@test.com");
        assert_eq!(req.code, "123456");
        assert_eq!(req.new_password, "Test123!");
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherApproveResetRequest {
    pub new_password: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherRejectResetRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherSetPasswordRequest {
    pub new_password: String,
}


