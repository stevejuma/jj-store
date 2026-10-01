use super::{LocalProfile, LocalRepository, map_metadata};
use crate::work::*;
use jj_store_jj::local_metadata::{Domain, GitOid, Preparation, PublishOutcome, Snapshot, Update};
use jj_store_model as m;
use jj_store_wal::publication::{WorkModelError, WorkStateV1};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalWorkRootV1 {
    version: u32,
    repository: m::RepositoryId,
    projects: BTreeMap<ProjectId, ProjectV1>,
    tasks: BTreeMap<TaskId, TaskV1>,
    comments: BTreeMap<WorkCommentId, WorkCommentV1>,
}
impl LocalWorkRootV1 {
    fn state(self) -> WorkStateV1 {
        WorkStateV1 {
            projects: self.projects,
            tasks: self.tasks,
            comments: self.comments,
        }
    }
    fn from_state(repository: m::RepositoryId, state: WorkStateV1) -> Self {
        Self {
            version: 1,
            repository,
            projects: state.projects,
            tasks: state.tasks,
            comments: state.comments,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalWorkSnapshot {
    pub profile: LocalProfile,
    pub repo_id: Option<m::RepositoryId>,
    pub metadata_commit: Option<GitOid>,
    pub work_state_version: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalWorkReceipt {
    pub version: u32,
    pub profile: LocalProfile,
    pub request_id: m::RequestId,
    pub repository: m::RepositoryId,
    pub actor: m::PrincipalId,
    pub request_digest: m::IntentDigest,
    pub metadata_sequence: u64,
    pub work_state_version: u64,
    pub work_object: GitOid,
    pub result: WorkCommandResultV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LocalWorkOutcome {
    Committed { receipt: Box<LocalWorkReceipt> },
    OutcomeUnknown { request_id: m::RequestId },
    NotKnown { request_id: m::RequestId },
}
fn model(error: WorkModelError) -> Error {
    match error {
        WorkModelError::Invalid(_) => Error::Invalid,
        WorkModelError::NotFound(_) => Error::NotFound,
        WorkModelError::Limit => Error::Limit,
        WorkModelError::Stale => Error::Stale,
    }
}
fn intent(actor: m::PrincipalId, request: &Request) -> Result<m::IntentDigest> {
    let mut hash = Sha256::new_with_prefix(b"jj-store/local-work-intent/v1\0");
    hash.update(actor.as_bytes());
    hash.update(serde_json::to_vec(request).map_err(|_| Error::Invalid)?);
    Ok(m::IntentDigest::new(hash.finalize().into()))
}
fn receipt(value: &jj_store_jj::local_metadata::Receipt) -> Result<LocalWorkOutcome> {
    if value.domain != Domain::Work {
        return Err(Error::IntentConflict);
    }
    let result: WorkCommandResultV1 =
        serde_json::from_slice(&value.result).map_err(|_| Error::Corrupt)?;
    if serde_json::to_vec(&result).map_err(|_| Error::Corrupt)? != value.result {
        return Err(Error::Corrupt);
    }
    Ok(LocalWorkOutcome::Committed {
        receipt: Box::new(LocalWorkReceipt {
            version: 2,
            profile: LocalProfile::LocalJjGit,
            request_id: value.request,
            repository: value.repository,
            actor: value.actor,
            request_digest: value.intent,
            metadata_sequence: value.sequence,
            work_state_version: value.selection.version,
            work_object: value.selection.object,
            result,
        }),
    })
}
impl LocalRepository {
    pub(crate) fn work_state(&self, snapshot: Option<&Snapshot>) -> Result<WorkStateV1> {
        let Some(snapshot) = snapshot else {
            return Ok(WorkStateV1::default());
        };
        let Some(bytes) = self
            .metadata
            .read_domain(snapshot, Domain::Work)
            .map_err(map_metadata)?
        else {
            return Ok(WorkStateV1::default());
        };
        let root: LocalWorkRootV1 = serde_json::from_slice(&bytes).map_err(|_| Error::Corrupt)?;
        if root.version != 1
            || root.repository != snapshot.repository()
            || serde_json::to_vec(&root).map_err(|_| Error::Corrupt)? != bytes
        {
            return Err(Error::Corrupt);
        }
        let state = root.state();
        state.validate().map_err(model)?;
        Ok(state)
    }
    pub fn apply_work_command(&self, request: &Request) -> Result<LocalWorkOutcome> {
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        request.command.validate().map_err(model)?;
        let actor = self.ensure_actor()?;
        let digest = intent(actor, request)?;
        let proposed = Self::proposed_repository()?;
        let at = i64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::Unavailable)?
                .as_millis(),
        )
        .map_err(|_| Error::Limit)?;
        for _ in 0..8 {
            let base = self.metadata.read().map_err(map_metadata)?;
            if let Some(value) = base
                .as_ref()
                .and_then(|s| s.receipt(actor, request.request_id))
            {
                if value.intent != digest {
                    return Err(Error::IntentConflict);
                }
                return receipt(value);
            }
            let repository = base.as_ref().map_or(proposed, |s| s.repository());
            let (state, result) = self
                .work_state(base.as_ref())?
                .apply(&request.command, actor, at)
                .map_err(model)?;
            let update = Update {
                repository,
                actor,
                request: request.request_id,
                intent: digest,
                domain: Domain::Work,
                expected: base
                    .as_ref()
                    .and_then(|s| s.selection(Domain::Work))
                    .cloned(),
                bytes: serde_json::to_vec(&LocalWorkRootV1::from_state(repository, state))
                    .map_err(|_| Error::Corrupt)?,
                result: serde_json::to_vec(&result).map_err(|_| Error::Corrupt)?,
                retained: BTreeMap::new(),
            };
            let prepared = self
                .metadata
                .prepare(base.as_ref(), &update)
                .map_err(map_metadata)?;
            let candidate = match prepared {
                Preparation::Committed(value) => return receipt(&value),
                Preparation::Ready(candidate) => candidate,
            };
            match self.metadata.publish(*candidate).map_err(map_metadata)? {
                PublishOutcome::Committed(value) => return receipt(&value),
                PublishOutcome::Unknown => {
                    return Ok(LocalWorkOutcome::OutcomeUnknown {
                        request_id: request.request_id,
                    });
                }
                PublishOutcome::Conflict => continue,
            }
        }
        Err(Error::Busy)
    }
    pub fn work_command_status(&self, request: m::RequestId) -> Result<LocalWorkOutcome> {
        let actor = self.actor()?;
        let base = self.metadata.read().map_err(map_metadata)?;
        if let (Some(actor), Some(base)) = (actor, base)
            && let Some(value) = base.receipt(actor, request)
        {
            return receipt(value);
        }
        Ok(LocalWorkOutcome::NotKnown {
            request_id: request,
        })
    }
    pub(crate) fn work_snapshot(snapshot: Option<&Snapshot>) -> LocalWorkSnapshot {
        LocalWorkSnapshot {
            profile: LocalProfile::LocalJjGit,
            repo_id: snapshot.map(|s| s.repository()),
            metadata_commit: snapshot.map(|s| s.commit()),
            work_state_version: snapshot
                .and_then(|s| s.selection(Domain::Work))
                .map_or(0, |s| s.version),
        }
    }
}
