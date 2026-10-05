//! Cursor-ready recursive file rows with bounded last-change metadata.
use super::native::{map_backend, path as native_path, terms};
use super::*;
use jj_lib::{matchers::PrefixMatcher, object_id::ObjectId as _, tree::TreeMergeExt as _};
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LastChangeMode {
    #[default]
    Quick,
    History,
}

const LAST_CHANGE_VISITS: usize = 64;
fn include_metadata() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub commit: m::CommitId,
    pub path: String,
    pub recursive: bool,
    #[serde(default = "include_metadata")]
    pub include_last_change: bool,
    #[serde(default)]
    pub last_change_mode: LastChangeMode,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
/// Provenance of the displayed commit, not a claim about all-branch history.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Attribution {
    PathChange,
    Snapshot,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LastChange {
    Available {
        attribution: Attribution,
        commit: m::CommitId,
        change: m::ChangeId,
        description: String,
        author: Box<dto::Signature>,
        committer: Box<dto::Signature>,
    },
    Unavailable {
        reason: LastChangeUnavailable,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LastChangeUnavailable {
    Merge,
    Budget,
    NotRequested,
    Pending,
    NotFound,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Entry {
    pub path: String,
    pub terms: Vec<Option<dto::TreeValue>>,
    pub conflicted: bool,
    pub last_change: LastChange,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Page {
    pub version: u32,
    pub commit: m::CommitId,
    pub path: String,
    pub recursive: bool,
    pub entries: Vec<Entry>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Page> {
    if request.limit == 0
        || request.limit > context.limits.max_page
        || request.path.len() > 4096
        || request.anchor.as_ref().is_some_and(|v| v.len() > 4096)
    {
        return Err(Error::Limit);
    }
    context.member(m::ObjectKey::Commit(request.commit)).await?;
    let prefix = native_path(&request.path)?;
    let store = context.native_store()?;
    let commit = context.commit(&store, request.commit).await?;
    let matcher = PrefixMatcher::new([prefix.as_ref()]);
    let mut found = VecDeque::new();
    let mut has_previous = false;
    let mut has_next = false;
    // Load only the selected directory terms for a direct-child page. The
    // recursive iterator descends into every subtree and never emits directories.
    let trees = if request.recursive {
        None
    } else {
        commit
            .tree()
            .trees()
            .await
            .map_err(map_backend)?
            .sub_tree_recursive(&prefix)
            .await
            .map_err(map_backend)?
    };
    let values: Box<dyn Iterator<Item = (_, _)>> = if request.recursive {
        Box::new(commit.tree().entries_matching(&matcher))
    } else if let Some(trees) = &trees {
        Box::new(
            jj_lib::merged_tree::all_merged_tree_entries(trees)
                .map(|(name, _)| (prefix.join(name), Ok(trees.value(name).cloned()))),
        )
    } else {
        Box::new(std::iter::empty())
    };
    for (path, value) in values {
        context.budget.authorize(context.access.as_ref()).await?;
        context.budget.work(1)?;
        let value = value.map_err(map_backend)?;
        if value.as_resolved() == Some(&None) {
            continue;
        }
        let path = path.as_internal_file_string().to_owned();
        if !request.recursive && parent(&path) != request.path {
            continue;
        }
        match request.direction {
            Direction::Forward => {
                if request
                    .anchor
                    .as_ref()
                    .is_some_and(|anchor| path <= *anchor)
                {
                    has_previous = true;
                    continue;
                }
                if found.len() == request.limit {
                    has_next = true;
                    break;
                }
                found.push_back((path, value));
            }
            Direction::Backward => {
                if request
                    .anchor
                    .as_ref()
                    .is_some_and(|anchor| path >= *anchor)
                {
                    has_next = true;
                    break;
                }
                found.push_back((path, value));
                if found.len() > request.limit {
                    found.pop_front();
                    has_previous = true;
                }
            }
        }
    }
    let previous = (has_previous && !found.is_empty()).then(|| found.front().unwrap().0.clone());
    let next = (has_next && !found.is_empty()).then(|| found.back().unwrap().0.clone());
    let changes = if request.include_last_change {
        context
            .last_changes(&store, request.commit, &found, request.last_change_mode)
            .await?
    } else {
        vec![
            LastChange::Unavailable {
                reason: LastChangeUnavailable::NotRequested
            };
            found.len()
        ]
    };
    let mut entries = vec![];
    for ((path, value), last_change) in found.into_iter().zip(changes) {
        let entry = Entry {
            path,
            conflicted: !value.is_resolved(),
            terms: terms(&value)?,
            last_change,
        };
        context.budget.output(&entry)?;
        entries.push(entry);
    }
    let result = Page {
        version: 1,
        commit: request.commit,
        path: request.path,
        recursive: request.recursive,
        entries,
        previous,
        next,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(parent, _)| parent)
}
impl Context {
    async fn last_changes(
        &self,
        store: &std::sync::Arc<jj_lib::store::Store>,
        mut id: m::CommitId,
        rows: &VecDeque<(String, jj_lib::backend::MergedTreeValue)>,
        mode: LastChangeMode,
    ) -> Result<Vec<LastChange>> {
        if rows.is_empty() {
            return Ok(vec![]);
        }
        if mode == LastChangeMode::History {
            let index = self
                .code_index
                .as_ref()
                .map(|p| p.index.clone())
                .ok_or(Error::IndexRebuildRequired)?;
            use crate::query::code_index::path_history::Change;
            let changes = index
                .history
                .read(
                    &index.raw,
                    store,
                    jj_lib::backend::CommitId::from_bytes(id.as_bytes()),
                    rows.iter()
                        .map(|(path, _)| native_path(path))
                        .collect::<Result<Vec<_>>>()?,
                    |work| self.budget.work(work),
                )
                .await?;
            let mut result = Vec::with_capacity(changes.len());
            for change in changes {
                self.budget.authorize(self.access.as_ref()).await?;
                result.push(match change {
                    Change::Found(commit) => {
                        self.last_change_available(
                            m::CommitId::from_slice(commit.as_bytes())
                                .map_err(|_| Error::Corrupt)?,
                            Attribution::PathChange,
                        )
                        .await?
                    }
                    Change::Pending => LastChange::Unavailable {
                        reason: LastChangeUnavailable::Pending,
                    },
                    Change::Missing => LastChange::Unavailable {
                        reason: LastChangeUnavailable::NotFound,
                    },
                });
            }
            return Ok(result);
        }

        // A snapshot is useful final metadata when the bounded scan finds no
        // path change. It always describes the explicitly selected commit.
        let fallback = self
            .last_change_available(id, Attribution::Snapshot)
            .await?;
        let mut results = vec![fallback; rows.len()];
        let mut pending: Vec<_> = (0..rows.len()).collect();
        for _ in 0..LAST_CHANGE_VISITS {
            if pending.is_empty() {
                break;
            }
            let commit = self.commit(store, id).await?;
            let Some(parent_id) = commit.parent_ids().first() else {
                break;
            };
            let parent_id =
                m::CommitId::from_slice(parent_id.as_bytes()).map_err(|_| Error::Corrupt)?;
            let prior = self.commit(store, parent_id).await?;
            if commit.tree_ids() == prior.tree_ids() {
                id = parent_id;
                continue;
            }
            // Group by directory, including recursive pages, so sibling rows
            // share a single tree lookup. Never open file contents or merge parents.
            let mut directories = std::collections::BTreeMap::<&str, Vec<usize>>::new();
            for row in &pending {
                directories
                    .entry(parent(&rows[*row].0))
                    .or_default()
                    .push(*row);
            }
            let roots = prior.tree().trees().await.map_err(map_backend)?;
            let mut unchanged = Vec::new();
            let mut changed = None;
            for (directory, indices) in directories {
                let prefix = native_path(directory)?;
                let trees = roots
                    .sub_tree_recursive(&prefix)
                    .await
                    .map_err(map_backend)?;
                let wanted: std::collections::BTreeMap<_, _> = indices
                    .iter()
                    .map(|row| (rows[*row].0.as_str(), *row))
                    .collect();
                let mut values = std::collections::BTreeMap::new();
                if let Some(trees) = trees {
                    for (name, _) in jj_lib::merged_tree::all_merged_tree_entries(&trees) {
                        self.budget.work(1)?;
                        let path = prefix.join(name);
                        if let Some(row) = wanted.get(path.as_internal_file_string()) {
                            values.insert(*row, trees.value(name).cloned());
                        }
                    }
                }
                for row in indices {
                    self.budget.authorize(self.access.as_ref()).await?;
                    self.budget.work(1)?;
                    let prior_value = values
                        .remove(&row)
                        .unwrap_or_else(|| jj_lib::merge::Merge::resolved(None));
                    if prior_value == rows[row].1 {
                        unchanged.push(row);
                    } else {
                        if changed.is_none() {
                            changed = Some(
                                self.last_change_available(id, Attribution::PathChange)
                                    .await?,
                            );
                        }
                        results[row] = changed.as_ref().unwrap().clone();
                    }
                }
            }
            pending = unchanged;
            id = parent_id;
        }
        Ok(results)
    }
    async fn last_change_available(
        &self,
        id: m::CommitId,
        attribution: Attribution,
    ) -> Result<LastChange> {
        let commit = self.commit_dto(id).await?;
        Ok(LastChange::Available {
            attribution,
            commit: id,
            change: commit.change_id,
            description: commit.description,
            author: Box::new(commit.author),
            committer: Box::new(commit.committer),
        })
    }
}
