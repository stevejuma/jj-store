use crate::{Cancellation, Context, Error, Handle, History, OperationId, Result, cache};
use crate::{
    scopes::scoped,
    service::{Entry, Output},
};
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::RwLock;
use uuid::Uuid;

impl History {
    pub async fn open(
        self: &Arc<Self>,
        caller: &Context,
        vault: &str,
        operation: OperationId,
    ) -> Result<Handle> {
        let access = self.admit(caller, vault).await?;
        let scope = self.scope(caller)?;
        let lease = scope.lease().await?;
        let permit = self
            .preparation
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        let handle = Handle(Uuid::new_v4());
        let cancellation = Cancellation::default();
        let repository = access.repository();
        let generation = access.generation();
        let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
        if self.closing.load(Ordering::SeqCst)
            || scope.cancellation.is_cancelled()
            || entries.len() >= self.config.max_handles
        {
            return Err(Error::Busy);
        }
        let access = scoped(
            access,
            vec![scope.cancellation.clone(), cancellation.clone()],
        );
        let owner = self.clone();
        let cancel = cancellation.clone();
        let task = tokio::spawn(async move {
            let _permit = permit;
            let _lease = lease;
            let result = async {
                access.authorize().await?;
                let root = owner.root.clone();
                let policy = owner.config.cache;
                let namespace = tokio::task::spawn_blocking(move || {
                    cache::managed::Namespace::acquire(
                        &root,
                        cache::managed::Binding::Local {
                            repository,
                            generation,
                        },
                        policy,
                    )
                })
                .await
                .map_err(|_| Error::Unavailable)??;
                access.authorize().await?;
                owner
                    .executor
                    .prepare_managed_code_index(
                        access,
                        operation,
                        namespace,
                        owner.config.preparation,
                        cancel,
                    )
                    .await
            }
            .await;
            if let Ok(mut entries) = owner.entries.lock()
                && let Some(entry) = entries.get_mut(&handle)
            {
                entry.output = match result {
                    Ok(prepared) => Output::Ready(Arc::new(prepared)),
                    Err(error) => Output::Failed(error),
                };
            }
        });
        entries.insert(
            handle,
            Entry {
                caller: caller.clone(),
                vault: vault.into(),
                repository,
                generation,
                cancellation,
                task: Some(task),
                output: Output::Preparing,
                scope,
                readers: Arc::new(RwLock::new(())),
            },
        );
        Ok(handle)
    }
    /// Cleanup stays possible after read admission is withdrawn.
    pub async fn close(&self, caller: &Context, vault: &str, handle: Handle) -> Result<()> {
        let _cleanup = self.lifecycle.lock().await;
        let entry = {
            let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            let entry = entries.get(&handle).ok_or(Error::Denied)?;
            if &entry.caller != caller || entry.vault != vault {
                return Err(Error::Denied);
            }
            entries.remove(&handle).unwrap()
        };
        finish(entry).await
    }
    /// Native-only lifecycle entry. A renderer cannot name a different window.
    pub async fn revoke(&self, window: &str) -> Result<()> {
        let _cleanup = self.lifecycle.lock().await;
        let scope = {
            let mut windows = self.windows.lock().map_err(|_| Error::Unavailable)?;
            match windows.get_mut(window) {
                Some(scope @ Some(_)) => scope.take(),
                Some(None) => return Err(Error::Busy),
                None => return Ok(()),
            }
        };
        if let Some(scope) = &scope {
            scope.cancellation.cancel();
        }
        let removed = {
            let mut entries = self.entries.lock().map_err(|_| Error::Unavailable)?;
            let handles: Vec<_> = entries
                .iter()
                .filter(|(_, e)| e.caller.window == window)
                .map(|(h, _)| *h)
                .collect();
            handles
                .into_iter()
                .map(|h| entries.remove(&h).unwrap())
                .collect::<Vec<_>>()
        };
        let result = finish_all(removed).await;
        if let Some(scope) = scope {
            scope.drain().await;
        }
        self.windows
            .lock()
            .map_err(|_| Error::Unavailable)?
            .remove(window);
        result
    }
    pub async fn shutdown(&self) -> Result<()> {
        self.closing.store(true, Ordering::SeqCst);
        let _cleanup = self.lifecycle.lock().await;
        let scopes: Vec<_> = self
            .windows
            .lock()
            .map_err(|_| Error::Unavailable)?
            .values_mut()
            .filter_map(Option::take)
            .collect();
        for scope in &scopes {
            scope.cancellation.cancel();
        }
        let removed = self
            .entries
            .lock()
            .map_err(|_| Error::Unavailable)?
            .drain()
            .map(|(_, e)| e)
            .collect();
        let result = finish_all(removed).await;
        for scope in scopes {
            scope.drain().await;
        }
        if result.is_ok() {
            self.windows.lock().map_err(|_| Error::Unavailable)?.clear();
        }
        result
    }
}
async fn finish(mut entry: Entry) -> Result<()> {
    entry.cancellation.cancel();
    let result = if let Some(task) = entry.task.take() {
        task.await.map_err(|_| Error::Unavailable)
    } else {
        Ok(())
    };
    drop(entry.readers.write().await);
    result
}
async fn finish_all(entries: Vec<Entry>) -> Result<()> {
    for entry in &entries {
        entry.cancellation.cancel();
    }
    let mut failed = false;
    for entry in entries {
        failed |= finish(entry).await.is_err();
    }
    if failed {
        Err(Error::Unavailable)
    } else {
        Ok(())
    }
}
