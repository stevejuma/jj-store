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

pub(crate) async fn local_sync_barrier(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<Json<Empty>, axum::extract::rejection::JsonRejection>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(_) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .capture(context.id("repo")?)
            .await
            .map_err(failure)?,
    )
}
pub(crate) async fn local_sync_wait(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<
        Json<jj_store_service::replication_control::WaitRequestV1>,
        axum::extract::rejection::JsonRejection,
    >,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(mut request) = body.map_err(|_| Error::invalid())?;
    if !(1..=1000).contains(&request.timeout_millis) {
        return Err(Error::invalid());
    }
    request.timeout_millis = request
        .timeout_millis
        .min((state.config.timeout.as_millis() / 2).max(1) as u64);
    state.json(
        &manager
            .wait(context.id("repo")?, request)
            .await
            .map_err(failure)?,
    )
}
