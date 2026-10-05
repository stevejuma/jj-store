//! Upstream prefix ambiguity and offsets with exact compact visibility.
use super::*;

pub(super) struct Changes {
    pub decorations: Arc<Decorations>,
    pub budget: Arc<Budget>,
    pub matches: bool,
}
impl Changes {
    fn check(&self) -> IndexResult<()> {
        self.budget
            .work(1)
            .map_err(|e| IndexError::Other(Box::new(e)))?;
        if !self.matches {
            return Err(IndexError::Other(Box::new(
                self.budget.fail(Error::NotCommitted),
            )));
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl ChangeIdIndex for Changes {
    async fn resolve_prefix(
        &self,
        prefix: &HexPrefix,
    ) -> IndexResult<PrefixResolution<ResolvedChangeTargets>> {
        self.check()?;
        let mut result = self.decorations.changes.resolve_prefix(prefix).await?;
        if let PrefixResolution::SingleMatch(targets) = &mut result {
            for (commit, state) in &mut targets.targets {
                self.budget
                    .work(32)
                    .map_err(|e| IndexError::Other(Box::new(e)))?;
                *state = if self
                    .decorations
                    .visible
                    .contains(&id(commit).map_err(|e| IndexError::Other(Box::new(e)))?)
                {
                    ResolvedChangeState::Visible
                } else {
                    ResolvedChangeState::Hidden
                };
            }
        }
        self.budget
            .check()
            .map_err(|e| IndexError::Other(Box::new(e)))?;
        Ok(result)
    }
    async fn shortest_unique_prefix_len(&self, id: &b::ChangeId) -> IndexResult<usize> {
        self.check()?;
        self.decorations
            .changes
            .shortest_unique_prefix_len(id)
            .await
    }
}
