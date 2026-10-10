//! A data-only search projection of one completely validated immutable root.
use super::*;
use jj_collab::{CollabGraphV1, CollabObjectV1, ThreadStateV1, ThreadStatusV1};
use jj_store_wal::publication::WorkStateV1;
use std::collections::BTreeMap;

pub(super) const MAX_BYTES: usize = 64 * 1024 * 1024;
pub(super) struct Index {
    docs: Vec<Document>,
    recent_order: Vec<usize>,
    search_order: Vec<usize>,
    pub objects: usize,
    pub bytes: usize,
    #[cfg(test)]
    hydrated: std::sync::atomic::AtomicUsize,
}
struct Document {
    hit: ContentHit,
    body: String,
    title_fold: String,
    body_fold: String,
    field: Field,
    recent: Option<(i64, String)>,
}
#[derive(Clone, Copy)]
enum Field {
    Title,
    Body,
    Both,
}
fn folded_len(value: &str) -> usize {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .map(char::len_utf8)
        .sum()
}
impl Index {
    fn new(objects: usize) -> Self {
        Self {
            docs: vec![],
            recent_order: vec![],
            search_order: vec![],
            objects,
            bytes: 1024,
            #[cfg(test)]
            hydrated: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    fn add(&mut self, hit: ContentHit, body: &str, field: Field, recent: bool) -> Result<usize> {
        let bytes = hit.title.len()
            + hit.revision.len()
            + hit.id.len()
            + 1024
            + body.len()
            + folded_len(body)
            + folded_len(&hit.title);
        self.bytes = self.bytes.checked_add(bytes).ok_or(Error::Limit)?;
        if self.bytes > MAX_BYTES {
            return Err(Error::Limit);
        }
        let id = self.docs.len();
        self.docs.push(Document {
            title_fold: hit.title.to_lowercase(),
            body_fold: body.to_lowercase(),
            body: body.into(),
            field,
            recent: recent.then(|| (hit.at_millis, String::new())),
            hit,
        });
        Ok(id)
    }
    fn finish(mut self) -> Self {
        self.recent_order = (0..self.docs.len())
            .filter(|id| self.docs[*id].recent.is_some())
            .collect();
        self.search_order = (0..self.docs.len()).collect();
        let mut recent = std::mem::take(&mut self.recent_order);
        let mut search = std::mem::take(&mut self.search_order);
        recent.sort_by(|a, b| self.compare(*a, *b, true));
        search.sort_by(|a, b| self.compare(*a, *b, false));
        self.recent_order = recent;
        self.search_order = search;
        self
    }
    fn time(&self, id: usize, recent: bool) -> i64 {
        if recent {
            self.docs[id].recent.as_ref().expect("recent order").0
        } else {
            self.docs[id].hit.at_millis
        }
    }
    fn compare(&self, a: usize, b: usize, recent: bool) -> std::cmp::Ordering {
        self.time(b, recent)
            .cmp(&self.time(a, recent))
            .then(self.docs[a].hit.id.cmp(&self.docs[b].hit.id))
    }
    pub fn ordered(&self, query: &ContentQuery, filter: &str) -> Vec<usize> {
        let order = if filter.is_empty() {
            &self.recent_order
        } else {
            &self.search_order
        };
        order
            .iter()
            .copied()
            .filter(|id| {
                let doc = &self.docs[*id];
                let hit = &doc.hit;
                if hit.thread.is_some() {
                    if !matches!(query.section, ContentSection::All | ContentSection::Threads)
                        || query.project.is_some()
                        || query.task.is_some()
                        || query.thread.is_some_and(|id| hit.thread != Some(id))
                    {
                        return false;
                    }
                } else if query.section == ContentSection::Threads
                    || query.thread.is_some()
                    || query.project.is_some_and(|id| hit.project != Some(id))
                    || query.task.is_some_and(|id| hit.task != Some(id))
                    || (hit.kind == ContentKind::Project && query.section == ContentSection::Tasks)
                {
                    return false;
                }
                filter.is_empty()
                    || match doc.field {
                        Field::Title => doc.title_fold.contains(filter),
                        Field::Body => doc.body_fold.contains(filter),
                        Field::Both => {
                            doc.title_fold.contains(filter) || doc.body_fold.contains(filter)
                        }
                    }
            })
            .collect()
    }
    pub fn hit(&self, id: usize, filter: &str) -> ContentHit {
        #[cfg(test)]
        self.hydrated
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let doc = &self.docs[id];
        let mut hit = doc.hit.clone();
        if filter.is_empty() && hit.kind == ContentKind::Thread {
            let (time, preview) = doc.recent.as_ref().expect("selected recent thread");
            hit.at_millis = *time;
            hit.excerpt = preview.clone();
        } else if hit.kind != ContentKind::Thread {
            hit.excerpt = extract::excerpt_folded(&doc.body, &doc.body_fold, filter);
        }
        hit
    }
    pub fn threads(state: &ThreadStateV1, graph: &CollabGraphV1) -> Result<Self> {
        let mut index = Self::new(graph.objects.len());
        let mut rows = BTreeMap::new();
        for (id, reference) in &state.threads {
            let CollabObjectV1::ThreadVersion(t) =
                graph.get(*reference).map_err(|_| Error::Corrupt)?
            else {
                return Err(Error::Corrupt);
            };
            if t.status == ThreadStatusV1::Tombstoned || !t.is_user_thread() {
                continue;
            }
            let row = index.add(
                ContentHit {
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
                },
                "",
                Field::Title,
                t.status != ThreadStatusV1::Archived,
            )?;
            let preview = index.docs[row].hit.excerpt.clone();
            if let Some((_, excerpt)) = &mut index.docs[row].recent {
                *excerpt = preview;
            }
            rows.insert(*id, row);
        }
        for reference in state.metadata.values() {
            let CollabObjectV1::ConversationMetadata(metadata) =
                graph.get(*reference).map_err(|_| Error::Corrupt)?
            else {
                return Err(Error::Corrupt);
            };
            if let Some(row) = rows.get(&metadata.thread_id).copied() {
                let title = metadata
                    .value
                    .title()
                    .unwrap_or(&index.docs[row].hit.title)
                    .to_owned();
                let body = metadata
                    .value
                    .generated
                    .as_ref()
                    .map(|g| {
                        format!(
                            "{}\n{}\n{}",
                            g.summary,
                            g.keywords.join(" "),
                            g.goals.join(" ")
                        )
                    })
                    .unwrap_or_default();
                if body.len() > 16 * 1024 {
                    return Err(Error::Limit);
                }
                index.docs[row].hit.title = title.clone();
                index.docs[row].title_fold = title.to_lowercase();
                index.bytes += body.len() + folded_len(&body) + title.len() + folded_len(&title);
                if index.bytes > MAX_BYTES {
                    return Err(Error::Limit);
                }
                index.docs[row].body_fold = body.to_lowercase();
                index.docs[row].body = body.clone();
                index.docs[row].field = Field::Both;
                index.docs[row].hit.excerpt = extract::excerpt(&body, "");
                index.docs[row].hit.revision = reference.id.to_string();
            }
        }
        for (id, reference) in &state.messages {
            let CollabObjectV1::HumanMessageRevision(message) =
                graph.get(*reference).map_err(|_| Error::Corrupt)?
            else {
                return Err(Error::Corrupt);
            };
            let Some(row) = rows.get(&message.thread_id).copied() else {
                continue;
            };
            let Some(body) = &message.body else { continue };
            if let Some((time, preview)) = &mut index.docs[row].recent
                && message.accepted_at_millis >= *time
            {
                *time = message.accepted_at_millis;
                *preview = extract::excerpt(body, "");
            }
            index.add(
                ContentHit {
                    kind: ContentKind::Message,
                    id: id.to_string(),
                    title: index.docs[row].hit.title.clone(),
                    excerpt: String::new(),
                    actor: message.actor,
                    at_millis: message.accepted_at_millis,
                    revision: reference.id.to_string(),
                    thread: Some(message.thread_id),
                    message: Some(*id),
                    project: None,
                    task: None,
                    comment: None,
                },
                body,
                Field::Body,
                false,
            )?;
        }
        Ok(index.finish())
    }
    pub fn work(state: &WorkStateV1) -> Result<Self> {
        let mut index = Self::new(state.projects.len() + state.tasks.len() + state.comments.len());
        for p in state.projects.values() {
            index.add(
                ContentHit {
                    kind: ContentKind::Project,
                    id: p.id.to_string(),
                    title: p.title.clone(),
                    excerpt: String::new(),
                    actor: p.actor.updated_by,
                    at_millis: p.actor.updated_at_millis,
                    revision: p.version.to_string(),
                    thread: None,
                    message: None,
                    project: Some(p.id),
                    task: None,
                    comment: None,
                },
                &p.body,
                Field::Both,
                true,
            )?;
        }
        for t in state.tasks.values() {
            index.add(
                ContentHit {
                    kind: ContentKind::Task,
                    id: t.id.to_string(),
                    title: t.title.clone(),
                    excerpt: String::new(),
                    actor: t.actor.updated_by,
                    at_millis: t.actor.updated_at_millis,
                    revision: t.version.to_string(),
                    thread: None,
                    message: None,
                    project: Some(t.project),
                    task: Some(t.id),
                    comment: None,
                },
                &t.body,
                Field::Both,
                t.state != crate::work::TaskStateV1::Done,
            )?;
        }
        for c in state.comments.values() {
            let Some(body) = &c.body else { continue };
            let t = state.tasks.get(&c.task).ok_or(Error::Corrupt)?;
            index.add(
                ContentHit {
                    kind: ContentKind::Comment,
                    id: c.id.to_string(),
                    title: t.title.clone(),
                    excerpt: String::new(),
                    actor: c.actor.updated_by,
                    at_millis: c.actor.updated_at_millis,
                    revision: c.version.to_string(),
                    thread: None,
                    message: None,
                    project: Some(t.project),
                    task: Some(t.id),
                    comment: Some(c.id),
                },
                body,
                Field::Body,
                false,
            )?;
        }
        Ok(index.finish())
    }
}

#[cfg(test)]
#[path = "index_tests.rs"]
mod tests;
pub(super) struct Selection {
    pub sources: Vec<Arc<Index>>,
    pub keys: Vec<(usize, usize)>,
    pub objects: usize,
}
impl Selection {
    pub fn new(sources: Vec<Arc<Index>>, query: &ContentQuery, filter: &str) -> Result<Self> {
        let mut keys: Vec<(usize, usize)> = Vec::new();
        let mut objects = 0usize;
        for (source, index) in sources.iter().enumerate() {
            objects = objects.checked_add(index.objects).ok_or(Error::Limit)?;
            let incoming = index.ordered(query, filter);
            // Each immutable source order is already sorted. Stable merging
            // keeps the original Thread/Message then Work tie behavior.
            let mut merged = Vec::with_capacity(keys.len() + incoming.len());
            let mut old = keys.into_iter().peekable();
            for id in incoming {
                let doc = &index.docs[id];
                while let Some(&(domain, before)) = old.peek() {
                    let previous = &sources[domain].docs[before];
                    let recent = filter.is_empty();
                    let cmp = index
                        .time(id, recent)
                        .cmp(&sources[domain].time(before, recent))
                        .then(previous.hit.id.cmp(&doc.hit.id));
                    if cmp.is_gt() {
                        break;
                    }
                    merged.push(old.next().expect("peeked key"));
                }
                merged.push((source, id));
            }
            merged.extend(old);
            keys = merged;
        }
        Ok(Self {
            sources,
            keys,
            objects,
        })
    }
    pub fn page(&self, filter: &str, offset: usize, limit: usize) -> Vec<ContentHit> {
        self.keys
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(source, id)| self.sources[*source].hit(*id, filter))
            .collect()
    }
}
