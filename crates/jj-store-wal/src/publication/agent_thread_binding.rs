//! Explicit prepared capture assignment, selected atomically with its Agent delta.
use super::*;
use jj_collab::ThreadBindingV1;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentThreadBindingV1 {
    pub binding: ThreadBindingV1,
    /// Chosen by capture preparation and retained across durable retries.
    pub accepted_at_millis: i64,
}
impl AgentDeltaRequest {
    pub fn with_thread_binding(mut self, binding: ThreadBindingV1, at: i64) -> Result<Self> {
        self.thread_binding = Some(AgentThreadBindingV1 {
            binding,
            accepted_at_millis: at,
        });
        self.validate()?;
        Ok(self)
    }
    pub(super) fn binding_session(&self) -> Result<&jj_agent::AgentCatalogEditV1> {
        let mut sessions = self
            .delta
            .edits
            .iter()
            .filter(|e| e.catalog == AgentCatalogKind::Sessions);
        let one = sessions
            .next()
            .ok_or(Error::Corrupt("bound capture requires one session"))?;
        if sessions.next().is_some()
            || one
                .replacement
                .is_none_or(|r| r.kind != AgentObjectKind::SessionSnapshot)
        {
            return Err(Error::Corrupt(
                "bound capture requires one session snapshot",
            ));
        }
        Ok(one)
    }
    pub(super) fn validate_thread_binding(&self) -> Result<()> {
        if let Some(value) = &self.thread_binding {
            if value.accepted_at_millis < 0 {
                return Err(Error::Corrupt("capture binding time"));
            }
            value
                .binding
                .validate()
                .map_err(|_| Error::Corrupt("capture Thread binding"))?;
            self.binding_session()?;
        }
        Ok(())
    }
}
impl Coordinator {
    pub(super) async fn stage_agent_thread_binding(
        &self,
        request: &AgentDeltaRequest,
        expected: DomainState,
    ) -> Result<DomainSelectionV1> {
        let binding = request
            .thread_binding
            .as_ref()
            .ok_or(Error::Corrupt("missing capture binding"))?;
        let edit = request.binding_session()?;
        let snapshot = edit
            .replacement
            .ok_or(Error::Corrupt("capture session replacement"))?;
        let AgentObjectV1::SessionSnapshot(session) = self.read_agent_object(snapshot).await?
        else {
            return Err(Error::Corrupt("capture session kind"));
        };
        let key = AgentCatalogKey::derive("session", session.session_id.as_bytes())
            .map_err(super::agent::agent_error)?;
        if edit.key != key {
            return Err(Error::Corrupt("capture session identity"));
        }
        let command = binding
            .binding
            .capture_link(
                session.session_id,
                snapshot,
                request.principal,
                request.request_id,
            )
            .map_err(|_| Error::Corrupt("capture link intent"))?;
        let mut threads = self.native_thread_store(expected.root).await?;
        let (root, _) = threads
            .state
            .apply(
                &command,
                request.principal,
                request.request_id,
                binding.accepted_at_millis,
                &mut threads.graph,
            )
            .map_err(|error| match error {
                jj_collab::Error::Stale => Error::Stale,
                jj_collab::Error::Limit => Error::Limit("capture binding"),
                _ => Error::Corrupt("capture binding transition"),
            })?;
        let root = self.stage_native_thread_root(&root, &threads.graph).await?;
        Ok(DomainSelectionV1 {
            domain: Domain::Thread,
            expected,
            root,
        })
    }
}
