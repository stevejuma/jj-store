use super::cursors::WorkCursorQuery;
use super::{reads::authorize_pin, *};
use crate::native::NativeSession;
use std::sync::Arc;

impl WorkPager {
    pub async fn browse(
        &self,
        session: Arc<NativeSession>,
        query: WorkBrowseQuery,
    ) -> Result<WorkBrowsePage> {
        if !(1..=256).contains(&query.limit)
            || query.q.len() > 512
            || (query.kind == WorkBrowseKind::Projects
                && (query.parent.is_some() || query.state.is_some()))
        {
            return Err(Error::Invalid);
        }
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        tokio::time::timeout(session.limits.timeout, self.browse_inner(session, query))
            .await
            .map_err(|_| Error::Limit)?
    }
    async fn browse_inner(
        &self,
        session: Arc<NativeSession>,
        query: WorkBrowseQuery,
    ) -> Result<WorkBrowsePage> {
        session.principal(crate::auth::Permission::Read).await?;
        let (pin, offset) = if query.cursor.is_some() {
            self.registry
                .lookup(session.repository, &WorkCursorQuery::Browse(query.clone()))?
        } else {
            let mut pin = self.capture_pin(&session).await?;
            pin.selection = Some(pin.data.projection.select(&pin.data.state, &query)?);
            (Arc::new(pin), 0)
        };
        authorize_pin(&session, &pin).await?;
        let rows = browse_selected(
            &pin.data.state,
            &pin.data.projection,
            pin.selection.as_ref().ok_or(Error::Corrupt)?,
            query.limit,
            offset,
        )?;
        let mut page = WorkBrowsePage {
            version: 1,
            snapshot: pin.snapshot.clone(),
            projects: rows.projects,
            tasks: rows.tasks,
            total_matches: rows.total_matches,
            next_cursor: None,
        };
        let (count, more) = (rows.count, rows.more);
        authorize_pin(&session, &pin).await?;
        if more {
            page.next_cursor = Some(self.registry.retain(
                pin,
                WorkCursorQuery::Browse(query),
                offset.checked_add(count).ok_or(Error::Limit)?,
            )?);
        }
        Ok(page)
    }
}

/// Provider-independent, complete-root filter and bounded page projection.
pub(crate) struct BrowseRows {
    pub projects: Vec<WorkProjectSummary>,
    pub tasks: Vec<WorkTaskSummary>,
    pub total_matches: usize,
    pub count: usize,
    pub more: bool,
    #[cfg(test)]
    pub hydrated: usize,
}
pub(crate) fn browse_rows(
    state: &jj_store_wal::publication::WorkStateV1,
    query: &WorkBrowseQuery,
    offset: usize,
) -> Result<BrowseRows> {
    let projection = super::projection::Projection::new(state)?;
    let selection = projection.select(state, query)?;
    browse_selected(state, &projection, &selection, query.limit, offset)
}
pub(super) fn browse_selected(
    state: &jj_store_wal::publication::WorkStateV1,
    projection: &super::projection::Projection,
    selection: &super::projection::Selection,
    limit: usize,
    offset: usize,
) -> Result<BrowseRows> {
    use super::projection::Selection;
    let mut page = BrowseRows {
        projects: vec![],
        tasks: vec![],
        total_matches: selection.len(),
        count: 0,
        more: false,
        #[cfg(test)]
        hydrated: 0,
    };
    let mut bytes = 1024;
    match selection {
        Selection::Projects(ids) => {
            for id in ids.iter().skip(offset).take(limit) {
                let row = projection.project(state.projects.get(id).ok_or(Error::Corrupt)?);
                #[cfg(test)]
                {
                    page.hydrated += 1;
                }
                if !push_row(row, &mut page.projects, &mut bytes)? {
                    break;
                }
            }
            page.count = page.projects.len();
        }
        Selection::Tasks(ids) => {
            for id in ids.iter().skip(offset).take(limit) {
                let row = projection.task(state, state.tasks.get(id).ok_or(Error::Corrupt)?)?;
                #[cfg(test)]
                {
                    page.hydrated += 1;
                }
                if !push_row(row, &mut page.tasks, &mut bytes)? {
                    break;
                }
            }
            page.count = page.tasks.len();
        }
    }
    page.more = offset.checked_add(page.count).ok_or(Error::Limit)? < page.total_matches;
    Ok(page)
}
fn push_row<T: serde::Serialize>(row: T, rows: &mut Vec<T>, bytes: &mut usize) -> Result<bool> {
    let size = serde_json::to_vec(&row).map_err(|_| Error::Corrupt)?.len() + 1;
    let next = bytes.checked_add(size).ok_or(Error::Limit)?;
    if next > 4 * 1024 * 1024 {
        return Ok(false);
    }
    rows.push(row);
    *bytes = next;
    Ok(true)
}

#[cfg(test)]
#[path = "browse_tests.rs"]
mod tests;
