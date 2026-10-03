use super::*;
use crate::auth::Permission;
use jj_collab::{CollabObjectV1, ThreadStatusV1};
use std::collections::BTreeMap;

pub(super) async fn capture(
    session: &NativeSession,
    query: &ContentQuery,
    expires: u64,
) -> Result<Arc<cursors::Pin>> {
    let viewer = session.principal(Permission::Read).await?.id();
    let pin = session.snapshot().await?;
    let manifest = pin.manifest();
    let snapshot = ContentSnapshot {
        repo_id: session.repository,
        epoch: manifest.epoch,
        manifest_digest: pin.digest(),
        published_seq: manifest.published,
        thread_root: manifest.thread_root.map(|r| r.digest),
        work_root: manifest.work_root.map(|r| r.digest),
    };
    let mut results = Vec::new();
    let mut source_nodes = 0usize;
    let filter = query.q.trim().to_lowercase();
    if matches!(query.section, ContentSection::All | ContentSection::Threads)
        && query.project.is_none()
        && query.task.is_none()
        && let Some(root) = manifest.thread_root
    {
        let store = session.coordinator.native_thread_store(root).await?;
        source_nodes = source_nodes
            .checked_add(store.graph.objects.len())
            .ok_or(Error::Limit)?;
        if source_nodes > session.limits.inventory_nodes {
            return Err(Error::Limit);
        }
        let mut threads = BTreeMap::new();
        let mut recent_rows = BTreeMap::new();
        for (id, reference) in &store.state.threads {
            let CollabObjectV1::ThreadVersion(t) =
                store.graph.get(*reference).map_err(|_| Error::Corrupt)?
            else {
                return Err(Error::Corrupt);
            };
            if t.status == ThreadStatusV1::Tombstoned
                || query.thread.is_some_and(|selected| selected != *id)
            {
                continue;
            }
            threads.insert(*id, t);
            if (filter.is_empty() && t.status != ThreadStatusV1::Archived)
                || (!filter.is_empty() && matches_text(&filter, &t.title, ""))
            {
                recent_rows.insert(*id, results.len());
                results.push(ContentHit {
                    kind: ContentKind::Thread,
                    id: id.to_string(),
                    title: t.title.clone(),
                    excerpt: format!("{:?}", t.status).to_lowercase(),
                    actor: t.actor,
                    at_millis: t.accepted_at_millis,
                    revision: reference.id.to_string(),
                    thread: Some(*id),
                    message: None,
                    project: None,
                    task: None,
                    comment: None,
                });
            }
        }
        for (id, reference) in &store.state.messages {
            let CollabObjectV1::HumanMessageRevision(message) =
                store.graph.get(*reference).map_err(|_| Error::Corrupt)?
            else {
                return Err(Error::Corrupt);
            };
            let Some(thread) = threads.get(&message.thread_id) else {
                continue;
            };
            let Some(body) = &message.body else { continue };
            if filter.is_empty() {
                if let Some(row) = recent_rows
                    .get(&message.thread_id)
                    .map(|index| &mut results[*index])
                    && message.accepted_at_millis >= row.at_millis
                {
                    row.at_millis = message.accepted_at_millis;
                    row.excerpt = excerpt(body, "");
                }
            } else if matches_text(&filter, "", body) {
                results.push(ContentHit {
                    kind: ContentKind::Message,
                    id: id.to_string(),
                    title: thread.title.clone(),
                    excerpt: excerpt(body, &filter),
                    actor: message.actor,
                    at_millis: message.accepted_at_millis,
                    revision: reference.id.to_string(),
                    thread: Some(message.thread_id),
                    message: Some(*id),
                    project: None,
                    task: None,
                    comment: None,
                });
            }
        }
    }
    if matches!(
        query.section,
        ContentSection::All | ContentSection::Projects | ContentSection::Tasks
    ) && query.thread.is_none()
        && let Some(root) = manifest.work_root
    {
        let work = session.coordinator.work_store(root).await?;
        let state = work.state();
        source_nodes = source_nodes
            .checked_add(state.projects.len() + state.tasks.len() + state.comments.len())
            .ok_or(Error::Limit)?;
        if source_nodes > session.limits.inventory_nodes {
            return Err(Error::Limit);
        }
        for p in state
            .projects
            .values()
            .filter(|p| query.project.is_none_or(|id| id == p.id) && query.task.is_none())
        {
            if query.section != ContentSection::Tasks && matches_text(&filter, &p.title, &p.body) {
                results.push(ContentHit {
                    kind: ContentKind::Project,
                    id: p.id.to_string(),
                    title: p.title.clone(),
                    excerpt: excerpt(&p.body, &filter),
                    actor: p.actor.updated_by,
                    at_millis: p.actor.updated_at_millis,
                    revision: p.version.to_string(),
                    thread: None,
                    message: None,
                    project: Some(p.id),
                    task: None,
                    comment: None,
                });
            }
        }
        for t in state.tasks.values().filter(|t| {
            query.project.is_none_or(|id| id == t.project) && query.task.is_none_or(|id| id == t.id)
        }) {
            if (filter.is_empty() && t.state == crate::work::TaskStateV1::Done)
                || !matches_text(&filter, &t.title, &t.body)
            {
                continue;
            }
            results.push(ContentHit {
                kind: ContentKind::Task,
                id: t.id.to_string(),
                title: t.title.clone(),
                excerpt: excerpt(&t.body, &filter),
                actor: t.actor.updated_by,
                at_millis: t.actor.updated_at_millis,
                revision: t.version.to_string(),
                thread: None,
                message: None,
                project: Some(t.project),
                task: Some(t.id),
                comment: None,
            });
        }
        if !filter.is_empty() {
            for c in state.comments.values() {
                let Some(body) = &c.body else { continue };
                let Some(t) = state.tasks.get(&c.task) else {
                    return Err(Error::Corrupt);
                };
                if query.project.is_some_and(|id| id != t.project)
                    || query.task.is_some_and(|id| id != t.id)
                    || !matches_text(&filter, "", body)
                {
                    continue;
                }
                results.push(ContentHit {
                    kind: ContentKind::Comment,
                    id: c.id.to_string(),
                    title: t.title.clone(),
                    excerpt: excerpt(body, &filter),
                    actor: c.actor.updated_by,
                    at_millis: c.actor.updated_at_millis,
                    revision: c.version.to_string(),
                    thread: None,
                    message: None,
                    project: Some(t.project),
                    task: Some(t.id),
                    comment: Some(c.id),
                });
            }
        }
    }
    if results.len() > session.limits.inventory_nodes {
        return Err(Error::Limit);
    }
    if results
        .iter()
        .map(|r| r.title.len() + r.excerpt.len() + r.revision.len() + 512)
        .sum::<usize>()
        > 64 * 1024 * 1024
    {
        return Err(Error::Limit);
    }
    results.sort_by(|a, b| b.at_millis.cmp(&a.at_millis).then(a.id.cmp(&b.id)));
    Ok(Arc::new(cursors::Pin {
        expires,
        snapshot,
        viewer,
        results,
    }))
}
pub(super) fn matches_text(filter: &str, title: &str, body: &str) -> bool {
    filter.is_empty()
        || title.to_lowercase().contains(filter)
        || body.to_lowercase().contains(filter)
}
pub(super) fn excerpt(body: &str, filter: &str) -> String {
    let chars: Vec<_> = body.chars().collect();
    // Map the match in the folded string back to original character positions.
    let offset = if filter.is_empty() {
        0
    } else {
        let mut positions = Vec::new();
        for (index, character) in chars.iter().enumerate() {
            for folded in character.to_lowercase() {
                positions.extend(std::iter::repeat_n(index, folded.len_utf8()));
            }
        }
        body.to_lowercase()
            .find(filter)
            .and_then(|byte| positions.get(byte).copied())
            .unwrap_or(0)
            .saturating_sub(60)
    };
    let mut text: String = chars.iter().skip(offset).take(240).collect();
    if offset > 0 {
        text.insert(0, '…');
    }
    if chars.len() > offset + 240 {
        text.push('…');
    }
    text
}

#[cfg(test)]
mod tests {
    #[test]
    fn excerpt_keeps_unicode_expansion_match_in_original_text() {
        let body = format!("{}needle{}", "İ".repeat(400), "x".repeat(400));
        let excerpt = super::excerpt(&body, "needle");
        assert!(excerpt.contains("needle"));
        assert!(excerpt.starts_with('…') && excerpt.ends_with('…'));
        assert!(excerpt.chars().count() <= 242);
    }
}
