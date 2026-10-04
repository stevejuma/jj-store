use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackBookmarkTarget")]
#[serde(deny_unknown_fields)]
pub struct BookmarkTarget {
    pub name: String,
    pub remote: Option<String>,
    pub head: CommitIdentity,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPlacement")]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    Above,
    Below,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPlacementBoundary")]
#[serde(deny_unknown_fields)]
pub struct Boundary {
    pub anchor: CommitIdentity,
    pub placement: Placement,
    /// Exact selected graph parent boundary, not inferred from list position.
    pub parents: Vec<CommitIdentity>,
}
impl Boundary {
    fn validate(&self, code: &CodeSelection) -> Result<()> {
        code.commit(self.anchor)?;
        if self.parents.is_empty() || self.parents.len() > 64 {
            return Err(Error::Invalid);
        }
        for parent in &self.parents {
            code.commit(*parent)?;
        }
        if self
            .parents
            .iter()
            .enumerate()
            .any(|(i, p)| self.parents[..i].contains(p))
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackWorkspaceCommand")]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    Setup {
        target: BookmarkTarget,
        name: String,
    },
    CreateCombination {
        name: String,
    },
    SwitchCombination {
        combination: CombinationId,
    },
    DeleteCombination {
        combination: CombinationId,
    },
    ApplyStack {
        stack: StackId,
        head: CommitIdentity,
    },
    UnapplyStack {
        stack: StackId,
    },
    CreateStack {
        name: String,
        parent: Option<CommitIdentity>,
    },
    RenameStack {
        stack: StackId,
        name: String,
    },
    RenameBookmark {
        target: BookmarkTarget,
        name: String,
    },
    DeleteBookmark {
        target: BookmarkTarget,
    },
    /// Abandonment is distinct from removing a bookmark. Providers calculate and
    /// qualify the exact rewrite closure before offering the preview.
    AbandonSegment {
        stack: StackId,
        head: CommitIdentity,
        base: CommitIdentity,
    },
    AddEmptyCommit {
        boundary: Boundary,
        message: String,
    },
    CreateBranch {
        boundary: Boundary,
        name: String,
    },
    AbandonCommit {
        commit: CommitIdentity,
    },
    Reword {
        commit: CommitIdentity,
        message: String,
    },
    CreateCommit {
        stack: StackId,
        destination: CommitIdentity,
        message: String,
        selections: Vec<SelectionToken>,
    },
    EditCommit {
        commit: CommitIdentity,
        message: String,
        selections: Vec<SelectionToken>,
    },
    MoveCommit {
        commit: CommitIdentity,
        destination: CommitIdentity,
    },
    ReorderCommit {
        commit: CommitIdentity,
        boundary: Boundary,
    },
    Squash {
        commit: CommitIdentity,
        destination: CommitIdentity,
    },
    Split {
        commit: CommitIdentity,
        message: String,
        selections: Vec<SelectionToken>,
    },
    Transfer {
        selections: Vec<SelectionToken>,
        destination: ContentSource,
    },
    Uncommit {
        commit: CommitIdentity,
    },
    Rebase {
        stack: StackId,
        destination: CommitIdentity,
    },
    ResolveConflict {
        path: String,
        fingerprint: m::ConflictFingerprint,
        upload: m::ConflictUploadId,
    },
    Undo {
        request_id: m::RequestId,
        operation: m::OperationId,
    },
}
impl Command {
    pub fn validate(&self, selection: &Selection) -> Result<()> {
        selection.validate()?;
        if selection.combination.is_none() && !matches!(self, Self::Setup { .. }) {
            return Err(Error::Invalid);
        }
        if selection.combination.is_some() && matches!(self, Self::Setup { .. }) {
            return Err(Error::Invalid);
        }
        let code = &selection.code;
        match self {
            Self::Setup { target, name } => {
                target.validate(code)?;
                text(name, 256)?;
            }
            Self::CreateCombination { name } | Self::RenameStack { name, .. } => text(name, 256)?,
            Self::SwitchCombination { .. }
            | Self::DeleteCombination { .. }
            | Self::UnapplyStack { .. }
            | Self::Undo { .. } => (),
            Self::CreateStack { name, parent } => {
                text(name, 1024)?;
                if let Some(parent) = parent {
                    code.commit(*parent)?;
                }
            }
            Self::RenameBookmark { target, name } => {
                target.validate(code)?;
                text(name, 1024)?;
            }
            Self::DeleteBookmark { target } => target.validate(code)?,
            Self::AbandonSegment { head, base, .. } => {
                code.commit(*head)?;
                code.commit(*base)?;
                if head == base {
                    return Err(Error::Invalid);
                }
            }
            Self::CreateBranch { boundary, name } => {
                boundary.validate(code)?;
                text(name, 1024)?;
            }
            Self::AddEmptyCommit { boundary, message } => {
                boundary.validate(code)?;
                message_limit(message)?;
            }
            Self::AbandonCommit { commit } | Self::Uncommit { commit } => code.commit(*commit)?,
            Self::Reword { commit, message } => {
                code.commit(*commit)?;
                message_limit(message)?;
            }
            Self::ApplyStack { head, .. } => code.commit(*head)?,
            Self::Rebase { destination, .. } => code.commit(*destination)?,
            Self::MoveCommit {
                commit,
                destination,
            }
            | Self::Squash {
                commit,
                destination,
            } => {
                code.commit(*commit)?;
                code.commit(*destination)?;
                if commit == destination {
                    return Err(Error::Invalid);
                }
            }
            Self::ReorderCommit { commit, boundary } => {
                code.commit(*commit)?;
                boundary.validate(code)?;
                if *commit == boundary.anchor {
                    return Err(Error::Invalid);
                }
            }
            Self::CreateCommit {
                destination,
                message,
                selections,
                ..
            } => {
                code.commit(*destination)?;
                message_limit(message)?;
                tokens(selections, false)?;
            }
            Self::EditCommit {
                commit,
                message,
                selections,
            } => {
                code.commit(*commit)?;
                message_limit(message)?;
                tokens(selections, true)?;
            }
            Self::Split {
                commit,
                message,
                selections,
            } => {
                code.commit(*commit)?;
                message_limit(message)?;
                tokens(selections, false)?;
            }
            Self::Transfer {
                selections,
                destination,
            } => {
                tokens(selections, false)?;
                match destination {
                    ContentSource::WorkingCopy => (),
                    ContentSource::Commit { commit } => code.commit(*commit)?,
                    // Aggregate reads do not imply a unique mutation destination.
                    ContentSource::Branch { .. } => return Err(Error::Invalid),
                }
            }
            Self::ResolveConflict { path, .. } => {
                // The provider additionally checks native ordered conflict terms.
                super::content::validate_path(path)?;
            }
        }
        Ok(())
    }
}
impl BookmarkTarget {
    fn validate(&self, code: &CodeSelection) -> Result<()> {
        text(&self.name, 1024)?;
        if let Some(remote) = &self.remote {
            text(remote, 256)?;
        }
        code.commit(self.head)
    }
}
fn message_limit(value: &str) -> Result<()> {
    if value.contains('\0') {
        return Err(Error::Invalid);
    }
    if value.len() > 64 * 1024 {
        return Err(Error::Limit);
    }
    Ok(())
}
fn tokens(values: &[SelectionToken], allow_empty: bool) -> Result<()> {
    if values.len() > MAX_SELECTIONS {
        return Err(Error::Limit);
    }
    if (!allow_empty && values.is_empty())
        || values
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != values.len()
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
