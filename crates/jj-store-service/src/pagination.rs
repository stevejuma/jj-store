//! Signed graph continuations over bounded, credential-free snapshot pins.
use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
    reads::{self, HeadSnapshot, OpenRead, ReadLimits, ReadPin, ReadSnapshot},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use jj_store_jj::query::{self, GraphExecutor, GraphPage, Request};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Semaphore;
use zeroize::Zeroizing;

const MAX_CURSOR: usize = 4096;
const DOMAIN: &[u8] = b"jj-store/graph-cursor/v1/jj-0.45.1/native-order\0";
type Signature = Hmac<Sha256>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("graph page is not authorized")]
    Denied,
    #[error("invalid graph cursor or changed query")]
    InvalidCursor,
    #[error("graph cursor expired or its serving instance changed; select a new snapshot")]
    CursorReset,
    #[error("graph pagination resource limit exceeded")]
    Limit,
    #[error("graph pagination is busy")]
    Busy,
    #[error("graph pagination is unavailable")]
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
#[derive(Clone)]
struct Entry {
    pin: ReadPin,
    query: [u8; 32],
    page_size: usize,
    issued: u64,
    expires: u64,
}
#[derive(Default)]
struct Registry {
    entries: BTreeMap<[u8; 16], Entry>,
    nodes: usize,
}
impl Registry {
    fn prune(&mut self, now: u64) {
        self.entries
            .retain(|_, entry| entry.issued <= now && now < entry.expires);
        self.nodes = self
            .entries
            .values()
            .map(|entry| entry.pin.inventory.len())
            .sum();
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Claims {
    version: u32,
    pin: [u8; 16],
    repository: m::RepositoryId,
    epoch: m::EpochId,
    operation: m::OperationId,
    position: m::WalPosition,
    revision: u64,
    query: [u8; 32],
    page_size: usize,
    after: m::ObjectKey,
    issued: u64,
    expires: u64,
}
impl Claims {
    fn new(id: [u8; 16], entry: &Entry, after: m::ObjectKey) -> Self {
        let snapshot = &entry.pin.snapshot;
        Self {
            version: 1,
            pin: id,
            repository: snapshot.authority.repo_id,
            epoch: snapshot.authority.epoch,
            operation: snapshot.operation_id,
            position: snapshot.authority.published_seq,
            revision: snapshot.authority.manifest_revision,
            query: entry.query,
            page_size: entry.page_size,
            after,
            issued: entry.issued,
            expires: entry.expires,
        }
    }
    fn matches(&self, entry: &Entry) -> bool {
        let expected = Self::new(self.pin, entry, self.after);
        self.version == expected.version
            && self.repository == expected.repository
            && self.epoch == expected.epoch
            && self.operation == expected.operation
            && self.position == expected.position
            && self.revision == expected.revision
            && self.query == expected.query
            && self.page_size == expected.page_size
            && self.issued == expected.issued
            && self.expires == expected.expires
    }
}
/// One instance lives for the serving process. Expiry/restart requires reselection;
/// a cursor never substitutes for fresh repository authentication.
pub struct GraphPager {
    executor: GraphExecutor,
    limits: Limits,
    clock: Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    registry: Mutex<Registry>,
    permits: Semaphore,
}
impl fmt::Debug for GraphPager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphPager")
            .field("signing_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}
impl GraphPager {
    pub fn new(executor: GraphExecutor, limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        if !(1..=1024).contains(&limits.snapshots)
            || !(1..=2_000_000).contains(&limits.inventory_nodes)
            || !(1..=64).contains(&limits.concurrency)
            || limits.ttl < Duration::from_secs(1)
            || limits.ttl > Duration::from_secs(3600)
            || limits.read.max_objects == 0
            || limits.read.timeout.is_zero()
            || limits.read.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        let mut key = Zeroizing::new([0; 32]);
        let mut instance = [0; 16];
        getrandom::fill(key.as_mut()).map_err(|_| Error::Unavailable)?;
        getrandom::fill(&mut instance).map_err(|_| Error::Unavailable)?;
        Ok(Self {
            executor,
            limits,
            clock,
            instance: hex::encode(instance),
            key,
            registry: Mutex::default(),
            permits: Semaphore::new(limits.concurrency),
        })
    }
    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }
    fn sign(&self, claims: &Claims) -> Result<String> {
        let body =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).map_err(|_| Error::Unavailable)?);
        let prefix = format!("jsg1.{}.{}", self.instance, body);
        let mut mac =
            Signature::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(prefix.as_bytes());
        let token = format!(
            "{prefix}.{}",
            URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
        );
        if token.len() > MAX_CURSOR {
            return Err(Error::Limit);
        }
        Ok(token)
    }
    fn verify(&self, token: &str) -> Result<Claims> {
        if token.len() > MAX_CURSOR {
            return Err(Error::InvalidCursor);
        }
        let parts: Vec<_> = token.split('.').collect();
        let ["jsg1", instance, body, tag] = parts.as_slice() else {
            return Err(Error::InvalidCursor);
        };
        if instance.len() != 32 || !instance.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::InvalidCursor);
        }
        if *instance != self.instance {
            return Err(Error::CursorReset);
        }
        let tag = URL_SAFE_NO_PAD
            .decode(tag)
            .map_err(|_| Error::InvalidCursor)?;
        let prefix = &token[..token.rfind('.').ok_or(Error::InvalidCursor)?];
        let mut mac =
            Signature::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(prefix.as_bytes());
        mac.verify_slice(&tag).map_err(|_| Error::InvalidCursor)?;
        let bytes = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::InvalidCursor)?;
        let claims: Claims = serde_json::from_slice(&bytes).map_err(|_| Error::InvalidCursor)?;
        if claims.version != 1 {
            return Err(Error::InvalidCursor);
        }
        let now = self.now()?;
        if now < claims.issued || now >= claims.expires {
            return Err(Error::CursorReset);
        }
        Ok(claims)
    }
    async fn authorized(&self, session: &NativeSession) -> Result<()> {
        session
            .principal(Permission::Read)
            .await
            .map(|_| ())
            .map_err(|e| match e {
                jj_store_wal::Error::Denied => Error::Denied,
                _ => Error::Unavailable,
            })
    }
    /// First page. Any multiple-head response is explicit and creates no pin.
    pub async fn begin(
        &self,
        session: Arc<NativeSession>,
        operation: Option<m::OperationId>,
        request: Request,
        page_size: usize,
    ) -> Result<Start> {
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        within_timeout(self.limits.read.timeout, async {
            self.authorized(&session).await?;
            let fingerprint = fingerprint(&request)?;
            let read = match session.open_read(operation, self.limits.read).await? {
                OpenRead::SelectHead(snapshot) => return Ok(Start::SelectHead { snapshot }),
                OpenRead::Pinned(read) => Arc::new(read),
            };
            let mut graph = read
                .graph(
                    &self.executor,
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
                let now = self.now()?;
                let entry = Entry {
                    pin,
                    query: fingerprint,
                    page_size,
                    issued: now,
                    expires: now
                        .checked_add(self.limits.ttl.as_secs())
                        .ok_or(Error::Limit)?,
                };
                let mut registry = self.registry.lock().map_err(|_| Error::Unavailable)?;
                registry.prune(now);
                let nodes = registry
                    .nodes
                    .checked_add(entry.pin.inventory.len())
                    .ok_or(Error::Limit)?;
                if registry.entries.len() >= self.limits.snapshots
                    || nodes > self.limits.inventory_nodes
                {
                    return Err(Error::Busy);
                }
                let mut id = [0; 16];
                getrandom::fill(&mut id).map_err(|_| Error::Unavailable)?;
                if registry.entries.contains_key(&id) {
                    return Err(Error::Unavailable);
                }
                let token = self.sign(&Claims::new(id, &entry, after))?;
                registry.nodes = nodes;
                registry.entries.insert(id, entry);
                Some(token)
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
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        within_timeout(self.limits.read.timeout, async {
            self.authorized(&session).await?;
            let claims = self.verify(cursor)?;
            if claims.repository != session.repository
                || claims.query != fingerprint(&request)?
                || claims.page_size != page_size
                || operation.is_some_and(|id| id != claims.operation)
            {
                return Err(Error::InvalidCursor);
            }
            let entry = {
                let mut registry = self.registry.lock().map_err(|_| Error::Unavailable)?;
                registry.prune(self.now()?);
                registry
                    .entries
                    .get(&claims.pin)
                    .cloned()
                    .ok_or(Error::CursorReset)?
            };
            if !claims.matches(&entry) {
                return Err(Error::InvalidCursor);
            }
            let read = entry.pin.resume(session, self.limits.read).await?;
            let mut graph = read
                .graph(
                    &self.executor,
                    request,
                    query::Page {
                        limit: page_size,
                        after: Some(claims.after),
                    },
                )
                .await?;
            let now = self.now()?;
            if now < entry.issued || now >= entry.expires {
                return Err(Error::CursorReset);
            }
            let next_cursor = graph
                .next
                .take()
                .map(|after| self.sign(&Claims::new(claims.pin, &entry, after)))
                .transpose()?;
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
    let bytes = serde_json::to_vec(request).map_err(|_| Error::Unavailable)?;
    let mut hash = Sha256::new_with_prefix(DOMAIN);
    hash.update(bytes);
    Ok(hash.finalize().into())
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
