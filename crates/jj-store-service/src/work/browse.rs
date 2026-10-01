use super::cursors::WorkCursorQuery;
use super::{
    reads::{authorize_pin, capture_pin, take_page},
    *,
};
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
            (capture_pin(&session).await?, 0)
        };
        authorize_pin(&session, &pin).await?;
        let state = &pin.state;
        if let Some(id) = query.project
            && !state.projects.contains_key(&id)
        {
            return Err(Error::NotFound);
        }
        if let Some(id) = query.parent {
            let parent = state.tasks.get(&id).ok_or(Error::NotFound)?;
            if query.project.is_some_and(|id| id != parent.project) {
                return Err(Error::Invalid);
            }
        }
        let text = query.q.trim().to_lowercase();
        let matches = |title: &str, body: &str| {
            text.is_empty()
                || title.to_lowercase().contains(&text)
                || body.to_lowercase().contains(&text)
        };
        let mut page = WorkBrowsePage {
            version: 1,
            snapshot: pin.snapshot.clone(),
            projects: vec![],
            tasks: vec![],
            total_matches: 0,
            next_cursor: None,
        };
        let (count, more) = match query.kind {
            WorkBrowseKind::Projects => {
                let mut rows: Vec<_> = state
                    .projects
                    .values()
                    .filter(|p| {
                        query.project.is_none_or(|id| p.id == id)
                            && query
                                .repository
                                .is_none_or(|id| p.repositories.contains_key(&id))
                            && matches(&p.title, &p.body)
                    })
                    .map(|p| {
                        let mut counts = WorkTaskCounts::default();
                        for t in state.tasks.values().filter(|t| t.project == p.id) {
                            counts.total += 1;
                            if t.state == TaskStateV1::Done {
                                counts.done += 1;
                            } else {
                                counts.active += 1;
                            }
                        }
                        WorkProjectSummary {
                            id: p.id,
                            title: p.title.clone(),
                            excerpt: p.body.chars().take(240).collect(),
                            version: p.version,
                            repositories: p.repositories.clone(),
                            task_counts: counts,
                            actor: WorkSummaryActor {
                                created_by: p.actor.created_by,
                                updated_by: p.actor.updated_by,
                                created_at_millis: p.actor.created_at_millis,
                                updated_at_millis: p.actor.updated_at_millis,
                            },
                        }
                    })
                    .collect();
                rows.sort_by(|a, b| {
                    a.title
                        .to_lowercase()
                        .cmp(&b.title.to_lowercase())
                        .then(a.id.cmp(&b.id))
                });
                page.total_matches = rows.len();
                take_page(rows.iter(), offset, query.limit, &mut page.projects, 1024)?
            }
            WorkBrowseKind::Tasks => {
                let mut rows = Vec::new();
                for t in state.tasks.values().filter(|t| {
                    query.project.is_none_or(|id| t.project == id)
                        && query.repository.is_none_or(|id| t.repository == Some(id))
                        && query.parent.is_none_or(|id| t.parent == Some(id))
                        && query.state.is_none_or(|f| f.matches(t.state))
                        && matches(&t.title, &t.body)
                }) {
                    let project = state.projects.get(&t.project).ok_or(Error::Corrupt)?;
                    rows.push(WorkTaskSummary {
                        id: t.id,
                        project: t.project,
                        project_title: project.title.clone(),
                        title: t.title.clone(),
                        state: t.state,
                        repository: t.repository,
                        parent: t.parent,
                        parent_title: t
                            .parent
                            .and_then(|id| state.tasks.get(&id).map(|p| p.title.clone())),
                        comment_count: state
                            .comments
                            .values()
                            .filter(|c| c.task == t.id && c.body.is_some())
                            .count(),
                        subtask_count: state
                            .tasks
                            .values()
                            .filter(|c| c.parent == Some(t.id))
                            .count(),
                        version: t.version,
                        actor: WorkSummaryActor {
                            created_by: t.actor.created_by,
                            updated_by: t.actor.updated_by,
                            created_at_millis: t.actor.created_at_millis,
                            updated_at_millis: t.actor.updated_at_millis,
                        },
                    });
                }
                rows.sort_by(|a, b| {
                    b.actor
                        .updated_at_millis
                        .cmp(&a.actor.updated_at_millis)
                        .then(a.id.cmp(&b.id))
                });
                page.total_matches = rows.len();
                take_page(rows.iter(), offset, query.limit, &mut page.tasks, 1024)?
            }
        };
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
