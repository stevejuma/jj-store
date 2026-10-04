//! Cursor-ready recursive file rows with bounded last-change metadata.
use super::native::{map_backend, path as native_path, terms};
use super::*;
use jj_lib::{matchers::PrefixMatcher, object_id::ObjectId as _, tree::TreeMergeExt as _};
use serde::Deserialize;
use std::collections::VecDeque;

const LAST_CHANGE_VISITS: usize = 64;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "LocalPathsRequest")]
pub struct Request {
    pub commit: m::GitSha1Id,
    pub path: String,
    pub recursive: bool,
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
    let mut entries = vec![];
    for (path, value) in found {
        let last_change = context
            .last_change(&store, request.commit, &path, value.clone())
            .await?;
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
    async fn last_change(
        &self,
        store: &std::sync::Arc<jj_lib::store::Store>,
        mut id: m::GitSha1Id,
        path: &str,
        mut value: jj_lib::backend::MergedTreeValue,
    ) -> Result<LastChange> {
        let path = native_path(path)?;
        for _ in 0..LAST_CHANGE_VISITS {
            self.budget.authorize(self.access.as_ref()).await?;
            self.budget.work(1)?;
            let commit = self.commit(store, id).await?;
            let parents = commit.parent_ids();
            if parents.is_empty() {
                return self.last_change_available(id).await;
            }
            if parents.len() != 1 {
                return Ok(LastChange::Unavailable {
                    reason: LastChangeUnavailable::Merge,
                });
            }
            let parent =
                m::GitSha1Id::from_slice(parents[0].as_bytes()).map_err(|_| Error::Corrupt)?;
            let prior = self.commit(store, parent).await?;
            let prior_value = prior.tree().path_value(&path).await.map_err(map_backend)?;
            if prior_value != value {
                return self.last_change_available(id).await;
            }
            id = parent;
            value = prior_value;
        }
        Ok(LastChange::Unavailable {
            reason: LastChangeUnavailable::Budget,
        })
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
