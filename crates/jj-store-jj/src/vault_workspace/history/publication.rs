//! Immutable publication policy accesses only its owning reader's pinned state.
use super::*;
use crate::vault_workspace::remote::publication::{PushIntent, prepare};
impl HistoryReader {
    /// Pin this reader's retained checkpoint. Does not capture, reserve or transfer.
    pub async fn prepare_push(
        &self,
        remote: &RemoteTransfer,
        id: &str,
        before: Option<String>,
    ) -> Result<PushIntent> {
        prepare::prepare(
            &self.binding,
            &self.repo,
            self.checkpoint()?,
            remote,
            id,
            before,
        )
        .await
    }
    /// Counts this exact retained intent, independent of a later capture.
    pub async fn push_divergence(&self, intent: &PushIntent) -> Result<CommitDivergence> {
        prepare::divergence(&self.binding, &self.repo, intent).await
    }
    pub(in crate::vault_workspace) async fn validate_push(
        &self,
        intent: &PushIntent,
    ) -> Result<()> {
        prepare::validate(&self.binding, &self.repo, intent).await
    }
}
