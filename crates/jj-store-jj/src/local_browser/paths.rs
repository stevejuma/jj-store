//! Cursor-ready recursive file rows with bounded last-change metadata.
use super::native::{map_backend, path as native_path, terms};
use super::*;
use jj_lib::{matchers::PrefixMatcher, object_id::ObjectId as _, tree::TreeMergeExt as _};
use serde::Deserialize;
use std::collections::VecDeque;

const LAST_CHANGE_VISITS: usize = 64;
fn include_metadata() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalPathsRequest")]
pub struct Request {
    pub commit: m::GitSha1Id,
    pub path: String,
    pub recursive: bool,
    #[serde(default = "include_metadata")]
    pub include_last_change: bool,
    pub limit: usize,
    pub anchor: Option<String>,
    pub direction: Direction,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
#[schemars(rename = "LocalPathsLastChange")]
pub enum LastChange {
    Available {
        commit: m::GitSha1Id,
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
#[schemars(rename = "LocalPathsLastChangeUnavailable")]
pub enum LastChangeUnavailable {
    Merge,
    Budget,
    NotRequested,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalPathsEntry")]
pub struct Entry {
    pub path: String,
    pub terms: Vec<Option<dto::TreeValue>>,
    pub conflicted: bool,
    pub last_change: LastChange,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "LocalPathsPage")]
pub struct Page {
    pub version: u32,
    pub commit: m::GitSha1Id,
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
    context.member(ObjectKey::Commit(request.commit)).await?;
    let prefix = native_path(&request.path)?;
    let store = context.code_store()?;
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
        context.last_changes(&store, request.commit, &found).await?
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
        mut id: m::GitSha1Id,
        rows: &VecDeque<(String, jj_lib::backend::MergedTreeValue)>,
    ) -> Result<Vec<LastChange>> {
        let mut results = vec![
            LastChange::Unavailable {
                reason: LastChangeUnavailable::Budget
            };
            rows.len()
        ];
        let mut pending = rows
            .iter()
            .enumerate()
            .map(|(index, (path, value))| Ok((index, native_path(path)?, value)))
            .collect::<Result<Vec<_>>>()?;
        // Traverse the shared first-parent chain once for the entire window.
        // A merge terminates attribution explicitly; no row invents a result.
        for _ in 0..LAST_CHANGE_VISITS {
            if pending.is_empty() {
                break;
            }
            let commit = self.commit(store, id).await?;
            let parents = commit.parent_ids();
            if parents.is_empty() {
                let value = self.last_change_available(id).await?;
                for (index, _, _) in pending {
                    results[index] = value.clone();
                }
                break;
            }
            if parents.len() != 1 {
                for (index, _, _) in pending {
                    results[index] = LastChange::Unavailable {
                        reason: LastChangeUnavailable::Merge,
                    };
                }
                break;
            }
            let parent =
                m::GitSha1Id::from_slice(parents[0].as_bytes()).map_err(|_| Error::Corrupt)?;
            let prior = self.commit(store, parent).await?;
            let mut unchanged = Vec::new();
            let mut changed = None;
            for (index, path, value) in pending {
                self.budget.authorize(self.access.as_ref()).await?;
                self.budget.work(1)?;
                let prior_value = prior.tree().path_value(&path).await.map_err(map_backend)?;
                if prior_value == *value {
                    unchanged.push((index, path, value));
                } else {
                    if changed.is_none() {
                        changed = Some(self.last_change_available(id).await?);
                    }
                    results[index] = changed.as_ref().unwrap().clone();
                }
            }
            pending = unchanged;
            id = parent;
        }
        Ok(results)
    }
    async fn last_change_available(&self, id: m::GitSha1Id) -> Result<LastChange> {
        let commit = self.commit_dto(id).await?;
        Ok(LastChange::Available {
            commit: id,
            change: commit.change_id,
            description: commit.description,
            author: Box::new(commit.author),
            committer: Box::new(commit.committer),
        })
    }
}
