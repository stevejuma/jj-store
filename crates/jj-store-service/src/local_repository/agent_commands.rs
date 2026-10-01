use super::{
    LocalError, LocalProfile, LocalRepository, LocalResult, agent_store::LocalAgentRootV1,
};
use jj_store_jj::local_metadata::{Domain, Preparation, PublishOutcome, Receipt, Update};
use jj_store_model as m;
use jj_store_wal::publication::{
    AgentCatalogEditor, AgentProjector, CaptureBatchId, NormalizerProfileV1,
    SanitizedObservationBatchV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum LocalAgentCommand {
    Initialize,
    Capture {
        profile: NormalizerProfileV1,
        batch: Box<SanitizedObservationBatchV1>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAgentRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: LocalAgentCommand,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentResult {
    root: m::RecordDigest,
    accepted_at_millis: i64,
    batch: Option<CaptureBatchId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalAgentReceipt {
    pub version: u32,
    pub profile: LocalProfile,
    pub repository: m::RepositoryId,
    pub actor: m::PrincipalId,
    pub request_id: m::RequestId,
    pub intent: m::IntentDigest,
    pub metadata_sequence: u64,
    pub agent_state_version: u64,
    pub agent_root: m::RecordDigest,
    pub accepted_at_millis: i64,
    pub batch: Option<CaptureBatchId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LocalAgentOutcome {
    Committed { receipt: Box<LocalAgentReceipt> },
    OutcomeUnknown { request_id: m::RequestId },
    NotKnown { request_id: m::RequestId },
}
fn receipt(value: &Receipt) -> LocalResult<LocalAgentOutcome> {
    if value.domain != Domain::Agent {
        return Err(LocalError::IntentConflict);
    }
    let result: AgentResult =
        serde_json::from_slice(&value.result).map_err(|_| LocalError::Corrupt)?;
    if serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)? != value.result {
        return Err(LocalError::Corrupt);
    }
    Ok(LocalAgentOutcome::Committed {
        receipt: Box::new(LocalAgentReceipt {
            version: 2,
            profile: LocalProfile::LocalJjGit,
            repository: value.repository,
            actor: value.actor,
            request_id: value.request,
            intent: value.intent,
            metadata_sequence: value.sequence,
            agent_state_version: value.selection.version,
            agent_root: result.root,
            accepted_at_millis: result.accepted_at_millis,
            batch: result.batch,
        }),
    })
}
impl LocalRepository {
    pub async fn apply_agent_command(
        &self,
        request: &LocalAgentRequest,
    ) -> LocalResult<LocalAgentOutcome> {
        if request.version != 1 {
            return Err(LocalError::Invalid);
        }
        if let LocalAgentCommand::Capture { profile, batch } = &request.command {
            batch.validate().map_err(|_| LocalError::Invalid)?;
            profile.validate().map_err(|_| LocalError::Unsupported)?;
        }
        let actor = self.ensure_actor()?;
        let proposed = Self::proposed_repository()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/local-agent-command/v1\0");
        hash.update(actor.as_bytes());
        hash.update(serde_json::to_vec(request).map_err(|_| LocalError::Invalid)?);
        let intent = m::IntentDigest::new(hash.finalize().into());
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| LocalError::Unavailable)?
            .as_millis()
            .try_into()
            .map_err(|_| LocalError::Limit)?;
        for _ in 0..8 {
            let selected = self.metadata.read()?;
            if let Some(value) = selected
                .as_ref()
                .and_then(|s| s.receipt(actor, request.request_id))
            {
                if value.intent != intent {
                    return Err(LocalError::IntentConflict);
                }
                return receipt(value);
            }
            let base = self.agent_store_at(selected.as_ref(), proposed).await?;
            let root = match &request.command {
                LocalAgentCommand::Initialize => {
                    jj_store_wal::publication::AgentSnapshotRead::root(&base).clone()
                }
                LocalAgentCommand::Capture { profile, batch } => {
                    let delta = AgentProjector::new(base.io.as_ref())
                        .stage_agent_capture(&base, profile, batch, at)
                        .await?;
                    AgentCatalogEditor::new(base.io.as_ref())
                        .apply_delta(&base, &delta)
                        .await?
                }
            };
            let reference = base.io.stage_root(&root).await?;
            let result = AgentResult {
                root: reference.digest,
                accepted_at_millis: at,
                batch: match &request.command {
                    LocalAgentCommand::Capture { batch, .. } => Some(batch.batch_id),
                    _ => None,
                },
            };
            let repository = base.io.store.repository;
            let update = Update {
                repository,
                actor,
                request: request.request_id,
                intent,
                domain: Domain::Agent,
                expected: selected
                    .as_ref()
                    .and_then(|s| s.selection(Domain::Agent))
                    .cloned(),
                bytes: serde_json::to_vec(&LocalAgentRootV1 {
                    version: 1,
                    repository,
                    root: reference,
                    records: base.io.store.inventory(),
                })
                .map_err(|_| LocalError::Corrupt)?,
                result: serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)?,
                retained: base.io.store.retained(),
            };
            let candidate = match self.metadata.prepare(selected.as_ref(), &update)? {
                Preparation::Committed(value) => return receipt(&value),
                Preparation::Ready(candidate) => candidate,
            };
            match self.metadata.publish(*candidate)? {
                PublishOutcome::Committed(value) => return receipt(&value),
                PublishOutcome::Conflict => continue,
                PublishOutcome::Unknown => {
                    return Ok(LocalAgentOutcome::OutcomeUnknown {
                        request_id: request.request_id,
                    });
                }
            }
        }
        Err(LocalError::Busy)
    }
    pub fn agent_command_status(&self, request: m::RequestId) -> LocalResult<LocalAgentOutcome> {
        if let Some(actor) = self.actor()?
            && let Some(selected) = self.metadata.read()?
            && let Some(value) = selected.receipt(actor, request)
        {
            return receipt(value);
        }
        Ok(LocalAgentOutcome::NotKnown {
            request_id: request,
        })
    }
}
