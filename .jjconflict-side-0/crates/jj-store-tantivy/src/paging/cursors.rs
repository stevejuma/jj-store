use super::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use sha2::{Digest as _, Sha256};
use std::sync::Mutex;
use zeroize::Zeroizing;

type Mac = Hmac<Sha256>;
const DOMAIN: &[u8] = b"jj-store/search-cursor/v1\0";
const MAX_CURSOR: usize = 4096;
#[derive(Clone)]
pub(super) struct Entry {
    pub reader: PinnedReader,
    pub view: SearchView,
    query: [u8; 32],
    issued: u64,
    expires: u64,
}
impl Entry {
    fn nodes(&self) -> usize {
        match &self.view {
            SearchView::Historical => 0,
            SearchView::VisibleCommits(view) => view.commits().len(),
        }
    }
    fn view_digest(&self) -> Result<[u8; 32]> {
        let snapshot = match &self.view {
            SearchView::Historical => None,
            SearchView::VisibleCommits(view) => Some(view.snapshot()),
        };
        Ok(Sha256::digest(serde_json::to_vec(&snapshot).map_err(|_| Error::Unavailable)?).into())
    }
}
#[derive(Default)]
struct Registry {
    entries: BTreeMap<[u8; 16], Entry>,
}
impl Registry {
    fn prune(&mut self, now: u64) {
        self.entries
            .retain(|_, e| e.issued <= now && now < e.expires);
    }
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Claims {
    version: u32,
    id: [u8; 16],
    repository: m::RepositoryId,
    epoch: m::EpochId,
    position: m::WalPosition,
    artifact: m::RecordDigest,
    view: [u8; 32],
    query: [u8; 32],
    pub offset: usize,
    issued: u64,
    expires: u64,
}
impl Claims {
    fn new(id: [u8; 16], entry: &Entry, offset: usize) -> Result<Self> {
        let checkpoint = entry.reader.checkpoint();
        Ok(Self {
            version: 1,
            id,
            repository: checkpoint.source.repository,
            epoch: checkpoint.source.epoch,
            position: checkpoint.source.position,
            artifact: checkpoint.artifact,
            view: entry.view_digest()?,
            query: entry.query,
            offset,
            issued: entry.issued,
            expires: entry.expires,
        })
    }
}
struct Maintenance(tokio::task::AbortHandle);
impl Drop for Maintenance {
    fn drop(&mut self) {
        self.0.abort();
    }
}
pub(super) struct Cursors {
    registry: Arc<Mutex<Registry>>,
    limits: Limits,
    clock: Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    _maintenance: Maintenance,
}
impl Cursors {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        let mut key = Zeroizing::new([0; 32]);
        let mut instance = [0; 16];
        getrandom::fill(key.as_mut()).map_err(|_| Error::Unavailable)?;
        getrandom::fill(&mut instance).map_err(|_| Error::Unavailable)?;
        let registry = Arc::new(Mutex::new(Registry::default()));
        let weak = Arc::downgrade(&registry);
        let timer_clock = clock.clone();
        let period = limits.ttl.min(Duration::from_secs(30));
        let task = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::Unavailable)?
            .spawn(async move {
                loop {
                    tokio::time::sleep(period).await;
                    let Some(registry) = weak.upgrade() else {
                        break;
                    };
                    if let Ok(now) = timer_clock.unix_seconds()
                        && let Ok(mut registry) = registry.lock()
                    {
                        registry.prune(now);
                    }
                }
            });
        Ok(Self {
            registry,
            limits,
            clock,
            instance: URL_SAFE_NO_PAD.encode(instance),
            key,
            _maintenance: Maintenance(task.abort_handle()),
        })
    }
    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }
    pub fn prune(&self) -> Result<()> {
        self.registry
            .lock()
            .map_err(|_| Error::Unavailable)?
            .prune(self.now()?);
        Ok(())
    }
    fn sign(&self, claims: &Claims) -> Result<String> {
        let body =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).map_err(|_| Error::Unavailable)?);
        let prefix = format!("jss1.{}.{body}", self.instance);
        let mut mac = Mac::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
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
        let ["jss1", instance, body, tag] = parts.as_slice() else {
            return Err(Error::InvalidCursor);
        };
        if URL_SAFE_NO_PAD
            .decode(instance)
            .map_err(|_| Error::InvalidCursor)?
            .len()
            != 16
        {
            return Err(Error::InvalidCursor);
        }
        if *instance != self.instance {
            return Err(Error::CursorReset);
        }
        let tag = URL_SAFE_NO_PAD
            .decode(tag)
            .map_err(|_| Error::InvalidCursor)?;
        let prefix = &token[..token.rfind('.').ok_or(Error::InvalidCursor)?];
        let mut mac = Mac::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(prefix.as_bytes());
        mac.verify_slice(&tag).map_err(|_| Error::InvalidCursor)?;
        let body = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::InvalidCursor)?;
        let claims: Claims = serde_json::from_slice(&body).map_err(|_| Error::InvalidCursor)?;
        if claims.version != 1 || claims.offset == 0 || claims.offset > self.limits.query.max_offset
        {
            return Err(Error::InvalidCursor);
        }
        self.live(claims.issued, claims.expires)?;
        Ok(claims)
    }
    fn live(&self, issued: u64, expires: u64) -> Result<()> {
        let now = self.now()?;
        if now < issued || now >= expires {
            return Err(Error::CursorReset);
        }
        Ok(())
    }
    pub fn retain(
        &self,
        reader: PinnedReader,
        view: SearchView,
        request: Request,
        after: usize,
    ) -> Result<(String, [u8; 16])> {
        let now = self.now()?;
        let entry = Entry {
            reader,
            view,
            query: fingerprint(&request)?,
            issued: now,
            expires: now
                .checked_add(self.limits.ttl.as_secs())
                .ok_or(Error::Limit)?,
        };
        let mut registry = self.registry.lock().map_err(|_| Error::Unavailable)?;
        registry.prune(now);
        let nodes = registry
            .entries
            .values()
            .try_fold(entry.nodes(), |count, e| count.checked_add(e.nodes()))
            .ok_or(Error::Limit)?;
        if registry.entries.len() >= self.limits.pins || nodes > self.limits.view_nodes {
            return Err(Error::Busy);
        }
        let mut id = [0; 16];
        getrandom::fill(&mut id).map_err(|_| Error::Unavailable)?;
        if registry.entries.contains_key(&id) {
            return Err(Error::Unavailable);
        }
        let token = self.sign(&Claims::new(id, &entry, after)?)?;
        registry.entries.insert(id, entry);
        Ok((token, id))
    }
    pub fn release(&self, id: [u8; 16]) -> Result<()> {
        self.registry
            .lock()
            .map_err(|_| Error::Unavailable)?
            .entries
            .remove(&id);
        Ok(())
    }
    pub fn lookup(
        &self,
        token: &str,
        current: &HeadSnapshot,
        request: &Request,
    ) -> Result<(Claims, Entry)> {
        let claims = self.verify(token)?;
        if claims.repository != current.repo_id || claims.query != fingerprint(request)? {
            return Err(Error::InvalidCursor);
        }
        if claims.epoch != current.epoch {
            return Err(Error::EpochChanged);
        }
        let entry = {
            let mut registry = self.registry.lock().map_err(|_| Error::Unavailable)?;
            registry.prune(self.now()?);
            registry
                .entries
                .get(&claims.id)
                .cloned()
                .ok_or(Error::CursorReset)?
        };
        if claims != Claims::new(claims.id, &entry, claims.offset)? {
            return Err(Error::InvalidCursor);
        }
        Ok((claims, entry))
    }
    pub fn next(
        &self,
        claims: &Claims,
        entry: &Entry,
        after: Option<usize>,
    ) -> Result<Option<String>> {
        self.live(entry.issued, entry.expires)?;
        after
            .map(|offset| self.sign(&Claims::new(claims.id, entry, offset)?))
            .transpose()
    }
}
fn fingerprint(request: &Request) -> Result<[u8; 32]> {
    let mut hash = Sha256::new_with_prefix(DOMAIN);
    hash.update(serde_json::to_vec(request).map_err(|_| Error::Invalid)?);
    Ok(hash.finalize().into())
}
