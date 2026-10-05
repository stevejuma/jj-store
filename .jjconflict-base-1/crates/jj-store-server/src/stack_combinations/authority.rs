use super::*;
#[async_trait::async_trait]
impl p::CombinationAuthority for CombinationStore {
    async fn current(
        &self,
        repository: m::RepositoryId,
        physical: s::PhysicalWorkspaceId,
        workspace: &str,
    ) -> p::Result<Option<p::ActiveCombination>> {
        if repository != self.scope.repository
            || physical != self.scope.physical_workspace
            || workspace != self.scope.jj_workspace
        {
            return Err(p::Error::Forbidden);
        }
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let gate = this.root.gate().map_err(|_| p::Error::Unsupported)?;
            let catalog = this.read(&gate).map_err(|_| p::Error::Unsupported)?.0;
            if catalog.records.iter().any(|r| r.phase == Phase::Pending) {
                return Err(p::Error::OutcomeUnknown);
            }
            Ok(catalog.selected.active().map(|value| p::ActiveCombination {
                selection: value.selection.clone(),
                code: value.code.clone(),
                immutable_heads: value.immutable_heads.clone(),
            }))
        })
        .await
        .map_err(|_| p::Error::Unsupported)?
    }
}
