//! Read-side lineage lookup: placeholders are boundaries, never fake histories.
use super::{
    diff::{Source, SourceKind},
    native::map_backend,
    *,
};
use jj_lib::{
    backend::MergedTreeValueExt as _, merged_tree::MergedTree, object_id::ObjectId as _,
    repo_path::RepoPath,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Default)]
pub(super) struct Copies {
    graph: BTreeMap<b::CopyId, b::CopyHistory>,
}
impl Copies {
    pub async fn sources(
        &mut self,
        context: &Context,
        before: &MergedTree,
        after: &MergedTree,
        id: &b::CopyId,
        target: &RepoPath,
    ) -> Result<Vec<Source>> {
        if id.as_bytes().is_empty() {
            return Ok(vec![]);
        }
        if !self.graph.contains_key(id) {
            let related = before
                .store()
                .backend()
                .get_related_copies(id)
                .await
                .map_err(map_backend)?;
            for related in related {
                context.budget.work(1)?;
                self.graph.insert(related.id, related.history);
                if self.graph.len() > context.limits.max_history_visits {
                    return Err(Error::Limit);
                }
            }
        }
        let history = self.graph.get(id).ok_or(Error::Incomplete)?;
        if history.current_path.as_ref() != target {
            return Err(Error::Corrupt);
        }
        let mut found = self
            .find(context, before, history.parents.clone(), false)
            .await?;
        if found.is_empty() {
            found = self.find(context, before, vec![id.clone()], true).await?;
        }
        let mut result = vec![];
        for (path, value) in found {
            context.budget.authorize(context.access.as_ref()).await?;
            let current = after.path_value(&path).await.map_err(map_backend)?;
            let kind = if path.as_ref() == target {
                SourceKind::Normal
            } else if current.is_absent() || current.is_tree() {
                SourceKind::Rename
            } else {
                SourceKind::Copy
            };
            let source = Source {
                kind,
                path: path.as_internal_file_string().to_owned(),
                terms: native::terms(&value)?,
            };
            context.budget.output(&source)?;
            result.push(source);
        }
        Ok(result)
    }
    async fn find(
        &self,
        context: &Context,
        tree: &MergedTree,
        start: Vec<b::CopyId>,
        reverse: bool,
    ) -> Result<BTreeMap<jj_lib::repo_path::RepoPathBuf, b::MergedTreeValue>> {
        let mut children: BTreeMap<&b::CopyId, Vec<&b::CopyId>> = BTreeMap::new();
        if reverse {
            for (id, history) in &self.graph {
                for parent in &history.parents {
                    context.budget.work(1)?;
                    if !parent.as_bytes().is_empty() {
                        children.entry(parent).or_default().push(id);
                    }
                }
            }
        }
        let mut pending: VecDeque<_> = start.into();
        let mut visited = BTreeSet::new();
        let mut found = BTreeMap::new();
        while let Some(id) = pending.pop_front() {
            context.budget.work(1)?;
            if id.as_bytes().is_empty() || !visited.insert(id.clone()) {
                continue;
            }
            let history = self.graph.get(&id).ok_or(Error::Incomplete)?;
            context.budget.authorize(context.access.as_ref()).await?;
            let value = tree
                .path_value(&history.current_path)
                .await
                .map_err(map_backend)?;
            context
                .budget
                .work(value.iter().len().saturating_mul(value.iter().len()))?;
            if value
                .simplify()
                .adds()
                .any(|v| matches!(v,Some(b::TreeValue::File{copy_id,..}) if *copy_id==id))
            {
                found.insert(history.current_path.clone(), value);
            } else if reverse {
                if let Some(ids) = children.get(&id) {
                    pending.extend(ids.iter().map(|id| (*id).clone()));
                }
            } else {
                pending.extend(history.parents.iter().cloned());
            }
            if pending.len() > context.limits.max_work {
                return Err(Error::Limit);
            }
        }
        Ok(found)
    }
}
