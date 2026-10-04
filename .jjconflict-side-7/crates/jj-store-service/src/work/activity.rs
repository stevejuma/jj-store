//! Body-free directory aggregates from one complete immutable WorkStore root.
use super::{cursors::WorkCursorQuery, reads::authorize_pin, *};
use crate::{auth::Permission, native::NativeSession};
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

fn default_limit() -> usize {
    50
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkActivityQuery {
    #[serde(default = "default_limit")]
    #[schemars(range(min = 1, max = 256))]
    pub limit: usize,
    pub cursor: Option<String>,
}
impl Default for WorkActivityQuery {
    fn default() -> Self {
        Self {
            limit: default_limit(),
            cursor: None,
        }
    }
}
impl super::cursors::CursorQuery for WorkActivityQuery {
    fn cursor(&self) -> Option<&String> {
        self.cursor.as_ref()
    }
    fn clear_cursor(&mut self) {
        self.cursor = None;
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct WorkActivityGroup {
    pub repository: Option<RepositoryId>,
    pub project: ProjectId,
    pub tasks: usize,
    pub updated_at_millis: i64,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct WorkActivityPage {
    pub version: u32,
    pub snapshot: WorkReadSnapshot,
    #[schemars(length(max = 256))]
    pub groups: Vec<WorkActivityGroup>,
    pub total_groups: usize,
    pub next_cursor: Option<String>,
}

pub(crate) fn activity_rows(
    projection: &super::projection::Projection,
    query: &WorkActivityQuery,
    offset: usize,
) -> Result<(Vec<WorkActivityGroup>, bool)> {
    if !(1..=256).contains(&query.limit) {
        return Err(Error::Invalid);
    }
    if offset > projection.activity.len() {
        return Err(Error::InvalidCursor);
    }
    let end = offset
        .saturating_add(query.limit)
        .min(projection.activity.len());
    Ok((
        projection.activity[offset..end].to_vec(),
        end < projection.activity.len(),
    ))
}
impl WorkPager {
    pub async fn activity(
        &self,
        session: Arc<NativeSession>,
        query: WorkActivityQuery,
    ) -> Result<WorkActivityPage> {
        if !(1..=256).contains(&query.limit) {
            return Err(Error::Invalid);
        }
        let _permit = self.permits.try_acquire().map_err(|_| Error::Busy)?;
        tokio::time::timeout(session.limits.timeout, self.activity_inner(session, query))
            .await
            .map_err(|_| Error::Limit)?
    }
    async fn activity_inner(
        &self,
        session: Arc<NativeSession>,
        query: WorkActivityQuery,
    ) -> Result<WorkActivityPage> {
        session.principal(Permission::Read).await?;
        let (pin, offset) = if query.cursor.is_some() {
            self.registry.lookup(
                session.repository,
                &WorkCursorQuery::Activity(query.clone()),
            )?
        } else {
            (Arc::new(self.capture_pin(&session).await?), 0)
        };
        authorize_pin(&session, &pin).await?;
        let (groups, more) = activity_rows(&pin.data.projection, &query, offset)?;
        let mut page = WorkActivityPage {
            version: 1,
            snapshot: pin.snapshot.clone(),
            groups,
            total_groups: pin.data.projection.activity.len(),
            next_cursor: None,
        };
        authorize_pin(&session, &pin).await?;
        if more {
            page.next_cursor = Some(self.registry.retain(
                pin,
                WorkCursorQuery::Activity(query),
                offset.checked_add(page.groups.len()).ok_or(Error::Limit)?,
            )?);
        }
        Ok(page)
    }
}
