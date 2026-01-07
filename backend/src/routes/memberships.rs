use axum::{routing::{get, post, patch, delete}, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::state::AppState;
use crate::auth::AuthContext;
use crate::api::memberships::{
    MembershipResponse, CreateMembershipRequest, ChangeProjectLeaderRequest, UpdateMembershipTagsRequest,
    get_project_memberships, get_user_memberships, create_membership,
    change_project_leader, update_membership_tags, delete_membership,
};
use crate::infra::errors::AppResult;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/memberships/projects/{project_id}", get(get_project_memberships_handler).post(create_membership_handler))
        .route("/memberships/projects/{project_id}/leader", post(change_leader_handler))
        .route("/memberships/me", get(get_user_memberships_handler))
        .route("/memberships/{membership_id}/tags", patch(update_tags_handler))
        .route("/memberships/{membership_id}", delete(delete_membership_handler))
        .with_state(state)
}

async fn get_project_memberships_handler(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MembershipResponse>>> {
    get_project_memberships(auth, Path(project_id), State(state)).await
}

async fn create_membership_handler(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateMembershipRequest>,
) -> AppResult<impl axum::response::IntoResponse> {
    create_membership(auth, Path(project_id), State(state), Json(req)).await
}

async fn change_leader_handler(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<ChangeProjectLeaderRequest>,
) -> AppResult<Json<MembershipResponse>> {
    change_project_leader(auth, Path(project_id), State(state), Json(req)).await
}

async fn get_user_memberships_handler(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MembershipResponse>>> {
    get_user_memberships(auth, State(state)).await
}

async fn update_tags_handler(
    auth: AuthContext,
    Path(membership_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateMembershipTagsRequest>,
) -> AppResult<Json<MembershipResponse>> {
    update_membership_tags(auth, Path(membership_id), State(state), Json(req)).await
}

async fn delete_membership_handler(
    auth: AuthContext,
    Path(membership_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    delete_membership(auth, Path(membership_id), State(state)).await
}

