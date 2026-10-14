//! Data-only, operation-scoped JJ annotation. No source capability is retained.
use super::{Error, Result, Signature, Stats, Timestamp, memory};
use futures::{AsyncReadExt as _, StreamExt as _, TryStreamExt as _};
use jj_lib::object_id::ObjectId as _;
use jj_lib::{
    annotate::FileAnnotator, backend as b, commit::Commit, repo::Repo, repo_path::RepoPathBuf,
    revset::RevsetExpression,
};
use jj_store_model as m;
use serde::Serialize;
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

pub const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_LINES: usize = 32_768;
const MAX_PENDING: usize = 32;
const TTL: Duration = Duration::from_secs(600);
const ALGORITHM: &str = "jj045-annotation-v1";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Selection {
    pub scope: String,
    pub repository: m::RepositoryId,
    pub operation: m::OperationId,
    pub commit: String,
    pub path: String,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "BlameRange")]
pub struct Range {
    pub start: usize,
    pub end: usize,
    pub commit_id: String,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "BlameCommit")]
pub struct Summary {
    pub id: String,
    pub change_id: String,
    pub description: String,
    pub author: Signature,
    pub committer: Signature,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
#[schemars(rename = "BlamePage")]
pub struct Page {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub operation: m::OperationId,
    pub commit: String,
    pub path: String,
    pub file_id: String,
    pub line_count: usize,
    pub ranges: Vec<Range>,
    pub commits: BTreeMap<String, Summary>,
    pub complete: bool,
    pub continuation: Option<String>,
    pub history_commits: u64,
    pub stats: Stats,
}
#[derive(Clone)]
struct Progress {
    annotator: FileAnnotator,
    page: Page,
}
struct Session {
    nonce: String,
    step: u64,
    progress: Option<Progress>,
    lease: Option<memory::Lease>,
}
struct Entry {
    session: tokio::sync::Mutex<Session>,
    expires: Mutex<Instant>,
}
#[derive(Default)]
pub struct Cache {
    entries: Mutex<BTreeMap<Selection, Arc<memory::Retained<Entry>>>>,
}
impl Cache {
    fn entry(&self, key: &Selection, continuation: Option<&str>) -> Result<Arc<Entry>> {
        let mut entries = self.entries.lock().expect("blame cache");
        entries.retain(|_, e| {
            e.evict_idle_if(|e| *e.expires.lock().expect("blame expiry") <= Instant::now());
            e.present()
        });
        if let Some(entry) = entries.get(key).and_then(|e| e.get()) {
            return Ok(entry);
        }
        if continuation.is_some() {
            return Err(Error::BlameReset);
        }
        if entries.len() >= 8 {
            let mut oldest: Vec<_> = entries
                .iter()
                .filter_map(|(key, value)| {
                    let entry = value.get()?;
                    let expires = *entry.expires.lock().expect("blame expiry");
                    Some((expires, key.clone()))
                })
                .collect();
            oldest.sort();
            for (_, key) in oldest {
                if entries[&key].evict_idle_if(|_| true) {
                    entries.remove(&key);
                    break;
                }
            }
            if entries.len() >= 8 {
                return Err(Error::Busy);
            }
        }
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let nonce = format!(
            "{:x}",
            Sha256::digest(
                format!(
                    "{ALGORITHM}:{key:?}:{:?}:{}",
                    std::time::SystemTime::now(),
                    SEQUENCE.fetch_add(1, Ordering::Relaxed)
                )
                .as_bytes()
            )
        );
        let entry = Arc::new(Entry {
            expires: Mutex::new(Instant::now() + TTL),
            session: tokio::sync::Mutex::new(Session {
                nonce,
                step: 0,
                progress: None,
                lease: None,
            }),
        });
        entries.insert(key.clone(), memory::Retained::new(entry.clone()));
        Ok(entry)
    }
}

fn signature(s: &b::Signature) -> Signature {
    Signature {
        name: s.name.clone(),
        email: s.email.clone(),
        timestamp: Timestamp {
            millis: s.timestamp.timestamp.0,
            offset_minutes: s.timestamp.tz_offset,
        },
    }
}
fn summary(c: &Commit) -> Summary {
    Summary {
        id: c.id().hex(),
        change_id: c.change_id().hex(),
        description: c.description().to_owned(),
        author: signature(c.author()),
        committer: signature(c.committer()),
    }
}
fn token(session: &Session) -> String {
    format!("{}-{:x}", session.nonce, session.step)
}
fn validate_token(session: &Session, cursor: &str) -> Result<u64> {
    let (nonce, step) = cursor.rsplit_once('-').ok_or(Error::BlameReset)?;
    let step = u64::from_str_radix(step, 16).map_err(|_| Error::BlameReset)?;
    if nonce != session.nonce || step > session.step {
        return Err(Error::BlameReset);
    }
    Ok(step)
}

/// Caller has opened a fresh authorized pinned repo on an admitted blocking
/// worker. `check` must reject cancellation/deadline even after synchronous JJ.
pub(crate) async fn read(
    repo: &dyn Repo,
    cache: &Cache,
    key: Selection,
    continuation: Option<&str>,
    check: impl Fn() -> Result<()>,
    stats: impl Fn() -> Stats,
    finish: impl AsyncFn() -> Result<()>,
) -> Result<Page> {
    check()?;
    let path = RepoPathBuf::from_internal_string(key.path.clone()).map_err(|_| Error::Invalid)?;
    if path.is_root()
        || key.path.len() > 4096
        || key.path.contains('\0')
        || key.path.split('/').any(|v| matches!(v, "." | ".." | ""))
    {
        return Err(Error::Invalid);
    }
    let id = b::CommitId::try_from_hex(&key.commit).ok_or(Error::Invalid)?;
    if !repo
        .index()
        .has_id(&id)
        .await
        .map_err(|_| Error::Unavailable)?
    {
        return Err(Error::NotCommitted);
    }
    let commit = repo
        .store()
        .get_commit_async(&id)
        .await
        .map_err(map_backend)?;
    let value = commit.tree().path_value(&path).await.map_err(map_backend)?;
    let Some(Some(b::TreeValue::File { id: file, .. })) = value.as_resolved() else {
        return Err(Error::Invalid);
    };
    // Validate selected file even on cache hits. Never trust a retained capability.
    let file_id = file.hex();
    let entry = cache.entry(&key, continuation)?;
    let mut session = entry.session.lock().await;
    check()?;
    finish().await?;
    if session
        .progress
        .as_ref()
        .is_some_and(|p| p.page.file_id != file_id)
    {
        return Err(Error::Corrupt);
    }
    let cached = |page: &Page| {
        *entry.expires.lock().expect("blame expiry") = Instant::now() + TTL;
        let mut page = page.clone();
        page.stats = stats();
        page
    };
    if let Some(cursor) = continuation {
        let step = validate_token(&session, cursor)?;
        if step < session.step {
            return session
                .progress
                .as_ref()
                .map(|p| cached(&p.page))
                .ok_or(Error::BlameReset);
        }
    } else if let Some(progress) = &session.progress {
        return Ok(cached(&progress.page));
    }
    if let Some(progress) = &session.progress
        && progress.page.complete
    {
        return Ok(cached(&progress.page));
    }
    // Covers the trial clone, JJ source maps, bounded comparison scratch and
    // projected summaries. The retained allocation is admitted separately.
    let _scratch = memory::Lease::reserve(256 * 1024 * 1024)?;
    let mut progress = if let Some(progress) = &session.progress {
        progress.clone()
    } else {
        let mut text = Vec::new();
        repo.store()
            .read_file(&path, file)
            .await
            .map_err(map_backend)?
            .take((MAX_FILE_BYTES + 1) as u64)
            .read_to_end(&mut text)
            .await
            .map_err(|_| Error::Unavailable)?;
        if text.len() > MAX_FILE_BYTES {
            return Err(Error::Limit);
        }
        if text.contains(&0) || std::str::from_utf8(&text).is_err() {
            return Err(Error::Invalid);
        }
        let annotator = FileAnnotator::with_file_content(&id, &path, text);
        let line_count = annotator.to_annotation().line_origins().count();
        if line_count > MAX_LINES {
            return Err(Error::Limit);
        }
        Progress {
            annotator,
            page: Page {
                version: 1,
                repository: key.repository,
                operation: key.operation,
                commit: key.commit.clone(),
                path: key.path.clone(),
                file_id,
                line_count,
                ranges: vec![],
                commits: BTreeMap::new(),
                complete: line_count == 0,
                continuation: None,
                history_commits: 0,
                stats: stats(),
            },
        }
    };
    let started = Instant::now();
    for _ in 0..64 {
        check()?;
        if progress.page.complete {
            break;
        }
        let before: Vec<_> = progress.annotator.pending_commits().cloned().collect();
        if before.is_empty() {
            progress.page.complete = true;
            break;
        }
        if before.len() > MAX_PENDING {
            return Err(Error::Limit);
        }
        let ids: Vec<_> = RevsetExpression::commits(before.clone())
            .ancestors()
            .evaluate(repo)
            .map_err(|_| Error::Unavailable)?
            .stream()
            .take(64)
            .try_collect()
            .await
            .map_err(|_| Error::Unavailable)?;
        // Topological windows allow JJ to skip unchanged revisions. All
        // parents remain in its frontier; missing edges are never origins.
        for id in &ids {
            if repo
                .store()
                .get_commit_async(id)
                .await
                .map_err(map_backend)?
                .parent_ids()
                .len()
                > 16
            {
                return Err(Error::Limit);
            }
        }
        let visited = ids.len() as u64;
        progress
            .annotator
            .compute(repo, &RevsetExpression::commits(ids))
            .await
            .map_err(|_| Error::Unavailable)?;
        check()?;
        progress.page.history_commits += visited;
        let after: Vec<_> = progress.annotator.pending_commits().cloned().collect();
        if after.len() > MAX_PENDING {
            return Err(Error::Limit);
        }
        if after.is_empty() || before == after {
            progress.page.complete = true;
        }
        if started.elapsed() >= Duration::from_secs(1) {
            break;
        }
    }
    let annotation = progress.annotator.to_annotation();
    progress.page.ranges.clear();
    for (index, (origin, _)) in annotation.line_origins().enumerate() {
        let Ok(origin) = origin else {
            continue;
        };
        let id = origin.commit_id.hex();
        if !progress.page.commits.contains_key(&id) {
            check()?;
            let c = repo
                .store()
                .get_commit_async(&origin.commit_id)
                .await
                .map_err(map_backend)?;
            progress.page.commits.insert(id.clone(), summary(&c));
        }
        if let Some(last) = progress.page.ranges.last_mut()
            && last.end == index + 1
            && last.commit_id == id
        {
            last.end += 1;
            continue;
        }
        progress.page.ranges.push(Range {
            start: index + 1,
            end: index + 2,
            commit_id: id,
        });
    }
    check()?;
    let pending = progress.annotator.pending_commits().count();
    let retained = (MAX_FILE_BYTES * 2
        + progress.page.line_count * 160
        + pending * (MAX_FILE_BYTES + progress.page.line_count * 32)
        + serde_json::to_vec(&progress.page)
            .map_err(|_| Error::Unavailable)?
            .len()
            * 2) as u64;
    let lease = memory::Lease::reserve(retained)?;
    finish().await?;
    check()?;
    session.step += 1;
    progress.page.continuation = (!progress.page.complete).then(|| token(&session));
    progress.page.stats = stats();
    let page = progress.page.clone();
    session.progress = Some(progress);
    session.lease = Some(lease);
    *entry.expires.lock().expect("blame expiry") = Instant::now() + TTL;
    Ok(page)
}
pub(crate) fn map_backend(error: b::BackendError) -> Error {
    match error {
        b::BackendError::Other(error) => error
            .downcast::<Error>()
            .map(|e| *e)
            .unwrap_or(Error::Unavailable),
        b::BackendError::ObjectNotFound { .. } => Error::Incomplete,
        _ => Error::Unavailable,
    }
}

#[cfg(test)]
mod tests;
