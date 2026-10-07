use super::*;
use super::{Error, Result};
use jj_agent::*;
use jj_store_service::collab_reads::{PinnedCollabRead, RepositoryMemberReactionActors};

impl Supervisor {
    pub(super) async fn feedback_saved(&self, saved: &SavedSession) -> Result<Feedback> {
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        let rows = recorded_rows(saved)?;
        let mut feedback = if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
                .map_err(|_| Error::Stale)?;
            local.agent_feedback(&rows).await?
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
        feedback.profile = Some(saved.profile.clone());
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        Ok(feedback)
    }
    pub(super) async fn prompt_feedback(
        &self,
        saved: &SavedSession,
        carries_initial_context: bool,
    ) -> Result<context::ContextBatch> {
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        let (rows, sessions) = recorded_targets(saved)?;
        let mut excluded =
            self.read(|state| Ok(super::delivery::excluded(state, &saved.view.id)))?;
        if carries_initial_context {
            excluded.extend(saved.context_replies.iter().cloned());
        }
        let result = if saved.profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(saved.view.repository)
                .map_err(|_| Error::Stale)?;
            local
                .agent_prompt_context(&rows, &sessions, &excluded)
                .await?
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
            pin.agent_prompt_context(session, &rows, &sessions, &excluded)
                .await?
        };
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        Ok(result)
    }
}

fn recorded_rows(saved: &SavedSession) -> Result<Vec<(String, AgentSessionId, MessageId)>> {
    Ok(recorded_targets(saved)?.0)
}
fn recorded_targets(
    saved: &SavedSession,
) -> Result<(
    Vec<(String, AgentSessionId, MessageId)>,
    std::collections::BTreeSet<AgentSessionId>,
)> {
    let mut rows = BTreeMap::new();
    let mut sessions = std::collections::BTreeSet::new();
    // Use the capture normalizer, never duplicate its canonical identity algorithm.
    for event in saved.events.iter().filter(|e| {
        e.committed
            && matches!(
                e.payload["type"].as_str(),
                Some("prompt" | "record" | "opened")
            )
    }) {
        let normalized = normalize_event(saved, event)?;
        for alias in normalized.aliases {
            if let AgentEntityRefV1::Session { session_id } = &alias.target {
                sessions.insert(*session_id);
            }
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
                sessions.insert(session_id);
                rows.insert(row.clone(), (row, session_id, message_id));
            }
        }
    }
    if rows.len() > 256 {
        return Err(Error::Limit);
    }
    Ok((rows.into_values().collect(), sessions))
}

/// Return only exact aliases from committed evidence, using the owning normalizer.
pub(super) fn attach_identity(session: &mut Session, saved: &SavedSession) -> Result<()> {
    session.recorded_session_ids = recorded_session_ids(saved)?;
    session.workspace_name =
        (saved.workspace_ready && session.state != "cleaned").then(|| saved.workspace.name.clone());
    session.workspace_operation = session
        .workspace_name
        .as_ref()
        .map(|_| saved.workspace.operation);
    Ok(())
}
pub(super) fn recorded_session_ids(saved: &SavedSession) -> Result<Vec<AgentSessionId>> {
    let Some(event) = saved.events.iter().find(|event| {
        event.committed
            && event.payload["controllerSessionId"].as_str() == Some(saved.view.id.as_str())
    }) else {
        return Ok(Vec::new());
    };
    Ok(normalize_event(saved, event)?
        .aliases
        .into_iter()
        .filter_map(|alias| {
            if let AgentEntityRefV1::Session { session_id } = alias.target {
                Some(session_id)
            } else {
                None
            }
        })
        .collect())
}
fn normalize_event(saved: &SavedSession, event: &Event) -> Result<NormalizationBatchV1> {
    let source = SourceInstanceId::new(publication::identity(&saved.view.id));
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
    normalize_batch(&NormalizerProfileV1::acp_controller_v2(), &batch).map_err(|_| Error::Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn committed_session_aliases_preserve_capture_identity_and_isolate_instances() {
        let id = "44".repeat(16);
        let view = json!({"id":id,"setup_id":"helper","repository":"11".repeat(16),
            "target_id":"fixture","runner":"codex","title":"Nyra","avatar":"🐝",
            "workspace":"/tmp/workspace","base_commit":"22".repeat(20),
            "review_commit":"22".repeat(20),"operation":"33".repeat(64),"state":"ended",
            "adapter_session_id":"same-adapter","capabilities":{},
            "preferences":{"model":null,"effort":null,"mode":null},
            "last_sequence":1,"published_sequence":1,"last_error":null});
        let mut saved: SavedSession = serde_json::from_value(json!({"view":view,
            "workspace":{"source":"/tmp/source","path":"/tmp/workspace","metadata":"/tmp/source/.jj",
                "operation":"33".repeat(64),"commit":"22".repeat(20),"name":"exact-workspace"},
            "profile":"local_jj_git","instructions":"","context":null,"workspace_ready":true,
            "events":[{"sequence":1,"committed":true,
                "payload":{"type":"prompt","controllerSessionId":id,"requestId":"turn","text":"Hello"}}]
        })).unwrap();
        let ids = recorded_session_ids(&saved).unwrap();
        saved.workspace.operation = "77".repeat(64).parse().unwrap();
        assert_eq!(ids.len(), 1);
        // These are the same native targets used by ordinary repository discussions.
        assert_eq!(recorded_targets(&saved).unwrap().0[0].1, ids[0]);
        let mut presented = saved.view.clone();
        attach_identity(&mut presented, &saved).unwrap();
        assert_eq!(presented.recorded_session_ids, ids);
        assert_eq!(presented.workspace_name.as_deref(), Some("exact-workspace"));
        assert_eq!(
            presented.workspace_operation,
            Some(saved.workspace.operation)
        );
        assert_ne!(presented.workspace_operation, Some(presented.operation));
        presented.state = "cleaned".into();
        attach_identity(&mut presented, &saved).unwrap();
        assert!(presented.workspace_name.is_none());
        assert!(presented.workspace_operation.is_none());
        assert_eq!(presented.recorded_session_ids, ids);
        saved.workspace_ready = false;
        presented.state = "preparing".into();
        attach_identity(&mut presented, &saved).unwrap();
        assert!(presented.workspace_name.is_none());
        saved.events[0].committed = false;
        assert!(recorded_session_ids(&saved).unwrap().is_empty());
        saved.events[0].committed = true;
        saved.events[0].payload["controllerSessionId"] = json!("55".repeat(16));
        assert!(recorded_session_ids(&saved).unwrap().is_empty());
        saved.view.id = "55".repeat(16);
        assert_ne!(recorded_session_ids(&saved).unwrap(), ids);
        // A missing legacy timestamp is irrelevant to the exact identity association.
        assert!(saved.events[0].observed_at_millis.is_none());
    }
}
