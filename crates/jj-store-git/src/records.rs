//! Existing repository-bound canonical records carried directly by immutable Git blobs.
use crate::metadata::{GitOid, MetadataStore, RetainedKind};
use jj_store_core::{Error, Result, record_format as format};
use jj_store_model::{AttemptId, RecordDigest, RepositoryId};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordKind {
    AgentRoot,
    AgentObject,
    ReviewRoot,
    ReviewObject,
}
impl RecordKind {
    fn path(self) -> &'static str {
        match self {
            Self::AgentRoot => "agent-roots",
            Self::AgentObject => "agent-objects",
            Self::ReviewRoot => "review-roots",
            Self::ReviewObject => "review-objects",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordRef {
    pub kind: RecordKind,
    pub digest: RecordDigest,
    pub attempt: Option<AttemptId>,
}
impl RecordRef {
    pub fn key(self, repository: RepositoryId) -> Result<String> {
        if self.attempt.is_some() {
            return Err(Error::Corrupt("Git immutable record attempt"));
        }
        Ok(format!(
            "repos/{repository}/{}/{}.json",
            self.kind.path(),
            self.digest
        ))
    }
}
pub struct GitRecords {
    pub repository: RepositoryId,
    metadata: MetadataStore,
    records: Mutex<BTreeMap<String, GitOid>>,
}
impl GitRecords {
    pub fn new(
        repository: RepositoryId,
        metadata: MetadataStore,
        records: BTreeMap<String, GitOid>,
    ) -> Result<Self> {
        if records.len() > 16_384 {
            return Err(Error::Limit("local record inventory"));
        }
        let value = Self {
            repository,
            metadata,
            records: Mutex::new(records),
        };
        for key in value.records.lock().unwrap().keys() {
            value.admit(key)?;
        }
        Ok(value)
    }
    fn admit(&self, key: &str) -> Result<()> {
        let prefix = format!("repos/{}/", self.repository);
        let (kind, name) = key
            .strip_prefix(&prefix)
            .and_then(|s| s.split_once('/'))
            .ok_or(Error::Corrupt("local record repository/namespace"))?;
        if !matches!(
            kind,
            "agent-roots" | "agent-objects" | "review-roots" | "review-objects"
        ) || name
            .strip_suffix(".json")
            .and_then(|s| s.parse::<RecordDigest>().ok())
            .is_none()
        {
            return Err(Error::Corrupt("local record kind/identity"));
        }
        Ok(())
    }
    pub fn inventory(&self) -> BTreeMap<String, GitOid> {
        self.records.lock().unwrap().clone()
    }
    pub fn retained(&self) -> BTreeMap<GitOid, RetainedKind> {
        self.records
            .lock()
            .unwrap()
            .values()
            .map(|id| (*id, RetainedKind::Blob))
            .collect()
    }
    fn bytes(&self, reference: RecordRef) -> Result<Vec<u8>> {
        let key = reference.key(self.repository)?;
        self.admit(&key)?;
        let id = *self
            .records
            .lock()
            .unwrap()
            .get(&key)
            .ok_or(Error::NotFound)?;
        let bytes = self
            .metadata
            .read_staged_blob(id)
            .map_err(|_| Error::Corrupt("local immutable blob"))?;
        if format::digest(&bytes) != reference.digest {
            return Err(Error::Corrupt("control record checksum"));
        }
        Ok(bytes)
    }
}
pub struct Records {
    store: Arc<GitRecords>,
    repository: RepositoryId,
}
impl Records {
    pub fn new(store: Arc<GitRecords>, repository: RepositoryId) -> Self {
        Self { store, repository }
    }
    pub async fn put<T: Serialize>(
        &self,
        kind: RecordKind,
        attempt: Option<AttemptId>,
        payload: &T,
    ) -> Result<RecordRef> {
        if self.store.repository != self.repository || attempt.is_some() {
            return Err(Error::Corrupt("Git record binding"));
        }
        let bytes = format::encode(
            &format::Envelope {
                format: 1,
                repository: self.repository,
                kind,
                payload,
            },
            format::MAX_RECORD_BYTES,
        )?;
        let reference = RecordRef {
            kind,
            digest: format::digest(&bytes),
            attempt,
        };
        let key = reference.key(self.repository)?;
        let blob = self
            .store
            .metadata
            .stage_blob(&bytes)
            .map_err(|_| Error::Unavailable("Git blob staging".into()))?;
        let mut records = self.store.records.lock().unwrap();
        if let Some(old) = records.get(&key) {
            if *old != blob {
                return Err(Error::Corrupt("immutable record collision"));
            }
        } else {
            if records.len() >= 16_384 {
                return Err(Error::Limit("local records"));
            }
            records.insert(key, blob);
        }
        Ok(reference)
    }
    pub async fn get<T: Serialize + DeserializeOwned>(
        &self,
        reference: RecordRef,
        expected: RecordKind,
    ) -> Result<T> {
        if reference.kind != expected || self.store.repository != self.repository {
            return Err(Error::Corrupt("record kind/binding"));
        }
        {
            let envelope: format::Envelope<T, RecordKind> =
                format::decode(&self.store.bytes(reference)?)?;
            if envelope.format != 1
                || envelope.repository != self.repository
                || envelope.kind != expected
            {
                return Err(Error::Corrupt("record envelope"));
            }
            Ok(envelope.payload)
        }
    }
}
