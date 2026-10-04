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
mod monitors;
mod producers;
pub(super) use monitors::Monitors;

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
    Router::new().route("/events", get(events))
}
pub(super) fn document(document: &mut contract::Document) {
    document.endpoint::<Query, Event>(
        "get",
        "/events",
        "application_events",
        contract::Reply::ApplicationEvents,
    );
}
struct Subscription {
    state: Arc<State>,
    auth: authorization::Authorization,
    leases: BTreeMap<Option<m::RepositoryId>, monitors::Lease>,
    updates: tokio::sync::watch::Receiver<u64>,
    revision: u64,
    pending: VecDeque<SseEvent>,
    closed: bool,
    heartbeat: Instant,
}
impl Subscription {
    async fn synchronize(&mut self) -> Result<()> {
        let repositories = self.auth.repositories(&self.state).await?;
        self.leases.retain(|repo, _| {
            repo.is_none() || repositories.iter().any(|(id, _)| Some(*id) == *repo)
        });
        for (repository, local) in repositories {
            if !self.leases.contains_key(&Some(repository)) {
                let lease = self
                    .state
                    .event_monitors
                    .acquire(&self.state, Some(repository), local)
                    .await?;
                self.leases.insert(Some(repository), lease);
            }
        }
        Ok(())
    }
    fn witnesses(&self) -> Vec<Change> {
        self.state
            .freshness
            .snapshot()
            .1
            .into_iter()
            .filter(|change| self.auth.visible(change))
            .collect()
    }
    fn frame(&self, revision: u64, name: &str, event: Event) -> Result<SseEvent> {
        SseEvent::default()
            .id(self.auth.cursor(&self.state, revision))
            .event(name)
            .json_data(event)
            .map_err(|_| Error::unavailable())
    }
    async fn load(&mut self) -> Result<()> {
        let Some(records) = self.state.freshness.after(self.revision) else {
            self.revision = self.state.freshness.snapshot().0;
            self.pending.push_back(self.frame(
                self.revision,
                "reset",
                Event::Reset {
                    version: 1,
                    reason: "replay_window",
                    witnesses: self.witnesses(),
                },
            )?);
            self.closed = true;
            return Ok(());
        };
        if records
            .iter()
            .any(|(_, c)| matches!(c.domain, Domain::Discovery | Domain::Workspaces))
        {
            self.synchronize().await?;
        }
        for (revision, change) in records {
            self.revision = revision;
            if self.auth.visible(&change) {
                self.auth.authorize(&self.state, change.repository).await?;
                self.pending.push_back(self.frame(
                    revision,
                    "changed",
                    Event::Changed { version: 1, change },
                )?);
            }
        }
        Ok(())
    }
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
    let auth = authorization::Authorization::new(&state, &headers)?;
    // Install the listener before any capture so a racing publication stays replayable.
    let updates = state.freshness.subscribe();
    let mut subscription = Subscription {
        state,
        auth,
        leases: BTreeMap::new(),
        updates,
        revision: 0,
        pending: VecDeque::new(),
        closed: false,
        heartbeat: Instant::now() + Duration::from_secs(15),
    };
    subscription.synchronize().await?;
    let lease = subscription
        .state
        .event_monitors
        .acquire(&subscription.state, None, false)
        .await?;
    subscription.leases.insert(None, lease);
    subscription.synchronize().await?;
    let current = subscription.state.freshness.snapshot().0;
    let after = query.after.as_deref().or(header);
    match after
        .map(|cursor| subscription.auth.resume(&subscription.state, cursor))
        .transpose()?
    {
        Some(Some(revision)) => {
            subscription.revision = revision;
            subscription.load().await?;
        }
        Some(None) => {
            subscription.revision = current;
            subscription.pending.push_back(subscription.frame(
                current,
                "reset",
                Event::Reset {
                    version: 1,
                    reason: "generation_or_scope",
                    witnesses: subscription.witnesses(),
                },
            )?);
            subscription.closed = true;
        }
        None => subscription.revision = current,
    }
    if !subscription.closed {
        subscription.pending.push_back(subscription.frame(
            subscription.revision,
            "ready",
            Event::Ready {
                version: 1,
                witnesses: subscription.witnesses(),
            },
        )?);
    }
    let stream = futures::stream::try_unfold(subscription, |mut s| async move {
        loop {
            if !s.closed && Instant::now() >= s.heartbeat {
                s.auth
                    .authorize(&s.state, None)
                    .await
                    .map_err(|_| std::io::Error::other("freshness access denied"))?;
                s.heartbeat = Instant::now() + Duration::from_secs(15);
                return Ok::<_, std::io::Error>(Some((
                    SseEvent::default().comment("keep-alive"),
                    s,
                )));
            }
            if let Some(frame) = s.pending.pop_front() {
                s.auth
                    .authorize(&s.state, None)
                    .await
                    .map_err(|_| std::io::Error::other("freshness access denied"))?;
                return Ok::<_, std::io::Error>(Some((frame, s)));
            }
            if s.closed {
                return Ok(None);
            }
            tokio::select! {
                result = s.updates.changed() => {
                    result.map_err(std::io::Error::other)?;
                    s.load().await.map_err(|_| std::io::Error::other("freshness unavailable"))?;
                }
                _ = tokio::time::sleep_until(s.heartbeat) => {
                    s.auth.authorize(&s.state, None).await.map_err(|_| std::io::Error::other("freshness access denied"))?;
                    s.heartbeat = Instant::now() + Duration::from_secs(15);
                    return Ok(Some((SseEvent::default().comment("keep-alive"), s)));
                }
            }
        }
    });
    let mut response = Sse::new(stream).into_response();
    response
        .headers_mut()
        .insert("x-accel-buffering", "no".parse().unwrap());
    Ok(response)
}

#[cfg(test)]
mod tests;
