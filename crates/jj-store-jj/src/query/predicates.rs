//! Admission for native recursive merges and file predicates; evaluation stays native.
use super::*;
use jj_lib::{
    backend as b,
    repo::{ReadonlyRepo, Repo as _},
    revset::{RevsetExpression as E, RevsetFilterPredicate as F, UserRevsetExpression},
};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_file_bytes: u64,
    pub max_total_file_bytes: u64,
    pub max_lines: u64,
    pub max_merge_terms: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_file_bytes: 256 * 1024,
            max_total_file_bytes: 4 * 1024 * 1024,
            max_lines: 1000,
            max_merge_terms: 65,
        }
    }
}
impl Limits {
    pub(super) fn validate(self) -> Result<()> {
        if !(1..=1024 * 1024).contains(&self.max_file_bytes)
            || !(1..=16 * 1024 * 1024).contains(&self.max_total_file_bytes)
            || !(1..=4096).contains(&self.max_lines)
            || !(1..=257).contains(&self.max_merge_terms)
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
fn count(expression: &UserRevsetExpression, budget: &Budget) -> Result<usize> {
    let mut stack = vec![expression];
    let mut filters = 0usize;
    while let Some(expression) = stack.pop() {
        budget.work(1)?;
        match expression {
            E::Filter(F::File(_) | F::DiffLines { .. }) => filters += 1,
            E::Ancestors { heads, .. } => stack.push(heads),
            E::Descendants { roots, .. } => stack.push(roots),
            E::Range { roots, heads, .. } | E::DagRange { roots, heads } => {
                stack.extend([roots.as_ref(), heads.as_ref()])
            }
            E::Reachable { sources, domain } => stack.extend([sources.as_ref(), domain.as_ref()]),
            E::HeadsRange {
                roots,
                heads,
                filter,
                ..
            } => stack.extend([roots.as_ref(), heads.as_ref(), filter.as_ref()]),
            E::Heads(a)
            | E::Roots(a)
            | E::ForkPoint(a)
            | E::MergePoint(a)
            | E::Bisect(a)
            | E::AsFilter(a)
            | E::Present(a)
            | E::NotIn(a) => stack.push(a),
            E::HasSize { candidates, .. }
            | E::Latest { candidates, .. }
            | E::AtOperation { candidates, .. }
            | E::WithinReference { candidates, .. }
            | E::WithinVisibility { candidates, .. } => stack.push(candidates),
            E::Coalesce(a, b) | E::Union(a, b) | E::Intersection(a, b) | E::Difference(a, b) => {
                stack.extend([a.as_ref(), b.as_ref()])
            }
            E::None
            | E::All
            | E::VisibleHeads
            | E::VisibleHeadsOrReferenced
            | E::Root
            | E::Commits(_)
            | E::CommitRef(_)
            | E::Forks
            | E::Filter(_)
            | E::Divergent => {}
        }
    }
    Ok(filters)
}
#[derive(Clone, Copy)]
struct Shape {
    terms: usize,
    trees: usize,
    label_bytes: usize,
}
impl Shape {
    fn check(self, budget: &Budget) -> Result<Self> {
        if self.terms > budget.limits.predicates.max_merge_terms
            || self.trees > budget.limits.predicates.max_merge_terms
            || self.label_bytes > budget.limits.max_output_bytes
        {
            return Err(budget.fail(Error::Limit));
        }
        Ok(self)
    }
    fn add(self, other: Self, budget: &Budget) -> Result<Self> {
        Self {
            terms: self.terms.checked_add(other.terms).ok_or(Error::Limit)?,
            trees: self.trees.checked_add(other.trees).ok_or(Error::Limit)?,
            label_bytes: self
                .label_bytes
                .checked_add(other.label_bytes)
                .ok_or(Error::Limit)?,
        }
        .check(budget)
    }
}
struct CommitShape {
    parents: Vec<b::CommitId>,
    shape: Shape,
}
struct Frame {
    ids: Vec<b::CommitId>,
    pos: usize,
    shape: Shape,
}
/// Count the exact unsimplified terms in pinned find_recursive_merge_commits.
/// No native Merge is constructed here. Common-ancestor selection is delegated
/// to the same native index; its input domain is charged before each call.
async fn merge_shape(
    repo: &ReadonlyRepo,
    shapes: &BTreeMap<b::CommitId, CommitShape>,
    ids: Vec<b::CommitId>,
    evaluations: usize,
    budget: &Budget,
) -> Result<()> {
    let shape = |id: &b::CommitId| shapes.get(id).map(|c| c.shape).ok_or(Error::Incomplete);
    if ids.len() < 2 {
        return Ok(());
    }
    let start = shape(&ids[0])?;
    let mut stack = vec![Frame {
        ids,
        pos: 1,
        shape: start,
    }];
    loop {
        budget.check()?;
        let top = stack.last_mut().ok_or(Error::Unavailable)?;
        if top.pos < top.ids.len() {
            let cost = shapes
                .len()
                .max(1)
                .checked_mul(top.pos + 1)
                .and_then(|n| n.checked_mul(evaluations))
                .ok_or(Error::Limit)?;
            budget.work(cost)?;
            let ancestors = budget
                .until(
                    repo.index()
                        .common_ancestors(&top.ids[..top.pos], &top.ids[top.pos..top.pos + 1]),
                )
                .await?
                .map_err(|_| Error::Incomplete)?;
            let ancestor = match ancestors.len() {
                0 => shape(repo.store().root_commit_id())?,
                1 => shape(&ancestors[0])?,
                _ => {
                    let first = shape(&ancestors[0])?;
                    // The stack cannot exceed the admitted work and index size.
                    stack.push(Frame {
                        ids: ancestors,
                        pos: 1,
                        shape: first,
                    });
                    continue;
                }
            };
            top.shape = top
                .shape
                .add(ancestor, budget)?
                .add(shape(&top.ids[top.pos])?, budget)?;
            top.pos += 1;
        } else {
            let completed = stack.pop().ok_or(Error::Unavailable)?.shape;
            // Reserve the native tree/label simplification comparison work.
            budget.work(
                completed
                    .trees
                    .checked_mul(completed.trees)
                    .and_then(|n| n.checked_mul(evaluations))
                    .ok_or(Error::Limit)?,
            )?;
            let Some(parent) = stack.last_mut() else {
                return Ok(());
            };
            parent.shape = parent
                .shape
                .add(completed, budget)?
                .add(shape(&parent.ids[parent.pos])?, budget)?;
            parent.pos += 1;
        }
    }
}
pub(super) async fn preflight(
    repo: &ReadonlyRepo,
    expression: &UserRevsetExpression,
    access: &Arc<dyn ReadAccess>,
    budget: &Budget,
) -> Result<()> {
    let filters = count(expression, budget)?;
    if filters == 0 {
        return Ok(());
    }
    let mut shapes = BTreeMap::new();
    for key in access.inventory() {
        let m::ObjectKey::Commit(id) = key else {
            continue;
        };
        budget.authorize(access.as_ref()).await?;
        let id = b::CommitId::from_bytes(id.as_bytes());
        if !budget
            .until(repo.index().has_id(&id))
            .await?
            .map_err(|_| Error::Incomplete)?
        {
            continue;
        }
        let commit = repo
            .store()
            .get_commit_async(&id)
            .await
            .map_err(|_| Error::Unavailable)?;
        let native = commit.store_commit();
        // Native code constructs the commit label even when existing conflict
        // labels later replace it. Reserve both temporary and retained strings.
        let label_bytes = native.conflict_labels.iter().try_fold(
            native
                .description
                .len()
                .checked_add(256)
                .ok_or(Error::Limit)?,
            |sum, label| sum.checked_add(label.len()).ok_or(Error::Limit),
        )?;
        let shape = Shape {
            terms: 1,
            trees: native.root_tree.iter().len(),
            label_bytes,
        }
        .check(budget)?;
        shapes.insert(
            id,
            CommitShape {
                parents: commit.parent_ids().to_vec(),
                shape,
            },
        );
    }
    for commit in shapes.values() {
        merge_shape(repo, &shapes, commit.parents.clone(), filters + 1, budget).await?;
    }
    Ok(())
}
