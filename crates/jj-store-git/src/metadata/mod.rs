//! Companion metadata publication for an existing SHA-1 Git object database.
//! This store neither replaces GitBackend nor publishes JJ operation heads.
mod blobs;
mod evidence_files;
#[doc(hidden)]
pub mod objects;
mod prepare;
mod types;
use gix::refs::{
    Target,
    transaction::{Change, LogChange, PreviousValue, RefEdit},
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
pub use types::*;

/// Header in the qualified SHA-1 JJ 0.45 profile.
pub const JJ_TREES_COMMIT_HEADER: &str = "jj:trees";
pub const STATE_REF: &str = "refs/lapis/state/v1";
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid local metadata: {0}")]
    Invalid(&'static str),
    #[error("local metadata exceeds admission limits")]
    Limit,
    #[error("local metadata selection is stale")]
    Stale,
    #[error("request identity was used for a different intent")]
    IntentConflict,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("local Git metadata access: {0}")]
    Other(String),
}
impl Error {
    pub fn other(value: impl std::fmt::Display) -> Self {
        Self::Other(value.to_string())
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub object_bytes: usize,
    pub root_bytes: usize,
    pub records: usize,
    pub history: usize,
    pub objects: usize,
    pub closure_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            object_bytes: 16 * 1024 * 1024,
            root_bytes: 4 * 1024 * 1024,
            records: 16_384,
            history: 100_000,
            objects: 100_000,
            closure_bytes: 256 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Debug)]
pub struct MetadataStore {
    git_dir: PathBuf,
    limits: Limits,
    evidence: Option<std::sync::Arc<dyn crate::evidence::EvidenceValidator>>,
}
impl MetadataStore {
    pub fn open(path: &Path, limits: Limits) -> Result<Self> {
        if limits.object_bytes == 0
            || limits.root_bytes == 0
            || limits.records == 0
            || limits.history == 0
            || limits.objects == 0
            || limits.closure_bytes == 0
        {
            return Err(Error::Invalid("zero admission limit"));
        }
        let repo = gix::open(path).map_err(Error::other)?;
        if repo.object_hash() != gix::hash::Kind::Sha1 || repo.is_shallow() {
            return Err(Error::Invalid("requires complete SHA-1 Git repository"));
        }
        Ok(Self {
            git_dir: repo.git_dir().to_path_buf(),
            limits,
            evidence: None,
        })
    }
    pub fn with_evidence_validator(
        mut self,
        validator: std::sync::Arc<dyn crate::evidence::EvidenceValidator>,
    ) -> Self {
        self.evidence = Some(validator);
        self
    }
    pub(crate) fn evidence_validator(&self) -> Option<&dyn crate::evidence::EvidenceValidator> {
        self.evidence.as_deref()
    }
    pub fn limits(&self) -> Limits {
        self.limits
    }
    pub fn git_dir(&self) -> &Path {
        &self.git_dir
    }
    fn selected_commit(&self) -> Result<Option<GitOid>> {
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        let Some(reference) = repo.try_find_reference(STATE_REF).map_err(Error::other)? else {
            return Ok(None);
        };
        let id = reference
            .try_id()
            .ok_or(Error::Invalid("symbolic owned state ref"))?;
        GitOid::from_git(id.detach()).map(Some)
    }
    /// Current identity only: no domain/evidence closure is admitted by this observation.
    pub fn identity(&self) -> Result<Option<Identity>> {
        self.selected_commit()?
            .map(|commit| {
                let selected = self.root_at(commit)?;
                Ok(Identity {
                    commit,
                    repository: selected.repository(),
                })
            })
            .transpose()
    }
    pub fn read(&self) -> Result<Option<Snapshot>> {
        self.selected_commit()?
            .map(|commit| self.read_at(commit))
            .transpose()
    }
    pub fn read_at(&self, commit: GitOid) -> Result<Snapshot> {
        let selected = self.root_at(commit)?;
        let root = &selected.root;
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        for selection in root.domains.values() {
            objects::object(
                &repo,
                selection.object,
                gix::objs::Kind::Blob,
                self.limits.object_bytes,
            )?;
        }
        if let Some(previous) = root.previous {
            objects::object(
                &repo,
                previous,
                gix::objs::Kind::Commit,
                self.limits.root_bytes,
            )?;
        }
        let extras = objects::audit(&repo, &root.retained, self.limits, false)?;
        if extras
            .iter()
            .any(|(id, kind)| root.retained.get(id) != Some(kind))
        {
            return Err(Error::Invalid("missing JJ conflict retention"));
        }
        Ok(selected)
    }
    /// Verified root framing is sufficient to observe identity, not to read domains.
    fn root_at(&self, commit: GitOid) -> Result<Snapshot> {
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        let bytes = objects::object(
            &repo,
            commit,
            gix::objs::Kind::Commit,
            self.limits.root_bytes,
        )?;
        let decoded = gix::objs::CommitRef::from_bytes(&bytes, gix::hash::Kind::Sha1)
            .map_err(Error::other)?;
        let tree_id = GitOid::from_git(decoded.tree())?;
        let links = objects::tree_links(&repo, tree_id, self.limits.root_bytes)?;
        let &(state_id, mode) = links.get("state").ok_or(Error::Invalid("missing state"))?;
        if mode != gix::objs::tree::EntryKind::Blob {
            return Err(Error::Invalid("state mode"));
        }
        let state_bytes = objects::object(
            &repo,
            state_id,
            gix::objs::Kind::Blob,
            self.limits.root_bytes,
        )?;
        let root: RootV1 = serde_json::from_slice(&state_bytes).map_err(Error::other)?;
        if serde_json::to_vec(&root).map_err(Error::other)? != state_bytes {
            return Err(Error::Invalid("noncanonical state"));
        }
        self.validate_root(&root)?;
        let mut expected =
            BTreeMap::from([("state".into(), (state_id, gix::objs::tree::EntryKind::Blob))]);
        for (&domain, selection) in &root.domains {
            expected.insert(
                domain.name().into(),
                (selection.object, gix::objs::tree::EntryKind::Blob),
            );
        }
        let retained = objects::retained_links(&root.retained);
        if !retained.is_empty() {
            let &(retained_id, mode) = links
                .get("retained")
                .ok_or(Error::Invalid("missing retention tree"))?;
            if mode != gix::objs::tree::EntryKind::Tree
                || objects::tree_links(&repo, retained_id, self.limits.root_bytes)? != retained
            {
                return Err(Error::Invalid("retention tree mismatch"));
            }
            expected.insert("retained".into(), (retained_id, mode));
        }
        if links != expected {
            return Err(Error::Invalid("metadata tree mismatch"));
        }
        let actual_parents = decoded
            .parents()
            .map(GitOid::from_git)
            .collect::<Result<Vec<_>>>()?;
        if actual_parents != objects::parents(root.previous, &root.retained) {
            return Err(Error::Invalid("retention parents mismatch"));
        }
        Ok(Snapshot { commit, root })
    }
    fn validate_root(&self, root: &RootV1) -> Result<()> {
        if !matches!(root.version, 1 | 2)
            || (root.version == 1
                && root
                    .domains
                    .keys()
                    .any(|d| matches!(d, Domain::Thread | Domain::Reaction)))
            || root.sequence == 0
            || root.domains.is_empty()
            || (root.sequence == 1) != root.previous.is_none()
        {
            return Err(Error::Invalid("root version/sequence"));
        }
        if root.receipts.len() > self.limits.records || root.retained.len() > self.limits.records {
            return Err(Error::Limit);
        }
        for selection in root.domains.values() {
            if selection.version == 0 || selection.version > root.sequence {
                return Err(Error::Invalid("domain version"));
            }
        }
        for (key, receipt) in &root.receipts {
            if key != &Receipt::key(receipt.actor, receipt.request)
                || receipt.repository != root.repository
                || receipt.sequence == 0
                || receipt.sequence > root.sequence
                || receipt.result.len() > 1024 * 1024
                || (root.version == 1
                    && (!receipt.selections.is_empty()
                        || matches!(receipt.domain, Domain::Thread | Domain::Reaction)))
                || receipt.selection.version == 0
                || receipt.selection.version > receipt.sequence
                || (!receipt.selections.is_empty() && receipt.selections.len() < 2)
                || !root.domains.contains_key(&receipt.domain)
                || (!receipt.selections.is_empty()
                    && receipt.selections.get(&receipt.domain) != Some(&receipt.selection))
                || receipt
                    .selections
                    .values()
                    .any(|s| s.version == 0 || s.version > receipt.sequence)
            {
                return Err(Error::Invalid("receipt identity/sequence"));
            }
        }
        Ok(())
    }
    pub fn read_domain(&self, snapshot: &Snapshot, domain: Domain) -> Result<Option<Vec<u8>>> {
        let Some(selection) = snapshot.selection(domain) else {
            return Ok(None);
        };
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        objects::object(
            &repo,
            selection.object,
            gix::objs::Kind::Blob,
            self.limits.object_bytes,
        )
        .map(Some)
    }
    pub fn history(&self, snapshot: &Snapshot) -> Result<Vec<Snapshot>> {
        let mut history = vec![snapshot.clone()];
        let mut bytes = serde_json::to_vec(&snapshot.root)
            .map_err(Error::other)?
            .len();
        while let Some(previous) = history.last().and_then(|s| s.root.previous) {
            if history.len() >= self.limits.history {
                return Err(Error::Limit);
            }
            let next = self.read_at(previous)?;
            bytes = bytes
                .checked_add(serde_json::to_vec(&next.root).map_err(Error::other)?.len())
                .ok_or(Error::Limit)?;
            if bytes > self.limits.closure_bytes {
                return Err(Error::Limit);
            }
            let current = history.last().unwrap();
            if next.repository() != current.repository()
                || next.sequence().checked_add(1) != Some(current.sequence())
            {
                return Err(Error::Invalid("historical sequence"));
            }
            if next
                .root
                .receipts
                .iter()
                .any(|(id, receipt)| current.root.receipts.get(id) != Some(receipt))
            {
                return Err(Error::Invalid("historical receipt changed"));
            }
            history.push(next);
        }
        Ok(history)
    }
    pub fn publish(&self, candidate: Candidate) -> Result<PublishOutcome> {
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        self.read_at(candidate.snapshot.commit)?;
        let expected = candidate
            .expected
            .map_or(PreviousValue::MustNotExist, |id| {
                PreviousValue::MustExistAndMatch(Target::Object(id.0))
            });
        let edit = RefEdit {
            name: STATE_REF.try_into().map_err(Error::other)?,
            deref: false,
            change: Change::Update {
                log: LogChange::default(),
                expected,
                new: Target::Object(candidate.snapshot.commit.0),
            },
        };
        let result = repo.edit_references_as([edit], None);
        if result.is_ok() {
            let path = repo.common_dir().join(STATE_REF);
            if std::fs::File::open(&path)
                .and_then(|file| file.sync_all())
                .is_err()
            {
                return Ok(PublishOutcome::Unknown);
            }
            let mut dir = path.parent();
            while let Some(parent) = dir {
                if objects::sync_dir(parent).is_err() {
                    return Ok(PublishOutcome::Unknown);
                }
                if parent == repo.common_dir() {
                    break;
                }
                dir = parent.parent();
            }
            return Ok(PublishOutcome::Committed(candidate.receipt));
        }
        match self.read() {
            Ok(Some(snapshot)) => {
                if let Some(receipt) =
                    snapshot.receipt(candidate.receipt.actor, candidate.receipt.request)
                {
                    if receipt.intent != candidate.receipt.intent {
                        return Err(Error::IntentConflict);
                    }
                    return Ok(PublishOutcome::Committed(receipt.clone()));
                }
                if Some(snapshot.commit) != candidate.expected {
                    return Ok(PublishOutcome::Conflict);
                }
            }
            Ok(None) if candidate.expected.is_some() => return Ok(PublishOutcome::Conflict),
            _ => {}
        }
        Ok(PublishOutcome::Unknown)
    }
}

pub fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= 4096
        && !path.contains('\\')
        && !path.chars().any(char::is_control)
        && path.split('/').all(|p| {
            !p.is_empty()
                && p != "."
                && p != ".."
                && !p.eq_ignore_ascii_case(".jj")
                && !p.eq_ignore_ascii_case(".git")
        })
}
