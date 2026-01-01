pub async fn create_artifact(
    auth: AuthContext,
    Path(task_id): Path<String>,
    State(state): State<AppState>,
    Json(req): Json<CreateArtifactRequest>,
) -> AppResult<Json<ArtifactResponse>> {
    let tid = parse_id(&task_id)?;
    ensure_can_edit_task(&auth, tid, &state).await?;

    let now = chrono::Utc::now();
    let artifact = Artifact {
        id: generate_id(),
        task_id: tid,
        name: req.name,
        uri: req.uri,
        kind: req.kind,
        created_at: now,
        updated_at: now,
    };

    let created = state.artifact_service.create(artifact).await?;
    Ok(Json(created.into()))
}

pub async fn delete_artifact(
    auth: AuthContext,
    Path((task_id, artifact_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> AppResult<()> {
    let tid = parse_id(&task_id)?;
    let aid = parse_id(&artifact_id)?;
    ensure_can_edit_task(&auth, tid, &state).await?;

    state.artifact_service.delete(tid, aid).await?;
    Ok(())
}

