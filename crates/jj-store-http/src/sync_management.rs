use crate::{
    routes::{Empty, Input, input},
    *,
};
use axum::Json;
use jj_store_service::sync_management::{self as sync, SyncCommand, SyncManager};

async fn manager<'a>(state: &'a State, context: &Context) -> Result<&'a Arc<dyn SyncManager>> {
    crate::local_management::admit(state, context).await?;
    state
        .config
        .local_sync
        .as_ref()
        .ok_or_else(Error::unavailable)
}
fn failure(error: sync::Error) -> Error {
    let (status, code) = match error {
        sync::Error::NotConfigured => (StatusCode::CONFLICT, "sync_not_configured"),
        sync::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_sync_command"),
        sync::Error::Unavailable => (StatusCode::SERVICE_UNAVAILABLE, "sync_unavailable"),
    };
    Error::new(status, code)
}
pub(crate) async fn local_sync_status(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    state.json(&manager.status(context.id("repo")?).await.map_err(failure)?)
}
pub(crate) async fn local_sync_control(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<Json<SyncCommand>, axum::extract::rejection::JsonRejection>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(command) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .control(context.id("repo")?, command)
            .await
            .map_err(failure)?,
    )
}
