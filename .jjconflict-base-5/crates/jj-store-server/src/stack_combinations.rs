//! Private device combination state; code publication and checkout are separate.
use crate::{capture_journal_files as files, stack_command_journal as j, stack_preparation as p};
use jj_store_jj::local_metadata::{GitOid, StackPreparationKey, StackRetention};
use jj_store_model as m;
use jj_store_service::stack_workspace as s;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::Path};
mod authority;
mod catalog;
mod linkage;
pub use linkage::CombinationBindingV1;
mod validation;
use catalog::Catalog;

pub const MAX_COMBINATIONS: usize = 64;
pub const MAX_STACKS: usize = 4096;
pub const MAX_RECORDS: usize = 128;
pub const MAX_BYTES: usize = 8 * 1024 * 1024;
const LAYOUT: files::Layout = files::Layout {
    lock: "combinations.lock",
    temporary: ".stack-combination-",
    max_bytes: MAX_BYTES,
};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedCode {
    pub key: StackPreparationKey,
    pub commit: GitOid,
    pub archive: m::RecordDigest,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppliedStack {
    pub id: s::StackId,
    pub bookmark: String,
    pub head: m::GitSha1Id,
    pub base: m::GitSha1Id,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Combination {
    pub selection: s::CombinationSelection,
    pub name: String,
    pub target: s::BookmarkTarget,
    pub immutable_heads: Vec<m::GitSha1Id>,
    pub merge: m::GitSha1Id,
    pub code: s::CodeSelection,
    pub stacks: Vec<AppliedStack>,
    pub retention: RetainedCode,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub version: u32,
    pub revision: u64,
    pub active: Option<s::CombinationId>,
    pub combinations: Vec<Combination>,
}
impl Snapshot {
    fn empty() -> Self {
        Self {
            version: 1,
            revision: 0,
            active: None,
            combinations: vec![],
        }
    }
    fn active(&self) -> Option<&Combination> {
        self.combinations
            .iter()
            .find(|c| Some(c.selection.id) == self.active)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Pending,
    Selected,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    preview: s::Preview,
    before_revision: u64,
    after: Snapshot,
    retention: RetainedCode,
    phase: Phase,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub phase: Phase,
    pub replayed: bool,
}
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid combination update")]
    Invalid,
    #[error("combination selection is stale")]
    Stale,
    #[error("combination update scope or principal mismatch")]
    Forbidden,
    #[error("combination request identity has different intent")]
    IntentMismatch,
    #[error("combination update requires original pending recovery")]
    Pending,
    #[error("combination state capacity exhausted")]
    Capacity,
    #[error("combination state is unavailable")]
    Unavailable,
    #[error("combination update outcome is unknown")]
    OutcomeUnknown,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<files::Error> for Error {
    fn from(value: files::Error) -> Self {
        match value {
            files::Error::Unavailable => Self::Unavailable,
            files::Error::Capacity => Self::Capacity,
            files::Error::OutcomeUnknown => Self::OutcomeUnknown,
        }
    }
}
#[derive(Clone)]
pub struct CombinationStore {
    root: files::Root,
    scope: j::Scope,
    principal: m::PrincipalId,
}
impl CombinationStore {
    /// Explicit private state creation. No repository initialization or setup.
    pub fn create(path: &Path, scope: j::Scope, principal: m::PrincipalId) -> Result<Self> {
        validation::scope(&scope, principal)?;
        let root = files::Root::create(path, LAYOUT)?;
        let gate = root.gate()?;
        root.persist(
            &gate,
            &Catalog::new(scope.clone(), principal).encode()?,
            None,
        )?;
        drop(gate);
        Ok(Self {
            root,
            scope,
            principal,
        })
    }
    pub fn open(path: &Path, scope: j::Scope, principal: m::PrincipalId) -> Result<Self> {
        validation::scope(&scope, principal)?;
        let store = Self {
            root: files::Root::open(path, LAYOUT)?,
            scope,
            principal,
        };
        store.snapshot()?;
        Ok(store)
    }
    fn read(&self, gate: &files::Gate) -> Result<(Catalog, files::Stamp)> {
        let (bytes, stamp) = self.root.read(gate)?;
        Ok((Catalog::decode(&bytes, &self.scope, self.principal)?, stamp))
    }
    /// The selected state stays readable during recovery; current command
    /// authority is separately denied while there is an unresolved update.
    pub fn snapshot(&self) -> Result<Snapshot> {
        let gate = self.root.gate()?;
        Ok(self.read(&gate)?.0.selected)
    }
    pub(crate) fn require_uninitialized(
        &self,
        scope: &j::Scope,
        principal: m::PrincipalId,
    ) -> Result<()> {
        self.check_scope(scope, principal)?;
        let gate = self.root.gate()?;
        let (catalog, _) = self.read(&gate)?;
        if catalog.records.iter().any(|r| r.phase == Phase::Pending) {
            return Err(Error::Pending);
        }
        if catalog.selected != Snapshot::empty() {
            return Err(Error::Stale);
        }
        Ok(())
    }
    /// Only trusted provider composition may propose these complete effects.
    /// An inbound client can never supply this state or retention evidence.
    pub fn stage(
        &self,
        principal: m::PrincipalId,
        expected_revision: u64,
        preview: &s::Preview,
        after: &Snapshot,
        retained: &StackRetention,
    ) -> Result<Outcome> {
        if principal != self.principal {
            return Err(Error::Forbidden);
        }
        preview.validate().map_err(|_| Error::Invalid)?;
        let retention = validation::retained(&self.scope, principal, preview, retained)?;
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let record = Record {
            preview: preview.clone(),
            before_revision: expected_revision,
            after: after.clone(),
            retention,
            phase: Phase::Pending,
        };
        if let Some(original) = catalog.lookup(&preview.plan.request)? {
            let mut same = original.clone();
            same.phase = Phase::Pending;
            if same != record {
                return Err(Error::IntentMismatch);
            }
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Outcome {
                phase: original.phase,
                replayed: true,
            });
        }
        if catalog.records.iter().any(|r| r.phase == Phase::Pending) {
            return Err(Error::Pending);
        }
        validation::proposed(&self.scope, &catalog.selected, &record, retained)?;
        if catalog.records.len() >= MAX_RECORDS {
            return Err(Error::Capacity);
        }
        catalog.records.push(record);
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(Outcome {
            phase: Phase::Pending,
            replayed: false,
        })
    }
    /// Select only after the exact journal has qualified checkout. The journal
    /// caller is responsible for actual provider publication/checkout evidence.
    pub fn finish(
        &self,
        journal: &j::StackCommandJournal,
        principal: m::PrincipalId,
        apply: &s::ApplyRequest,
    ) -> Result<Outcome> {
        if principal != self.principal || journal.scope() != &self.scope {
            return Err(Error::Forbidden);
        }
        // Do not nest journal acquisition beneath combination coordination.
        let journal_record = journal.status(principal, apply.request_id, apply.intent_digest);
        let gate = self.root.gate()?;
        let (mut catalog, stamp) = self.read(&gate)?;
        let index = catalog
            .records
            .iter()
            .position(|r| r.preview.plan.request.request_id == apply.request_id)
            .ok_or(Error::Unavailable)?;
        let original = &catalog.records[index];
        apply
            .validate(&original.preview.plan.request, original.preview.prepared)
            .map_err(|_| Error::IntentMismatch)?;
        if original.phase != Phase::Pending {
            self.root.sync_selected(&gate, stamp)?;
            return Ok(Outcome {
                phase: original.phase,
                replayed: true,
            });
        }
        let record = journal_record
            .map_err(|_| Error::Unavailable)?
            .ok_or(Error::Unavailable)?;
        if record.preview != original.preview {
            return Err(Error::IntentMismatch);
        }
        let execution = record.execution.ok_or(Error::Invalid)?;
        if execution.key != original.retention.key
            || execution.retention != Some(original.retention.commit)
            || execution.archive != original.retention.archive
        {
            return Err(Error::IntentMismatch);
        }
        if let Some(binding) = &execution.combination {
            if CombinationBindingV1::from_record(&self.scope, principal, original)? != *binding {
                return Err(Error::IntentMismatch);
            }
            if !matches!(record.receipt.state, s::ReceiptState::Rejected { .. }) {
                return Err(Error::Pending);
            }
        }
        let phase = if record.receipt.completed() {
            Phase::Selected
        } else if matches!(record.receipt.state, s::ReceiptState::Rejected { .. }) {
            Phase::Rejected
        } else {
            return Err(Error::Pending);
        };
        if catalog.selected.revision != original.before_revision {
            return Err(Error::Stale);
        }
        if phase == Phase::Selected {
            catalog.selected = original.after.clone();
        }
        catalog.records[index].phase = phase;
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        Ok(Outcome {
            phase,
            replayed: false,
        })
    }
}
#[cfg(all(test, unix))]
pub(crate) mod tests;
