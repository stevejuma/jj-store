use crate::{
    routes::{Empty, Input, input},
    *,
};
use axum::Json;
use jj_store_service::search_management::{self as settings, SearchManager};
async fn manager<'a>(state: &'a State, context: &Context) -> Result<&'a Arc<dyn SearchManager>> {
    crate::local_management::admit(state, context).await?;
    state
        .config
        .local_search
        .as_ref()
        .ok_or_else(Error::unavailable)
}
fn failure(error: settings::Error) -> Error {
    let (status, code) = match error {
        settings::Error::Conflict => (StatusCode::CONFLICT, "search_settings_changed"),
        settings::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_search_settings"),
        settings::Error::Busy => (StatusCode::CONFLICT, "search_operation_in_progress"),
        settings::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "search_management_unavailable",
        ),
        settings::Error::Unknown => (
            StatusCode::SERVICE_UNAVAILABLE,
            "search_save_outcome_unknown",
        ),
    };
    Error::new(status, code)
}
pub(crate) async fn local_search_settings(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    state.json(&manager.status(context.id("repo")?).await.map_err(failure)?)
}
pub(crate) async fn save_search_settings(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<
        Json<settings::SaveSearchSettings>,
        axum::extract::rejection::JsonRejection,
    >,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(body) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .save(context.id("repo")?, body)
            .await
            .map_err(failure)?,
    )
}
pub(crate) async fn control_search(
    ExtractState(state): ExtractState<Arc<State>>,
    context: Context,
    query: Input<Empty>,
    body: std::result::Result<
        Json<settings::SearchCommand>,
        axum::extract::rejection::JsonRejection,
    >,
) -> Result<Response> {
    input(query)?;
    let manager = manager(&state, &context).await?;
    let Json(body) = body.map_err(|_| Error::invalid())?;
    state.json(
        &manager
            .control(context.id("repo")?, body)
            .await
            .map_err(failure)?,
    )
}
