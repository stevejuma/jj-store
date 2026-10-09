use super::{
    LocalError, LocalRepository, LocalResult, review_io::LocalReviewRootV1, review_types::*,
};
use jj_store_jj::local_metadata::{Domain, Preparation, PublishOutcome, Update};
use jj_store_model as m;
use jj_store_wal::publication::{
    ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1, ReviewCommandV1, SensitivityV1,
};
use jj_store_wal::publication::{ReviewCommandRequest, ReviewProjector, ReviewSnapshotRead};

impl LocalRepository {
    pub async fn apply_review_command(
        &self,
        request: &LocalReviewRequest,
    ) -> LocalResult<LocalReviewOutcome> {
        request.validate()?;
        self.check_identity(self.metadata.read()?.as_ref())?;
        let actor = self.ensure_actor()?;
        let proposed = Self::proposed_repository()?;
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
            if let Some(value) = selected
                .as_ref()
                .and_then(|s| s.receipt(actor, request.request_id))
            {
                if value.intent != intent {
                    return Err(LocalError::IntentConflict);
                }
                return receipt(value);
            }
            let base = self.review_store_at(selected.as_ref(), proposed).await?;
            if let LocalReviewCommand::WithEvidence { evidence, .. } = &request.command {
                if evidence
                    .iter()
                    .any(|e| e.reference.repository != base.repository())
                {
                    return Err(LocalError::Invalid);
                }
                base.io
                    .evidence
                    .as_ref()
                    .ok_or(LocalError::Corrupt)?
                    .stage(evidence)?;
            }
            let (root, command) = match &request.command {
                LocalReviewCommand::Initialize => (base.root().clone(), None),
                other => {
                    let command = match other {
                        LocalReviewCommand::RepositoryThread { body } => {
                            ReviewCommandV1::CreateThread {
                                anchor: Box::new(ArtifactAnchorV1 {
                                    schema_version: 1,
                                    repository_id: base.repository(),
                                    target: ArtifactTargetV1::Repository,
                                    created_context: CreatedContextV1 {
                                        publication_position: None,
                                        code_state_version: None,
                                        agent_root: None,
                                    },
                                    sensitivity: SensitivityV1::Repository,
                                }),
                                body: body.clone(),
                            }
                        }
                        LocalReviewCommand::Apply { intent: command }
                        | LocalReviewCommand::WithEvidence {
                            intent: command, ..
                        } => command.as_ref().clone(),
                        LocalReviewCommand::Initialize => unreachable!(),
                    };
                    let typed = ReviewCommandRequest::new(
                        actor,
                        request.request_id,
                        m::PublicationId::new(*request.request_id.as_bytes()),
                        command,
                    )?;
                    let applied = ReviewProjector::new(base.io.as_ref())
                        .stage_command(&base, &typed, at)
                        .await?;
                    (applied.root, Some(applied.result))
                }
            };
            let reference = base.io.stage_root(&root).await?;
            let result = ReviewResult {
                root: reference.digest,
                accepted_at_millis: at,
                command,
            };
            let repository = base.repository();
            let update = Update {
                repository,
                actor,
                request: request.request_id,
                intent,
                domain: Domain::Review,
                expected: selected
                    .as_ref()
                    .and_then(|s| s.selection(Domain::Review))
                    .cloned(),
                bytes: serde_json::to_vec(&LocalReviewRootV1 {
                    version: 1,
                    repository,
                    root: reference,
                    records: base.io.store.inventory(),
                    evidence: base
                        .io
                        .evidence
                        .as_ref()
                        .ok_or(LocalError::Corrupt)?
                        .inventory(),
                })
                .map_err(|_| LocalError::Corrupt)?,
                result: serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)?,
                retained: {
                    let mut retained = base.io.store.retained();
                    retained.extend(
                        base.io
                            .evidence
                            .as_ref()
                            .ok_or(LocalError::Corrupt)?
                            .retained(),
                    );
                    retained
                },
            };
            let candidate = match self.metadata.prepare(selected.as_ref(), &update)? {
                Preparation::Committed(value) => return receipt(&value),
                Preparation::Ready(candidate) => candidate,
            };
            match self.metadata.publish(*candidate)? {
                PublishOutcome::Committed(value) => return receipt(&value),
                PublishOutcome::Conflict => continue,
                PublishOutcome::Unknown => {
                    return Ok(LocalReviewOutcome::OutcomeUnknown {
                        request_id: request.request_id,
                    });
                }
            }
        }
        Err(LocalError::Busy)
    }
    pub fn review_command_status(&self, request: m::RequestId) -> LocalResult<LocalReviewOutcome> {
        self.check_identity(self.metadata.read()?.as_ref())?;
        if let Some(actor) = self.actor()?
            && let Some(selected) = self.metadata.read()?
            && let Some(value) = selected.receipt(actor, request)
        {
            return receipt(value);
        }
        Ok(LocalReviewOutcome::NotKnown {
            request_id: request,
        })
    }
}
