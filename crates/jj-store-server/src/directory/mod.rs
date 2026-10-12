//! Installation authority: immutable records/media, one CAS-selected directory.
mod audit;
mod mutation;
mod persistence;
mod reads;
#[cfg(test)]
mod tests;
mod validation;
use jj_store_model::{PrincipalId, RequestId, TokenId};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, VersionToken, WriteOutcome};
use jj_store_service::directory::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use subtle::ConstantTimeEq as _;
const MAX_RECORD: u64 = 4 * 1024 * 1024;
const MAX_ENTRIES: usize = 4096;
const MAX_RECEIPTS: usize = 8192;
#[derive(Clone)]
pub struct Manager {
    store: Arc<dyn ObjectStore>,
    installation: String,
    local: Option<PrincipalId>,
    gate: Arc<tokio::sync::Mutex<()>>,
    signals: jj_store_service::freshness::Signals,
}
impl Manager {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        installation: String,
        local: Option<PrincipalId>,
    ) -> Result<Self> {
        if installation.len() != 32 || !validation::hexadecimal(&installation) {
            return Err(Error::Invalid);
        }
        Ok(Self {
            store,
            installation,
            local,
            gate: Default::default(),
            signals: Default::default(),
        })
    }
    pub fn signals(&self) -> jj_store_service::freshness::Signals {
        self.signals.clone()
    }
    fn key(&self, suffix: &str) -> Result<Key> {
        Key::new(format!(
            "installations/{}/directory/{suffix}",
            self.installation
        ))
        .map_err(|_| Error::Invalid)
    }
    fn digest(bytes: &[u8]) -> String {
        hex::encode(Sha256::digest(bytes))
    }
    fn actor(&self, root: &Root, caller: &Caller) -> Result<PrincipalId> {
        match caller {
            Caller::Local(id) if Some(*id) == self.local => Ok(*id),
            Caller::Operator(id) => Ok(*id),
            Caller::Token(token) => {
                let binding = root.bindings.get(&token.id()).ok_or(Error::Denied)?;
                let now = jj_store_service::auth::Clock::unix_seconds(
                    &jj_store_service::auth::SystemClock,
                )
                .map_err(|_| Error::Unavailable)?;
                let digest = token.installation_verifier(&self.installation);
                let mut expected = [0u8; 32];
                hex::decode_to_slice(&binding.verifier, &mut expected)
                    .map_err(|_| Error::Unavailable)?;
                if binding.revoked
                    || !bool::from(digest.ct_eq(&expected))
                    || binding.expires_at.is_some_and(|at| now >= at)
                {
                    return Err(Error::Denied);
                }
                Ok(binding.principal)
            }
            _ => Err(Error::Denied),
        }
    }
    fn personal(&self, root: &Root, caller: &Caller) -> Result<PrincipalId> {
        if matches!(caller, Caller::Operator(_)) {
            return Err(Error::Denied);
        }
        self.actor(root, caller)
    }
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Root {
    profiles: BTreeMap<PrincipalId, String>,
    bindings: BTreeMap<TokenId, Binding>,
    emojis: BTreeMap<RequestId, String>,
    media: BTreeMap<String, MediaRecord>,
    receipts: BTreeMap<String, String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    revoked: bool,
    principal: PrincipalId,
    verifier: String,
    expires_at: Option<u64>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MediaRecord {
    media_type: jj_collab::ImageMediaTypeV1,
    owners: BTreeSet<PrincipalId>,
    preview: String,
    published: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format: u32,
    installation: String,
    generation: u64,
    root: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope<T> {
    format: u32,
    installation: String,
    kind: String,
    data: T,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedProfile {
    principal: PrincipalId,
    details: ProfileDetails,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedEmoji {
    id: RequestId,
    owner: PrincipalId,
    shortcode: String,
    asset: String,
    preview: String,
    retired: bool,
}
impl SavedEmoji {
    fn view(self, revision: String) -> LibraryEmoji {
        LibraryEmoji {
            id: self.id,
            owner: self.owner,
            shortcode: self.shortcode,
            asset: self.asset,
            preview: self.preview,
            retired: self.retired,
            revision,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedReceipt {
    intent: String,
    receipt: DirectoryReceipt,
}
struct Selection {
    root: Root,
    generation: u64,
    version: Option<VersionToken>,
}
