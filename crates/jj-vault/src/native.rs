use super::*;
/// Native JJ provider; each command reopens/revalidates the admitted binding.
pub struct NativeRepository {
    pub binding: Binding,
    pub exclusions: Vec<String>,
}
impl Repository for NativeRepository {
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
            if result.checkpoint.is_none() {
                result.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            }
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
                .capture(&self.exclusions, "Automatic vault backup")
                .await
                .map_err(|e| e.to_string())?;
            if value.checkpoint.is_none() {
                value.checkpoint = session.checkpoint().map_err(|e| e.to_string())?;
            }
            Ok(value)
        })
    }
}
fn runtime() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())
}
