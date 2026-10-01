use super::*;
use jj_store_wal::publication::{
    AgentEventId, AgentEventV1, AgentObjectV1, AgentSessionId, AgentSessionSnapshotV1,
    normalized_event_id,
};

const MAX_TIMING_EVENTS: usize = 4_096;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SessionTiming {
    pub first_activity_at_millis: Option<i64>,
    pub last_activity_at_millis: Option<i64>,
}

pub(crate) async fn resolve(
    store: &dyn wal::publication::AgentSnapshotRead,
    session: &AgentSessionSnapshotV1,
) -> Result<SessionTiming> {
    let mut timing = SessionTiming::default();
    let mut inspected = 0usize;
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
        for event in &batch.events {
            let Some((event_id, kind)) = timing_event(event, session.session_id) else {
                continue;
            };
            inspected = inspected.checked_add(1).ok_or(Error::Limit)?;
            if inspected > MAX_TIMING_EVENTS {
                return Ok(SessionTiming::default());
            }
            let occurred_at_millis = evidence.observations.iter().find_map(|observation| {
                (normalized_event_id(
                    evidence.source_instance_id,
                    evidence.integration,
                    observation.source_sequence,
                    kind,
                ) == event_id)
                    .then_some(observation.occurred_at_millis)
                    .flatten()
            });
            if let Some(value) = occurred_at_millis {
                timing.first_activity_at_millis = Some(
                    timing
                        .first_activity_at_millis
                        .map_or(value, |current| current.min(value)),
                );
                timing.last_activity_at_millis = Some(
                    timing
                        .last_activity_at_millis
                        .map_or(value, |current| current.max(value)),
                );
            }
        }
    }
    Ok(timing)
}

fn timing_event(
    event: &AgentEventV1,
    session_id: AgentSessionId,
) -> Option<(AgentEventId, &'static str)> {
    match event {
        AgentEventV1::SessionStateChanged {
            event_id,
            session_id: actual,
            ..
        } if *actual == session_id => Some((*event_id, "session")),
        AgentEventV1::MessageCompleted {
            event_id,
            session_id: actual,
            ..
        } if *actual == session_id => Some((*event_id, "message")),
        AgentEventV1::ToolStateChanged {
            event_id,
            session_id: actual,
            ..
        } if *actual == session_id => Some((*event_id, "tool")),
        AgentEventV1::PermissionObserved {
            event_id,
            session_id: actual,
            ..
        } if *actual == session_id => Some((*event_id, "permission")),
        AgentEventV1::FileObserved {
            event_id,
            session_id: actual,
            ..
        } if *actual == session_id => Some((*event_id, "file")),
        _ => None,
    }
}
