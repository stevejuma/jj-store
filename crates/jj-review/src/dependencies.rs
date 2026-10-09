use crate::{AgentContentOwnerV1, ArtifactAnchorV1, ArtifactTargetV1, Result};
use jj_agent::AgentObjectRef;
use jj_store_model::ObjectKey;
use std::collections::BTreeSet;

impl ArtifactAnchorV1 {
    pub fn code_evidence(&self) -> Result<BTreeSet<jj_store_model::portable::CodeEvidenceRefV1>> {
        self.validate()?;
        Ok(self.code_evidence_unchecked())
    }
    pub(crate) fn code_evidence_unchecked(
        &self,
    ) -> BTreeSet<jj_store_model::portable::CodeEvidenceRefV1> {
        match &self.target {
            ArtifactTargetV1::PortableCodeArtifact(v) => {
                BTreeSet::from([v.revision.evidence.clone()])
            }
            ArtifactTargetV1::PortableCodeText(v) => {
                let mut refs = BTreeSet::from([v.artifact.revision.evidence.clone()]);
                if let Some(diff) = &v.diff_context {
                    refs.extend([diff.before.evidence.clone(), diff.after.evidence.clone()]);
                }
                refs
            }
            _ => BTreeSet::new(),
        }
    }

    pub fn native_dependencies(&self) -> Result<BTreeSet<ObjectKey>> {
        self.validate()?;
        let mut values = BTreeSet::new();
        match &self.target {
            ArtifactTargetV1::CodeText(value) => {
                values.extend([
                    ObjectKey::Operation(value.operation_id),
                    ObjectKey::Commit(value.commit_id),
                    ObjectKey::File(value.file_id),
                ]);
                if let Some(diff) = &value.diff_context {
                    values.extend([
                        ObjectKey::Operation(diff.before_operation_id),
                        ObjectKey::Commit(diff.before_commit_id),
                        ObjectKey::Operation(diff.after_operation_id),
                        ObjectKey::Commit(diff.after_commit_id),
                    ]);
                }
            }
            ArtifactTargetV1::CodeArtifact(value) => {
                values.extend([
                    ObjectKey::Operation(value.operation_id),
                    ObjectKey::Commit(value.commit_id),
                    value.object,
                ]);
            }
            ArtifactTargetV1::Revision {
                operation_id,
                commit_id,
                ..
            } => {
                values.extend([
                    ObjectKey::Operation(*operation_id),
                    ObjectKey::Commit(*commit_id),
                ]);
            }
            ArtifactTargetV1::Operation { operation_id } => {
                values.insert(ObjectKey::Operation(*operation_id));
            }
            _ => {}
        }
        Ok(values)
    }

    pub fn agent_dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        let mut values = BTreeSet::new();
        match &self.target {
            ArtifactTargetV1::AgentText(value) => {
                values.extend([
                    value.session_snapshot,
                    owner_snapshot(&value.owner),
                    value.block_version,
                    value.content,
                ]);
            }
            ArtifactTargetV1::AgentArtifact(value) => {
                values.extend([value.session_snapshot, owner_snapshot(&value.owner)]);
            }
            ArtifactTargetV1::AgentSession {
                session_snapshot, ..
            } => {
                values.insert(*session_snapshot);
            }
            _ => {}
        }
        Ok(values)
    }
}

fn owner_snapshot(owner: &AgentContentOwnerV1) -> AgentObjectRef {
    match owner {
        AgentContentOwnerV1::Tool { snapshot, .. }
        | AgentContentOwnerV1::Message { snapshot, .. }
        | AgentContentOwnerV1::Plan { snapshot, .. } => *snapshot,
    }
}
