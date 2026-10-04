use super::*;
use crate::agent_provenance_journal::ProvenanceJournal;
use jj_agent::{
    AgentCatalogKind, AgentCheckpointVersionV2, AgentObjectRef, AgentObjectV1, CodeAssociationV2,
};
use jj_store_model::{
    RequestId,
    portable::{CodeEvidenceRefV1, CodeProfileV1},
};
use jj_store_service::local_repository::{
    LocalAgentCommand, LocalAgentRequest, LocalCodeEvidenceV1, LocalRepository,
};
impl Backend {
    pub(super) async fn provenance_action(
        &self,
        helper: &CommandHelper,
        command: &AgentCommand,
        state: &Path,
    ) -> Result<Option<serde_json::Value>, CommandError> {
        if !matches!(
            command,
            AgentCommand::CodeEvidence { .. }
                | AgentCommand::Associate { .. }
                | AgentCommand::Checkpoint { .. }
                | AgentCommand::RetryProvenance { .. }
                | AgentCommand::Associations { .. }
                | AgentCommand::Checkpoints { .. }
        ) {
            return Ok(None);
        }
        let Self::Local(repo) = self else {
            return Err(error(
                "portable provenance commands currently require the local Git profile",
            ));
        };
        let value = match command {
            AgentCommand::Associations { limit, cursor }
            | AgentCommand::Checkpoints { limit, cursor } => {
                let catalog = if matches!(command, AgentCommand::Associations { .. }) {
                    AgentCatalogKind::Associations
                } else {
                    AgentCatalogKind::Checkpoints
                };
                serde_json::to_value(
                    repo.agent_provenance_page(catalog, *limit, cursor.as_deref())
                        .await
                        .map_err(error)?,
                )
                .map_err(error)?
            }
            AgentCommand::CodeEvidence { operation } => {
                let repository = repo.repository().map_err(error)?.ok_or_else(|| {
                    error("initialize local metadata before inspecting code evidence")
                })?;
                let captured = jj_store_jj::local_evidence::capture(
                    helper.settings(),
                    helper.workspace_loader()?.repo_path(),
                    repository,
                    *operation,
                    repo.metadata_store(),
                )
                .await
                .map_err(error)?;
                let capsule = jj_store_jj::local_evidence::GitEvidenceCapsuleV1::decode(
                    captured.capsule,
                    &repo
                        .metadata_store()
                        .read_staged_blob(captured.blob)
                        .map_err(error)?,
                )
                .map_err(error)?;
                serde_json::json!({"version":2,"reference":CodeEvidenceRefV1 { repository, profile:CodeProfileV1::GitSha1Jj045, operation:*operation, capsule:captured.capsule },"capsule":capsule,"selected":false})
            }
            AgentCommand::Associate {
                request_id,
                record,
                expected_version,
            }
            | AgentCommand::Checkpoint {
                request_id,
                record,
                expected_version,
            } => {
                let bytes = jj_store_server::policy::read_regular(
                    &helper.cwd().join(record),
                    16 * 1024 * 1024,
                )
                .map_err(error)?;
                let object = if matches!(command, AgentCommand::Associate { .. }) {
                    AgentObjectV1::PortableAssociation(
                        serde_json::from_slice::<CodeAssociationV2>(&bytes)
                            .map_err(|_| error("invalid portable association document"))?,
                    )
                } else {
                    AgentObjectV1::PortableCheckpointVersion(
                        serde_json::from_slice::<AgentCheckpointVersionV2>(&bytes)
                            .map_err(|_| error("invalid portable checkpoint document"))?,
                    )
                };
                object.validate().map_err(error)?;
                let expected = expected_version.map(|id| AgentObjectRef::new(object.kind(), id));
                publish(helper, repo, state, *request_id, expected, object).await?
            }
            AgentCommand::RetryProvenance { request_id } => {
                let journal = journal(repo, state)?;
                let request = journal
                    .pending(*request_id)
                    .map_err(error)?
                    .ok_or_else(|| {
                        error("no pending provenance request; inspect its durable receipt")
                    })?;
                serde_json::to_value(journal.publish(repo, &request).await.map_err(error)?)
                    .map_err(error)?
            }
            _ => unreachable!(),
        };
        Ok(Some(value))
    }
}
fn journal(repo: &LocalRepository, state: &Path) -> Result<ProvenanceJournal, CommandError> {
    let repository = repo
        .repository()
        .map_err(error)?
        .ok_or_else(|| error("initialize local metadata before publishing provenance"))?;
    let actor = repo.ensure_actor().map_err(error)?;
    ProvenanceJournal::open(state, repository, actor).map_err(error)
}
async fn publish(
    helper: &CommandHelper,
    repo: &LocalRepository,
    state: &Path,
    request_id: RequestId,
    expected: Option<AgentObjectRef>,
    object: AgentObjectV1,
) -> Result<serde_json::Value, CommandError> {
    let journal = journal(repo, state)?;
    let request = if let Some(request) = journal.pending(request_id).map_err(error)? {
        match &request.command {
            LocalAgentCommand::Provenance {
                expected: previous,
                object: original,
                ..
            } if *previous == expected && original.as_ref() == &object => {}
            _ => {
                return Err(error(
                    "provenance retry differs from original record or version",
                ));
            }
        }
        request
    } else {
        let selected = repo.agent_store().await.map_err(error)?;
        let repository = selected
            .snapshot
            .repo_id
            .ok_or_else(|| error("local metadata identity unavailable"))?;
        let mut evidence = Vec::new();
        for reference in object.code_evidence().map_err(error)? {
            if let Ok(capsule) = selected.code_evidence(&reference) {
                evidence.push(LocalCodeEvidenceV1 {
                    reference,
                    bytes: capsule.encode().map_err(error)?,
                });
                continue;
            }
            if reference.repository != repository
                || reference.profile != CodeProfileV1::GitSha1Jj045
            {
                return Err(error(
                    "external source evidence is not selected in this local repository",
                ));
            }
            let captured = jj_store_jj::local_evidence::capture(
                helper.settings(),
                helper.workspace_loader()?.repo_path(),
                repository,
                reference.operation,
                repo.metadata_store(),
            )
            .await
            .map_err(error)?;
            if captured.capsule != reference.capsule {
                return Err(error("source capsule differs from the requested evidence"));
            }
            evidence.push(LocalCodeEvidenceV1 {
                reference,
                bytes: repo
                    .metadata_store()
                    .read_staged_blob(captured.blob)
                    .map_err(error)?,
            });
        }
        LocalAgentRequest {
            version: 1,
            request_id,
            command: LocalAgentCommand::Provenance {
                expected,
                object: Box::new(object),
                evidence,
            },
        }
    };
    journal.prepare(&request).map_err(error)?;
    serde_json::to_value(journal.publish(repo, &request).await.map_err(error)?).map_err(error)
}
