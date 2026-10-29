use super::*;

impl Backup {
    /// Drain final capture, retaining this worker and its failed status on error.
    /// Hosts must retain the owner and retry before releasing vault authority.
    /// Successful retries are idempotent, including after a lost close response.
    pub async fn try_close(&mut self) -> Result<(), String> {
        self.finish(true).await
    }

    /// Consume the owner and retire its worker even if final capture fails.
    /// Hosts that need a retryable save guard must use `try_close` instead.
    pub async fn close(mut self) -> Result<(), String> {
        self.finish(false).await
    }

    async fn finish(&mut self, retryable: bool) -> Result<(), String> {
        let result = if self.state.borrow().phase == Phase::Closed {
            Ok(())
        } else {
            self.request_close(retryable).await
        };
        // A prior cancelled await may already have queued the successful close.
        let result = if self.state.borrow().phase == Phase::Closed {
            Ok(())
        } else {
            result
        };
        if (result.is_ok() || !retryable)
            && let Some(worker) = self.worker.take()
        {
            tokio::task::spawn_blocking(move || worker.join())
                .await
                .map_err(|e| e.to_string())?
                .map_err(|_| "Vault backup worker failed".to_string())?;
        }
        result
    }

    async fn request_close(&self, retryable: bool) -> Result<(), String> {
        let (reply, receiver) = oneshot::channel();
        // Queue saturation must never discard the final-capture barrier.
        let commands = self.commands.clone();
        tokio::task::spawn_blocking(move || {
            commands
                .send(Command::Close { reply, retryable })
                .map_err(|_| "Vault backup worker is closed".to_owned())
        })
        .await
        .map_err(|e| e.to_string())??;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
}

impl Drop for Backup {
    fn drop(&mut self) {
        // Notification clones do not own worker lifetime. Never block a UI
        // thread on abandonment; wake idle receive, or let queued work wake it.
        self.abandoned.store(true, Ordering::Release);
        let _ = self.commands.try_send(Command::Wake);
    }
}
