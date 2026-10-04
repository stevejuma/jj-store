use crate::*;
use axum::{Json, extract::DefaultBodyLimit, routing::get};
use jj_store_service::server_settings as settings;
fn manager(state: &State) -> Result<(&Arc<dyn settings::ServerSettingsManager>, m::PrincipalId)> {
    let Access::LocalTrusted {
        principal,
        permission: Permission::Admin,
    } = state.config.access
    else {
        return Err(Error::denied());
    };
    Ok((
        state
            .config
            .local_settings
            .as_ref()
            .ok_or_else(Error::unavailable)?,
        principal,
    ))
}
fn failure(error: settings::Error) -> Error {
    let (status, code) = match error {
        settings::Error::Denied => (StatusCode::FORBIDDEN, "server_settings_denied"),
        settings::Error::Invalid => (StatusCode::BAD_REQUEST, "invalid_server_settings"),
        settings::Error::Stale => (StatusCode::CONFLICT, "server_settings_changed"),
        settings::Error::Busy => (StatusCode::CONFLICT, "server_settings_busy"),
        settings::Error::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "server_settings_unavailable",
        ),
        settings::Error::Unknown => (
            StatusCode::SERVICE_UNAVAILABLE,
            "server_settings_outcome_unknown",
        ),
    };
    Error::new(status, code)
}
pub(crate) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/local/settings", get(status).post(save))
        .layer(DefaultBodyLimit::max(144 * 1024))
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    state.json(&manager.status(actor).await.map_err(failure)?)
}
async fn save(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
    body: std::result::Result<
        Json<settings::SaveServerSettings>,
        axum::extract::rejection::JsonRejection,
    >,
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
    state.json(&manager.save(actor, command).await.map_err(failure)?)
}
pub(crate) fn document(document: &mut crate::contract::Document) {
    document.local_endpoint::<routes::Empty, settings::LocalServerSettings>(
        "get",
        "/local/settings",
        "local_server_settings",
    );
    document.local_endpoint::<routes::Empty, settings::LocalServerSettings>(
        "post",
        "/local/settings",
        "local_server_save_settings",
    );
    document.body::<settings::SaveServerSettings>("/v1/local/settings", "post");
}
