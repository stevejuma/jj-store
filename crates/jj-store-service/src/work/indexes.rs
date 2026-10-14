//! Secondary task/project candidates; full source validation stays with WorkStore.
use super::*;
use crate::native::NativeSession;
use jj_store_read_index::{Index, Row};
use jj_store_wal::{publication::WorkStateV1, records::RecordRef};

impl projection::Projection {
    pub(crate) fn discard_indexes(&mut self) {
        self.bytes = self.bytes.saturating_sub(
            self.project_index.as_ref().map_or(0, Index::bytes)
                + self.task_index.as_ref().map_or(0, Index::bytes),
        );
        self.project_index = None;
        self.task_index = None;
    }
    pub(crate) async fn accelerate(
        &mut self,
        state: &WorkStateV1,
        session: &NativeSession,
        root: RecordRef,
    ) {
        let projects = state
            .projects
            .values()
            .map(|p| {
                let mut row = Row::new(
                    p.id.to_string(),
                    p.version.to_string(),
                    p.title.to_lowercase().into_bytes(),
                )
                .facet("project", p.id)
                .text(&p.title)
                .text(&p.body);
                for repository in p.repositories.keys() {
                    row = row.facet("repository", repository);
                }
                row
            })
            .collect();
        let tasks = state
            .tasks
            .values()
            .map(|t| {
                let mut row = Row::new(
                    t.id.to_string(),
                    t.version.to_string(),
                    crate::workflow::projection::newest(t.actor.updated_at_millis),
                )
                .facet("project", t.project)
                .facet("state", format!("{:?}", t.state))
                .text(&t.title)
                .text(&t.body);
                if let Some(repository) = t.repository {
                    row = row.facet("repository", repository);
                }
                if let Some(parent) = t.parent {
                    row = row.facet("parent", parent);
                }
                if t.state != TaskStateV1::Done {
                    row = row.facet("active", "true");
                }
                row
            })
            .collect();
        let Ok(snapshot) = session.snapshot().await else {
            return;
        };
        let epoch = snapshot.manifest().epoch;
        self.project_index = session
            .read_indexes
            .prepare(
                session.repository.to_string(),
                epoch.to_string(),
                root.digest.to_string(),
                "work-projects-v1",
                projects,
            )
            .await
            .ok();
        self.task_index = session
            .read_indexes
            .prepare(
                session.repository.to_string(),
                epoch.to_string(),
                root.digest.to_string(),
                "work-tasks-v1",
                tasks,
            )
            .await
            .ok();
        self.bytes += self.project_index.as_ref().map_or(0, Index::bytes)
            + self.task_index.as_ref().map_or(0, Index::bytes);
    }
    pub(super) fn indexed(&self, query: &WorkBrowseQuery) -> Option<Result<projection::Selection>> {
        let mut filters = vec![];
        if let Some(project) = query.project {
            filters.push(("project", project.to_string()));
        }
        if let Some(repository) = query.repository {
            filters.push(("repository", repository.to_string()));
        }
        if let Some(parent) = query.parent {
            filters.push(("parent", parent.to_string()));
        }
        if let Some(state) = query.state {
            filters.push(match state {
                WorkStateFilter::Active => ("active", "true".into()),
                _ => ("state", format!("{state:?}")),
            });
        }
        Some(match query.kind {
            WorkBrowseKind::Projects => self
                .project_index
                .as_ref()?
                .select(&filters, &query.q)
                .ids
                .into_iter()
                .map(|id| id.parse().map_err(|_| Error::Corrupt))
                .collect::<Result<Vec<_>>>()
                .map(projection::Selection::Projects),
            WorkBrowseKind::Tasks => self
                .task_index
                .as_ref()?
                .select(&filters, &query.q)
                .ids
                .into_iter()
                .map(|id| id.parse().map_err(|_| Error::Corrupt))
                .collect::<Result<Vec<_>>>()
                .map(projection::Selection::Tasks),
        })
    }
}
