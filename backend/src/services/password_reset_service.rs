use async_trait::async_trait;
use std::sync::Arc;
use chrono::{Duration, Utc};

use crate::domain::{PasswordResetCode, ResetRequestType, ResetRequestStatus, Notification, NotificationType};
use crate::infra::errors::{AppError, AppResult};
use crate::infra::repositories::{PasswordResetRepository, UserRepository, NotificationRepository};
use crate::infra::email::SmtpService;
use crate::auth::{generate_reset_code, hash_reset_code, verify_reset_code, hash_password, validate_password};
use crate::utils::{Id, generate_id};

#[async_trait]
pub trait PasswordResetService: Send + Sync {
    /// Request password reset via email (generates and emails code)
    async fn request_reset_via_email(&self, email: &str) -> AppResult<()>;
    
    /// Confirm reset code and change password (email-based flow)
    async fn confirm_reset_and_change_password(&self, email: &str, code: &str, new_password: &str) -> AppResult<()>;
    
    /// Change password in active session (requires old password)
    async fn change_password_authenticated(&self, user_id: Id, old_password: &str, new_password: &str) -> AppResult<()>;
    
    /// Request password reset via teacher approval (no email access flow)
    async fn request_reset_via_teacher(&self, email: &str) -> AppResult<()>;
    
    /// Teacher: get all pending password reset requests
    async fn get_pending_teacher_requests(&self) -> AppResult<Vec<(PasswordResetCode, String, String)>>;
    
    /// Teacher: approve request and set new password for student
    async fn teacher_approve_and_set_password(&self, request_id: Id, teacher_id: Id, new_password: &str, notes: Option<String>) -> AppResult<()>;
    
    /// Teacher: reject password reset request
    async fn teacher_reject_request(&self, request_id: Id, teacher_id: Id, reason: String) -> AppResult<()>;
    
    /// Teacher: directly set student password (without request)
    async fn teacher_set_student_password(&self, student_id: Id, teacher_id: Id, new_password: &str) -> AppResult<()>;
    
    /// Cleanup: expire old codes (run periodically)
    async fn expire_old_codes(&self) -> AppResult<u64>;
}

pub struct PasswordResetServiceImpl {
    reset_repo: Arc<dyn PasswordResetRepository>,
    user_repo: Arc<dyn UserRepository>,
    notification_repo: Arc<dyn NotificationRepository>,
    email_service: Arc<SmtpService>,
}

impl PasswordResetServiceImpl {
    pub fn new(
        reset_repo: Arc<dyn PasswordResetRepository>,
        user_repo: Arc<dyn UserRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
        email_service: Arc<SmtpService>,
    ) -> Self {
        Self {
            reset_repo,
            user_repo,
            notification_repo,
            email_service,
        }
    }
}

#[async_trait]
impl PasswordResetService for PasswordResetServiceImpl {
    async fn request_reset_via_email(&self, email: &str) -> AppResult<()> {
        // Find user
        let (user, _) = self.user_repo
            .find_by_email(email)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))?;
        
        // Check if there's already a pending request
        if let Some(existing) = self.reset_repo
            .find_pending_by_user(user.id, ResetRequestType::Email)
            .await
            .map_err(AppError::Internal)?
        {
            // If existing request is still valid (not expired), reject new request
            if let Some(expires_at) = existing.expires_at {
                if expires_at > Utc::now() {
                    return Err(AppError::Validation(
                        "A password reset code was already sent. Please check your email or wait before requesting another.".into()
                    ));
                }
            }
        }
        
        // Generate reset code (6-digit)
        let code = generate_reset_code();
        let code_hash = hash_reset_code(&code)?;
        
        // Create reset record (expires in 15 minutes)
        let reset = PasswordResetCode {
            id: generate_id(),
            user_id: user.id,
            code_hash: Some(code_hash),
            request_type: ResetRequestType::Email,
            status: ResetRequestStatus::Pending,
            expires_at: Some(Utc::now() + Duration::minutes(15)),
            created_at: Utc::now(),
            reviewed_by: None,
            reviewed_at: None,
            notes: None,
        };
        
        self.reset_repo
            .insert(&reset)
            .await
            .map_err(AppError::Internal)?;
        
        // Send email with code
        if let Err(e) = self.email_service
            .send_password_reset_code(&user.email, &user.name, &code)
            .await
        {
            tracing::error!("Failed to send password reset email: {:?}", e);
            // Don't fail the request if email fails (dev env might not have SMTP configured)
            tracing::warn!("Password reset code not sent via email (SMTP not configured or failed). Code: {}", code);
        }
        
        Ok(())
    }
    
    async fn confirm_reset_and_change_password(&self, email: &str, code: &str, new_password: &str) -> AppResult<()> {
        tracing::info!("Confirming password reset for email: {}", email);
        
        // Validate new password
        if let Err(e) = validate_password(new_password) {
            tracing::warn!("Password validation failed: {:?}", e);
            return Err(e);
        }
        
        // Find user
        let (user, _) = self.user_repo
            .find_by_email(email)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| {
                tracing::warn!("User not found for email: {}", email);
                AppError::NotFound(format!("User with email {} not found", email))
            })?;
        
        // Find pending reset request
        let reset = self.reset_repo
            .find_pending_by_user(user.id, ResetRequestType::Email)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| {
                tracing::warn!("No pending password reset request found for user: {}", user.id);
                AppError::Validation("No pending password reset request found".into())
            })?;
        
        tracing::info!("Found pending reset request for user {}, checking expiry", user.id);
        
        // Check if expired
        if let Some(expires_at) = reset.expires_at {
            if expires_at <= Utc::now() {
                tracing::warn!("Password reset code expired for user {}", user.id);
                self.reset_repo
                    .update_status(reset.id, ResetRequestStatus::Expired, None, None)
                    .await
                    .map_err(AppError::Internal)?;
                return Err(AppError::Validation("Password reset code has expired. Please request a new one.".into()));
            }
        }
        
        // Verify code
        let code_hash = reset.code_hash.as_ref()
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Reset code hash missing")))?;
        
        tracing::info!("Verifying reset code for user {}", user.id);
        if !verify_reset_code(code_hash, code)? {
            tracing::warn!("Invalid verification code for user {}", user.id);
            return Err(AppError::Validation("Invalid verification code".into()));
        }
        
        tracing::info!("Code verified successfully for user {}", user.id);
        
        // Hash new password and update
        let password_hash = hash_password(new_password)?;
        let updated = self.user_repo
            .update_password(user.id, &password_hash)
            .await
            .map_err(AppError::Internal)?;
        
        if !updated {
            return Err(AppError::Internal(anyhow::anyhow!("Failed to update password")));
        }
        
        // Mark reset as used
        self.reset_repo
            .update_status(reset.id, ResetRequestStatus::Used, None, None)
            .await
            .map_err(AppError::Internal)?;
        
        Ok(())
    }
    
    async fn change_password_authenticated(&self, user_id: Id, old_password: &str, new_password: &str) -> AppResult<()> {
        // Validate new password
        validate_password(new_password)?;
        
        // Get user with current password hash
        let user = self.user_repo
            .find_by_id(user_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        
        let (_user, current_hash) = self.user_repo
            .find_by_email(&user.email)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        
        // Verify old password
        if !crate::auth::verify_password(&current_hash, old_password)? {
            return Err(AppError::Validation("Current password is incorrect".into()));
        }
        
        // Hash new password and update
        let password_hash = hash_password(new_password)?;
        let updated = self.user_repo
            .update_password(user_id, &password_hash)
            .await
            .map_err(AppError::Internal)?;
        
        if !updated {
            return Err(AppError::Internal(anyhow::anyhow!("Failed to update password")));
        }
        
        Ok(())
    }
    
    async fn request_reset_via_teacher(&self, email: &str) -> AppResult<()> {
        // Find user
        let (user, _) = self.user_repo
            .find_by_email(email)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))?;
        
        // Check if there's already a pending teacher request
        if let Some(_existing) = self.reset_repo
            .find_pending_by_user(user.id, ResetRequestType::TeacherRequest)
            .await
            .map_err(AppError::Internal)?
        {
            return Err(AppError::Validation(
                "You already have a pending password reset request. Please wait for teacher approval.".into()
            ));
        }
        
        // Create teacher request (no code, no expiry)
        let reset = PasswordResetCode {
            id: generate_id(),
            user_id: user.id,
            code_hash: None,
            request_type: ResetRequestType::TeacherRequest,
            status: ResetRequestStatus::Pending,
            expires_at: None,
            created_at: Utc::now(),
            reviewed_by: None,
            reviewed_at: None,
            notes: None,
        };
        
        self.reset_repo
            .insert(&reset)
            .await
            .map_err(AppError::Internal)?;
        
        // Notify all teachers via in-app notification
        let teachers = self.user_repo
            .find_all()
            .await
            .map_err(AppError::Internal)?
            .into_iter()
            .filter(|u| u.global_role == crate::domain::GlobalRole::Teacher)
            .collect::<Vec<_>>();
        
        for teacher in teachers {
            let notification = Notification {
                id: generate_id(),
                user_id: teacher.id,
                notification_type: NotificationType::TaskSubmitted, // Reusing for now (можно добавить PasswordResetRequested)
                title: format!("Password Reset Request: {}", user.name),
                message: Some(format!("Student {} ({}) has requested a password reset without email access.", user.name, user.email)),
                task_id: None,
                project_id: None,
                is_read: false,
                created_at: Utc::now(),
            };
            
            let _ = self.notification_repo.insert(&notification).await; // Don't fail if notification fails
        }
        
        Ok(())
    }
    
    async fn get_pending_teacher_requests(&self) -> AppResult<Vec<(PasswordResetCode, String, String)>> {
        self.reset_repo
            .find_all_teacher_requests_pending()
            .await
            .map_err(AppError::Internal)
    }
    
    async fn teacher_approve_and_set_password(&self, request_id: Id, teacher_id: Id, new_password: &str, notes: Option<String>) -> AppResult<()> {
        // Validate new password
        validate_password(new_password)?;
        
        // Find request
        let reset = self.reset_repo
            .find_by_id(request_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("Password reset request not found".into()))?;
        
        if reset.status != ResetRequestStatus::Pending {
            return Err(AppError::Validation("This request has already been processed".into()));
        }
        
        // Hash new password and update user
        let password_hash = hash_password(new_password)?;
        let updated = self.user_repo
            .update_password(reset.user_id, &password_hash)
            .await
            .map_err(AppError::Internal)?;
        
        if !updated {
            return Err(AppError::Internal(anyhow::anyhow!("Failed to update password")));
        }
        
        // Mark request as approved
        self.reset_repo
            .update_status(reset.id, ResetRequestStatus::Approved, Some(teacher_id), notes.as_deref())
            .await
            .map_err(AppError::Internal)?;
        
        // Notify student
        let user = self.user_repo
            .find_by_id(reset.user_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        
        let notification = Notification {
            id: generate_id(),
            user_id: reset.user_id,
            notification_type: NotificationType::TaskAccepted, // Reusing
            title: "Password Reset Approved".to_string(),
            message: Some("Your password has been reset by a teacher. You can now login with your new password.".to_string()),
            task_id: None,
            project_id: None,
            is_read: false,
            created_at: Utc::now(),
        };
        
        let _ = self.notification_repo.insert(&notification).await;
        
        Ok(())
    }
    
    async fn teacher_reject_request(&self, request_id: Id, teacher_id: Id, reason: String) -> AppResult<()> {
        // Find request
        let reset = self.reset_repo
            .find_by_id(request_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("Password reset request not found".into()))?;
        
        if reset.status != ResetRequestStatus::Pending {
            return Err(AppError::Validation("This request has already been processed".into()));
        }
        
        // Mark request as rejected
        self.reset_repo
            .update_status(reset.id, ResetRequestStatus::Rejected, Some(teacher_id), Some(&reason))
            .await
            .map_err(AppError::Internal)?;
        
        // Notify student
        let notification = Notification {
            id: generate_id(),
            user_id: reset.user_id,
            notification_type: NotificationType::TaskRejected, // Reusing
            title: "Password Reset Request Rejected".to_string(),
            message: Some(format!("Your password reset request was rejected. Reason: {}", reason)),
            task_id: None,
            project_id: None,
            is_read: false,
            created_at: Utc::now(),
        };
        
        let _ = self.notification_repo.insert(&notification).await;
        
        Ok(())
    }
    
    async fn teacher_set_student_password(&self, student_id: Id, _teacher_id: Id, new_password: &str) -> AppResult<()> {
        // Validate new password
        validate_password(new_password)?;
        
        // Verify student exists
        let _student = self.user_repo
            .find_by_id(student_id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound("Student not found".into()))?;
        
        // Hash new password and update
        let password_hash = hash_password(new_password)?;
        let updated = self.user_repo
            .update_password(student_id, &password_hash)
            .await
            .map_err(AppError::Internal)?;
        
        if !updated {
            return Err(AppError::Internal(anyhow::anyhow!("Failed to update password")));
        }
        
        // Notify student
        let notification = Notification {
            id: generate_id(),
            user_id: student_id,
            notification_type: NotificationType::TaskAccepted, // Reusing
            title: "Password Changed by Teacher".to_string(),
            message: Some("Your password has been changed by a teacher. Please use your new password to login.".to_string()),
            task_id: None,
            project_id: None,
            is_read: false,
            created_at: Utc::now(),
        };
        
        let _ = self.notification_repo.insert(&notification).await;
        
        Ok(())
    }
    
    async fn expire_old_codes(&self) -> AppResult<u64> {
        self.reset_repo
            .expire_old_codes()
            .await
            .map_err(AppError::Internal)
    }
}


