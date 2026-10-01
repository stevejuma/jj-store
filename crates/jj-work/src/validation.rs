use crate::*;
use std::collections::BTreeSet;

pub(crate) fn title(value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 512 || value.contains(['\n', '\r', '\0']) {
        return Err(Error::Invalid("title"));
    }
    Ok(())
}
pub(crate) fn body(value: &str) -> Result<()> {
    if value.len() > MAX_BODY_BYTES {
        return Err(Error::Limit);
    }
    if value.contains('\0') {
        return Err(Error::Invalid("body"));
    }
    Ok(())
}
fn actor(actor: &WorkActorV1, version: u64, schema: u32) -> Result<()> {
    if schema != 1
        || version == 0
        || actor.created_at_millis < 0
        || actor.updated_at_millis < actor.created_at_millis
    {
        return Err(Error::Invalid("version/actor"));
    }
    Ok(())
}
impl WorkStateV1 {
    pub fn validate(&self) -> Result<()> {
        if [self.projects.len(), self.tasks.len(), self.comments.len()]
            .into_iter()
            .any(|n| n > MAX_ENTITIES)
        {
            return Err(Error::Limit);
        }
        let mut bytes = 0usize;
        for (id, p) in &self.projects {
            if *id != p.id || p.repositories.len() > 256 {
                return Err(Error::Invalid("project identity/membership"));
            }
            title(&p.title)?;
            body(&p.body)?;
            actor(&p.actor, p.version, p.schema_version)?;
            bytes += p.body.len();
        }
        for (id, t) in &self.tasks {
            if *id != t.id {
                return Err(Error::Invalid("task identity"));
            }
            title(&t.title)?;
            body(&t.body)?;
            actor(&t.actor, t.version, t.schema_version)?;
            bytes += t.body.len();
            let project = self
                .projects
                .get(&t.project)
                .ok_or(Error::NotFound("task project"))?;
            if t.repository
                .is_some_and(|r| !project.repositories.contains_key(&r))
            {
                return Err(Error::Invalid("task repository"));
            }
            self.validate_parent(t.id, t.project, t.parent)?;
        }
        for (id, c) in &self.comments {
            if *id != c.id || !self.tasks.contains_key(&c.task) {
                return Err(Error::Invalid("comment identity/task"));
            }
            actor(&c.actor, c.version, c.schema_version)?;
            if let Some(text) = &c.body {
                body(text)?;
                if text.trim().is_empty() {
                    return Err(Error::Invalid("empty comment"));
                }
                bytes += text.len();
            }
            let mut parent = c.reply_to;
            let mut seen = BTreeSet::from([c.id]);
            while let Some(id) = parent {
                if !seen.insert(id) {
                    return Err(Error::Invalid("comment cycle"));
                }
                let p = self
                    .comments
                    .get(&id)
                    .ok_or(Error::NotFound("reply comment"))?;
                if p.task != c.task {
                    return Err(Error::Invalid("cross-task reply"));
                }
                parent = p.reply_to;
            }
        }
        if bytes > MAX_TOTAL_BODY_BYTES {
            return Err(Error::Limit);
        }
        Ok(())
    }
    pub(crate) fn validate_parent(
        &self,
        id: TaskId,
        project: ProjectId,
        mut parent: Option<TaskId>,
    ) -> Result<()> {
        let mut seen = BTreeSet::from([id]);
        while let Some(id) = parent {
            if !seen.insert(id) {
                return Err(Error::Invalid("task cycle"));
            }
            let task = self.tasks.get(&id).ok_or(Error::NotFound("parent task"))?;
            if task.project != project {
                return Err(Error::Invalid("cross-project parent"));
            }
            parent = task.parent;
        }
        Ok(())
    }
}
