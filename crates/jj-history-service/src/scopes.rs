use crate::{Cancellation, Context, Error, History, Result, local};
use std::{
    path::Path,
    sync::{Arc, atomic::Ordering},
};
use tokio::sync::{OwnedRwLockReadGuard, RwLock};

pub(crate) struct Scope {
    pub caller: Context,
    pub cancellation: Cancellation,
    pub activity: Arc<RwLock<()>>,
}
impl Scope {
    pub async fn lease(&self) -> Result<OwnedRwLockReadGuard<()>> {
        let guard = self.activity.clone().read_owned().await;
        if self.cancellation.is_cancelled() {
            return Err(Error::Canceled);
        }
        Ok(guard)
    }
    pub async fn drain(&self) {
        drop(self.activity.write().await);
    }
}
impl History {
    pub(crate) fn scope(&self, caller: &Context) -> Result<Arc<Scope>> {
        let mut windows = self.windows.lock().map_err(|_| Error::Unavailable)?;
        if self.closing.load(Ordering::SeqCst) {
            return Err(Error::Denied);
        }
        if let Some(scope) = windows.get(&caller.window) {
            return match scope {
                Some(scope) if &scope.caller == caller => Ok(scope.clone()),
                _ => Err(Error::Denied),
            };
        }
        if windows.len() >= 16 {
            return Err(Error::Busy);
        }
        let scope = Arc::new(Scope {
            caller: caller.clone(),
            cancellation: Cancellation::default(),
            activity: Arc::new(RwLock::new(())),
        });
        windows.insert(caller.window.clone(), Some(scope.clone()));
        Ok(scope)
    }
}
struct Access {
    inner: Arc<dyn local::Access>,
    cancellations: Vec<Cancellation>,
}
pub(crate) fn scoped(
    inner: Arc<dyn local::Access>,
    cancellations: Vec<Cancellation>,
) -> Arc<dyn local::Access> {
    Arc::new(Access {
        inner,
        cancellations,
    })
}
impl Access {
    fn live(&self) -> Result<()> {
        if self.cancellations.iter().any(Cancellation::is_cancelled) {
            Err(Error::Canceled)
        } else {
            Ok(())
        }
    }
}
#[async_trait::async_trait]
impl local::Access for Access {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.inner.repository()
    }
    fn generation(&self) -> u64 {
        self.inner.generation()
    }
    fn metadata_path(&self) -> &Path {
        self.inner.metadata_path()
    }
    async fn authorize(&self) -> Result<()> {
        self.live()?;
        self.inner.authorize().await?;
        self.live()
    }
    async fn check(&self) -> Result<()> {
        self.live()?;
        self.inner.check().await?;
        self.live()
    }
}
