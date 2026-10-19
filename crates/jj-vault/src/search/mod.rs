//! Rebuildable changed-version text/path/metadata projection, never restore authority.
mod cache;
pub mod indexed;
mod native;
pub use jj_store_jj::vault_workspace::{HistoryFile, HistoryFiles, HistoryPage, HistoryRevision};
pub use native::NativeHistory;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub text: String,
    pub path: Option<String>,
    pub author: Option<String>,
    pub from_millis: Option<i64>,
    pub to_millis: Option<i64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cursor {
    pub anchor: String,
    pub revision: u32,
    pub after_path: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub query: Query,
    pub cursor: Option<Cursor>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hit {
    pub revision: HistoryRevision,
    pub path: Option<String>,
    pub line: Option<u32>,
    pub excerpt: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub hits: Vec<Hit>,
    pub next: Option<Cursor>,
    pub checkpoints: u32,
    pub unindexed: u32,
}
/// Inject a retained immutable reader. Cached file rows cannot authorize reads.
pub trait Source {
    fn revision(&mut self, anchor: Option<&str>, offset: u32) -> Result<HistoryPage, String>;
    fn files(&mut self, commit: &str, after: Option<&str>) -> Result<HistoryFiles, String>;
}
/// The host supplies a private cache directory and a namespace including its
/// current backup exclusion policy. Removing this directory loses no history.
pub struct Index {
    directory: PathBuf,
    namespace: String,
}
impl Index {
    pub fn new(directory: PathBuf, namespace: String) -> Self {
        Self {
            directory,
            namespace,
        }
    }
    /// One checkpoint and at most eight changed file versions per call. Continue
    /// with the returned fixed-anchor cursor; never infer completion from no hits.
    pub fn search(&self, source: &mut impl Source, request: &Request) -> Result<Page, String> {
        let query = &request.query;
        if query.text.len() > 512
            || query.path.as_ref().is_some_and(|v| v.len() > 4096)
            || query.author.as_ref().is_some_and(|v| v.len() > 256)
            || query
                .from_millis
                .zip(query.to_millis)
                .is_some_and(|(a, b)| a > b)
        {
            return Err("Invalid historical search filters".into());
        }
        let cursor = request.cursor.as_ref();
        let offset = cursor.map_or(0, |c| c.revision);
        let after = cursor.and_then(|c| c.after_path.as_deref());
        let page = source.revision(cursor.map(|c| c.anchor.as_str()), offset)?;
        let Some(revision) = page.revision else {
            return Ok(Page {
                hits: vec![],
                next: None,
                checkpoints: offset,
                unindexed: 0,
            });
        };
        let anchor = page.anchor.ok_or("History anchor is unavailable")?;
        let next_revision = || {
            page.more.then(|| Cursor {
                anchor: anchor.clone(),
                revision: offset + 1,
                after_path: None,
            })
        };
        let author = format!("{} {}", revision.author, revision.email).to_lowercase();
        let time = revision.checkpoint.timestamp_millis;
        if query
            .author
            .as_ref()
            .is_some_and(|v| !author.contains(&v.to_lowercase()))
            || query.from_millis.is_some_and(|from| time < from)
            || query.to_millis.is_some_and(|to| time > to)
        {
            return Ok(Page {
                hits: vec![],
                next: next_revision(),
                checkpoints: offset + 1,
                unindexed: 0,
            });
        }
        let terms = query
            .text
            .split_whitespace()
            .map(str::to_lowercase)
            .collect::<Vec<_>>();
        let matches = |s: &str| {
            let s = s.to_lowercase();
            terms.iter().all(|t| s.contains(t))
        };
        let mut hits = vec![];
        if after.is_none()
            && query.path.as_ref().is_none_or(|p| p.is_empty())
            && matches(&format!(
                "{} {} {}",
                revision.checkpoint.description, revision.checkpoint.commit, author
            ))
        {
            hits.push(Hit {
                excerpt: revision
                    .checkpoint
                    .description
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(240)
                    .collect(),
                revision: revision.clone(),
                path: None,
                line: None,
            });
        }
        let files = self.files(source, &revision.checkpoint.commit, after)?;
        let unindexed = files.files.iter().filter(|f| f.unindexed).count() as u32;
        for file in files.files {
            if query
                .path
                .as_ref()
                .is_some_and(|p| !file.path.to_lowercase().contains(&p.to_lowercase()))
            {
                continue;
            }
            if !matches(&format!(
                "{}\n{}",
                file.path,
                file.text.as_deref().unwrap_or("")
            )) {
                continue;
            }
            let (line, excerpt) = excerpt(file.text.as_deref(), &terms);
            hits.push(Hit {
                revision: revision.clone(),
                path: Some(file.path),
                line,
                excerpt,
            });
        }
        let next = files
            .next
            .map(|path| Cursor {
                anchor: anchor.clone(),
                revision: offset,
                after_path: Some(path),
            })
            .or_else(next_revision);
        Ok(Page {
            hits,
            next,
            checkpoints: offset + 1,
            unindexed,
        })
    }
}
fn excerpt(text: Option<&str>, terms: &[String]) -> (Option<u32>, String) {
    let Some(text) = text else {
        return (
            None,
            "Path indexed; file contents are unavailable for text search".into(),
        );
    };
    let (n, line) = text
        .lines()
        .enumerate()
        .find(|(_, line)| terms.iter().any(|t| line.to_lowercase().contains(t)))
        .or_else(|| text.lines().next().map(|line| (0, line)))
        .unwrap_or((0, ""));
    (Some(n as u32 + 1), line.chars().take(240).collect())
}
