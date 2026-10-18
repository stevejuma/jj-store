//! Upgrade private derived data at the same operation; never replay a mutation.
use crate::service::{Output, Selection};
use crate::{Error, Handle, History, Result, cache, local};
use std::sync::Arc;

impl History {
    pub(crate) async fn prepare_file_history(
        &self,
        selection: &Selection,
        handle: Handle,
        previous: Arc<local::code_index::Prepared>,
    ) -> Result<Arc<local::code_index::Prepared>> {
        let _permit = self
            .preparation
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        let cancellation = self
            .entries
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(&handle)
            .ok_or(Error::Denied)?
            .cancellation
            .clone();
        selection.access.authorize().await?;
        let root = self.root.clone();
        let policy = self.config.cache;
        let binding = cache::managed::Binding::Local {
            repository: selection.access.repository(),
            generation: selection.access.generation(),
        };
        let namespace = tokio::task::spawn_blocking(move || {
            cache::managed::Namespace::acquire(&root, binding, policy)
        })
        .await
        .map_err(|_| Error::Unavailable)??;
        let indexed = Arc::new(
            self.executor
                .prepare_managed_path_index(
                    selection.access.clone(),
                    namespace,
                    self.config.preparation,
                    cancellation,
                    previous.clone(),
                )
                .await?,
        );
        selection.access.authorize().await?;
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        let entry = entries.get_mut(&handle).ok_or(Error::Denied)?;
        if !matches!(&entry.output, Output::Ready(current) if Arc::ptr_eq(current, &previous)) {
            return Err(Error::Busy);
        }
        entry.output = Output::Ready(indexed.clone());
        Ok(indexed)
    }
}
