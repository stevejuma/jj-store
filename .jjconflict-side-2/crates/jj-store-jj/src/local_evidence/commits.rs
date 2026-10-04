use super::*;
use crate::native_dto::SecureSig;
use jj_lib::backend as b;
impl TryFrom<&b::Commit> for GitCommitV1 {
    type Error = Error;
    fn try_from(c: &b::Commit) -> Result<Self> {
        Ok(Self {
            parents: c.parents.iter().map(git_commit_id).collect::<Result<_>>()?,
            predecessors: c
                .predecessors
                .iter()
                .map(git_commit_id)
                .collect::<Result<_>>()?,
            root_tree: c.root_tree.iter().map(git_tree_id).collect::<Result<_>>()?,
            conflict_labels: c.conflict_labels.iter().cloned().collect(),
            change_id: change_id(&c.change_id)?,
            description: c.description.clone(),
            author: (&c.author).into(),
            committer: (&c.committer).into(),
            secure_sig: c.secure_sig.as_ref().map(|s| SecureSig {
                data: s.data.clone(),
                sig: s.sig.clone(),
            }),
        })
    }
}
impl TryFrom<GitCommitV1> for b::Commit {
    type Error = Error;
    fn try_from(c: GitCommitV1) -> Result<Self> {
        if c.root_tree.len() != c.conflict_labels.len()
            || (c.root_tree.len() == 1 && !c.conflict_labels[0].is_empty())
        {
            return Err(Error::Invalid(
                "conflict labels must match tree arity; resolved label must be empty",
            ));
        }
        Ok(Self {
            parents: c.parents.into_iter().map(git_native_commit_id).collect(),
            predecessors: c
                .predecessors
                .into_iter()
                .map(git_native_commit_id)
                .collect(),
            root_tree: merge(c.root_tree.into_iter().map(git_native_tree_id).collect())?,
            conflict_labels: merge(c.conflict_labels)?,
            change_id: native_change_id(c.change_id),
            description: c.description,
            author: c.author.into(),
            committer: c.committer.into(),
            secure_sig: c.secure_sig.map(|s| b::SecureSig {
                data: s.data,
                sig: s.sig,
            }),
        })
    }
}
