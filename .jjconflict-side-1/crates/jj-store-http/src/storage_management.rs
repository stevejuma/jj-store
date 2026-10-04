use crate::{
    routes::{Empty, Input, input},
    *,
};
use axum::Json;
use jj_store_service::storage_management::{self as storage, StorageCommand, StorageManager};
async fn manager<'a>(state: &'a State, context: &Context) -> Result<&'a Arc<dyn StorageManager>> {
    crate::local_management::admit(state, context).await?;
    state
        .config
        .local_storage
        .as_ref()
        .ok_or_else(Error::unavailable)
}
fn failure(error: storage::Error) -> Error {
    let (status, code) = match error {
        storage::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_storage_command"),
        storage::Error::Conflict => (StatusCode::CONFLICT, "storage_intent_conflict"),
        storage::Error::Busy => (StatusCode::CONFLICT, "storage_operation_pending"),
        storage::Error::NotFound => (StatusCode::NOT_FOUND, "storage_operation_not_found"),
        storage::Error::Limit => (StatusCode::CONFLICT, "storage_journal_full"),
        _ => (StatusCode::SERVICE_UNAVAILABLE, "storage_unavailable"),
    };
    Error::new(status, code)
}
pub(crate) async fn local_storage_overview(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
) -> Result<Response> {
    input(query)?;
    state.json(
        &manager(&state, &context)
            .await?
            .overview(context.id("repo")?)
            .await
            .map_err(failure)?,
    )
}
pub(crate) async fn local_storage_operation(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
) -> Result<Response> {
    input(query)?;
    state.json(
        &manager(&state, &context)
            .await?
            .operation(context.id("repo")?, context.id("request_id")?)
            .await
            .map_err(failure)?,
    )
}
pub(crate) async fn local_storage_command(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<Json<StorageCommand>, axum::extract::rejection::JsonRejection>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(command) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .command(context.id("repo")?, command)
            .await
            .map_err(failure)?,
    )
}
