//! Live journal observations publish through existing durable capture receipts.
use super::*;
use jj_agent::{
    CaptureBatchId, CaptureCursorV1, IntegrationId, NormalizerProfileV1, ObservationId,
    SanitizedMediaType, SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
    SanitizedRepresentation, SourceInstanceId, SourceKind, SourcePositionV1, SourceTrust,
};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool, agent_publication::drain_capture_spool,
    local_agent_publication::drain_local_capture_spool, transcript_sanitization::TRANSCRIPT_POLICY,
};
use sha2::{Digest as _, Sha256};
pub(super) fn identity(value: &str) -> [u8; 16] {
    Sha256::digest(value.as_bytes())[..16]
        .try_into()
        .expect("digest")
}
impl Supervisor {
    pub(super) async fn publish(&self) -> Result<()> {
        let _guard = self.0.publication.lock().await;
        let sessions = self.read(|s| Ok(s.sessions.values().cloned().collect::<Vec<_>>()))?;
        for saved in sessions {
            if saved.view.published_sequence == saved.view.last_sequence {
                self.publish_comment_answers_status(&saved.view.id).await?;
                if self.metadata_suggestions(&saved.view.id).await.is_ok() {
                    let _ = self.metadata_automatic(&saved.view.id).await;
                }
                continue;
            }
            if self
                .authorized(saved.view.repository, &saved.profile, true)
                .await
                .is_err()
            {
                let _ = self.send(
                    &saved.view.id,
                    json!({"version":2,"type":"close","requestId":"authorization-changed"}),
                );
                continue;
            }
            let result = self.publish_session(&saved).await;
            if result.is_ok() {
                self.publish_comment_answers_status(&saved.view.id).await?;
                if self.metadata_suggestions(&saved.view.id).await.is_ok() {
                    let _ = self.metadata_automatic(&saved.view.id).await;
                }
            }
            if result.is_err() {
                self.update(|s| {
                    let saved = s.sessions.get_mut(&saved.view.id).ok_or(Error::Invalid)?;
                    if !saved
                        .view
                        .last_error
                        .as_deref()
                        .is_some_and(|e| e.starts_with("capture_publication_"))
                    {
                        saved.view.last_error = Some("capture_publication_pending".into());
                    }
                    Ok(())
                })?;
            }
        }
        Ok(())
    }
    async fn publish_comment_answers_status(&self, id: &str) -> Result<()> {
        let result = self.publish_comment_answers(id).await;
        self.update(|s| {
            let saved = s.sessions.get_mut(id).ok_or(Error::Invalid)?;
            if result.is_err() && saved.view.last_error.is_none() {
                saved.view.last_error = Some("comment_reply_publication_pending".into());
            } else if result.is_ok()
                && saved.view.last_error.as_deref() == Some("comment_reply_publication_pending")
            {
                saved.view.last_error = None;
            }
            Ok(())
        })
    }
    async fn publish_session(&self, saved: &SavedSession) -> Result<()> {
        let root = self.0.resources.root.join("spools").join(&saved.view.id);
        let mut spool =
            AgentCaptureSpool::open(&root, 16 * 1024 * 1024).map_err(|_| Error::Unavailable)?;
        let profile = NormalizerProfileV1::acp_controller_v2();
        let source = SourceInstanceId::new(identity(&saved.view.id));
        // A recovered spool retains its original batch and receipt identity.
        let pending = spool.pending().map_err(|_| Error::Unavailable)?;
        let recovered_end = pending
            .iter()
            .filter_map(|entry| match entry.cursor.position {
                SourcePositionV1::NativeSequence { sequence } => Some(sequence),
                _ => None,
            })
            .max();
        let end = if let Some(end) = recovered_end {
            end
        } else {
            let end = match saved.pending_capture_end {
                Some(end) => end,
                None => {
                    let mut bytes = 0;
                    let mut end = saved.view.published_sequence;
                    for event in saved
                        .events
                        .iter()
                        .filter(|e| e.sequence > saved.view.published_sequence)
                        .take(128)
                    {
                        let size = serde_json::to_vec(&event.payload)
                            .map_err(|_| Error::Unavailable)?
                            .len();
                        if bytes + size > 4 * 1024 * 1024 && end > saved.view.published_sequence {
                            break;
                        }
                        bytes += size;
                        end = event.sequence;
                    }
                    self.update(|s| {
                        s.sessions
                            .get_mut(&saved.view.id)
                            .ok_or(Error::Invalid)?
                            .pending_capture_end = Some(end);
                        Ok(())
                    })?;
                    end
                }
            };
            let first = saved.view.published_sequence + 1;
            let observations = saved
                .events
                .iter()
                .filter(|e| e.sequence >= first && e.sequence <= end)
                .map(|event| {
                    Ok(SanitizedObservationV1 {
                        observation_id: ObservationId::new(identity(&format!(
                            "observation/{}/{}",
                            saved.view.id, event.sequence
                        ))),
                        source_sequence: event.sequence,
                        occurred_at_millis: event
                            .observed_at_millis
                            .and_then(|time| i64::try_from(time).ok()),
                        source_kind: SourceKind::Controller,
                        source_trust: SourceTrust::AuthenticatedController,
                        payload: SanitizedPayloadV1::new(
                            SanitizedMediaType::Json,
                            &serde_json::to_vec(&event.payload).map_err(|_| Error::Unavailable)?,
                        )
                        .map_err(|_| Error::Unavailable)?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            let batch = SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: CaptureBatchId::new(identity(&format!(
                    "{}/{first}/{end}",
                    saved.view.id
                ))),
                integration: IntegrationId::OwnedController,
                source_instance_id: source,
                sanitizer_version: TRANSCRIPT_POLICY.into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations,
            };
            let cursor = CaptureCursorV1 {
                schema_version: 1,
                integration: IntegrationId::OwnedController,
                source_instance_id: source,
                generation: 1,
                parser_version: "jj-acp-control-2/v1".into(),
                native_session_key: Some(saved.view.id.clone()),
                position: SourcePositionV1::NativeSequence { sequence: end },
            };
            spool
                .enqueue(saved.view.repository, profile, cursor, batch)
                .map_err(|_| Error::Unavailable)?;
            end
        };
        let report = if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
                .map_err(|_| Error::Stale)?;
            local
                .initialize_for_browser(
                    RequestId::new(identity(&format!("initialize/{}", saved.view.id))),
                    saved.view.repository,
                )
                .await
                .map_err(|_| Error::Unavailable)?;
            drain_local_capture_spool(&mut spool, &local)
                .await
                .map_err(|error| self.publication_failure(&saved.view.id, &*error))?
        } else {
            let publisher = self
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
                .map_err(|_| Error::Denied)?;
            drain_capture_spool(&mut spool, saved.view.repository, &publisher)
                .await
                .map_err(|error| self.publication_failure(&saved.view.id, &*error))?
        };
        if report.outcome_unknown || report.remaining != 0 {
            return Err(Error::Unavailable);
        }
        self.update(|s| {
            let saved = s.sessions.get_mut(&saved.view.id).ok_or(Error::Invalid)?;
            saved.view.published_sequence = end;
            saved.pending_capture_end = None;
            for e in &mut saved.events {
                if e.sequence <= end {
                    e.committed = true;
                }
            }
            if saved
                .view
                .last_error
                .as_deref()
                .is_some_and(|e| e.starts_with("capture_publication_"))
            {
                saved.view.last_error = None;
            }
            Ok(())
        })?;
        Ok(())
    }
    fn publication_failure(&self, id: &str, error: &dyn std::fmt::Display) -> Error {
        // Retain only fixed classification vocabulary, never source diagnostics.
        let diagnostic = error.to_string();
        let description = diagnostic.to_ascii_lowercase();
        let markers = [
            "alias", "cursor", "session", "tool", "content", "payload", "identity", "sequence",
            "receipt", "intent", "conflict", "limit", "version", "state", "commit", "lock",
            "closure", "snapshot",
        ];
        let classification = markers
            .into_iter()
            .filter(|marker| description.contains(marker))
            .collect::<Vec<_>>()
            .join("_");
        let _ = self.update(|s| {
            s.sessions
                .get_mut(id)
                .ok_or(Error::Invalid)?
                .view
                .last_error = Some(format!("capture_publication_pending_{classification}"));
            Ok(())
        });
        Error::Unavailable
    }
}
