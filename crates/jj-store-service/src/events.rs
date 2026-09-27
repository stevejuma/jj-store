//! Ordered, resumable notifications from selected WAL records, never wake hints.
use crate::{auth::Permission, native::NativeSession, reads::HeadSnapshot};
use jj_store_model::{EpochId, RepositoryId, WalPosition};
use jj_store_wal as wal;
use serde::Serialize;
use std::{collections::VecDeque, fmt, future::Future, str::FromStr, sync::Arc, time::Duration};
use tokio::{sync::broadcast, time::Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "EventCursor")]
pub struct Cursor {
    pub repository: RepositoryId,
    pub epoch: EpochId,
    pub position: WalPosition,
}
impl Cursor {
    fn current(snapshot: &HeadSnapshot) -> Self {
        Self {
            repository: snapshot.repo_id,
            epoch: snapshot.epoch,
            position: snapshot.published_seq,
        }
    }
}
impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "v1:{}:{}:{}",
            self.repository, self.epoch, self.position.0
        )
    }
}
impl FromStr for Cursor {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self> {
        if value.len() > 128 {
            return Err(Error::InvalidCursor);
        }
        let fields: Vec<_> = value.split(':').collect();
        let ["v1", repository, epoch, position] = fields.as_slice() else {
            return Err(Error::InvalidCursor);
        };
        let cursor = Self {
            repository: repository.parse().map_err(|_| Error::InvalidCursor)?,
            epoch: epoch.parse().map_err(|_| Error::InvalidCursor)?,
            position: WalPosition(position.parse().map_err(|_| Error::InvalidCursor)?),
        };
        if cursor.to_string() != value {
            return Err(Error::InvalidCursor);
        }
        Ok(cursor)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "EventResetReason")]
#[serde(rename_all = "snake_case")]
pub enum ResetReason {
    EpochChanged,
    FuturePosition,
    ReplayWindow,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "EventKind")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventKind {
    Ready {
        snapshot: Box<HeadSnapshot>,
    },
    Changed,
    Reset {
        reason: ResetReason,
        snapshot: Box<HeadSnapshot>,
    },
    /// HTTP encodes this internal wake as a comment, never JSON event data.
    #[schemars(skip)]
    Heartbeat,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "CommittedEvent")]
pub struct Event {
    pub version: u32,
    pub cursor: Cursor,
    #[serde(flatten)]
    pub kind: EventKind,
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub poll_interval: Duration,
    pub read_timeout: Duration,
    pub replay_window: u64,
    pub page_size: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
            read_timeout: Duration::from_secs(5),
            replay_window: 1024,
            page_size: 32,
        }
    }
}
impl Limits {
    pub fn validate(&self) -> Result<()> {
        if !(Duration::from_millis(10)..=Duration::from_secs(60)).contains(&self.poll_interval)
            || self.read_timeout.is_zero()
            || self.read_timeout > Duration::from_secs(60)
            || !(1..=100_000).contains(&self.replay_window)
            || !(1..=128).contains(&self.page_size)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("event access denied")]
    Denied,
    #[error("invalid event cursor")]
    InvalidCursor,
    #[error("event resource limit")]
    Limit,
    #[error("committed event history unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<wal::Error> for Error {
    fn from(value: wal::Error) -> Self {
        match value {
            wal::Error::Denied => Self::Denied,
            wal::Error::Limit(_) => Self::Limit,
            _ => Self::Unavailable,
        }
    }
}
/// Fixed-size process-local wake hints. No credentials, per-repository registry,
/// subscriber callbacks, or persistence authority are retained here.
#[derive(Clone)]
pub(crate) struct Hints(broadcast::Sender<RepositoryId>);
impl Default for Hints {
    fn default() -> Self {
        Self(broadcast::channel(64).0)
    }
}
impl Hints {
    pub fn published(&self, repository: RepositoryId) {
        let _ = self.0.send(repository);
    }
    fn subscribe(&self) -> broadcast::Receiver<RepositoryId> {
        self.0.subscribe()
    }
}
async fn bounded<T>(timeout: Duration, work: impl Future<Output = Result<T>>) -> Result<T> {
    let deadline = Instant::now() + timeout;
    let value = tokio::time::timeout_at(deadline, work)
        .await
        .map_err(|_| Error::Limit)??;
    if Instant::now() >= deadline {
        Err(Error::Limit)
    } else {
        Ok(value)
    }
}
pub struct Subscription {
    session: Arc<NativeSession>,
    hints: broadcast::Receiver<RepositoryId>,
    limits: Limits,
    cursor: Cursor,
    pending: VecDeque<Cursor>,
    initial: bool,
    closed: bool,
    poll_at: Instant,
}
impl Subscription {
    pub async fn open(
        session: Arc<NativeSession>,
        after: Option<Cursor>,
        limits: Limits,
    ) -> Result<Self> {
        limits.validate()?;
        // Subscribe before capturing the manifest so a racing wake cannot be lost.
        let hints = session.events.subscribe();
        let snapshot =
            bounded(limits.read_timeout, async { Ok(session.snapshot().await?) }).await?;
        let head = HeadSnapshot::from(snapshot.manifest());
        let cursor = after.unwrap_or_else(|| Cursor::current(&head));
        if cursor.repository != session.repository {
            return Err(Error::InvalidCursor);
        }
        Ok(Self {
            session,
            hints,
            limits,
            cursor,
            pending: VecDeque::new(),
            initial: true,
            closed: false,
            poll_at: Instant::now(),
        })
    }
    pub async fn next(&mut self) -> Result<Option<Event>> {
        if self.closed {
            return Ok(None);
        }
        let value = self.next_inner().await;
        if value.is_err() {
            self.closed = true;
            self.pending.clear();
        }
        value.map(Some)
    }
    fn reset(&mut self, snapshot: HeadSnapshot, reason: ResetReason) -> Event {
        self.closed = true;
        self.pending.clear();
        self.cursor = Cursor::current(&snapshot);
        Event {
            version: 1,
            cursor: self.cursor,
            kind: EventKind::Reset {
                reason,
                snapshot: Box::new(snapshot),
            },
        }
    }
    async fn next_inner(&mut self) -> Result<Event> {
        if !self.initial && self.pending.is_empty() {
            loop {
                tokio::select! {
                    biased;
                    _=tokio::time::sleep_until(self.poll_at)=>break,
                    hint=self.hints.recv()=>match hint {
                        Ok(repository) if repository==self.cursor.repository => break,
                        Err(broadcast::error::RecvError::Lagged(_)) => break,
                        Err(broadcast::error::RecvError::Closed) => {tokio::time::sleep_until(self.poll_at).await; break;},
                        _=>{}
                    }
                }
            }
        }
        loop {
            let snapshot = bounded(self.limits.read_timeout, async {
                Ok(self.session.snapshot().await?)
            })
            .await?;
            let head = HeadSnapshot::from(snapshot.manifest());
            if head.epoch != self.cursor.epoch {
                return Ok(self.reset(head, ResetReason::EpochChanged));
            }
            if self.cursor.position > head.published_seq {
                return Ok(self.reset(head, ResetReason::FuturePosition));
            }
            if head.published_seq.0 - self.cursor.position.0 > self.limits.replay_window {
                return Ok(self.reset(head, ResetReason::ReplayWindow));
            }
            if self.initial {
                self.initial = false;
                // Known backlog is read immediately; an idle stream waits for a
                // wake or poll deadline and cannot spin heartbeats on demand.
                self.poll_at = if head.published_seq > self.cursor.position {
                    Instant::now()
                } else {
                    Instant::now() + self.limits.poll_interval
                };
                return Ok(Event {
                    version: 1,
                    cursor: self.cursor,
                    kind: EventKind::Ready {
                        snapshot: Box::new(head),
                    },
                });
            }
            if let Some(cursor) = self.pending.pop_front() {
                self.cursor = cursor;
                return Ok(Event {
                    version: 1,
                    cursor,
                    kind: EventKind::Changed,
                });
            }
            if head.published_seq == self.cursor.position {
                self.poll_at = Instant::now() + self.limits.poll_interval;
                return Ok(Event {
                    version: 1,
                    cursor: self.cursor,
                    kind: EventKind::Heartbeat,
                });
            }
            let page = bounded(self.limits.read_timeout, async {
                let page = self
                    .session
                    .coordinator
                    .committed(&snapshot, self.cursor.position, self.limits.page_size)
                    .await?;
                self.session.principal(Permission::Read).await?;
                Ok(page)
            })
            .await?;
            if page.is_empty() {
                return Err(Error::Unavailable);
            }
            self.pending.extend(page.into_iter().map(|record| Cursor {
                repository: record.repository,
                epoch: record.after.epoch,
                position: record.after.position,
            }));
            // Reauthorize the buffered page and recheck epoch/window before the
            // first delivery, just as for every subsequent queued event.
            self.poll_at = Instant::now();
        }
    }
}
