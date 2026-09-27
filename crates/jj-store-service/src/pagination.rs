//! Signed graph and directory continuations over bounded, credential-free pins.
mod cursors;
pub mod directory;
use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
    reads::{self, HeadSnapshot, OpenRead, ReadLimits, ReadPin, ReadSnapshot},
};
use cursors::Continuation;
pub use cursors::SnapshotCursors;
use jj_store_jj::query::{self, GraphExecutor, GraphPage, Request};
use jj_store_model as m;
use serde::Serialize;
use std::{sync::Arc, time::Duration};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("read page is not authorized")]
    Denied,
    #[error("invalid read cursor or changed query")]
    InvalidCursor,
    #[error("read cursor expired or its serving instance changed; select a new snapshot")]
    CursorReset,
    #[error("native index preparation required for the selected snapshot")]
    IndexRebuildRequired { snapshot: Box<ReadSnapshot> },
    #[error("read pagination resource limit exceeded")]
    Limit,
    #[error("read pagination is busy")]
    Busy,
    #[error("read pagination is unavailable")]
    Unavailable,
    #[error(transparent)]
    Read(#[from] reads::Error),
    #[error(transparent)]
    Query(#[from] query::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub struct Limits {
    pub snapshots: usize,
    pub inventory_nodes: usize,
    pub concurrency: usize,
    pub ttl: Duration,
    pub read: ReadLimits,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            snapshots: 64,
            inventory_nodes: 1_000_000,
            concurrency: 8,
            ttl: Duration::from_secs(300),
            read: ReadLimits::default(),
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Page {
    pub snapshot: ReadSnapshot,
    /// Owned native data with its raw continuation removed. Only next_cursor
    /// crosses the application boundary.
    pub graph: GraphPage,
    pub next_cursor: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Start {
    SelectHead { snapshot: HeadSnapshot },
    Page(Box<Page>),
}
/// Graph interface over the shared serving-process snapshot registry.
pub struct GraphPager {
    executor: GraphExecutor,
    cursors: Arc<SnapshotCursors>,
}
impl std::fmt::Debug for GraphPager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphPager")
            .field("cursors", &self.cursors)
            .finish_non_exhaustive()
    }
}
impl GraphPager {
    pub fn new(executor: GraphExecutor, limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        Ok(Self::with_cursors(
            executor,
            Arc::new(SnapshotCursors::new(limits, clock)?),
        ))
    }
    pub fn with_cursors(executor: GraphExecutor, cursors: Arc<SnapshotCursors>) -> Self {
        Self { executor, cursors }
    }
    async fn execute_read(
        &self,
        read: &Arc<reads::CommittedRead>,
        request: Request,
        page: query::Page,
    ) -> Result<GraphPage> {
        match read.graph(&self.executor, request, page).await {
            Ok(page) => Ok(page),
            Err(query::Error::IndexRebuildRequired) => Err(Error::IndexRebuildRequired {
                snapshot: Box::new(read.snapshot().await?.clone()),
            }),
            Err(error) => Err(error.into()),
        }
    }
    /// First page. Any multiple-head response is explicit and creates no pin.
    pub async fn begin(
        &self,
        session: Arc<NativeSession>,
        operation: Option<m::OperationId>,
        request: Request,
        page_size: usize,
    ) -> Result<Start> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        within_timeout(self.cursors.limits.read.timeout, async {
            self.cursors.authorized(&session).await?;
            let fingerprint = fingerprint(&request)?;
            let read = match session
                .open_read(operation, self.cursors.limits.read)
                .await?
            {
                OpenRead::SelectHead(snapshot) => return Ok(Start::SelectHead { snapshot }),
                OpenRead::Pinned(read) => Arc::new(read),
            };
            let mut graph = self
                .execute_read(
                    &read,
                    request,
                    query::Page {
                        limit: page_size,
                        after: None,
                    },
                )
                .await?;
            let pin = read.pin().await?;
            let snapshot = pin.snapshot.clone();
            let next_cursor = if let Some(after) = graph.next.take() {
                Some(self.cursors.retain(
                    pin,
                    fingerprint,
                    page_size,
                    Continuation::Graph(after),
                )?)
            } else {
                None
            };
            Ok(Start::Page(Box::new(Page {
                snapshot,
                graph,
                next_cursor,
            })))
        })
        .await
    }
    /// Resumes with a new request/session and the original complete inventory.
    /// `operation` may be omitted but cannot contradict the cursor when supplied.
    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        operation: Option<m::OperationId>,
        request: Request,
        page_size: usize,
    ) -> Result<Page> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        within_timeout(self.cursors.limits.read.timeout, async {
            self.cursors.authorized(&session).await?;
            let (claims, entry) = self.cursors.lookup(
                session.repository,
                cursor,
                operation,
                fingerprint(&request)?,
                page_size,
            )?;
            let Continuation::Graph(after) = &claims.after else {
                return Err(Error::InvalidCursor);
            };
            let read = entry.pin.resume(session, self.cursors.limits.read).await?;
            let mut graph = self
                .execute_read(
                    &read,
                    request,
                    query::Page {
                        limit: page_size,
                        after: Some(*after),
                    },
                )
                .await?;
            let next_cursor =
                self.cursors
                    .next(&claims, &entry, graph.next.take().map(Continuation::Graph))?;
            Ok(Page {
                snapshot: entry.pin.snapshot,
                graph,
                next_cursor,
            })
        })
        .await
    }
}
async fn within_timeout<T>(
    timeout: Duration,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    let deadline = tokio::time::Instant::now()
        .checked_add(timeout)
        .ok_or(Error::Limit)?;
    if tokio::time::Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    let result = tokio::time::timeout_at(deadline, future)
        .await
        .map_err(|_| Error::Limit)?;
    if tokio::time::Instant::now() >= deadline {
        return Err(Error::Limit);
    }
    result
}

fn fingerprint(request: &Request) -> Result<[u8; 32]> {
    if let Request::Log {
        revset,
        workspace,
        user_email,
        ..
    } = request
        && (revset.len() > 16_384
            || workspace.as_ref().is_some_and(|w| w.len() > 1024)
            || user_email.len() > 1024)
    {
        return Err(Error::Limit);
    }
    cursors::fingerprint(b"graph\0", request)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn graph_cursor_rejects_immediately_ready_results_after_deadline() {
        assert_eq!(
            within_timeout(Duration::from_millis(1), async {
                std::thread::sleep(Duration::from_millis(5));
                Ok(())
            })
            .await,
            Err(Error::Limit)
        );
    }
}
