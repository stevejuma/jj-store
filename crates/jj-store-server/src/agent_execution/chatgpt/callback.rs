//! Short-lived callback admission independent of application HTTP routes.
use super::*;
use axum::{
    extract::{RawQuery, State as ExtractState},
    response::Html,
};
pub(super) struct Callback {
    pub state: String,
    pub sender: Mutex<Option<tokio::sync::oneshot::Sender<BTreeMap<String, String>>>>,
}
pub(super) async fn receive(
    ExtractState(state): ExtractState<Arc<Callback>>,
    RawQuery(query): RawQuery,
) -> (axum::http::StatusCode, Html<&'static str>) {
    let Some(query) = query.filter(|q| q.len() <= 8192) else {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Html("Invalid authorization callback"),
        );
    };
    let mut fields = BTreeMap::new();
    for (key, value) in url::form_urlencoded::parse(query.as_bytes()) {
        if fields
            .insert(key.into_owned(), value.into_owned())
            .is_some()
        {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Html("Invalid authorization callback"),
            );
        }
    }
    if fields.get("state") != Some(&state.state) {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Html("Invalid authorization callback"),
        );
    }
    if let Ok(mut sender) = state.sender.lock() {
        if let Some(sender) = sender.take() {
            let _ = sender.send(fields);
            return (
                axum::http::StatusCode::OK,
                Html("Authorization received. Return to JJ Store to see the result."),
            );
        }
    }
    (
        axum::http::StatusCode::BAD_REQUEST,
        Html("Authorization attempt has ended"),
    )
}
