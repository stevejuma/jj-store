//! Multiplexed scoped freshness with process-local replay and current authorization.
use crate::*;
use axum::{
    response::sse::{Event as SseEvent, Sse},
    routing::get,
};
use jj_store_service::freshness::{Change, Domain};
use schemars::JsonSchema;
use std::collections::{BTreeMap, VecDeque};
use tokio::time::Instant;
mod authorization;
mod code_indexes;
mod monitors;
mod producers;
mod subscription;
pub(crate) mod websocket;
pub(crate) use code_indexes::resume as resume_code_indexes;
pub(super) use monitors::Monitors;
use subscription::{Delivery, Subscription};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Query {
    after: Option<String>,
}
#[derive(Serialize, JsonSchema)]
#[schemars(rename = "ApplicationFreshnessEvent")]
#[serde(tag = "type", rename_all = "snake_case")]
enum Event {
    Ready {
        version: u32,
        witnesses: Vec<Change>,
    },
    Changed {
        version: u32,
        change: Change,
    },
    Reset {
        version: u32,
        reason: &'static str,
        witnesses: Vec<Change>,
    },
}
pub(super) fn router() -> Router<Arc<State>> {
    Router::new()
        .route("/events", get(events))
        .route("/events/ws", get(websocket::upgrade))
}
pub(super) fn document(document: &mut contract::Document) {
    document.endpoint::<Query, Event>(
        "get",
        "/events",
        "application_events",
        contract::Reply::ApplicationEvents,
    );
    websocket::document(document);
}
async fn events(
    ExtractState(state): ExtractState<Arc<State>>,
    args: routes::Input<Query>,
    headers: HeaderMap,
) -> Result<Response> {
    let query = routes::input(args)?;
    if headers.get_all("last-event-id").iter().count() > 1 {
        return Err(Error::invalid());
    }
    let header = headers
        .get("last-event-id")
        .map(|h| h.to_str().map_err(|_| Error::invalid()))
        .transpose()?;
    if query
        .after
        .as_deref()
        .zip(header)
        .is_some_and(|(a, b)| a != b)
    {
        return Err(Error::invalid());
    }
    let subscription =
        Subscription::open(state, &headers, query.after.as_deref().or(header)).await?;
    let stream = futures::stream::try_unfold(subscription, |mut s| async move {
        let frame = s
            .next()
            .await
            .map_err(|_| std::io::Error::other("freshness unavailable"))?;
        let event = match frame {
            Some(Delivery::Heartbeat) => SseEvent::default().comment("keep-alive"),
            Some(Delivery::Event(frame)) => {
                let name = match &frame.event {
                    Event::Ready { .. } => "ready",
                    Event::Changed { .. } => "changed",
                    Event::Reset { .. } => "reset",
                };
                SseEvent::default()
                    .id(frame.id)
                    .event(name)
                    .json_data(frame.event)
                    .map_err(std::io::Error::other)?
            }
            None => return Ok(None),
        };
        Ok::<_, std::io::Error>(Some((event, s)))
    });
    let mut response = Sse::new(stream).into_response();
    response
        .headers_mut()
        .insert("x-accel-buffering", "no".parse().unwrap());
    Ok(response)
}

#[cfg(test)]
mod tests;
