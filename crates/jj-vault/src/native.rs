use super::*;
/// Native JJ provider; each command reopens/revalidates the admitted binding.
pub struct NativeRepository {
    pub binding: Binding,
    pub exclusions: Vec<String>,
}
impl Repository for NativeRepository {
    fn recover_ai_history(&mut self) -> Result<(), String> {
        self.ai_recover()
    }
    fn ai_history(&mut self, request: AiHistoryRequest) -> Result<AiHistoryResponse, String> {
        self.ai_request(request)
    }
    fn adopt_history(&mut self, request: &AdoptionRequest) -> Result<Checkpoint, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .adopt_history(request, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn stop_adoption(&mut self, id: &str) -> Result<(), String> {
        runtime()?.block_on(async {
            let session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session.stop_adoption(id).map_err(|e| e.to_string())
        })
    }
    fn prepare_sync(&mut self, incoming: &str) -> Result<SyncProposal, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .prepare_sync(incoming, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn preview_sync(&mut self, candidate: &str) -> Result<VaultRestorePreview, String> {
        runtime()?.block_on(async {
            let session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .preview_sync(candidate, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn apply_sync(&mut self, request: &VaultRestoreRequest) -> Result<Capture, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let mut result = session
                .apply_sync(request, &self.exclusions)
                .await
                .map_err(|e| e.to_string())?;
            result.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            Ok(result)
        })
    }
    fn sync_conflict(&mut self, candidate: &str, path: &str) -> Result<SyncConflict, String> {
        runtime()?.block_on(async {
            let session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .sync_conflict(candidate, path)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn resolve_sync(
        &mut self,
        candidate: &str,
        path: &str,
        resolution: SyncResolution,
    ) -> Result<Checkpoint, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .resolve_sync(candidate, path, resolution, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn import_remote(&mut self, head: &FetchedHead) -> Result<Checkpoint, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .import_fetched(head)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn restore_remote(&mut self, request: &VaultRestoreRequest) -> Result<Capture, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let mut result = session
                .restore_imported(request, &self.exclusions)
                .await
                .map_err(|e| e.to_string())?;
            result.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            Ok(result)
        })
    }
    fn preview_vault_restore(&mut self, checkpoint: &str) -> Result<VaultRestorePreview, String> {
        runtime()?.block_on(async {
            let session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .preview_vault_restore(checkpoint, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn restore_vault(&mut self, request: &VaultRestoreRequest) -> Result<Capture, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let mut result = session
                .restore_vault(request, &self.exclusions)
                .await
                .map_err(|e| e.to_string())?;
            result.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            Ok(result)
        })
    }
    fn preview_restore(&mut self, checkpoint: &str, path: &str) -> Result<RestorePreview, String> {
        runtime()?.block_on(async {
            let session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .preview_file_restore(checkpoint, path)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn restore(&mut self, request: &RestoreRequest) -> Result<Capture, String> {
        runtime()?.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let mut result = session
                .restore_file(request, &self.exclusions)
                .await
                .map_err(|e| e.to_string())?;
            result.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            Ok(result)
        })
    }
    fn capture(&mut self) -> Result<Capture, String> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        runtime.block_on(async {
            let mut session = jj_store_jj::vault_workspace::Session::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            let mut value = session
                .capture_observed(&self.exclusions)
                .await
                .map_err(|e| e.to_string())?;
            if value.checkpoint.is_none() {
                value.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            }
            Ok(value)
        })
    }
}
pub(super) fn runtime() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())
}
