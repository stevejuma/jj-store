use super::*;
fn failure(error: capture::Error) -> Error {
    match error {
        capture::Error::Invalid => Error::new(StatusCode::BAD_REQUEST, "invalid_capture_settings"),
        capture::Error::Stale => Error::new(StatusCode::CONFLICT, "capture_settings_changed"),
        capture::Error::Unknown => Error::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "capture_settings_outcome_unknown",
        ),
        other => super::failure(other),
    }
}
pub(super) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    state.json(&manager.settings(actor).await.map_err(failure)?)
}
pub(super) async fn save(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
    body: std::result::Result<Json<capture::SaveCaptureSettings>, JsonRejection>,
) -> Result<Response> {
    routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    let Json(command) = body.map_err(|error| {
        if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
            Error::limit()
        } else {
            Error::invalid()
        }
    })?;
    state.json(
        &manager
            .save_settings(actor, command)
            .await
            .map_err(failure)?,
    )
}
