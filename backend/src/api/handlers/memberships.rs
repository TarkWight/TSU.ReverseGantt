use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::infra::errors::{
    AppResult,
    AppError,
};
use crate::utils::{
    generate_id,
    parse_id,
};
use crate::auth::AuthContext;
use crate::auth::permissions::{
    ensure_project_access,
    ensure_can_manage_members,
};
use crate::domain::{
    Membership,
    GlobalRole,
};
use crate::state::AppState;
use crate::api::requests::{
    CreateMembershipRequest,
    UpdateMembershipTagsRequest,
    ChangeProjectLeaderRequest,
};
use crate::api::models::MembershipResponse;

pub async fn get_project_memberships(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MembershipResponse>>> {
    let project_id = parse_id(&project_id)?;
    ensure_project_access(&auth, project_id, &state)
        .await?;

    let memberships = state.membership_service
        .get_by_project(project_id)
        .await?;
    Ok(Json(
        memberships
        .into_iter()
        .map(Into::into)
        .collect()
        )
    )
}

pub async fn get_user_memberships(
    auth: AuthContext,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<MembershipResponse>>> {
    let memberships = state.membership_service
        .get_by_user(auth.user_id)
        .await?;
    Ok(Json(memberships
        .into_iter()
        .map(Into::into)
        .collect()))
}

pub async fn create_membership(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateMembershipRequest>,
) -> AppResult<impl IntoResponse> {
    let project_id = parse_id(&project_id)?;
    ensure_can_manage_members(&auth, project_id, &state).await?;

    let user_id = parse_id(&req.user_id)?;

    let membership = Membership {
        id: generate_id(),
        project_id,
        user_id,
        is_leader: false,
        tags: req.tags.unwrap_or_default(),
    };

    let created = state.membership_service
        .create(membership)
        .await?;
    Ok((StatusCode::CREATED, Json(MembershipResponse::from(created))))
}

pub async fn update_membership_tags(
    auth: AuthContext,
    Path(membership_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<UpdateMembershipTagsRequest>,
) -> AppResult<Json<MembershipResponse>> {
    let membership_id = parse_id(&membership_id)?;
    let existing_membership = state.membership_service.get_by_id(membership_id).await?;
    ensure_can_manage_members(&auth, existing_membership.project_id, &state).await?;

    let membership = state.membership_service.update_tags(membership_id, req.tags).await?;
    Ok(Json(MembershipResponse::from(membership)))
}

pub async fn change_project_leader(
    auth: AuthContext,
    Path(project_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<ChangeProjectLeaderRequest>,
) -> AppResult<Json<MembershipResponse>> {
    let project_id = parse_id(&project_id)?;
    ensure_can_manage_members(&auth, project_id, &state).await?;

    let new_leader_id = parse_id(&req.new_leader_id)?;

    let new_leader_user = state.user_service.get_by_id(new_leader_id).await?;
    if new_leader_user.global_role != GlobalRole::Student {
        return Err(AppError::Validation(
            "New leader must be a Student, not a Teacher"
                .to_string())
        );
    }

    let new_leader_membership = state
        .membership_service
        .get_by_project_and_user(project_id, new_leader_id)
        .await?;

    let current_leader = state.membership_service.get_leader(project_id)
        .await?
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Project has no leader")))?;

    state.membership_service.update_leader_status(current_leader.id, false)
        .await?;
    let updated = state.membership_service.update_leader_status(new_leader_membership.id, true)
        .await?;

    Ok(Json(MembershipResponse::from(updated)))
}

pub async fn delete_membership(
    auth: AuthContext,
    Path(membership_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let membership_id = parse_id(&membership_id)?;
    let existing_membership = state.membership_service.get_by_id(membership_id)
        .await?;
    ensure_can_manage_members(&auth, existing_membership.project_id, &state)
        .await?;

    if existing_membership.is_leader {
        let memberships = state.membership_service.get_by_project(existing_membership.project_id)
            .await?;
        let leader_count = memberships
            .iter()
            .filter(|m| m.is_leader)
            .count();
        if leader_count <= 1 {
            return Err(AppError::Validation("Cannot remove the only project leader".to_string()));
        }
    }

    state.membership_service.delete(membership_id).await?;
    Ok(StatusCode::NO_CONTENT)
}