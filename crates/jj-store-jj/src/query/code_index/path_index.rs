//! Optional incremental changed-path coverage through JJ's own bounded builder.
use super::*;
impl Index {
    pub async fn extend_paths(
        mut self,
        previous: &Index,
        path: &Path,
        store: &Arc<Store>,
        mut check: impl FnMut() -> Result<()>,
    ) -> Result<Self> {
        // Keep cold optional preparation separate from interactive readers.
        let _scratch = memory::Lease::reserve(128 * 1024 * 1024)?;
        check()?;
        let raw = tokio::task::unconstrained(async {
            DefaultIndexStore::load(path)
                .build_changed_path_index_at_operation(
                    &self.operation,
                    store,
                    path_history::BATCH as u32,
                    |_| {},
                )
                .await
                .map_err(|_| Error::Unavailable)
        })
        .await?;
        check()?;
        let weight = memory::framing::weight(
            path,
            store.commit_id_length(),
            store.change_id_length(),
            raw.num_commits() as usize,
            &mut check,
        )?;
        let memory = Arc::new(memory::Lease::reserve(weight)?);
        self.raw = raw;
        self._memory = memory;
        self.history = previous.history.clone();
        Ok(self)
    }
}
