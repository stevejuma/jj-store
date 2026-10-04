use super::*;
use std::time::Duration;

impl Manager {
    /// Join the same FIFO as retained import work without waiting indefinitely.
    /// Cancellation removes only this waiter. After acquisition the caller moves
    /// the permit into its owned blocking observation, preserving policy checks.
    pub(super) async fn observation_permit(
        &self,
    ) -> Result<tokio::sync::OwnedMutexGuard<()>, Error> {
        tokio::time::timeout(Duration::from_secs(1), self.0.gate.clone().lock_owned())
            .await
            .map_err(|_| Error::Busy)
    }
}
