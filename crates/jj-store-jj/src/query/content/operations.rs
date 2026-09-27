//! Complete native view differences and bounded commit-parent visibility deltas.
use super::*;
use jj_lib::object_id::ObjectId as _;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub operation: m::OperationId,
    pub parent: Option<m::OperationId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SetDelta<T> {
    pub added: Vec<T>,
    pub removed: Vec<T>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NamedChange<T> {
    pub name: String,
    pub before: Option<T>,
    pub after: Option<T>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ViewDelta {
    pub heads: SetDelta<m::CommitId>,
    pub local_bookmarks: Vec<NamedChange<dto::RefTarget>>,
    pub local_tags: Vec<NamedChange<dto::RefTarget>>,
    pub remote_views: Vec<NamedChange<dto::RemoteView>>,
    pub git_refs: Vec<NamedChange<dto::RefTarget>>,
    pub git_heads: Vec<NamedChange<dto::RefTarget>>,
    pub workspaces: Vec<NamedChange<m::CommitId>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Difference {
    pub operation: m::OperationId,
    pub parent: m::OperationId,
    pub before_view: m::ViewId,
    pub after_view: m::ViewId,
    pub view: ViewDelta,
    pub visible_commits: SetDelta<m::CommitId>,
    pub commit_predecessors: Option<BTreeMap<m::CommitId, Vec<m::CommitId>>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Data {
    Root {
        operation: m::OperationId,
        view: m::ViewId,
    },
    SelectParent {
        operation: m::OperationId,
        parents: Vec<m::OperationId>,
    },
    Difference(Box<Difference>),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Response {
    pub version: u32,
    pub data: Data,
    pub stats: Stats,
}
pub(super) async fn read(context: Context, request: Request) -> Result<Response> {
    let operation = context.operation(request.operation).await?;
    let data = match request.parent {
        Some(parent) if !operation.parents.contains(&parent) => return Err(Error::Invalid),
        None if operation.parents.is_empty() => Data::Root {
            operation: request.operation,
            view: operation.view_id,
        },
        None if operation.parents.len() != 1 => Data::SelectParent {
            operation: request.operation,
            parents: operation.parents,
        },
        parent => {
            let parent = parent.unwrap_or(operation.parents[0]);
            Data::Difference(Box::new(
                context
                    .difference(request.operation, operation, parent)
                    .await?,
            ))
        }
    };
    let result = Response {
        version: 1,
        data,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
impl Context {
    async fn operation(&self, id: m::OperationId) -> Result<dto::Operation> {
        self.member(m::ObjectKey::Operation(id)).await?;
        let native = if id == roots::ROOT_OPERATION {
            roots::operation()
        } else {
            self.objects
                .get::<o::Operation>(m::ObjectKey::Operation(id))
                .await
                .map_err(map_object)?
        };
        dto::Operation::try_from(&native).map_err(|_| Error::Corrupt)
    }
    async fn view(&self, id: m::ViewId) -> Result<dto::View> {
        self.member(m::ObjectKey::View(id)).await?;
        let native = if id == roots::ROOT_VIEW {
            roots::view()
        } else {
            self.objects
                .get::<o::View>(m::ObjectKey::View(id))
                .await
                .map_err(map_object)?
        };
        dto::View::try_from(&native).map_err(|_| Error::Corrupt)
    }
    async fn difference(
        &self,
        id: m::OperationId,
        operation: dto::Operation,
        parent: m::OperationId,
    ) -> Result<Difference> {
        let previous = self.operation(parent).await?;
        let before = self.view(previous.view_id).await?;
        let after = self.view(operation.view_id).await?;
        let before_heads = before.head_ids.into_iter().collect();
        let after_heads = after.head_ids.into_iter().collect();
        let view = ViewDelta {
            heads: self.set_delta(&before_heads, &after_heads)?,
            local_bookmarks: self.map_delta(&before.local_bookmarks, &after.local_bookmarks)?,
            local_tags: self.map_delta(&before.local_tags, &after.local_tags)?,
            remote_views: self.map_delta(&before.remote_views, &after.remote_views)?,
            git_refs: self.map_delta(&before.git_refs, &after.git_refs)?,
            git_heads: self.map_delta(&before.git_heads, &after.git_heads)?,
            workspaces: self.map_delta(&before.wc_commit_ids, &after.wc_commit_ids)?,
        };
        let visible_commits = if before_heads == after_heads {
            SetDelta {
                added: vec![],
                removed: vec![],
            }
        } else {
            let mut visits = 0;
            let before = self.ancestors(before_heads, &mut visits).await?;
            let after = self.ancestors(after_heads, &mut visits).await?;
            self.set_delta(&before, &after)?
        };
        Ok(Difference {
            operation: id,
            parent,
            before_view: previous.view_id,
            after_view: operation.view_id,
            view,
            visible_commits,
            commit_predecessors: operation.commit_predecessors,
        })
    }
    fn map_delta<T: Clone + PartialEq + Serialize>(
        &self,
        before: &BTreeMap<String, T>,
        after: &BTreeMap<String, T>,
    ) -> Result<Vec<NamedChange<T>>> {
        let mut changes = vec![];
        // Both maps are bounded decoded metadata; iteration is deterministic.
        let names: BTreeSet<_> = before.keys().chain(after.keys()).collect();
        for name in names {
            self.budget.work(1)?;
            if before.get(name) == after.get(name) {
                continue;
            }
            let change = NamedChange {
                name: name.clone(),
                before: before.get(name).cloned(),
                after: after.get(name).cloned(),
            };
            self.budget.output(&change)?;
            changes.push(change);
        }
        Ok(changes)
    }
    fn set_delta<T: Clone + Ord + Serialize>(
        &self,
        before: &BTreeSet<T>,
        after: &BTreeSet<T>,
    ) -> Result<SetDelta<T>> {
        let collect = |items: std::collections::btree_set::Difference<'_, T>| -> Result<Vec<T>> {
            let mut values = vec![];
            for value in items {
                self.budget.work(1)?;
                self.budget.output(value)?;
                values.push(value.clone());
            }
            Ok(values)
        };
        Ok(SetDelta {
            added: collect(after.difference(before))?,
            removed: collect(before.difference(after))?,
        })
    }
    async fn ancestors(
        &self,
        heads: BTreeSet<m::CommitId>,
        visits: &mut usize,
    ) -> Result<BTreeSet<m::CommitId>> {
        let mut seen = BTreeSet::new();
        let mut pending = heads;
        while let Some(id) = pending.pop_first() {
            self.budget.work(1)?;
            if !seen.insert(id) {
                continue;
            }
            *visits = visits.checked_add(1).ok_or(Error::Limit)?;
            if *visits > self.limits.max_history_visits {
                return Err(Error::Limit);
            }
            self.member(m::ObjectKey::Commit(id)).await?;
            let native = if id == roots::ROOT_COMMIT {
                roots::commit()
            } else {
                self.objects
                    .get::<b::Commit>(m::ObjectKey::Commit(id))
                    .await
                    .map_err(map_object)?
            };
            for parent in native.parents {
                self.budget.work(1)?;
                let parent =
                    m::CommitId::from_slice(parent.as_bytes()).map_err(|_| Error::Corrupt)?;
                if !seen.contains(&parent) {
                    pending.insert(parent);
                }
                if pending.len() > self.limits.max_history_visits {
                    return Err(Error::Limit);
                }
            }
        }
        Ok(seen)
    }
}
