use super::*;
use jj_store_wal::publication::{
    AgentCatalogKey, AgentCatalogKind, AgentContentV1, AgentEventV1, AgentObjectKind,
    AgentObjectRef, AgentObjectV1, AgentRelationCursor, AgentRelationSubjectV1, AgentSessionId,
    SanitizedPayloadV1, ToolInvocationSnapshotV1, ToolKind,
};
use std::collections::BTreeSet;

const MAX_ACTIVITY: usize = 4_096;
const MAX_ACTIVITY_BYTES: usize = 8 * 1024 * 1024;
const MAX_CODE_ASSOCIATIONS: usize = 64;

impl AgentSessionPager {
    pub async fn detail(
        &self,
        session: Arc<NativeSession>,
        session_id: AgentSessionId,
    ) -> Result<AgentSessionDetail> {
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.detail_inner(session, session_id),
        )
        .await
    }

    async fn detail_inner(
        &self,
        session: Arc<NativeSession>,
        session_id: AgentSessionId,
    ) -> Result<AgentSessionDetail> {
        let pin = self.pin(&session).await?;
        let key = AgentCatalogKey::derive("session", session_id.as_bytes())
            .map_err(|_| Error::Corrupt)?;
        let reference = pin
            .store
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .map_err(map_wal)?
            .ok_or(Error::NotFound)?;
        let AgentObjectV1::SessionSnapshot(snapshot) =
            pin.store.read_object(reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if snapshot.session_id != session_id
            || snapshot.catalog_key().map_err(|_| Error::Corrupt)? != key
        {
            return Err(Error::Corrupt);
        }
        let mut transcripts = Vec::new();
        for transcript in &snapshot.transcripts {
            let AgentObjectV1::TranscriptManifest(manifest) =
                pin.store.read_object(*transcript).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if manifest.session_id != session_id {
                return Err(Error::Corrupt);
            }
            transcripts.extend(
                manifest
                    .streams
                    .iter()
                    .map(|stream| models::transcript_descriptor(transcript.id, stream)),
            );
        }
        let (code, code_truncated) = load_code(&pin, session_id).await?;
        let (activity, activity_truncated) = load_activity(&pin.store, &snapshot, &code).await?;
        let timing = super::agent_session_timing::resolve(&pin.store, &snapshot).await?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        Ok(models::detail(
            pin.snapshot,
            reference,
            snapshot,
            models::AgentSessionDetailContent {
                timing,
                transcripts,
                activity,
                activity_truncated,
                code,
                code_truncated,
            },
        ))
    }
}

async fn load_code(
    pin: &AgentPin,
    session_id: AgentSessionId,
) -> Result<(Vec<AgentCodeAssociation>, bool)> {
    let subject = AgentRelationSubjectV1::Session { session_id };
    let mut cursor: Option<AgentRelationCursor> = None;
    let mut code = Vec::new();
    let mut truncated = false;
    loop {
        let range = pin
            .store
            .relations(&subject, cursor, MAX_PAGE)
            .await
            .map_err(map_wal)?;
        cursor = range.next;
        for reference in range.selections {
            if reference.kind != AgentObjectKind::Association {
                continue;
            }
            if code.len() == MAX_CODE_ASSOCIATIONS {
                truncated = true;
                break;
            }
            let AgentObjectV1::Association(value) =
                pin.store.read_object(reference).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if value.session_id != session_id {
                return Err(Error::Corrupt);
            }
            code.push(models::code_association(reference.id, &value));
        }
        if truncated || cursor.is_none() {
            break;
        }
    }
    Ok((code, truncated || cursor.is_some()))
}

pub(crate) async fn load_activity(
    store: &dyn jj_store_wal::publication::AgentSnapshotRead,
    session: &jj_store_wal::publication::AgentSessionSnapshotV1,
    code: &[AgentCodeAssociation],
) -> Result<(Vec<AgentActivity>, bool)> {
    let mut events = Vec::new();
    let mut truncated = false;
    for reference in &session.event_batches {
        let AgentObjectV1::EventBatch(batch) =
            store.read_object(*reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        let AgentObjectV1::ObservationBatch(evidence) =
            store.read_object(batch.evidence).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if evidence.integration != session.integration {
            return Err(Error::Corrupt);
        }
        for event in batch.events {
            if events.len() == MAX_ACTIVITY {
                truncated = true;
                break;
            }
            let occurred_at_millis = match &event {
                AgentEventV1::MessageCompleted { event_id, .. } => {
                    super::agent_session_timing::event_at_millis(
                        &evidence,
                        &batch.normalizer_version,
                        *event_id,
                        "message",
                    )
                }
                _ => None,
            };
            events.push((*reference, event, occurred_at_millis));
        }
        if truncated {
            break;
        }
    }

    let observed_paths = events
        .iter()
        .filter_map(|(_, event, _)| match event {
            AgentEventV1::FileObserved { path, .. } => Some(path.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut bytes = 0usize;
    let mut output = Vec::new();
    let mut emitted_tools = BTreeSet::new();
    let mut edit_activity_indices = Vec::new();
    for (index, (batch, event, occurred_at_millis)) in events.into_iter().enumerate() {
        let sequence = index as u64;
        let item = match event {
            AgentEventV1::SessionStateChanged {
                event_id,
                session_id,
                state,
            } => {
                ensure_session(session_id, session.session_id)?;
                AgentActivity::SessionState {
                    sequence,
                    event_id,
                    state,
                }
            }
            AgentEventV1::MessageCompleted {
                event_id,
                session_id,
                turn_id,
                message_id,
                role,
                content,
            } => {
                ensure_session(session_id, session.session_id)?;
                let reaction_target = Some(jj_collab::ReactionTargetV1::AgentMessage {
                    session_id,
                    message_id,
                    event_id,
                    batch,
                    content,
                });
                let content = load_content(store, content).await?;
                if bytes
                    .checked_add(content.content.len())
                    .is_none_or(|value| value > MAX_ACTIVITY_BYTES)
                {
                    truncated = true;
                    break;
                }
                bytes += content.content.len();
                AgentActivity::Message {
                    reaction_target,
                    sequence,
                    event_id,
                    turn_id,
                    message_id,
                    occurred_at_millis,
                    role,
                    content,
                }
            }
            AgentEventV1::ToolStateChanged {
                event_id,
                session_id,
                tool_call_id,
                ..
            } => {
                ensure_session(session_id, session.session_id)?;
                if !emitted_tools.insert(tool_call_id) {
                    continue;
                }
                let reference = session.tools.get(&tool_call_id).ok_or(Error::Corrupt)?;
                let AgentObjectV1::ToolSnapshot(tool) =
                    store.read_object(*reference).await.map_err(map_wal)?
                else {
                    return Err(Error::Corrupt);
                };
                if tool.tool_call_id != tool_call_id || tool.session_id != session.session_id {
                    return Err(Error::Corrupt);
                }
                let (tool, content_bytes) = tool_activity(store, *reference, tool, code).await?;
                if bytes
                    .checked_add(content_bytes)
                    .is_none_or(|value| value > MAX_ACTIVITY_BYTES)
                {
                    truncated = true;
                    break;
                }
                bytes += content_bytes;
                if matches!(
                    tool.kind,
                    ToolKind::Edit | ToolKind::Delete | ToolKind::Move
                ) {
                    edit_activity_indices.push(output.len());
                }
                AgentActivity::Tool {
                    sequence,
                    event_id,
                    tool,
                }
            }
            AgentEventV1::PermissionObserved {
                event_id,
                session_id,
                tool_call_id,
                state,
            } => {
                ensure_session(session_id, session.session_id)?;
                AgentActivity::Permission {
                    sequence,
                    event_id,
                    tool_call_id,
                    state,
                }
            }
            AgentEventV1::FileObserved {
                event_id,
                session_id,
                path,
            } => {
                ensure_session(session_id, session.session_id)?;
                AgentActivity::FileObserved {
                    sequence,
                    event_id,
                    path,
                }
            }
            AgentEventV1::CaptureGap {
                event_id,
                session_id,
                missing_from,
                missing_to,
            } => {
                ensure_session(session_id, session.session_id)?;
                AgentActivity::CaptureGap {
                    sequence,
                    event_id,
                    missing_from,
                    missing_to,
                }
            }
        };
        output.push(item);
    }
    if let [index] = edit_activity_indices.as_slice()
        && let Some(AgentActivity::Tool { tool, .. }) = output.get_mut(*index)
    {
        tool.observed_paths = observed_paths;
    }
    Ok((output, truncated))
}

fn ensure_session(actual: AgentSessionId, expected: AgentSessionId) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::Corrupt)
    }
}

async fn tool_activity(
    store: &dyn jj_store_wal::publication::AgentSnapshotRead,
    reference: AgentObjectRef,
    tool: ToolInvocationSnapshotV1,
    code: &[AgentCodeAssociation],
) -> Result<(AgentToolActivity, usize)> {
    let input = load_optional_content(store, tool.input).await?;
    let output = load_optional_content(store, tool.output).await?;
    let bytes = input
        .as_ref()
        .map_or(0, |value| value.content.len())
        .saturating_add(output.as_ref().map_or(0, |value| value.content.len()));
    Ok((
        AgentToolActivity {
            tool_call_id: tool.tool_call_id,
            snapshot_id: reference.id,
            turn_id: tool.turn_id,
            native_name: tool.native_name,
            kind: tool.kind,
            state: tool.state,
            input,
            output,
            observed_paths: Vec::new(),
            code: code
                .iter()
                .filter(|value| value.tool_call_ids.contains(&tool.tool_call_id))
                .cloned()
                .collect(),
        },
        bytes,
    ))
}

async fn load_optional_content(
    store: &dyn jj_store_wal::publication::AgentSnapshotRead,
    reference: Option<AgentObjectRef>,
) -> Result<Option<AgentContent>> {
    match reference {
        Some(reference) => Ok(Some(load_content(store, reference).await?)),
        None => Ok(None),
    }
}

async fn load_content(
    store: &dyn jj_store_wal::publication::AgentSnapshotRead,
    reference: AgentObjectRef,
) -> Result<AgentContent> {
    if reference.kind != AgentObjectKind::Content {
        return Err(Error::Corrupt);
    }
    let AgentObjectV1::Content(AgentContentV1 { payload, .. }) =
        store.read_object(reference).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    payload_content(&payload)
}

fn payload_content(payload: &SanitizedPayloadV1) -> Result<AgentContent> {
    Ok(AgentContent {
        media_type: payload.media_type(),
        content: String::from_utf8(payload.bytes().to_vec()).map_err(|_| Error::Corrupt)?,
    })
}
