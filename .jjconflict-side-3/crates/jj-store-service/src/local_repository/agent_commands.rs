use super::{
    LocalCodeEvidenceV1, LocalError, LocalProfile, LocalRepository, LocalResult,
    agent_descriptor::LocalAgentRootV2,
};
use jj_store_jj::local_metadata::{
    BatchUpdate, Domain, DomainUpdate, Preparation, PublishOutcome, Receipt, Update,
};
use jj_store_model as m;
use jj_store_wal::publication::{
    AgentCatalogEditor, AgentObjectRef, AgentObjectV1, AgentProjector, CaptureBatchId,
    NormalizerProfileV1, SanitizedObservationBatchV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum LocalAgentCommand {
    Initialize,
    Capture {
        profile: NormalizerProfileV1,
        batch: Box<SanitizedObservationBatchV1>,
    },
    BoundCapture {
        profile: NormalizerProfileV1,
        batch: Box<SanitizedObservationBatchV1>,
        binding: jj_collab::ThreadBindingV1,
    },
    Provenance {
        expected: Option<AgentObjectRef>,
        object: Box<AgentObjectV1>,
        evidence: Vec<LocalCodeEvidenceV1>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAgentRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: LocalAgentCommand,
}
impl LocalAgentRequest {
    pub fn validate(&self) -> LocalResult<()> {
        if self.version != 1 {
            return Err(LocalError::Invalid);
        }
        if let LocalAgentCommand::Capture { profile, batch }
        | LocalAgentCommand::BoundCapture { profile, batch, .. } = &self.command
        {
            batch.validate().map_err(|_| LocalError::Invalid)?;
            profile.validate().map_err(|_| LocalError::Unsupported)?;
        }
        if let LocalAgentCommand::BoundCapture { binding, .. } = &self.command {
            binding.validate()?;
        }
        if let LocalAgentCommand::Provenance {
            expected,
            object,
            evidence,
        } = &self.command
        {
            if !matches!(
                object.as_ref(),
                AgentObjectV1::PortableAssociation(_) | AgentObjectV1::PortableCheckpointVersion(_)
            ) {
                return Err(LocalError::Unsupported);
            }
            if expected.is_some_and(|v| v.kind != object.kind()) {
                return Err(LocalError::Invalid);
            }
            object.validate().map_err(|_| LocalError::Invalid)?;
            let required = object.code_evidence().map_err(|_| LocalError::Invalid)?;
            let mut seen = std::collections::BTreeSet::new();
            let mut bytes = 0usize;
            if evidence.len() > 1024 {
                return Err(LocalError::Limit);
            }
            for value in evidence {
                bytes = bytes
                    .checked_add(value.bytes.len())
                    .ok_or(LocalError::Limit)?;
                if bytes > 16 * 1024 * 1024 {
                    return Err(LocalError::Limit);
                }
                if !required.contains(&value.reference) || !seen.insert(&value.reference) {
                    return Err(LocalError::Invalid);
                }
                value.validate()?;
            }
        }
        Ok(())
    }
    pub fn intent_for(&self, actor: m::PrincipalId) -> LocalResult<m::IntentDigest> {
        self.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/local-agent-command/v1\0");
        hash.update(actor.as_bytes());
        hash.update(serde_json::to_vec(self).map_err(|_| LocalError::Invalid)?);
        Ok(m::IntentDigest::new(hash.finalize().into()))
    }
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
        self.apply_agent_command_with_identity(request, None).await
    }
    /// Installation admission has durably recorded this identity before publication.
    pub async fn initialize_for_browser(
        &self,
        request_id: m::RequestId,
        repository: m::RepositoryId,
    ) -> LocalResult<LocalAgentOutcome> {
        self.apply_agent_command_with_identity(
            &LocalAgentRequest {
                version: 1,
                request_id,
                command: LocalAgentCommand::Initialize,
            },
            Some(repository),
        )
        .await
    }
    async fn apply_agent_command_with_identity(
        &self,
        request: &LocalAgentRequest,
        admitted: Option<m::RepositoryId>,
    ) -> LocalResult<LocalAgentOutcome> {
        request.validate()?;
        let initial = self.metadata.read()?;
        self.check_identity(initial.as_ref())?;
        if admitted.is_some_and(|id| initial.as_ref().is_some_and(|s| s.repository() != id)) {
            return Err(LocalError::Stale);
        }
        let actor = self.ensure_actor()?;
        let proposed = admitted.unwrap_or(Self::proposed_repository()?);
        let intent = request.intent_for(actor)?;
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| LocalError::Unavailable)?
            .as_millis()
            .try_into()
            .map_err(|_| LocalError::Limit)?;
        for _ in 0..8 {
            let selected = self.metadata.read()?;
            self.check_identity(selected.as_ref())?;
            if admitted.is_some_and(|id| selected.as_ref().is_some_and(|s| s.repository() != id)) {
                return Err(LocalError::Stale);
            }
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
            let mut captured = None;
            let root = match &request.command {
                LocalAgentCommand::Initialize => {
                    jj_store_wal::publication::AgentSnapshotRead::root(&base).clone()
                }
                LocalAgentCommand::Capture { profile, batch }
                | LocalAgentCommand::BoundCapture { profile, batch, .. } => {
                    let delta = AgentProjector::new(base.io.as_ref())
                        .stage_agent_capture(&base, profile, batch, at)
                        .await?;
                    let root = AgentCatalogEditor::new(base.io.as_ref())
                        .apply_delta(&base, &delta)
                        .await?;
                    captured = Some(delta);
                    root
                }
                LocalAgentCommand::Provenance {
                    expected,
                    object,
                    evidence,
                } => {
                    base.io.evidence.stage(evidence)?;
                    let delta = AgentProjector::new(base.io.as_ref())
                        .stage_agent_provenance(&base, *expected, object)
                        .await?;
                    AgentCatalogEditor::new(base.io.as_ref())
                        .apply_delta(&base, &delta)
                        .await?
                }
            };
            let bound = match (&request.command, captured.as_ref()) {
                (LocalAgentCommand::BoundCapture { binding, .. }, Some(delta)) => Some(
                    super::agent_thread_binding::CaptureBinding {
                        binding,
                        delta,
                        actor,
                        request: request.request_id,
                        at,
                    }
                    .stage(self, selected.as_ref(), &base)
                    .await?,
                ),
                _ => None,
            };
            let reference = base.io.stage_root(&root).await?;
            let result = AgentResult {
                root: reference.digest,
                accepted_at_millis: at,
                batch: match &request.command {
                    LocalAgentCommand::Capture { batch, .. }
                    | LocalAgentCommand::BoundCapture { batch, .. } => Some(batch.batch_id),
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
                bytes: serde_json::to_vec(&LocalAgentRootV2 {
                    version: 2,
                    repository,
                    root: reference,
                    records: base.io.store.inventory(),
                    evidence: base.io.evidence.inventory(),
                })
                .map_err(|_| LocalError::Corrupt)?,
                result: serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)?,
                retained: {
                    let mut retained = base.io.store.retained();
                    retained.extend(base.io.evidence.retained());
                    retained
                },
            };
            let preparation = if let Some((bytes, retained)) = bound {
                let mut combined = update.retained;
                for (oid, kind) in retained {
                    if combined.insert(oid, kind).is_some_and(|old| old != kind) {
                        return Err(LocalError::Corrupt);
                    }
                }
                self.metadata.prepare_many(
                    selected.as_ref(),
                    &BatchUpdate {
                        repository: update.repository,
                        actor: update.actor,
                        request: update.request,
                        intent: update.intent,
                        domains: std::collections::BTreeMap::from([
                            (
                                Domain::Agent,
                                DomainUpdate {
                                    expected: update.expected,
                                    bytes: update.bytes,
                                },
                            ),
                            (
                                Domain::Thread,
                                DomainUpdate {
                                    expected: selected
                                        .as_ref()
                                        .and_then(|s| s.selection(Domain::Thread))
                                        .cloned(),
                                    bytes,
                                },
                            ),
                        ]),
                        result: update.result,
                        retained: combined,
                    },
                )?
            } else {
                self.metadata.prepare(selected.as_ref(), &update)?
            };
            let candidate = match preparation {
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
