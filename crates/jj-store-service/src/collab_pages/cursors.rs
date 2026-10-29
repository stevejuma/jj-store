use super::{Error, Limits, PinnedCollabRead, Position, Query, Result};
use crate::auth::Clock;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, KeyInit as _, Mac as _};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use tokio::sync::{Semaphore, SemaphorePermit};
use zeroize::Zeroizing;
const DOMAIN: &[u8] = b"jj-store/collab-cursor/v1\0";
const MAX_TOKEN: usize = 4096;
type Signature = Hmac<Sha256>;

#[derive(Clone)]
struct Entry {
    pin: Arc<PinnedCollabRead>,
    viewer: m::PrincipalId,
    query: Query,
    issued: u64,
    expires: u64,
    objects: usize,
    bytes: usize,
    snapshot_digest: [u8; 32],
}
#[derive(Default)]
struct Entries {
    values: BTreeMap<[u8; 16], Entry>,
    objects: usize,
    bytes: usize,
}
impl Entries {
    fn prune(&mut self, now: u64) {
        self.values
            .retain(|_, entry| entry.issued <= now && now < entry.expires);
        self.objects = self.values.values().map(|e| e.objects).sum();
        self.bytes = self.values.values().map(|e| e.bytes).sum();
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Claims {
    version: u32,
    pin: [u8; 16],
    repository: m::RepositoryId,
    viewer: m::PrincipalId,
    snapshot_digest: [u8; 32],
    query: Query,
    pub after: Position,
    issued: u64,
    expires: u64,
}
impl Claims {
    fn new(id: [u8; 16], entry: &Entry, after: Position) -> Self {
        Self {
            version: 1,
            pin: id,
            repository: entry.pin.snapshot().repo_id,
            viewer: entry.viewer,
            snapshot_digest: entry.snapshot_digest,
            query: entry.query.clone(),
            after,
            issued: entry.issued,
            expires: entry.expires,
        }
    }
    fn matches(&self, entry: &Entry) -> bool {
        self.version == 1
            && self.repository == entry.pin.snapshot().repo_id
            && self.viewer == entry.viewer
            && self.snapshot_digest == entry.snapshot_digest
            && self.query == entry.query
            && self.issued == entry.issued
            && self.expires == entry.expires
    }
}
pub(super) struct Registry {
    limits: Limits,
    clock: Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    entries: Mutex<Entries>,
    permits: Semaphore,
}
impl Registry {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        if !(1..=1024).contains(&limits.snapshots)
            || !(1..=2_000_000).contains(&limits.objects)
            || !(1..=1024 * 1024 * 1024).contains(&limits.encoded_collab_bytes)
            || !(1..=64).contains(&limits.concurrency)
            || limits.ttl < std::time::Duration::from_secs(1)
            || limits.ttl > std::time::Duration::from_secs(3600)
            || limits.timeout.is_zero()
            || limits.timeout > std::time::Duration::from_secs(300)
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
            entries: Mutex::default(),
            permits: Semaphore::new(limits.concurrency),
        })
    }
    pub fn acquire(&self) -> Result<SemaphorePermit<'_>> {
        self.permits.try_acquire().map_err(|_| Error::Busy)
    }
    pub fn timeout(&self) -> std::time::Duration {
        self.limits.timeout
    }
    pub fn validate_pin(&self, pin: &PinnedCollabRead) -> Result<(usize, usize)> {
        let objects = pin.object_count();
        let bytes = pin.encoded_collab_bytes()?;
        if objects > self.limits.objects || bytes > self.limits.encoded_collab_bytes {
            return Err(Error::Limit);
        }
        Ok((objects, bytes))
    }
    pub fn retain(
        &self,
        pin: Arc<PinnedCollabRead>,
        viewer: m::PrincipalId,
        query: Query,
        after: Position,
    ) -> Result<String> {
        let now = self.now()?;
        let (objects, bytes) = self.validate_pin(&pin)?;
        let snapshot_digest =
            Sha256::digest(serde_json::to_vec(pin.snapshot()).map_err(|_| Error::Unavailable)?)
                .into();
        let entry = Entry {
            pin,
            viewer,
            query,
            issued: now,
            expires: now
                .checked_add(self.limits.ttl.as_secs())
                .ok_or(Error::Limit)?,
            objects,
            bytes,
            snapshot_digest,
        };
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.prune(now);
        let total_objects = entries.objects.checked_add(objects).ok_or(Error::Limit)?;
        let total_bytes = entries.bytes.checked_add(bytes).ok_or(Error::Limit)?;
        if entries.values.len() >= self.limits.snapshots
            || total_objects > self.limits.objects
            || total_bytes > self.limits.encoded_collab_bytes
        {
            return Err(Error::Busy);
        }
        let mut id = [0; 16];
        getrandom::fill(&mut id).map_err(|_| Error::Unavailable)?;
        if entries.values.contains_key(&id) {
            return Err(Error::Unavailable);
        }
        let token = self.sign(&Claims::new(id, &entry, after))?;
        entries.objects = total_objects;
        entries.bytes = total_bytes;
        entries.values.insert(id, entry);
        Ok(token)
    }
    pub fn lookup(
        &self,
        repository: m::RepositoryId,
        viewer: m::PrincipalId,
        query: &Query,
        token: &str,
    ) -> Result<(Claims, Arc<PinnedCollabRead>)> {
        let claims = self.verify(token)?;
        if claims.repository != repository || claims.viewer != viewer || claims.query != *query {
            return Err(Error::InvalidCursor);
        }
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.prune(self.now()?);
        let entry = entries.values.get(&claims.pin).ok_or(Error::CursorReset)?;
        if !claims.matches(entry) {
            return Err(Error::InvalidCursor);
        }
        Ok((claims, entry.pin.clone()))
    }
    pub fn next(&self, claims: &Claims, after: Option<Position>) -> Result<Option<String>> {
        let now = self.now()?;
        if now < claims.issued || now >= claims.expires {
            return Err(Error::CursorReset);
        }
        after
            .map(|after| {
                let mut next = claims.clone();
                next.after = after;
                self.sign(&next)
            })
            .transpose()
    }
    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }
    fn sign(&self, claims: &Claims) -> Result<String> {
        let body =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).map_err(|_| Error::Unavailable)?);
        let prefix = format!("jsc1.{}.{}", self.instance, body);
        let mut mac =
            Signature::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(prefix.as_bytes());
        let token = format!(
            "{prefix}.{}",
            URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
        );
        if token.len() > MAX_TOKEN {
            return Err(Error::Limit);
        }
        Ok(token)
    }
    fn verify(&self, token: &str) -> Result<Claims> {
        if token.len() > MAX_TOKEN {
            return Err(Error::InvalidCursor);
        }
        let parts = token.split('.').collect::<Vec<_>>();
        let ["jsc1", instance, body, tag] = parts.as_slice() else {
            return Err(Error::InvalidCursor);
        };
        if instance.len() != 32 || !instance.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::InvalidCursor);
        }
        if *instance != self.instance {
            return Err(Error::CursorReset);
        }
        let prefix = &token[..token.rfind('.').ok_or(Error::InvalidCursor)?];
        let mut mac =
            Signature::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(prefix.as_bytes());
        let tag = URL_SAFE_NO_PAD
            .decode(tag)
            .map_err(|_| Error::InvalidCursor)?;
        mac.verify_slice(&tag).map_err(|_| Error::InvalidCursor)?;
        let data = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::InvalidCursor)?;
        let claims: Claims = serde_json::from_slice(&data).map_err(|_| Error::InvalidCursor)?;
        let now = self.now()?;
        if claims.version != 1 || now < claims.issued || now >= claims.expires {
            return Err(Error::CursorReset);
        }
        Ok(claims)
    }
}
