use super::*;
use crate::Binding;
use jj_store_jj::vault_workspace::HistoryReader;
/// Revalidates native retained ancestry on every page. Call on a bounded worker,
/// outside NativeHost/file-writer locks; these reads never initiate capture.
pub struct NativeHistory {
    pub binding: Binding,
    pub exclusions: Vec<String>,
}
fn runtime() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())
}
impl NativeHistory {
    pub fn search(&mut self, request: &Request) -> Result<Page, String> {
        let namespace = serde_json::to_string(&(&self.binding.vault_id, &self.exclusions))
            .map_err(|e| e.to_string())?;
        Index::new(self.binding.state.join("history-search-v1"), namespace).search(self, request)
    }
}
impl Source for NativeHistory {
    fn revision(&mut self, anchor: Option<&str>, offset: u32) -> Result<HistoryPage, String> {
        runtime()?.block_on(async {
            let session = HistoryReader::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .history_revision(anchor, offset)
                .await
                .map_err(|e| e.to_string())
        })
    }
    fn files(&mut self, commit: &str, after: Option<&str>) -> Result<HistoryFiles, String> {
        runtime()?.block_on(async {
            let session = HistoryReader::open(self.binding.clone())
                .await
                .map_err(|e| e.to_string())?;
            session
                .history_files(commit, after, &self.exclusions)
                .await
                .map_err(|e| e.to_string())
        })
    }
}
