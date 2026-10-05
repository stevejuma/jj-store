use super::{Error, Result, ReviewReadSnapshot};
use crate::{auth::Clock, pagination::Limits};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use jj_store_model as m;
use jj_store_wal::publication::{ReviewCatalogKey, ReviewStoreSnapshot};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{collections::BTreeMap, fmt, sync::Mutex};
use tokio::sync::{Semaphore, SemaphorePermit};
use zeroize::Zeroizing;

const MAX_CURSOR: usize = 4096;
const DOMAIN: &[u8] = b"jj-store/review-thread-cursor/v1\0";
type Signature = Hmac<Sha256>;

#[derive(Clone)]
pub(super) struct ReviewPin {
    pub snapshot: ReviewReadSnapshot,
    pub store: ReviewStoreSnapshot,
}

#[derive(Clone)]
struct Entry {
    pin: ReviewPin,
    page_size: usize,
    issued: u64,
    expires: u64,
    objects: usize,
}

#[derive(Default)]
struct Entries {
    values: BTreeMap<[u8; 16], Entry>,
    objects: usize,
}

impl Entries {
    fn prune(&mut self, now: u64) {
        self.values
            .retain(|_, value| value.issued <= now && now < value.expires);
        self.objects = self.values.values().map(|value| value.objects).sum();
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Claims {
    version: u32,
    pin: [u8; 16],
    repository: m::RepositoryId,
    epoch: m::EpochId,
    published_seq: m::WalPosition,
    revision: u64,
    review_root: m::RecordDigest,
    review_state_version: u64,
    page_size: usize,
    pub after: ReviewCatalogKey,
    issued: u64,
    expires: u64,
}

impl Claims {
    fn new(id: [u8; 16], entry: &Entry, after: ReviewCatalogKey) -> Self {
        let value = &entry.pin.snapshot;
        Self {
            version: 1,
            pin: id,
            repository: value.repo_id,
            epoch: value.epoch,
            published_seq: value.published_seq,
            revision: value.manifest_revision,
            review_root: value.review_root,
            review_state_version: value.review_state_version,
            page_size: entry.page_size,
            after,
            issued: entry.issued,
            expires: entry.expires,
        }
    }

    fn matches(&self, entry: &Entry) -> bool {
        let value = Self::new(self.pin, entry, self.after);
        self.version == value.version
            && self.repository == value.repository
            && self.epoch == value.epoch
            && self.published_seq == value.published_seq
            && self.revision == value.revision
            && self.review_root == value.review_root
            && self.review_state_version == value.review_state_version
            && self.page_size == value.page_size
            && self.issued == value.issued
            && self.expires == value.expires
    }
}

pub(super) struct ReviewCursorRegistry {
    limits: Limits,
    clock: std::sync::Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    entries: Mutex<Entries>,
    permits: Semaphore,
}

impl fmt::Debug for ReviewCursorRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReviewCursorRegistry")
            .field("signing_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl ReviewCursorRegistry {
    pub fn new(limits: Limits, clock: std::sync::Arc<dyn Clock>) -> Result<Self> {
        if !(1..=1024).contains(&limits.snapshots)
            || !(1..=2_000_000).contains(&limits.inventory_nodes)
            || !(1..=64).contains(&limits.concurrency)
            || limits.ttl.is_zero()
            || limits.ttl > std::time::Duration::from_secs(3600)
            || limits.read.max_objects == 0
            || limits.read.timeout.is_zero()
            || limits.read.timeout > std::time::Duration::from_secs(300)
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
        self.limits.read.timeout
    }

    pub fn validate_pin(&self, pin: &ReviewPin) -> Result<()> {
        if pin.store.object_count() > self.limits.read.max_objects {
            Err(Error::Limit)
        } else {
            Ok(())
        }
    }

    pub fn retain(
        &self,
        pin: ReviewPin,
        page_size: usize,
        after: ReviewCatalogKey,
    ) -> Result<String> {
        let now = self.now()?;
        let objects = pin.store.object_count();
        self.validate_pin(&pin)?;
        let entry = Entry {
            pin,
            page_size,
            issued: now,
            expires: now
                .checked_add(self.limits.ttl.as_secs())
                .ok_or(Error::Limit)?,
            objects,
        };
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.prune(now);
        let retained = entries.objects.checked_add(objects).ok_or(Error::Limit)?;
        if entries.values.len() >= self.limits.snapshots || retained > self.limits.inventory_nodes {
            return Err(Error::Busy);
        }
        let mut id = [0; 16];
        getrandom::fill(&mut id).map_err(|_| Error::Unavailable)?;
        if entries.values.contains_key(&id) {
            return Err(Error::Unavailable);
        }
        let token = self.sign(&Claims::new(id, &entry, after))?;
        entries.objects = retained;
        entries.values.insert(id, entry);
        Ok(token)
    }

    pub fn lookup(
        &self,
        repository: m::RepositoryId,
        token: &str,
        page_size: usize,
    ) -> Result<(Claims, ReviewPin)> {
        let claims = self.verify(token)?;
        if claims.repository != repository || claims.page_size != page_size {
            return Err(Error::InvalidCursor);
        }
        let entry = {
            let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            entries.prune(self.now()?);
            entries
                .values
                .get(&claims.pin)
                .cloned()
                .ok_or(Error::CursorReset)?
        };
        if !claims.matches(&entry) {
            return Err(Error::InvalidCursor);
        }
        Ok((claims, entry.pin))
    }

    pub fn next(&self, claims: &Claims, after: Option<ReviewCatalogKey>) -> Result<Option<String>> {
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
        let parts = token.split('.').collect::<Vec<_>>();
        let ["jsr1", instance, body, tag] = parts.as_slice() else {
            return Err(Error::InvalidCursor);
        };
        if instance.len() != 32 || !instance.bytes().all(|byte| byte.is_ascii_hexdigit()) {
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
        let bytes = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::InvalidCursor)?;
        let claims: Claims = serde_json::from_slice(&bytes).map_err(|_| Error::InvalidCursor)?;
        let now = self.now()?;
        if claims.version != 1 || now < claims.issued || now >= claims.expires {
            return Err(Error::CursorReset);
        }
        Ok(claims)
    }
}
