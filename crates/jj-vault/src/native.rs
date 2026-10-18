use super::*;
/// Native JJ provider; each command reopens/revalidates the admitted binding.
pub struct NativeRepository {
    pub binding: Binding,
    pub exclusions: Vec<String>,
}
impl Repository for NativeRepository {
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
