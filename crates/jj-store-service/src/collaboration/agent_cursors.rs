use super::cursor_position::{Claims, Entry, Position};
use super::{AgentReadSnapshot, Error, Result};
use crate::{auth::Clock, pagination::Limits};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use jj_store_model as m;
use jj_store_wal::publication::AgentStoreSnapshot;
use sha2::Sha256;
use std::{collections::BTreeMap, fmt, sync::Mutex};
use tokio::sync::{Semaphore, SemaphorePermit};
use zeroize::Zeroizing;

const MAX_CURSOR: usize = 4096;
const DOMAIN: &[u8] = b"jj-store/agent-session-cursor/v1\0";
type Signature = Hmac<Sha256>;

#[derive(Clone)]
pub(super) struct AgentPin {
    pub snapshot: AgentReadSnapshot,
    pub store: AgentStoreSnapshot,
}

#[derive(Default)]
struct Entries {
    values: BTreeMap<[u8; 16], Entry>,
    objects: usize,
}

impl Entries {
    fn prune(&mut self, now: u64) {
        self.values
            .retain(|_, entry| entry.issued <= now && now < entry.expires);
        self.objects = self.values.values().map(|entry| entry.objects).sum();
    }
}

pub(super) struct AgentCursorRegistry {
    limits: Limits,
    clock: std::sync::Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    entries: Mutex<Entries>,
    permits: Semaphore,
}

impl fmt::Debug for AgentCursorRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AgentCursorRegistry")
            .field("signing_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl AgentCursorRegistry {
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

    pub fn validate_pin(&self, pin: &AgentPin) -> Result<()> {
        if pin.store.object_count() > self.limits.read.max_objects {
            Err(Error::Limit)
        } else {
            Ok(())
        }
    }

    pub fn retain(&self, pin: AgentPin, page_size: usize, position: Position) -> Result<String> {
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
        let token = self.sign(&Claims::new(id, &entry, position))?;
        entries.objects = retained;
        entries.values.insert(id, entry);
        Ok(token)
    }

    pub fn lookup(
        &self,
        repository: m::RepositoryId,
        token: &str,
        page_size: usize,
    ) -> Result<(Claims, Entry)> {
        let claims = self.verify(token)?;
        if claims.repository != repository || claims.page_size() != page_size {
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
        Ok((claims, entry))
    }

    pub fn next(
        &self,
        claims: &Claims,
        entry: &Entry,
        position: Option<Position>,
    ) -> Result<Option<String>> {
        let now = self.now()?;
        if now < entry.issued || now >= entry.expires {
            return Err(Error::CursorReset);
        }
        position
            .map(|position| self.sign(&Claims::new(claims.pin, entry, position)))
            .transpose()
    }

    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }

    fn sign(&self, claims: &Claims) -> Result<String> {
        let body =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).map_err(|_| Error::Unavailable)?);
        let prefix = format!("jsa1.{}.{}", self.instance, body);
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
        let ["jsa1", instance, body, tag] = parts.as_slice() else {
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
        if !claims.valid_version() {
            return Err(Error::InvalidCursor);
        }
        if now < claims.issued() || now >= claims.expires() {
            return Err(Error::CursorReset);
        }
        Ok(claims)
    }
}
