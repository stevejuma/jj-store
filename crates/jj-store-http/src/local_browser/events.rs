//! Loss-tolerant freshness hints from bounded external JJ heads and companion checks.
use super::*;
use axum::{
    http::HeaderMap,
    response::sse::{Event as SseEvent, Sse},
};
#[derive(Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(rename = "LocalFreshnessEvent")]
enum Event {
    Ready { snapshot: Snapshot },
    Changed { snapshot: Snapshot },
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new().route("/local/repos/{repo}/events", get(events))
}
async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    Path(repository): Path<m::RepositoryId>,
    args: routes::Input<routes::Empty>,
    headers: HeaderMap,
) -> Result<Response> {
    routes::input(args)?;
    if headers.contains_key("last-event-id") {
        return Err(Error::invalid());
    }
    let first = capture(&state, repository).await?;
    let signature = serde_json::to_vec(&first).map_err(|_| Error::unavailable())?;
    let stream = futures::stream::try_unfold(
        (state, Some(first), signature),
        move |(state, first, mut previous)| async move {
            let frame = if let Some(snapshot) = first {
                SseEvent::default()
                    .event("ready")
                    .json_data(Event::Ready { snapshot })
                    .map_err(std::io::Error::other)?
            } else {
                tokio::time::sleep(
                    state
                        .config
                        .events
                        .poll_interval
                        .max(Duration::from_secs(1)),
                )
                .await;
                let snapshot = capture(&state, repository)
                    .await
                    .map_err(|_| std::io::Error::other("local freshness unavailable"))?;
                let bytes = serde_json::to_vec(&snapshot).map_err(std::io::Error::other)?;
                if bytes == previous {
                    SseEvent::default().comment("keep-alive")
                } else {
                    previous = bytes;
                    SseEvent::default()
                        .event("repository-changed")
                        .json_data(Event::Changed { snapshot })
                        .map_err(std::io::Error::other)?
                }
            };
            Ok::<_, std::io::Error>(Some((frame, (state, None, previous))))
        },
    );
    let mut response = Sse::new(stream).into_response();
    response
        .headers_mut()
        .insert("x-accel-buffering", "no".parse().unwrap());
    Ok(response)
}
pub(super) fn document(document: &mut crate::contract::Document) {
    document.local_events::<Event>("/local/repos/{repo}/events", "local_freshness");
}
