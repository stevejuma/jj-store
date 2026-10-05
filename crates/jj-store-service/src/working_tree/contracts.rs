//! Complete read-only transport summaries. Fingerprints are observations, not tokens.
use super::*;
use jj_store_model as m;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ObserveWorkingTree {
    pub version: u32,
    pub binding_generation: u64,
    pub operation: m::OperationId,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkingTreeSnapshot {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub binding_generation: u64,
    pub operation: m::OperationId,
    pub view: m::ViewId,
    pub workspace: String,
    pub working_copy: m::GitSha1Id,
    pub trees: Vec<m::GitSha1Id>,
    pub tree_labels: Vec<String>,
    pub physical_workspace: PhysicalWorkspaceId,
    pub metadata_fingerprint: m::RecordDigest,
    pub physical_fingerprint: m::RecordDigest,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ObservedWorkingTree {
    pub snapshot: WorkingTreeSnapshot,
    #[schemars(length(max = 4096))]
    pub changes: Vec<WorkingTreeChange>,
    pub obstacle_count: u32,
}
#[derive(Clone, Copy, Debug, Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkingTreeFileKind {
    File { executable: bool },
    Symlink,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkingTreeFileState {
    pub kind: WorkingTreeFileKind,
    pub bytes: u64,
    pub content_digest: m::RecordDigest,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkingTreeChange {
    pub path: String,
    pub status: ChangeKind,
    pub before: Option<WorkingTreeFileState>,
    pub after: Option<WorkingTreeFileState>,
}
impl Observation {
    pub fn summary(self) -> Result<ObservedWorkingTree> {
        use jj_store_jj::local_browser::working_tree as baseline;
        if self.projection.changes.len() > baseline::MAX_ENTRIES {
            return Err(Error::Limit);
        }
        let current = self.current;
        let physical = self.projection.physical;
        Ok(ObservedWorkingTree {
            snapshot: WorkingTreeSnapshot {
                version: 1,
                repository: physical.repository,
                binding_generation: physical.generation,
                operation: current.baseline.operation,
                view: current.baseline.view,
                workspace: current.baseline.workspace,
                working_copy: current.baseline.commit,
                trees: current.baseline.trees,
                tree_labels: current.baseline.labels,
                physical_workspace: physical.workspace,
                metadata_fingerprint: current.fingerprint,
                physical_fingerprint: physical.fingerprint,
            },
            changes: self
                .projection
                .changes
                .into_iter()
                .map(|change| WorkingTreeChange {
                    path: change.path,
                    status: change.kind,
                    before: change.before.map(|entry| WorkingTreeFileState {
                        kind: match entry.kind {
                            baseline::Kind::File { executable } => {
                                WorkingTreeFileKind::File { executable }
                            }
                            baseline::Kind::Symlink => WorkingTreeFileKind::Symlink,
                        },
                        bytes: entry.bytes,
                        content_digest: entry.content,
                    }),
                    after: change.after.map(|entry| WorkingTreeFileState {
                        kind: match entry.kind {
                            Kind::File { executable } => WorkingTreeFileKind::File { executable },
                            Kind::Symlink => WorkingTreeFileKind::Symlink,
                        },
                        bytes: entry.bytes,
                        content_digest: entry.content,
                    }),
                })
                .collect(),
            obstacle_count: self.projection.obstacles.len() as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transport_summary_rejects_oversized_complete_change_sets() {
        use jj_store_jj::local_browser::working_tree as code;
        let baseline = code::Baseline {
            version: 1,
            repository: m::RepositoryId::new([1; 16]),
            generation: 7,
            operation: m::OperationId::new([2; 64]),
            view: m::ViewId::new([3; 64]),
            workspace: "default".into(),
            commit: m::GitSha1Id::new([4; 20]),
            trees: vec![m::GitSha1Id::new([5; 20])],
            labels: vec![],
            entries: vec![],
        };
        let checkout = code::Checkout {
            operation: baseline.operation,
            workspace: baseline.workspace.clone(),
            trees: baseline.trees.clone(),
            labels: vec![],
            sparse: vec![String::new()],
        };
        let physical = WorkingTree {
            directories: vec![],
            version: 1,
            repository: baseline.repository,
            generation: 7,
            workspace: PhysicalWorkspaceId(m::RecordDigest::new([6; 32])),
            fingerprint: m::RecordDigest::new([7; 32]),
            entries: vec![],
        };
        let current = code::CurrentState {
            baseline: baseline.clone(),
            checkout,
            physical_workspace: physical.workspace.0,
            fingerprint: m::RecordDigest::new([8; 32]),
            base_ignores: vec![b"private-policy".to_vec()],
            heads: vec![baseline.operation],
        };
        let projection = Projection {
            physical,
            baseline,
            changes: vec![
                Change {
                    path: "file".into(),
                    kind: ChangeKind::Deleted,
                    before: None,
                    after: None
                };
                4097
            ],
            obstacles: vec![],
        };
        assert_eq!(
            Observation {
                current: current.clone(),
                projection: projection.clone()
            }
            .summary()
            .unwrap_err(),
            Error::Limit
        );
        let value = Observation {
            current,
            projection: Projection {
                changes: vec![],
                ..projection
            },
        }
        .summary()
        .unwrap();
        assert!(
            !serde_json::to_string(&value)
                .unwrap()
                .contains("private-policy")
        );
    }
}
