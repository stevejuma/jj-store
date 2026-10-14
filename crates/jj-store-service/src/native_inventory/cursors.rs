use super::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac as _};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{collections::BTreeMap, sync::Mutex};
use zeroize::Zeroizing;
type Signature = Hmac<Sha256>;
const DOMAIN: &[u8] = b"jj-store/native-inventory/v1\0";
const MAX_CURSOR: usize = 1024;

#[derive(Clone)]
pub(super) struct Entry {
    pub base: Arc<ManifestSnapshot>,
    copies: bool,
    scan_limit: usize,
    issued: u64,
    expires: u64,
}
impl Entry {
    pub fn new(base: Arc<ManifestSnapshot>, copies: bool, scan_limit: usize) -> Self {
        Self {
            base,
            copies,
            scan_limit,
            issued: 0,
            expires: 0,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Claims {
    pin: [u8; 16],
    pub after: m::RecordDigest,
}
pub(super) struct Cursors {
    limits: Limits,
    clock: Arc<dyn Clock>,
    instance: String,
    key: Zeroizing<[u8; 32]>,
    entries: Mutex<BTreeMap<[u8; 16], Entry>>,
}
impl Cursors {
    pub fn new(limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        let mut instance = [0; 16];
        let mut key = Zeroizing::new([0; 32]);
        getrandom::fill(&mut instance).map_err(|_| Error::Unavailable)?;
        getrandom::fill(key.as_mut()).map_err(|_| Error::Unavailable)?;
        Ok(Self {
            limits,
            clock,
            instance: hex::encode(instance),
            key,
            entries: Mutex::default(),
        })
    }
    fn now(&self) -> Result<u64> {
        self.clock.unix_seconds().map_err(|_| Error::Unavailable)
    }
    fn active(entry: &Entry, now: u64) -> bool {
        entry.issued <= now && now < entry.expires
    }
    fn sign(&self, claims: &Claims) -> Result<String> {
        let body =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(claims).map_err(|_| Error::Unavailable)?);
        let prefix = format!("jni1.{}.{}", self.instance, body);
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
    pub fn lookup(
        &self,
        repository: m::RepositoryId,
        copies: bool,
        scan_limit: usize,
        token: &str,
    ) -> Result<(Entry, Claims)> {
        if token.len() > MAX_CURSOR {
            return Err(Error::InvalidCursor);
        }
        let mut parts = token.split('.');
        let (Some("jni1"), Some(instance), Some(body), Some(tag), None) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return Err(Error::InvalidCursor);
        };
        if instance.len() != 32 || !instance.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Error::InvalidCursor);
        }
        if instance != self.instance {
            return Err(Error::CursorReset);
        }
        let tag = URL_SAFE_NO_PAD
            .decode(tag)
            .map_err(|_| Error::InvalidCursor)?;
        let mut mac =
            Signature::new_from_slice(self.key.as_ref()).map_err(|_| Error::Unavailable)?;
        mac.update(DOMAIN);
        mac.update(&token.as_bytes()[..token.rfind('.').ok_or(Error::InvalidCursor)?]);
        mac.verify_slice(&tag).map_err(|_| Error::InvalidCursor)?;
        let body = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::InvalidCursor)?;
        let claims: Claims = serde_json::from_slice(&body).map_err(|_| Error::InvalidCursor)?;
        let now = self.now()?;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.retain(|_, entry| Self::active(entry, now));
        let entry = entries
            .get(&claims.pin)
            .cloned()
            .ok_or(Error::CursorReset)?;
        if entry.base.manifest().repository != repository
            || entry.copies != copies
            || entry.scan_limit != scan_limit
        {
            return Err(Error::InvalidCursor);
        }
        Ok((entry, claims))
    }
    pub fn finish(
        &self,
        mut entry: Entry,
        claims: Option<&Claims>,
        after: Option<m::RecordDigest>,
    ) -> Result<Option<String>> {
        let now = self.now()?;
        if let Some(claims) = claims {
            if !Self::active(&entry, now) {
                return Err(Error::CursorReset);
            }
            return after
                .map(|after| {
                    self.sign(&Claims {
                        pin: claims.pin,
                        after,
                    })
                })
                .transpose();
        }
        let Some(after) = after else {
            return Ok(None);
        };
        entry.issued = now;
        entry.expires = now
            .checked_add(self.limits.ttl.as_secs())
            .ok_or(Error::Limit)?;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        entries.retain(|_, entry| Self::active(entry, now));
        if entries.len() >= self.limits.snapshots {
            return Err(Error::Busy);
        }
        let mut pin = [0; 16];
        getrandom::fill(&mut pin).map_err(|_| Error::Unavailable)?;
        if entries.contains_key(&pin) {
            return Err(Error::Unavailable);
        }
        let token = self.sign(&Claims { pin, after })?;
        entries.insert(pin, entry);
        Ok(Some(token))
    }
}
