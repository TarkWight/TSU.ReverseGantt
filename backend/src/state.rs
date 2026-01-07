use std::sync::Arc;

use crate::config::Config;
use crate::infra::db::create_pool;
use crate::infra::email::SmtpService;
use crate::infra::repositories::{
    PgProjectRepository, PgMembershipRepository, PgUserRepository,
    PgTaskRepository, PgAssignmentRepository, PgDependencyRepository,
    PgReviewRepository, PgArtifactRepository, PgNotificationRepository,
    PgScheduleRepository,
    ProjectRepository, MembershipRepository, UserRepository,
    TaskRepository, AssignmentRepository, DependencyRepository,
    ReviewRepository, ArtifactRepository, NotificationRepository,
    ScheduleRepository,
};
use crate::services::{
    ProjectService, ProjectServiceImpl,
    TaskService, TaskServiceImpl,
    DependencyService, DependencyServiceImpl,
    ScheduleService, ScheduleServiceImpl,
    ReviewService, ReviewServiceImpl,
    UserService, UserServiceImpl,
    MembershipService, MembershipServiceImpl,
    AssignmentService, AssignmentServiceImpl,
    ArtifactService, ArtifactServiceImpl,
    NotificationService, NotificationServiceImpl,
};

#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<dyn ProjectService>,
    pub task_service: Arc<dyn TaskService>,
    pub dependency_service: Arc<dyn DependencyService>,
    pub schedule_service: Arc<dyn ScheduleService>,
    pub review_service: Arc<dyn ReviewService>,
    pub user_service: Arc<dyn UserService>,
    pub membership_service: Arc<dyn MembershipService>,
    pub assignment_service: Arc<dyn AssignmentService>,
    pub artifact_service: Arc<dyn ArtifactService>,
    pub notification_service: Arc<dyn NotificationService>,
    pub email_service: Arc<SmtpService>,
}

impl AppState {
    pub async fn init_with(cfg: Config) -> anyhow::Result<Self> {
        let pool = create_pool(&cfg.database_url).await?;

        // Repositories
        let project_repo: Arc<dyn ProjectRepository> = Arc::new(PgProjectRepository::new(pool.clone()));
        let membership_repo: Arc<dyn MembershipRepository> = Arc::new(PgMembershipRepository::new(pool.clone()));
        let user_repo: Arc<dyn UserRepository> = Arc::new(PgUserRepository::new(pool.clone()));
        let task_repo: Arc<dyn TaskRepository> = Arc::new(PgTaskRepository::new(pool.clone()));
        let assignment_repo: Arc<dyn AssignmentRepository> = Arc::new(PgAssignmentRepository::new(pool.clone()));
        let dependency_repo: Arc<dyn DependencyRepository> = Arc::new(PgDependencyRepository::new(pool.clone()));
        let review_repo: Arc<dyn ReviewRepository> = Arc::new(PgReviewRepository::new(pool.clone()));
        let artifact_repo: Arc<dyn ArtifactRepository> = Arc::new(PgArtifactRepository::new(pool.clone()));
        let notification_repo: Arc<dyn NotificationRepository> = Arc::new(PgNotificationRepository::new(pool.clone()));
        let schedule_repo: Arc<dyn ScheduleRepository> = Arc::new(PgScheduleRepository::new(pool.clone()));

        // Email service
        let email_service = Arc::new(SmtpService::new());

        // Services using repositories
        let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo.clone()));
        let membership_service: Arc<dyn MembershipService> = Arc::new(MembershipServiceImpl::new(membership_repo.clone()));
        let assignment_service: Arc<dyn AssignmentService> = Arc::new(AssignmentServiceImpl::new(assignment_repo.clone()));
        let dependency_service: Arc<dyn DependencyService> = Arc::new(DependencyServiceImpl::new(dependency_repo.clone()));
        let review_service: Arc<dyn ReviewService> = Arc::new(ReviewServiceImpl::new(review_repo.clone()));
        let artifact_service: Arc<dyn ArtifactService> = Arc::new(ArtifactServiceImpl::new(artifact_repo.clone()));
        let notification_service: Arc<dyn NotificationService> = Arc::new(NotificationServiceImpl::new(notification_repo.clone()));
        let schedule_service: Arc<dyn ScheduleService> = Arc::new(ScheduleServiceImpl::new(schedule_repo.clone()));

        let project_service: Arc<dyn ProjectService> = Arc::new(ProjectServiceImpl::new(
            project_repo.clone(),
            membership_repo.clone(),
            user_repo.clone(),
        ));

        let task_service: Arc<dyn TaskService> = Arc::new(TaskServiceImpl::new(
            task_repo.clone(),
            membership_repo.clone(),
            assignment_repo.clone(),
            user_repo.clone(),
            notification_service.clone(),
            review_service.clone(),
            email_service.clone(),
        ));

        Ok(Self {
            project_service,
            task_service,
            dependency_service,
            schedule_service,
            review_service,
            user_service,
            membership_service,
            assignment_service,
            artifact_service,
            notification_service,
            email_service,
        })
    }
}
