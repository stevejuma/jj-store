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
    pub(super) fn validate_thread_binding(&self) -> Result<()> {
        if let Some(value) = &self.thread_binding {
            if value.accepted_at_millis < 0 {
                return Err(Error::Corrupt("capture binding time"));
            }
            value
                .binding
                .validate()
                .map_err(|_| Error::Corrupt("capture Thread binding"))?;
            AgentProjector::validate_capture_binding_delta(&self.delta)?;
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
        let command = AgentProjector::new(self)
            .capture_binding_command(
                &request.delta,
                &binding.binding,
                request.principal,
                request.request_id,
            )
            .await?;
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
