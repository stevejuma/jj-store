use super::*;
pub(super) async fn import(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
    body: std::result::Result<Json<capture::CaptureImportCommand>, JsonRejection>,
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
    state.json(&manager.import(actor, command).await.map_err(failure)?)
}
pub(super) async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<capture::CaptureImportQuery>,
) -> Result<Response> {
    let query = routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    state.json(&manager.import_status(actor, query).await.map_err(failure)?)
}
