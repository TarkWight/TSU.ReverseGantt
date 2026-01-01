pub use crate::api::models::{
    LoginResponse,
    UserResponse,
};
pub use crate::api::requests::{
    LoginRequest,
    RegisterRequest,
    UpdateEmailNotificationsRequest.
};
pub use crate::api::handlers::users::{
    login,
    register,
    get_user,
    get_all_users,
    promote_to_teacher,
    update_email_notifications,
};