use crate::*;
use jj_store_model::{PrincipalId, RepositoryId};

fn next(version: u64) -> Result<u64> {
    version.checked_add(1).ok_or(Error::Limit)
}
fn expected(actual: u64, expected: u64) -> Result<()> {
    if actual != expected {
        return Err(Error::Stale);
    }
    Ok(())
}
impl WorkStateV1 {
    /// Work on a private candidate; any rejection leaves the caller's state intact.
    pub fn apply(
        &self,
        command: &WorkCommandV1,
        actor: PrincipalId,
        at: i64,
    ) -> Result<(Self, WorkCommandResultV1)> {
        command.validate()?;
        if at < 0 {
            return Err(Error::Invalid("command time"));
        }
        let mut state = self.clone();
        let result = state.apply_inner(command, actor, at)?;
        state.validate()?;
        Ok((state, result))
    }
    fn project(&self, id: ProjectId, version: u64) -> Result<&ProjectV1> {
        let p = self.projects.get(&id).ok_or(Error::NotFound("project"))?;
        expected(p.version, version)?;
        Ok(p)
    }
    fn target(
        &self,
        project: ProjectId,
        version: u64,
        repository: Option<RepositoryId>,
        previous: Option<RepositoryId>,
    ) -> Result<()> {
        let p = self.project(project, version)?;
        if repository != previous
            && repository.is_some_and(|id| p.repositories.get(&id) != Some(&true))
        {
            return Err(Error::Invalid("inactive repository target"));
        }
        Ok(())
    }
    fn touch_task(
        &mut self,
        id: TaskId,
        version: u64,
        actor: PrincipalId,
        at: i64,
    ) -> Result<TaskV1> {
        let task = self.tasks.get_mut(&id).ok_or(Error::NotFound("task"))?;
        expected(task.version, version)?;
        task.version = next(task.version)?;
        task.actor.update(actor, at);
        Ok(task.clone())
    }
    fn apply_inner(
        &mut self,
        command: &WorkCommandV1,
        actor: PrincipalId,
        at: i64,
    ) -> Result<WorkCommandResultV1> {
        use WorkCommandV1::*;
        let mut result = WorkCommandResultV1::default();
        match command {
            CreateProject { id, title, body } => {
                if self.projects.contains_key(id) {
                    return Err(Error::Stale);
                }
                let project = ProjectV1 {
                    schema_version: 1,
                    id: *id,
                    title: title.clone(),
                    body: body.clone(),
                    version: 1,
                    repositories: Default::default(),
                    actor: WorkActorV1::new(actor, at),
                };
                self.projects.insert(*id, project.clone());
                result.project = Some(project);
            }
            UpdateProject {
                id,
                expected_version,
                title,
                body,
            } => {
                let mut project = self.project(*id, *expected_version)?.clone();
                project.title = title.clone();
                project.body = body.clone();
                project.version = next(project.version)?;
                project.actor.update(actor, at);
                self.projects.insert(*id, project.clone());
                result.project = Some(project);
            }
            SetRepository {
                project,
                expected_version,
                repository,
                active,
            } => {
                let mut p = self.project(*project, *expected_version)?.clone();
                if !*active && !p.repositories.contains_key(repository) {
                    return Err(Error::NotFound("repository membership"));
                }
                if p.repositories.get(repository) == Some(active) {
                    return Err(Error::Invalid("unchanged membership"));
                }
                p.repositories.insert(*repository, *active);
                p.version = next(p.version)?;
                p.actor.update(actor, at);
                self.projects.insert(*project, p.clone());
                result.project = Some(p);
            }
            CreateTask {
                id,
                project,
                expected_project_version,
                title,
                body,
                repository,
                parent,
            } => {
                if self.tasks.contains_key(id) {
                    return Err(Error::Stale);
                }
                self.target(*project, *expected_project_version, *repository, None)?;
                self.validate_parent(*id, *project, *parent)?;
                let task = TaskV1 {
                    schema_version: 1,
                    id: *id,
                    project: *project,
                    title: title.clone(),
                    body: body.clone(),
                    state: TaskStateV1::Open,
                    repository: *repository,
                    parent: *parent,
                    version: 1,
                    actor: WorkActorV1::new(actor, at),
                };
                self.tasks.insert(*id, task.clone());
                result.task = Some(task);
            }
            UpdateTask {
                id,
                expected_version,
                expected_project_version,
                title,
                body,
                state,
                repository,
                parent,
            } => {
                let mut task = self.tasks.get(id).ok_or(Error::NotFound("task"))?.clone();
                expected(task.version, *expected_version)?;
                self.target(
                    task.project,
                    *expected_project_version,
                    *repository,
                    task.repository,
                )?;
                self.validate_parent(*id, task.project, *parent)?;
                task.title = title.clone();
                task.body = body.clone();
                task.state = *state;
                task.repository = *repository;
                task.parent = *parent;
                task.version = next(task.version)?;
                task.actor.update(actor, at);
                self.tasks.insert(*id, task.clone());
                result.task = Some(task);
            }
            AddComment {
                id,
                task,
                expected_task_version,
                reply_to,
                body,
            } => {
                if self.comments.contains_key(id) {
                    return Err(Error::Stale);
                }
                if let Some(parent) = reply_to
                    && self
                        .comments
                        .get(parent)
                        .ok_or(Error::NotFound("reply comment"))?
                        .task
                        != *task
                {
                    return Err(Error::Invalid("cross-task reply"));
                }
                result.task = Some(self.touch_task(*task, *expected_task_version, actor, at)?);
                let comment = WorkCommentV1 {
                    schema_version: 1,
                    id: *id,
                    task: *task,
                    reply_to: *reply_to,
                    body: Some(body.clone()),
                    version: 1,
                    actor: WorkActorV1::new(actor, at),
                };
                self.comments.insert(*id, comment.clone());
                result.comment = Some(comment);
            }
            EditComment {
                id,
                expected_version,
                expected_task_version,
                ..
            }
            | TombstoneComment {
                id,
                expected_version,
                expected_task_version,
            } => {
                let mut c = self
                    .comments
                    .get(id)
                    .ok_or(Error::NotFound("comment"))?
                    .clone();
                expected(c.version, *expected_version)?;
                if c.body.is_none() {
                    return Err(Error::Invalid("comment is tombstoned"));
                }
                result.task = Some(self.touch_task(c.task, *expected_task_version, actor, at)?);
                c.body = match command {
                    EditComment { body, .. } => Some(body.clone()),
                    _ => None,
                };
                c.version = next(c.version)?;
                c.actor.update(actor, at);
                self.comments.insert(*id, c.clone());
                result.comment = Some(c);
            }
        }
        Ok(result)
    }
}
