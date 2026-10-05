//! Explicit capture assignment intent shared by publication providers.
use crate::{Result, ThreadBindingV1, ThreadCommandV1, ThreadLinkId, ThreadLinkTargetV1};
use jj_agent::{AgentObjectKind, AgentObjectRef, AgentSessionId};
use jj_store_model::{PrincipalId, RequestId};
use sha2::{Digest as _, Sha256};
impl ThreadBindingV1 {
    /// Diagnostic evidence retains binding intent without fabricating a session.
    /// Providers must validate the exact staged source and empty event batch.
    pub fn capture_gap(
        &self,
        source_state: AgentObjectRef,
        event_batch: AgentObjectRef,
        actor: PrincipalId,
        request: RequestId,
    ) -> Result<ThreadCommandV1> {
        self.validate()?;
        if source_state.kind != AgentObjectKind::SourceState
            || event_batch.kind != AgentObjectKind::EventBatch
        {
            return Err(crate::Error::Invalid("bound capture gap references"));
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/capture-thread-gap/v1\0");
        hash.update(actor.as_bytes());
        hash.update(request.as_bytes());
        hash.update(source_state.id.as_bytes());
        hash.update(event_batch.id.as_bytes());
        Ok(ThreadCommandV1::Link {
            thread_id: self.thread_id,
            expected_thread: self.observed_thread,
            observed_primary: self.observed_assignment,
            link_id: ThreadLinkId::new(hash.finalize()[..16].try_into().expect("SHA-256 prefix")),
            target: ThreadLinkTargetV1::CaptureGap {
                source_state,
                event_batch,
                binding: self.clone(),
            },
        })
    }
    /// The provider must verify that the exact session replacement belongs to
    /// its prepared capture delta before atomically selecting both roots.
    pub fn capture_link(
        &self,
        session_id: AgentSessionId,
        snapshot: AgentObjectRef,
        actor: PrincipalId,
        request: RequestId,
    ) -> Result<ThreadCommandV1> {
        self.validate()?;
        if snapshot.kind != AgentObjectKind::SessionSnapshot {
            return Err(crate::Error::Invalid("bound capture session snapshot"));
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/capture-thread-link/v1\0");
        hash.update(actor.as_bytes());
        hash.update(request.as_bytes());
        hash.update(session_id.as_bytes());
        Ok(ThreadCommandV1::Link {
            thread_id: self.thread_id,
            expected_thread: self.observed_thread,
            observed_primary: self.observed_assignment,
            link_id: ThreadLinkId::new(hash.finalize()[..16].try_into().expect("SHA-256 prefix")),
            target: ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                primary: true,
            },
        })
    }
}
