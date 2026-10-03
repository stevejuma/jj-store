use crate::*;
use axum::{
    Json,
    extract::{DefaultBodyLimit, rejection::JsonRejection},
    routing::{get, post},
};
use jj_store_service::capture_management::{
    self as capture, CaptureControlCommand, CaptureManager, CaptureRescanCommand,
    CaptureSourceQuery,
};
mod imports;

macro_rules! endpoints {
    ($register:ident) => { $register! {
        (get, "/local/capture", status, "local_capture_status", routes::Empty, routes::Empty, capture::CaptureStatus),
        (get, "/local/capture/sources", sources, "local_capture_sources", CaptureSourceQuery, routes::Empty, capture::CaptureSourcePage),
        (post, "/local/capture/control", control, "local_capture_control", routes::Empty, CaptureControlCommand, capture::CaptureControlOutcome),
        (post, "/local/capture/rescan", rescan, "local_capture_rescan", routes::Empty, CaptureRescanCommand, capture::CaptureRescanOutcome),
        (post, "/local/capture/import", imports::import, "local_capture_import", routes::Empty, capture::CaptureImportCommand, capture::CaptureImportOutcome),
        (get, "/local/capture/imports", imports::status, "local_capture_import_status", capture::CaptureImportQuery, routes::Empty, capture::CaptureImportStatus)
    } };
}
pub(crate) fn router() -> Router<Arc<State>> {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
            Router::new()$(.route($path,$method($handler)))*
        };
    }
    endpoints!(register).layer(DefaultBodyLimit::max(4096))
}
pub(crate) fn document(document: &mut crate::contract::Document) {
    macro_rules! register {
        ($(($method:ident,$path:literal,$handler:path,$id:literal,$query:ty,$body:ty,$response:ty)),*) => {
            $(document.local_endpoint::<$query,$response>(stringify!($method),$path,$id);
            if stringify!($method)=="post" { document.body::<$body>(&format!("/v1{}",$path),"post"); })*
        };
    }
    endpoints!(register);
}
fn manager(state: &State) -> Result<(&Arc<dyn CaptureManager>, m::PrincipalId)> {
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
            .local_capture
            .as_ref()
            .ok_or_else(Error::unavailable)?,
        principal,
    ))
}
fn failure(error: capture::Error) -> Error {
    use capture::Error as E;
    let (status, code) = match error {
        E::Denied => (StatusCode::FORBIDDEN, "capture_management_denied"),
        E::Disabled => (StatusCode::CONFLICT, "capture_disabled"),
        E::Paused => (StatusCode::CONFLICT, "capture_paused"),
        E::Invalid => (StatusCode::BAD_REQUEST, "invalid_capture_control"),
        E::Conflict => (StatusCode::CONFLICT, "capture_control_intent_conflict"),
        E::Stale => (StatusCode::CONFLICT, "capture_policy_changed"),
        E::SourceStale => (StatusCode::CONFLICT, "capture_source_selection_changed"),
        E::Busy => (StatusCode::CONFLICT, "capture_control_busy"),
        E::Capacity => (StatusCode::PAYLOAD_TOO_LARGE, "capture_control_capacity"),
        E::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "capture_management_unavailable",
        ),
        E::Unknown => (
            StatusCode::SERVICE_UNAVAILABLE,
            "capture_control_outcome_unknown",
        ),
    };
    Error::new(status, code)
}
async fn sources(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<CaptureSourceQuery>,
) -> Result<Response> {
    let query = routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    state.json(&manager.sources(actor, query).await.map_err(failure)?)
}
async fn rescan(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
    body: std::result::Result<Json<CaptureRescanCommand>, JsonRejection>,
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
    state.json(&manager.rescan(actor, command).await.map_err(failure)?)
}
async fn status(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
) -> Result<Response> {
    routes::input(query)?;
    let (manager, actor) = manager(&state)?;
    state.json(&manager.status(actor).await.map_err(failure)?)
}
async fn control(
    ExtractState(state): ExtractState<Arc<State>>,
    query: routes::Input<routes::Empty>,
    body: std::result::Result<Json<CaptureControlCommand>, JsonRejection>,
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
    state.json(&manager.control(actor, command).await.map_err(failure)?)
}
