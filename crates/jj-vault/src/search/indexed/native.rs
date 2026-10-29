use super::*;
use crate::search::NativeHistory;
use jj_store_jj::vault_workspace::HistoryReader;
use sha2::{Digest as _, Sha256};

struct Native {
    reader: HistoryReader,
    runtime: tokio::runtime::Runtime,
    exclusions: Vec<String>,
}
impl Native {
    fn open(history: &NativeHistory) -> Result<Self, String> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(error)?;
        let reader = runtime
            .block_on(HistoryReader::open(history.binding.clone()))
            .map_err(error)?;
        Ok(Self {
            runtime,
            reader,
            exclusions: history.exclusions.clone(),
        })
    }
}
impl Source for Native {
    fn head(&mut self) -> Result<Option<String>, String> {
        Ok(self
            .runtime
            .block_on(self.reader.history_revision(None, 0))
            .map_err(error)?
            .anchor)
    }
    fn revision(&mut self, id: &str) -> Result<IndexRevision, String> {
        self.runtime
            .block_on(self.reader.index_revision(id))
            .map_err(error)
    }
    fn files(&mut self, id: &str, after: Option<&str>) -> Result<IndexFiles, String> {
        self.runtime
            .block_on(self.reader.index_files(id, after, &self.exclusions))
            .map_err(error)
    }
    fn text(&mut self, id: &str, path: &str) -> Result<Option<String>, String> {
        self.runtime
            .block_on(self.reader.index_text(id, path, &self.exclusions))
            .map_err(error)
    }
}
impl NativeHistory {
    fn engine(&self, read: bool) -> Result<Engine, String> {
        let namespace =
            serde_json::to_string(&(&self.binding.vault_id, &self.exclusions)).map_err(error)?;
        let key = super::super::lower_hex(Sha256::digest(namespace.as_bytes()));
        let root = self.binding.state.join("history-search-v2").join(key);
        if read {
            Engine::read(&root, &namespace)
        } else {
            Engine::open(&root, &namespace)
        }
    }
    pub fn index_step(&self) -> Result<Status, String> {
        let mut source = Native::open(self)?;
        self.engine(false)?.step(&mut source)
    }
    pub fn indexed_search(&self, request: &Request) -> Result<Page, String> {
        let mut source = Native::open(self)?;
        self.engine(true)?.search(&mut source, request)
    }
    pub fn indexed_text(&self, commit: &str, path: &str) -> Result<Option<String>, String> {
        Native::open(self)?.text(commit, path)
    }
}
