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

    let created = state.membership_service.create(membership).await?;
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

pub async fn delete_membership(
    auth: AuthContext,
    Path(membership_id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<StatusCode> {
    let membership_id = parse_id(&membership_id)?;
    let existing_membership = state.membership_service.get_by_id(membership_id).await?;
    ensure_can_manage_members(&auth, existing_membership.project_id, &state).await?;

    if existing_membership.is_leader {
        let memberships = state.membership_service.get_by_project(existing_membership.project_id).await?;
        let leader_count = memberships.iter().filter(|m| m.is_leader).count();
        if leader_count <= 1 {
            return Err(AppError::Validation("Cannot remove the only project leader".to_string()));
        }
    }

    state.membership_service.delete(membership_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
