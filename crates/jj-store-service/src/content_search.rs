//! Bounded current application content, independently pinned from code search.
mod cursors;
mod extract;
mod index;
mod materialization;
mod models;
use crate::work::{Error, Result};
use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
};
pub use models::*;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct ContentPager {
    registry: cursors::Registry,
    permits: Semaphore,
    cache: materialization::Cache,
}
impl ContentPager {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            registry: cursors::Registry::new(clock.clone()),
            cache: materialization::Cache::new(clock),
            permits: Semaphore::new(2),
        }
    }
    pub async fn page(
        &self,
        session: Arc<NativeSession>,
        query: ContentQuery,
    ) -> Result<ContentPage> {
        if !(1..=256).contains(&query.limit)
            || query.q.len() > 512
            || (query.thread.is_some() && (query.project.is_some() || query.task.is_some()))
        {
            return Err(Error::Invalid);
        }
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        tokio::time::timeout(session.limits.timeout, self.page_inner(session, query))
            .await
            .map_err(|_| Error::Limit)?
    }
    async fn page_inner(
        &self,
        session: Arc<NativeSession>,
        query: ContentQuery,
    ) -> Result<ContentPage> {
        session.principal(Permission::Read).await?;
        let (pin, offset) = if query.cursor.is_some() {
            self.registry.lookup(session.repository, &query)?
        } else {
            (
                extract::capture(&session, &query, self.registry.deadline()?, &self.cache).await?,
                0,
            )
        };
        authorize(&session, &pin).await?;
        let filter = query.q.trim().to_lowercase();
        let results = pin.selection.page(&filter, offset, query.limit);
        let next = offset.checked_add(results.len()).ok_or(Error::Limit)?;
        let next_cursor = if next < pin.selection.keys.len() {
            Some(self.registry.retain(pin.clone(), query, next)?)
        } else {
            None
        };
        authorize(&session, &pin).await?;
        Ok(ContentPage {
            version: 1,
            snapshot: pin.snapshot.clone(),
            results,
            next_cursor,
        })
    }
}
async fn authorize(session: &NativeSession, pin: &cursors::Pin) -> Result<()> {
    if session.repository != pin.snapshot.repo_id
        || session.principal(Permission::Read).await?.id() != pin.viewer
    {
        return Err(Error::Denied);
    }
    if pin.selection.objects > session.limits.inventory_nodes {
        return Err(Error::Limit);
    }
    if session.snapshot().await?.manifest().epoch != pin.snapshot.epoch {
        return Err(Error::CursorReset);
    }
    if session.principal(Permission::Read).await?.id() != pin.viewer {
        return Err(Error::Denied);
    }
    Ok(())
}
