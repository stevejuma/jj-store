//! Transport-independent exact Git representation catalog and strong edges.
use super::*;
use jj_store_model::{CommitId, FileId};

pub const GIT_PROFILE: &str = "jj-git-sha1-0.45.1-v1";
const MAX_GIT_RECORDS: usize = 100_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitObjectKind {
    Blob,
    Tree,
    Commit,
    Tag,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitObjectV1 {
    pub oid: String,
    pub kind: GitObjectKind,
    pub body: FileId,
    pub bytes: u64,
    pub dependencies: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitRepresentationV1 {
    pub native_commit: CommitId,
    pub git_commit: String,
    pub profile: String,
    pub imported_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum GitRecordV1 {
    Object(GitObjectV1),
    Representation(GitRepresentationV1),
}
impl GitRecordV1 {
    pub fn key(&self) -> Result<RecordDigest> {
        self.validate()?;
        let identity = match self {
            Self::Object(v) => format!("object:{}", v.oid),
            Self::Representation(v) => format!(
                "representation:{}:{}:{}",
                v.profile, v.native_commit, v.git_commit
            ),
        };
        let mut hash = Sha256::new_with_prefix(b"jj-store/git-record/v1\0");
        hash.update(identity);
        Ok(RecordDigest::new(hash.finalize().into()))
    }
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Object(v)
                if valid_git_oid(&v.oid)
                    && v.dependencies.len() <= MAX_GIT_RECORDS
                    && v.dependencies.iter().all(|id| valid_git_oid(id))
                    && v.bytes <= jj_store_object::staging::MAX_SINGLE_PUT =>
            {
                Ok(())
            }
            Self::Representation(v) if valid_git_oid(&v.git_commit) && v.profile == GIT_PROFILE => {
                Ok(())
            }
            _ => Err(Error::Corrupt("Git representation record")),
        }
    }
}
pub fn valid_git_oid(oid: &str) -> bool {
    oid.len() == 40
        && oid
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitRootV1 {
    pub schema_version: u32,
    pub profile: String,
    pub records: Option<RecordRef>,
}
#[derive(Clone, Debug)]
pub struct GitStoreSnapshot {
    pub repository: RepositoryId,
    pub reference: RecordRef,
    pub values: BTreeMap<RecordDigest, GitRecordV1>,
    pub(crate) records: BTreeSet<RecordRef>,
    pub native: BTreeSet<ObjectKey>,
}
impl Coordinator {
    pub async fn empty_git_store(&self) -> Result<GitStoreSnapshot> {
        let root = GitRootV1 {
            schema_version: 1,
            profile: GIT_PROFILE.into(),
            records: None,
        };
        let reference = self.records.put(RecordKind::GitRoot, None, &root).await?;
        self.git_store(reference).await
    }
    pub async fn git_store(&self, reference: RecordRef) -> Result<GitStoreSnapshot> {
        let root: GitRootV1 = self.records.get(reference, RecordKind::GitRoot).await?;
        if root.schema_version != 1 || root.profile != GIT_PROFILE {
            return Err(Error::Corrupt("Git root profile"));
        }
        let closure = self.catalog.closure(root.records, MAX_GIT_RECORDS).await?;
        let mut records = closure.pages;
        records.insert(reference);
        let mut values = BTreeMap::new();
        let mut native = BTreeSet::new();
        let mut objects = BTreeMap::new();
        for (key, reference) in closure.entries {
            if reference.kind != RecordKind::GitObject || reference.attempt.is_some() {
                return Err(Error::Corrupt("Git catalog reference"));
            }
            let value: GitRecordV1 = self.records.get(reference, RecordKind::GitObject).await?;
            if value.key()? != key {
                return Err(Error::Corrupt("Git catalog key"));
            }
            match &value {
                GitRecordV1::Object(v) => {
                    native.insert(ObjectKey::File(v.body));
                    objects.insert(v.oid.clone(), v.clone());
                }
                GitRecordV1::Representation(v) => {
                    native.insert(ObjectKey::Commit(v.native_commit));
                }
            }
            records.insert(reference);
            values.insert(key, value);
        }
        for value in values.values() {
            match value {
                GitRecordV1::Object(v)
                    if v.dependencies.iter().any(|id| !objects.contains_key(id)) =>
                {
                    return Err(Error::Corrupt("Git object closure"));
                }
                GitRecordV1::Representation(v)
                    if !objects
                        .get(&v.git_commit)
                        .is_some_and(|o| o.kind == GitObjectKind::Commit) =>
                {
                    return Err(Error::Corrupt("Git commit representation closure"));
                }
                _ => {}
            }
        }
        Ok(GitStoreSnapshot {
            repository: self.records.repository,
            reference,
            values,
            records,
            native,
        })
    }
    pub async fn stage_git_records(
        &self,
        base: &GitStoreSnapshot,
        records: &[GitRecordV1],
    ) -> Result<GitStoreSnapshot> {
        if base.repository != self.records.repository {
            return Err(Error::Corrupt("Git repository binding"));
        }
        if records.len() > MAX_GIT_RECORDS {
            return Err(Error::Limit("Git records"));
        }
        let root: GitRootV1 = self
            .records
            .get(base.reference, RecordKind::GitRoot)
            .await?;
        let mut edits = BTreeMap::new();
        for value in records {
            let key = value.key()?;
            if let Some(existing) = base.values.get(&key) {
                if existing != value {
                    return Err(Error::Corrupt("Git immutable mapping conflict"));
                }
                continue;
            }
            let reference = self.records.put(RecordKind::GitObject, None, value).await?;
            if edits
                .insert(key, reference)
                .is_some_and(|old| old != reference)
            {
                return Err(Error::Corrupt("Git batch mapping conflict"));
            }
        }
        let records = self.catalog.update(root.records, edits).await?;
        let root = GitRootV1 { records, ..root };
        let reference = self.records.put(RecordKind::GitRoot, None, &root).await?;
        self.git_store(reference).await
    }
}
