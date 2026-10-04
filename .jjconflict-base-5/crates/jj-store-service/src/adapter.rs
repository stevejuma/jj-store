//! Repository-token binding for read-only committed projection sources.
use crate::{auth::Permission, native::NativeSession};
use jj_store_adapters::{Authorizer, Error, Source};
use std::sync::Arc;
mod read;
pub use jj_store_jj::{native_dto, roots::ROOT_COMMIT};
pub use read::ProjectionRead;
struct Access(Arc<NativeSession>);
#[async_trait::async_trait]
impl Authorizer for Access {
    fn repository(&self) -> jj_store_model::RepositoryId {
        self.0.repository
    }
    async fn authorize(&self) -> Result<jj_store_model::EpochId, Error> {
        self.0
            .principal(Permission::Read)
            .await
            .map_err(Error::from)?;
        Ok(self
            .0
            .snapshot()
            .await
            .map_err(Error::from)?
            .manifest()
            .epoch)
    }
}
impl NativeSession {
    pub fn adapter_source(self: &Arc<Self>) -> Source {
        Source::new(self.object_store.clone(), Arc::new(Access(self.clone())))
    }
}
