use super::*;
impl Backup {
    pub async fn preview_vault_restore(
        &self,
        checkpoint: String,
    ) -> Result<VaultRestorePreview, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::PreviewVault(checkpoint, sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
    pub async fn restore_vault(&self, request: VaultRestoreRequest) -> Result<Capture, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::RestoreVault(request, sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
    pub async fn preview_restore(
        &self,
        checkpoint: String,
        path: String,
    ) -> Result<RestorePreview, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Preview(checkpoint, path, sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
    pub async fn restore(&self, request: RestoreRequest) -> Result<Capture, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Restore(request, sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
}
