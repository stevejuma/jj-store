//! Shared, credential-free retention and signing for bounded read continuations.
use super::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeMap, fmt, sync::Mutex};
use tokio::sync::Semaphore;
use zeroize::Zeroizing;

const MAX_CURSOR: usize = 4096;
const DOMAIN: &[u8] = b"jj-store/read-cursor/v1/jj-0.45.1/native-order\0";
type Signature = Hmac<Sha256>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "after", rename_all = "snake_case")]
pub(super) enum Continuation {
    Graph(m::ObjectKey),
    Directory(String),
    References {
        anchor: String,
        direction: jj_store_jj::query::content::Direction,
    },
    Paths {
        anchor: String,
        direction: jj_store_jj::query::content::Direction,
    },
    OperationGroups {
        anchor: String,
        direction: jj_store_jj::query::content::Direction,
    },
    DiffPaths {
        anchor: String,
        direction: jj_store_jj::query::content::Direction,
    },
}
#[derive(Clone)]
pub(super) struct Entry {
    pub pin: ReadPin,
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
pub(super) struct Claims {
    version: u32,
    pin: [u8; 16],
    repository: m::RepositoryId,
    epoch: m::EpochId,
    operation: m::OperationId,
    position: m::WalPosition,
    revision: u64,
    query: [u8; 32],
    page_size: usize,
    pub after: Continuation,
    issued: u64,
    expires: u64,
}
impl Claims {
    fn new(id: [u8; 16], entry: &Entry, after: Continuation) -> Self {
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
        let expected = Self::new(self.pin, entry, self.after.clone());
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
pub struct SnapshotCursors {
    pub(super) limits: Limits,
    clock: Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    registry: Mutex<Registry>,
    pub(super) permits: Semaphore,
}
impl fmt::Debug for SnapshotCursors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SnapshotCursors")
            .field("signing_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}
impl SnapshotCursors {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
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
        let prefix = format!("jsr1.{}.{}", self.instance, body);
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
        let ["jsr1", instance, body, tag] = parts.as_slice() else {
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
    pub(super) async fn authorized(&self, session: &NativeSession) -> Result<()> {
        session
            .principal(Permission::Read)
            .await
            .map(|_| ())
            .map_err(|e| match e {
                jj_store_wal::Error::Denied => Error::Denied,
                _ => Error::Unavailable,
            })
    }
    pub(super) fn retain(
        &self,
        pin: ReadPin,
        query: [u8; 32],
        page_size: usize,
        after: Continuation,
    ) -> Result<String> {
        let now = self.now()?;
        let entry = Entry {
            pin,
            query,
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
        if registry.entries.len() >= self.limits.snapshots || nodes > self.limits.inventory_nodes {
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
        Ok(token)
    }
    /// Caller must authenticate before cursor lookup. This only checks binding;
    /// ReadPin::resume independently reauthorizes and validates the current epoch.
    pub(super) fn lookup(
        &self,
        repository: m::RepositoryId,
        token: &str,
        operation: Option<m::OperationId>,
        query: [u8; 32],
        page_size: usize,
    ) -> Result<(Claims, Entry)> {
        let claims = self.verify(token)?;
        if claims.repository != repository
            || claims.query != query
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
        Ok((claims, entry))
    }
    pub(super) fn next(
        &self,
        claims: &Claims,
        entry: &Entry,
        after: Option<Continuation>,
    ) -> Result<Option<String>> {
        let now = self.now()?;
        if now < entry.issued || now >= entry.expires {
            return Err(Error::CursorReset);
        }
        after
            .map(|after| self.sign(&Claims::new(claims.pin, entry, after)))
            .transpose()
    }
}
pub(super) fn fingerprint(kind: &[u8], request: &impl Serialize) -> Result<[u8; 32]> {
    let bytes = serde_json::to_vec(request).map_err(|_| Error::Unavailable)?;
    let mut hash = Sha256::new_with_prefix(DOMAIN);
    hash.update(kind);
    hash.update(bytes);
    Ok(hash.finalize().into())
}
