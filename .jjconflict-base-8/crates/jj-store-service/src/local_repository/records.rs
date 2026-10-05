//! Git carrier for existing canonical, repository-bound immutable records.
use jj_store_jj::local_metadata::{GitOid, MetadataStore, RetainedKind};
use jj_store_model::RepositoryId;
use jj_store_object::{self as s, ObjectStore};
use std::{collections::BTreeMap, sync::Mutex};
use tokio::io::AsyncReadExt as _;

pub(crate) struct GitRecords {
    pub repository: RepositoryId,
    metadata: MetadataStore,
    records: Mutex<BTreeMap<String, GitOid>>,
}
impl GitRecords {
    pub fn new(
        repository: RepositoryId,
        metadata: MetadataStore,
        records: BTreeMap<String, GitOid>,
    ) -> s::Result<Self> {
        let value = Self {
            repository,
            metadata,
            records: Mutex::new(records),
        };
        if value.records.lock().unwrap().len() > 16_384 {
            return Err(s::Error::Limit);
        }
        for key in value.records.lock().unwrap().keys() {
            value.admit(&s::Key::new(key.clone())?)?;
        }
        Ok(value)
    }
    fn admit(&self, key: &s::Key) -> s::Result<()> {
        let prefix = format!("repos/{}/", self.repository);
        let remainder = key
            .as_str()
            .strip_prefix(&prefix)
            .ok_or(s::Error::Invalid("local record repository"))?;
        let (kind, name) = remainder
            .split_once('/')
            .ok_or(s::Error::Invalid("local record namespace"))?;
        if !matches!(
            kind,
            "agent-roots" | "agent-objects" | "review-roots" | "review-objects"
        ) {
            return Err(s::Error::Invalid("immutable local domain record required"));
        }
        let id = name
            .strip_suffix(".json")
            .ok_or(s::Error::Invalid("local record extension"))?;
        if id.parse::<jj_store_model::RecordDigest>().is_err() {
            return Err(s::Error::Invalid("local record identity"));
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
    fn bytes(&self, key: &s::Key) -> s::Result<(GitOid, Vec<u8>)> {
        self.admit(key)?;
        let id = *self
            .records
            .lock()
            .unwrap()
            .get(key.as_str())
            .ok_or(s::Error::NotFound)?;
        let bytes = self
            .metadata
            .read_staged_blob(id)
            .map_err(|_| s::Error::Response("invalid local immutable blob"))?;
        Ok((id, bytes))
    }
}
#[async_trait::async_trait]
impl ObjectStore for GitRecords {
    async fn head(&self, key: &s::Key) -> s::Result<s::Metadata> {
        let (id, bytes) = self.bytes(key)?;
        Ok(s::Metadata {
            len: bytes.len() as u64,
            version: s::VersionToken::new(id.to_string())?,
        })
    }
    async fn get(&self, key: &s::Key, range: Option<s::ByteRange>) -> s::Result<s::Download> {
        let (id, mut bytes) = self.bytes(key)?;
        if let Some(range) = range {
            let start = usize::try_from(range.start()).map_err(|_| s::Error::Range)?;
            let end = usize::try_from(range.end()).map_err(|_| s::Error::Range)?;
            bytes = bytes.get(start..end).ok_or(s::Error::Range)?.to_vec();
        }
        Ok(s::Download {
            metadata: s::Metadata {
                len: bytes.len() as u64,
                version: s::VersionToken::new(id.to_string())?,
            },
            reader: Box::pin(std::io::Cursor::new(bytes)),
        })
    }
    async fn put(
        &self,
        key: &s::Key,
        condition: s::Condition,
        upload: &s::StagedUpload,
    ) -> s::Result<s::WriteOutcome> {
        self.admit(key)?;
        if !matches!(condition, s::Condition::Absent) {
            return Err(s::Error::Invalid(
                "immutable local records require absent condition",
            ));
        }
        if upload.len() > 16 * 1024 * 1024 {
            return Err(s::Error::Limit);
        }
        let mut bytes = Vec::new();
        upload
            .reader()?
            .take(16 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .await?;
        if bytes.len() as u64 != upload.len() {
            return Err(s::Error::Response("local upload length"));
        }
        let blob = self
            .metadata
            .stage_blob(&bytes)
            .map_err(|_| s::Error::Transport)?;
        let mut records = self.records.lock().unwrap();
        if records.contains_key(key.as_str()) {
            return Ok(s::WriteOutcome::PreconditionFailed);
        }
        if records.len() >= 16_384 {
            return Err(s::Error::Limit);
        }
        records.insert(key.as_str().into(), blob);
        Ok(s::WriteOutcome::Applied(s::VersionToken::new(
            blob.to_string(),
        )?))
    }
}
