use super::{
    cursors::{Pin, Registry, WorkCursorQuery},
    *,
};
use crate::{
    auth::{Clock, Permission},
    native::NativeSession,
};
use jj_store_wal::publication::WorkStateV1;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct WorkPager {
    pub(super) registry: Registry<WorkCursorQuery>,
    pub(super) permits: Semaphore,
}
impl WorkPager {
    pub fn new(clock: Arc<dyn Clock>) -> Self {
        Self {
            registry: Registry::new(clock),
            permits: Semaphore::new(8),
        }
    }
    pub async fn page(&self, session: Arc<NativeSession>, query: WorkQuery) -> Result<WorkPage> {
        if !(1..=256).contains(&query.limit) {
            return Err(Error::Invalid);
        }
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        tokio::time::timeout(session.limits.timeout, self.page_inner(session, query))
            .await
            .map_err(|_| Error::Limit)?
    }
    async fn page_inner(&self, session: Arc<NativeSession>, query: WorkQuery) -> Result<WorkPage> {
        session.principal(Permission::Read).await?;
        let (pin, offset) = if query.cursor.is_some() {
            self.registry
                .lookup(session.repository, &WorkCursorQuery::Page(query.clone()))?
        } else {
            (capture_pin(&session).await?, 0)
        };
        authorize_pin(&session, &pin).await?;
        let state = &pin.state;
        let mut page = WorkPage {
            version: 1,
            snapshot: pin.snapshot.clone(),
            projects: Vec::new(),
            tasks: Vec::new(),
            comments: Vec::new(),
            next_cursor: None,
        };
        let (count, more) = if let Some(id) = query.task {
            let task = state.tasks.get(&id).ok_or(Error::NotFound)?;
            if query.project.is_some_and(|project| project != task.project) {
                return Err(Error::Invalid);
            }
            page.projects.push(
                state
                    .projects
                    .get(&task.project)
                    .ok_or(Error::Corrupt)?
                    .clone(),
            );
            page.tasks.push(task.clone());
            take_page(
                state.comments.values().filter(|c| c.task == id),
                offset,
                query.limit,
                &mut page.comments,
                overhead(&page.projects, &page.tasks)?,
            )?
        } else if let Some(id) = query.project {
            page.projects
                .push(state.projects.get(&id).ok_or(Error::NotFound)?.clone());
            take_page(
                state.tasks.values().filter(|t| t.project == id),
                offset,
                query.limit,
                &mut page.tasks,
                overhead(&page.projects, &Vec::<TaskV1>::new())?,
            )?
        } else {
            take_page(
                state.projects.values(),
                offset,
                query.limit,
                &mut page.projects,
                1024,
            )?
        };
        authorize_pin(&session, &pin).await?;
        if more {
            page.next_cursor = Some(self.registry.retain(
                pin,
                WorkCursorQuery::Page(query),
                offset.checked_add(count).ok_or(Error::Limit)?,
            )?);
        }
        Ok(page)
    }
}
pub(super) async fn authorize_pin(session: &NativeSession, pin: &Pin) -> Result<()> {
    session.principal(Permission::Read).await?;
    if session.snapshot().await?.manifest().epoch != pin.snapshot.epoch {
        return Err(Error::CursorReset);
    }
    session.principal(Permission::Read).await?;
    Ok(())
}
fn overhead(projects: &[ProjectV1], tasks: &[TaskV1]) -> Result<usize> {
    Ok(serde_json::to_vec(&(projects, tasks))
        .map_err(|_| Error::Corrupt)?
        .len()
        + 1024)
}
pub(super) fn take_page<'a, T: Serialize + Clone + 'a>(
    values: impl Iterator<Item = &'a T>,
    offset: usize,
    limit: usize,
    rows: &mut Vec<T>,
    overhead: usize,
) -> Result<(usize, bool)> {
    let mut iterator = values.skip(offset);
    let mut bytes = overhead;
    let mut count = 0usize;
    for row in iterator.by_ref() {
        let size = serde_json::to_vec(row).map_err(|_| Error::Corrupt)?.len() + 1;
        if count == limit || bytes + size > 4 * 1024 * 1024 {
            return Ok((count, true));
        }
        rows.push(row.clone());
        bytes += size;
        count += 1;
    }
    Ok((count, false))
}

pub(super) async fn capture_pin(session: &NativeSession) -> Result<Arc<Pin>> {
    session.principal(Permission::Read).await?;
    let current = session.snapshot().await?;
    let manifest = current.manifest();
    let state = match manifest.work_root {
        Some(root) => session.coordinator.work_store(root).await?.state().clone(),
        None => WorkStateV1::default(),
    };
    Ok(Arc::new(Pin {
        snapshot: WorkReadSnapshot {
            repo_id: session.repository,
            epoch: manifest.epoch,
            published_seq: manifest.published,
            manifest_revision: manifest.revision,
            work_root: manifest.work_root.map(|r| r.digest),
            work_state_version: manifest.work_state_version,
        },
        state,
    }))
}
