//! LocalTrusted default profile composed from standalone Git factories.
use super::*;
use jj_store_core::{self as c, *};
use jj_store_git::{GitProvider, LocalRepository as GitRepository};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
struct FilesystemHost {
    repository: GitRepository,
}
#[async_trait::async_trait]
impl StoreHost for FilesystemHost {
    // Folder admission is owned by the calling service; filesystem access remains
    // the LocalTrusted boundary, as for the existing synchronous compatibility API.
    async fn authorize(&self, _: StoreKind, _: Access) -> c::Result<()> {
        Ok(())
    }
    fn actor(&self, writing: bool) -> c::Result<Option<m::PrincipalId>> {
        if writing {
            Ok(Some(self.repository.ensure_actor()?))
        } else {
            Ok(self.repository.actor()?)
        }
    }
    fn now_millis(&self) -> c::Result<i64> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| c::Error::Unavailable("clock".into()))?
            .as_millis()
            .try_into()
            .map_err(|_| c::Error::Limit("clock"))
    }
}
impl LocalRepository {
    pub fn collaboration_stores(&self) -> c::Result<RepositoryStores> {
        let provider = GitProvider::new((*self.inner).clone());
        let mut factories = StoreFactories::empty();
        provider.register(&mut factories, "git")?;
        factories.loader().open(
            &StoreConfig::all("git"),
            StoreContext::new(
                provider.scope(),
                Arc::new(FilesystemHost {
                    repository: (*self.inner).clone(),
                }),
            ),
        )
    }
    pub async fn apply_agent_command(
        &self,
        request: &LocalAgentRequest,
    ) -> LocalResult<LocalAgentOutcome> {
        request.validate()?;
        let command = match &request.command {
            LocalAgentCommand::Initialize => AgentCommand::Initialize,
            LocalAgentCommand::Capture { profile, batch } => AgentCommand::Capture {
                profile: profile.clone(),
                batch: batch.clone(),
                binding: None,
            },
            LocalAgentCommand::BoundCapture {
                profile,
                batch,
                binding,
            } => AgentCommand::Capture {
                profile: profile.clone(),
                batch: batch.clone(),
                binding: Some(binding.clone()),
            },
            LocalAgentCommand::Provenance {
                expected,
                object,
                evidence,
            } => AgentCommand::Provenance {
                expected: *expected,
                object: object.clone(),
                evidence: evidence
                    .iter()
                    .map(|v| CodeEvidence {
                        reference: v.reference.clone(),
                        bytes: v.bytes.clone(),
                    })
                    .collect(),
            },
        };
        let value = self
            .collaboration_stores()?
            .execute_agent(&StoreRequest::new(request.request_id, command))
            .await
            .map_err(map_store_error)?;
        agent(value)
    }
    pub async fn agent_store_status(&self, id: m::RequestId) -> LocalResult<LocalAgentOutcome> {
        agent(
            self.collaboration_stores()?
                .agent()
                .ok_or(LocalError::Unsupported)?
                .status(id)
                .await
                .map_err(map_store_error)?,
        )
    }
    pub async fn apply_review_command(
        &self,
        request: &LocalReviewRequest,
    ) -> LocalResult<LocalReviewOutcome> {
        request.validate()?;
        let command = match &request.command {
            LocalReviewCommand::Initialize => ReviewCommand::Initialize,
            LocalReviewCommand::RepositoryThread { body } => {
                ReviewCommand::RepositoryThread { body: body.clone() }
            }
            LocalReviewCommand::Apply { intent } => ReviewCommand::Apply {
                command: intent.clone(),
                evidence: vec![],
            },
            LocalReviewCommand::WithEvidence { intent, evidence } => ReviewCommand::Apply {
                command: intent.clone(),
                evidence: evidence
                    .iter()
                    .map(|v| CodeEvidence {
                        reference: v.reference.clone(),
                        bytes: v.bytes.clone(),
                    })
                    .collect(),
            },
        };
        let stores = self.collaboration_stores()?;
        review(
            stores
                .review()
                .ok_or(LocalError::Unsupported)?
                .execute(&StoreRequest::new(request.request_id, command))
                .await
                .map_err(map_store_error)?,
        )
    }
    pub async fn review_store_status(&self, id: m::RequestId) -> LocalResult<LocalReviewOutcome> {
        review(
            self.collaboration_stores()?
                .review()
                .ok_or(LocalError::Unsupported)?
                .status(id)
                .await
                .map_err(map_store_error)?,
        )
    }
    pub async fn apply_collab_command(
        &self,
        request: &LocalCollabRequest,
    ) -> LocalResult<LocalCollabOutcome> {
        self.apply_collab_command_with_reviews(request, None).await
    }
    pub async fn apply_collab_command_with_reviews(
        &self,
        request: &LocalCollabRequest,
        qualified: Option<&dyn jj_collab::QualifiedReviewRead>,
    ) -> LocalResult<LocalCollabOutcome> {
        request.validate()?;
        let stores = self.collaboration_stores()?;
        let value = match &request.command {
            LocalCollabCommand::Thread { intent } => stores
                .thread()
                .ok_or(LocalError::Unsupported)?
                .execute_with_reviews(
                    &StoreRequest::new(request.request_id, intent.clone()),
                    qualified,
                )
                .await
                .map_err(map_store_error)?,
            LocalCollabCommand::Reaction { intent } => stores
                .reaction()
                .ok_or(LocalError::Unsupported)?
                .execute_with_reviews(
                    &StoreRequest::new(
                        request.request_id,
                        ReactionCommand::Reaction(intent.clone()),
                    ),
                    qualified,
                )
                .await
                .map_err(map_store_error)?,
            LocalCollabCommand::Emoji { intent } => stores
                .reaction()
                .ok_or(LocalError::Unsupported)?
                .execute_with_reviews(
                    &StoreRequest::new(request.request_id, ReactionCommand::Emoji(intent.clone())),
                    qualified,
                )
                .await
                .map_err(map_store_error)?,
        };
        collab(value)
    }
    pub async fn collab_store_status(&self, id: m::RequestId) -> LocalResult<LocalCollabOutcome> {
        let stores = self.collaboration_stores()?;
        let value = match stores
            .thread()
            .ok_or(LocalError::Unsupported)?
            .status(id)
            .await
        {
            Err(c::Error::IntentConflict) => stores
                .reaction()
                .ok_or(LocalError::Unsupported)?
                .status(id)
                .await
                .map_err(map_store_error)?,
            result => result.map_err(map_store_error)?,
        };
        collab(value)
    }
    pub async fn apply_work_command_async(
        &self,
        request: &crate::work::Request,
    ) -> crate::work::Result<LocalWorkOutcome> {
        if request.version != 1 {
            return Err(crate::work::Error::Invalid);
        }
        let stores = self.collaboration_stores()?;
        work(
            stores
                .work()
                .ok_or(c::Error::Unsupported("WorkStore disabled"))?
                .execute(&StoreRequest::new(
                    request.request_id,
                    request.command.clone(),
                ))
                .await?,
        )
    }
    pub async fn work_store_status(
        &self,
        id: m::RequestId,
    ) -> crate::work::Result<LocalWorkOutcome> {
        let stores = self.collaboration_stores()?;
        work(
            stores
                .work()
                .ok_or(c::Error::Unsupported("WorkStore disabled"))?
                .status(id)
                .await?,
        )
    }
}
fn agent(value: StoreOutcome<AgentResult>) -> LocalResult<LocalAgentOutcome> {
    Ok(match value {
        StoreOutcome::Committed(r) => LocalAgentOutcome::Committed {
            receipt: Box::new(
                serde_json::from_slice(&r.provider_receipt).map_err(|_| LocalError::Corrupt)?,
            ),
        },
        StoreOutcome::Unknown { request_id } => LocalAgentOutcome::OutcomeUnknown { request_id },
        StoreOutcome::NotKnown { request_id } => LocalAgentOutcome::NotKnown { request_id },
    })
}
fn review(
    value: StoreOutcome<Option<jj_review::ReviewCommandResultV1>>,
) -> LocalResult<LocalReviewOutcome> {
    Ok(match value {
        StoreOutcome::Committed(r) => LocalReviewOutcome::Committed {
            receipt: Box::new(
                serde_json::from_slice(&r.provider_receipt).map_err(|_| LocalError::Corrupt)?,
            ),
        },
        StoreOutcome::Unknown { request_id } => LocalReviewOutcome::OutcomeUnknown { request_id },
        StoreOutcome::NotKnown { request_id } => LocalReviewOutcome::NotKnown { request_id },
    })
}
fn collab(
    value: StoreOutcome<jj_collab::CollabCommandResultV1>,
) -> LocalResult<LocalCollabOutcome> {
    Ok(match value {
        StoreOutcome::Committed(r) => LocalCollabOutcome::Committed {
            receipt: Box::new(
                serde_json::from_slice(&r.provider_receipt).map_err(|_| LocalError::Corrupt)?,
            ),
        },
        StoreOutcome::Unknown { request_id } => LocalCollabOutcome::OutcomeUnknown { request_id },
        StoreOutcome::NotKnown { request_id } => LocalCollabOutcome::NotKnown { request_id },
    })
}
fn work(
    value: StoreOutcome<crate::work::WorkCommandResultV1>,
) -> crate::work::Result<LocalWorkOutcome> {
    Ok(match value {
        StoreOutcome::Committed(r) => LocalWorkOutcome::Committed {
            receipt: Box::new(
                serde_json::from_slice(&r.provider_receipt)
                    .map_err(|_| crate::work::Error::Corrupt)?,
            ),
        },
        StoreOutcome::Unknown { request_id } => LocalWorkOutcome::OutcomeUnknown { request_id },
        StoreOutcome::NotKnown { request_id } => LocalWorkOutcome::NotKnown { request_id },
    })
}

// Command errors have already passed the provider's storage boundary. Preserve
// their public semantic category without changing low-level immutable-read maps.
fn map_store_error(error: c::Error) -> LocalError {
    match error {
        c::Error::Invalid(_) => LocalError::Invalid,
        c::Error::NotFound => LocalError::NotFound,
        other => other.into(),
    }
}
