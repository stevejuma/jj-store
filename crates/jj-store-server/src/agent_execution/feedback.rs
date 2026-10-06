use super::*;
use super::{Error, Result};
use jj_agent::*;
use jj_store_service::collab_reads::{PinnedCollabRead, RepositoryMemberReactionActors};

impl Supervisor {
    pub(super) async fn feedback_saved(&self, saved: &SavedSession) -> Result<Feedback> {
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        let rows = recorded_rows(saved)?;
        let feedback = if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
                .map_err(|_| Error::Stale)?;
            let mut feedback = local.agent_feedback(&rows).await?;
            feedback.write_unavailable = Some(
                "Replies and reactions for registered folders are not available in the inbox yet."
                    .into(),
            );
            feedback
        } else {
            let session = self
                .0
                .resources
                .host
                .as_ref()
                .ok_or(Error::Unavailable)?
                .endpoint(saved.view.repository)
                .map_err(|_| Error::Denied)?
                .bind_local_trusted(
                    self.0.resources.actor,
                    jj_store_service::auth::Permission::Admin,
                )
                .map_err(|_| Error::Denied)?
                .into_session();
            let pin =
                PinnedCollabRead::open(session.clone(), Arc::new(RepositoryMemberReactionActors))
                    .await
                    .map_err(|_| Error::Unavailable)?;
            pin.agent_feedback(session, &rows).await?
        };
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        Ok(feedback)
    }
}

fn recorded_rows(saved: &SavedSession) -> Result<Vec<(String, AgentSessionId, MessageId)>> {
    let source = SourceInstanceId::new(publication::identity(&saved.view.id));
    let mut rows = BTreeMap::new();
    // Use the capture normalizer, never duplicate its canonical identity algorithm.
    for event in saved
        .events
        .iter()
        .filter(|e| e.committed && matches!(e.payload["type"].as_str(), Some("prompt" | "record")))
    {
        let batch = SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: CaptureBatchId::new([1; 16]),
            integration: IntegrationId::OwnedController,
            source_instance_id: source,
            sanitizer_version: jj_store_capture::transcript_sanitization::TRANSCRIPT_POLICY.into(),
            representation: SanitizedRepresentation::SanitizedNative,
            observations: vec![SanitizedObservationV1 {
                observation_id: ObservationId::new(publication::identity(&format!(
                    "observation/{}/{}",
                    saved.view.id, event.sequence
                ))),
                source_sequence: event.sequence,
                occurred_at_millis: event.observed_at_millis.and_then(|v| i64::try_from(v).ok()),
                source_kind: SourceKind::Controller,
                source_trust: SourceTrust::AuthenticatedController,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&event.payload).map_err(|_| Error::Invalid)?,
                )
                .map_err(|_| Error::Invalid)?,
            }],
        };
        let normalized = normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &batch)
            .map_err(|_| Error::Invalid)?;
        for alias in normalized.aliases {
            if let AgentEntityRefV1::Message {
                session_id,
                message_id,
            } = alias.target
            {
                let row = alias
                    .native_id
                    .strip_prefix(&format!("{}/", saved.view.id))
                    .ok_or(Error::Invalid)?
                    .to_owned();
                rows.insert(row.clone(), (row, session_id, message_id));
            }
        }
    }
    if rows.len() > 256 {
        return Err(Error::Limit);
    }
    Ok(rows.into_values().collect())
}
