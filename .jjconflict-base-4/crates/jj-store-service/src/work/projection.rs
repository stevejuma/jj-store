//! Immutable compact order, search fields and aggregate counts for one audited root.
use super::*;
use jj_store_wal::publication::WorkStateV1;
use std::collections::BTreeMap;

pub(crate) struct Projection {
    pub projects: Vec<ProjectId>,
    pub tasks: Vec<TaskId>,
    pub project_tasks: BTreeMap<ProjectId, Vec<TaskId>>,
    pub task_comments: BTreeMap<TaskId, Vec<WorkCommentId>>,
    pub counts: BTreeMap<ProjectId, WorkTaskCounts>,
    pub comments: BTreeMap<TaskId, usize>,
    pub subtasks: BTreeMap<TaskId, usize>,
    pub activity: Vec<WorkActivityGroup>,
    project_text: BTreeMap<ProjectId, (String, String)>,
    task_text: BTreeMap<TaskId, (String, String)>,
    pub bytes: usize,
}

pub(crate) enum Selection {
    Projects(Vec<ProjectId>),
    Tasks(Vec<TaskId>),
}
impl Selection {
    pub fn len(&self) -> usize {
        match self {
            Self::Projects(ids) => ids.len(),
            Self::Tasks(ids) => ids.len(),
        }
    }
    pub fn bytes(&self) -> usize {
        self.len() * 16 + 64
    }
}
impl Projection {
    pub fn new(state: &WorkStateV1) -> Result<Self> {
        let mut result = Self {
            projects: state.projects.keys().copied().collect(),
            tasks: state.tasks.keys().copied().collect(),
            project_tasks: BTreeMap::new(),
            task_comments: BTreeMap::new(),
            counts: BTreeMap::new(),
            comments: BTreeMap::new(),
            subtasks: BTreeMap::new(),
            activity: Vec::new(),
            project_text: BTreeMap::new(),
            task_text: BTreeMap::new(),
            bytes: 0,
        };
        for p in state.projects.values() {
            result
                .project_text
                .insert(p.id, (p.title.to_lowercase(), p.body.to_lowercase()));
        }
        let mut activity = BTreeMap::<_, WorkActivityGroup>::new();
        for t in state.tasks.values() {
            if !state.projects.contains_key(&t.project) {
                return Err(Error::Corrupt);
            }
            let counts = result.counts.entry(t.project).or_default();
            let group = activity
                .entry((t.repository, t.project))
                .or_insert(WorkActivityGroup {
                    repository: t.repository,
                    project: t.project,
                    tasks: 0,
                    updated_at_millis: t.actor.updated_at_millis,
                });
            group.tasks += 1;
            group.updated_at_millis = group.updated_at_millis.max(t.actor.updated_at_millis);
            counts.total += 1;
            if t.state == TaskStateV1::Done {
                counts.done += 1;
            } else {
                counts.active += 1;
            }
            result
                .project_tasks
                .entry(t.project)
                .or_default()
                .push(t.id);
            if let Some(parent) = t.parent {
                *result.subtasks.entry(parent).or_default() += 1;
            }
            result
                .task_text
                .insert(t.id, (t.title.to_lowercase(), t.body.to_lowercase()));
        }
        for c in state.comments.values() {
            result.task_comments.entry(c.task).or_default().push(c.id);
            if c.body.is_some() {
                *result.comments.entry(c.task).or_default() += 1;
            }
        }
        result.projects.sort_by(|a, b| {
            result.project_text[a]
                .0
                .cmp(&result.project_text[b].0)
                .then(a.cmp(b))
        });
        result.tasks.sort_by(|a, b| {
            state.tasks[b]
                .actor
                .updated_at_millis
                .cmp(&state.tasks[a].actor.updated_at_millis)
                .then(a.cmp(b))
        });
        result.activity = activity.into_values().collect();
        result.bytes = result
            .project_text
            .values()
            .chain(result.task_text.values())
            .map(|(title, body)| title.len() + body.len())
            .sum::<usize>()
            + (state.projects.len() + state.tasks.len() + state.comments.len()) * 512
            + result.activity.len() * 128;
        Ok(result)
    }
    pub fn select(&self, state: &WorkStateV1, query: &WorkBrowseQuery) -> Result<Selection> {
        validate(state, query)?;
        let text = query.q.trim().to_lowercase();
        let matches = |fields: &(String, String)| {
            text.is_empty() || fields.0.contains(&text) || fields.1.contains(&text)
        };
        Ok(match query.kind {
            WorkBrowseKind::Projects => Selection::Projects(
                self.projects
                    .iter()
                    .copied()
                    .filter(|id| {
                        let p = &state.projects[id];
                        query.project.is_none_or(|id| p.id == id)
                            && query
                                .repository
                                .is_none_or(|id| p.repositories.contains_key(&id))
                            && matches(&self.project_text[id])
                    })
                    .collect(),
            ),
            WorkBrowseKind::Tasks => Selection::Tasks(
                self.tasks
                    .iter()
                    .copied()
                    .filter(|id| {
                        let t = &state.tasks[id];
                        query.project.is_none_or(|id| t.project == id)
                            && query.repository.is_none_or(|id| t.repository == Some(id))
                            && query.parent.is_none_or(|id| t.parent == Some(id))
                            && query.state.is_none_or(|f| f.matches(t.state))
                            && matches(&self.task_text[id])
                    })
                    .collect(),
            ),
        })
    }
    pub fn project(&self, p: &ProjectV1) -> WorkProjectSummary {
        WorkProjectSummary {
            id: p.id,
            title: p.title.clone(),
            excerpt: p.body.chars().take(240).collect(),
            version: p.version,
            repositories: p.repositories.clone(),
            task_counts: self.counts.get(&p.id).cloned().unwrap_or_default(),
            actor: WorkSummaryActor {
                created_by: p.actor.created_by,
                updated_by: p.actor.updated_by,
                created_at_millis: p.actor.created_at_millis,
                updated_at_millis: p.actor.updated_at_millis,
            },
        }
    }
    pub fn task(&self, state: &WorkStateV1, t: &TaskV1) -> Result<WorkTaskSummary> {
        Ok(WorkTaskSummary {
            id: t.id,
            project: t.project,
            project_title: state
                .projects
                .get(&t.project)
                .ok_or(Error::Corrupt)?
                .title
                .clone(),
            title: t.title.clone(),
            state: t.state,
            repository: t.repository,
            parent: t.parent,
            parent_title: t
                .parent
                .and_then(|id| state.tasks.get(&id).map(|p| p.title.clone())),
            comment_count: self.comments.get(&t.id).copied().unwrap_or(0),
            subtask_count: self.subtasks.get(&t.id).copied().unwrap_or(0),
            version: t.version,
            actor: WorkSummaryActor {
                created_by: t.actor.created_by,
                updated_by: t.actor.updated_by,
                created_at_millis: t.actor.created_at_millis,
                updated_at_millis: t.actor.updated_at_millis,
            },
        })
    }
}
pub(crate) fn validate_shape(query: &WorkBrowseQuery) -> Result<()> {
    if !(1..=256).contains(&query.limit)
        || query.q.len() > 512
        || (query.kind == WorkBrowseKind::Projects
            && (query.parent.is_some() || query.state.is_some()))
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
fn validate(state: &WorkStateV1, query: &WorkBrowseQuery) -> Result<()> {
    validate_shape(query)?;
    if query
        .project
        .is_some_and(|id| !state.projects.contains_key(&id))
    {
        return Err(Error::NotFound);
    }
    if let Some(id) = query.parent {
        let parent = state.tasks.get(&id).ok_or(Error::NotFound)?;
        if query.project.is_some_and(|id| id != parent.project) {
            return Err(Error::Invalid);
        }
    }
    Ok(())
}
