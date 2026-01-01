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