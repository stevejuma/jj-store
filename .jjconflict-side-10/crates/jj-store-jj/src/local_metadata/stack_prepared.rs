//! Reserved local code retention, separate from portable companion selection.
use super::*;
use crate::local_evidence::GitEvidenceCapsuleV1;
use jj_store_model as m;
use serde::{Deserialize, Serialize};
mod objects;
mod selection;

pub const STACK_PREPARED_REF_PREFIX: &str = "refs/lapis/stacks/prepared/v1/";
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StackPreparationKey {
    pub repository: m::RepositoryId,
    pub physical_workspace: m::RecordDigest,
    pub principal: m::PrincipalId,
    pub request: m::RequestId,
    pub intent: m::IntentDigest,
    pub prepared: m::RecordDigest,
}
impl StackPreparationKey {
    fn validate(&self) -> Result<()> {
        if self.repository.as_bytes() == &[0; 16]
            || self.physical_workspace.as_bytes() == &[0; 32]
            || self.principal.as_bytes() == &[0; 16]
            || self.request.as_bytes() == &[0; 16]
            || self.intent.as_bytes() == &[0; 32]
            || self.prepared.as_bytes() == &[0; 32]
        {
            return Err(Error::Invalid("stack preparation key"));
        }
        Ok(())
    }
    fn reference(&self) -> String {
        format!(
            "{STACK_PREPARED_REF_PREFIX}{}/{}",
            self.physical_workspace, self.request
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StackRetention {
    pub commit: GitOid,
    pub key: StackPreparationKey,
    pub capsule: GitEvidenceCapsuleV1,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StackRetentionOutcome {
    Retained(Box<StackRetention>),
    /// No rejection or new JJ operation may be inferred from uncertainty.
    Unknown {
        commit: GitOid,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    version: u32,
    key: StackPreparationKey,
    operation: m::OperationId,
    view: m::ViewId,
    capsule: m::CodeCapsuleId,
    blob: GitOid,
    retained: BTreeMap<GitOid, RetainedKind>,
}
impl MetadataStore {
    /// No source loader, initialization, recovery or code publication.
    pub fn read_stack_prepared(&self, key: &StackPreparationKey) -> Result<Option<StackRetention>> {
        key.validate()?;
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        self.stack_selected(&repo, key)?
            .map(|commit| {
                let selected = self.stack_archive(&repo, commit, key, false)?;
                if self.stack_selected(&repo, key)? != Some(commit) {
                    return Err(Error::Stale);
                }
                Ok(selected)
            })
            .transpose()
    }
    /// Composition supplies the original preparation identity. This selects an
    /// immutable code retention ref, never JJ operation heads or companion state.
    pub fn retain_stack_prepared(
        &self,
        key: &StackPreparationKey,
        capsule: &GitEvidenceCapsuleV1,
    ) -> Result<StackRetentionOutcome> {
        key.validate()?;
        capsule.validate()?;
        if capsule.repository != key.repository
            || self
                .identity()?
                .is_some_and(|s| s.repository != key.repository)
        {
            return Err(Error::Invalid("stack capsule repository binding"));
        }
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        if let Some(commit) = self.stack_selected(&repo, key)? {
            let selected = self.stack_archive(&repo, commit, key, true)?;
            if selected.capsule != *capsule {
                return Err(Error::IntentConflict);
            }
            return self.stack_acknowledge(&repo, selected);
        }
        let bytes = capsule.encode()?;
        if bytes.len() > self.limits.object_bytes {
            return Err(Error::Limit);
        }
        let blob = super::objects::blob(&repo, &bytes)?;
        let mut retained = capsule.retention(blob)?;
        retained.extend(super::objects::audit(&repo, &retained, self.limits, true)?);
        let record = Record {
            version: 1,
            key: key.clone(),
            operation: capsule.operation_id,
            view: capsule.view_id,
            capsule: capsule.id()?,
            blob,
            retained,
        };
        let commit = self.stack_candidate(&repo, &record)?;
        let selected = self.stack_archive(&repo, commit, key, true)?;
        let edit = RefEdit {
            name: key.reference().try_into().map_err(Error::other)?,
            deref: false,
            change: Change::Update {
                log: LogChange::default(),
                expected: PreviousValue::MustNotExist,
                new: Target::Object(commit.0),
            },
        };
        let _outcome = repo.edit_references_as([edit], None);
        // A verified selected slot resolves the result; a collision is never
        // overwritten. Absence cannot prove that the ref edit was rejected.
        match self.stack_selected(&repo, key) {
            Ok(Some(current)) if current != selected.commit => Err(Error::IntentConflict),
            Ok(Some(current)) => match self.stack_archive(&repo, current, key, true) {
                Ok(current) => self.stack_acknowledge(&repo, current),
                Err(_) => Ok(StackRetentionOutcome::Unknown { commit }),
            },
            Ok(None) | Err(_) => Ok(StackRetentionOutcome::Unknown { commit }),
        }
    }
}
