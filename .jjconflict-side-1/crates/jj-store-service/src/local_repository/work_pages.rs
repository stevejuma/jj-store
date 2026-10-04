use super::{LocalRepository, LocalWorkSnapshot, map_metadata};
use crate::work::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jj_store_jj::local_metadata::GitOid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalWorkPage {
    pub version: u32,
    pub snapshot: LocalWorkSnapshot,
    pub projects: Vec<ProjectV1>,
    pub tasks: Vec<TaskV1>,
    pub comments: Vec<WorkCommentV1>,
    pub next_cursor: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    version: u32,
    commit: GitOid,
    project: Option<ProjectId>,
    task: Option<TaskId>,
    limit: usize,
    offset: usize,
}
impl LocalRepository {
    pub fn work_page(&self, query: &WorkQuery) -> Result<LocalWorkPage> {
        self.work_page_inner(None, query)
    }
    pub fn work_page_at(&self, commit: GitOid, query: &WorkQuery) -> Result<LocalWorkPage> {
        self.work_page_inner(Some(commit), query)
    }
    fn work_page_inner(&self, commit: Option<GitOid>, query: &WorkQuery) -> Result<LocalWorkPage> {
        if !(1..=256).contains(&query.limit) {
            return Err(Error::Invalid);
        }
        let current = match commit {
            Some(commit) => Some(self.selected_metadata(commit)?),
            None => self.metadata.read().map_err(map_metadata)?,
        };
        let (snapshot, offset) = if let Some(cursor) = &query.cursor {
            if cursor.len() > 2048 {
                return Err(Error::InvalidCursor);
            }
            let bytes = URL_SAFE_NO_PAD
                .decode(cursor)
                .map_err(|_| Error::InvalidCursor)?;
            let cursor: Cursor =
                serde_json::from_slice(&bytes).map_err(|_| Error::InvalidCursor)?;
            if commit.is_some_and(|selected| selected != cursor.commit)
                || cursor.version != 1
                || cursor.project != query.project
                || cursor.task != query.task
                || cursor.limit != query.limit
                || cursor.offset > 4096
                || serde_json::to_vec(&cursor).map_err(|_| Error::InvalidCursor)? != bytes
            {
                return Err(Error::InvalidCursor);
            }
            let Some(current) = current else {
                return Err(Error::CursorReset);
            };
            let snapshot = self
                .metadata
                .history(&current)
                .map_err(map_metadata)?
                .into_iter()
                .find(|s| s.commit() == cursor.commit)
                .ok_or(Error::InvalidCursor)?;
            (Some(snapshot), cursor.offset)
        } else {
            (current, 0)
        };
        let data = self.work_data(snapshot.as_ref())?;
        let state = &data.state;
        let mut page = LocalWorkPage {
            version: 2,
            snapshot: Self::work_snapshot(snapshot.as_ref()),
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
                data.projection
                    .task_comments
                    .get(&id)
                    .into_iter()
                    .flatten()
                    .filter_map(|id| state.comments.get(id)),
                offset,
                query.limit,
                &mut page.comments,
                overhead(&page.projects, &page.tasks)?,
            )?
        } else if let Some(id) = query.project {
            page.projects
                .push(state.projects.get(&id).ok_or(Error::NotFound)?.clone());
            take_page(
                data.projection
                    .project_tasks
                    .get(&id)
                    .into_iter()
                    .flatten()
                    .filter_map(|id| state.tasks.get(id)),
                offset,
                query.limit,
                &mut page.tasks,
                overhead(&page.projects, &[])?,
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
        if more {
            let cursor = Cursor {
                version: 1,
                commit: snapshot.as_ref().ok_or(Error::Corrupt)?.commit(),
                project: query.project,
                task: query.task,
                limit: query.limit,
                offset: offset.checked_add(count).ok_or(Error::Limit)?,
            };
            page.next_cursor = Some(
                URL_SAFE_NO_PAD.encode(serde_json::to_vec(&cursor).map_err(|_| Error::Corrupt)?),
            );
        }
        Ok(page)
    }
}
