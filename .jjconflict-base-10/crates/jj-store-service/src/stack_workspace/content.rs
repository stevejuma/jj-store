use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackContentSource")]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContentSource {
    WorkingCopy,
    Commit {
        commit: CommitIdentity,
    },
    /// Exact net tree comparison; never a union of per-commit patches.
    Branch {
        stack: StackId,
        name: String,
        base: CommitIdentity,
        head: CommitIdentity,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackSelectionScope")]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SelectionScope {
    WholeFile,
    Hunks {
        diff_version: u32,
        hunks: Vec<m::RecordDigest>,
    },
}
/// Server-owned retained claims. Accepting an incoming copy never validates the
/// token; providers must look up issued claims under current authorization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackExactContentSelection")]
#[serde(deny_unknown_fields)]
pub struct ExactSelection {
    pub version: u32,
    pub token: SelectionToken,
    pub selection: Selection,
    pub source: ContentSource,
    pub source_tree: TreeSelection,
    pub parent_tree: TreeSelection,
    pub path: String,
    /// Includes existence, object contents, mode and rename/copy identity.
    pub file_state: m::RecordDigest,
    pub scope: SelectionScope,
}
impl ExactSelection {
    pub fn validate(&self, current: &Selection) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.selection.check_current(current)?;
        validate_path(&self.path)?;
        self.source_tree.validate(&self.selection.code)?;
        self.parent_tree.validate(&self.selection.code)?;
        match &self.source {
            ContentSource::WorkingCopy => (),
            ContentSource::Commit { commit } => self.selection.code.commit(*commit)?,
            ContentSource::Branch {
                name, base, head, ..
            } => {
                text(name, 1024)?;
                self.selection.code.commit(*base)?;
                self.selection.code.commit(*head)?;
            }
        }
        if let SelectionScope::Hunks {
            diff_version,
            hunks,
        } = &self.scope
        {
            if *diff_version != VERSION
                || hunks.is_empty()
                || hunks
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    != hunks.len()
            {
                return Err(Error::Invalid);
            }
            if hunks.len() > MAX_SELECTIONS {
                return Err(Error::Limit);
            }
        }
        Ok(())
    }
    pub fn fingerprint(&self) -> Result<m::RecordDigest> {
        self.validate(&self.selection)?;
        digest(b"jj-store/stack-content-selection/v1\0", self)
    }
}
pub(super) fn validate_path(value: &str) -> Result<()> {
    text(value, 4096)?;
    if value.contains('\\')
        || value.split('/').any(|part| {
            part.is_empty()
                || matches!(part, "." | "..")
                || part.eq_ignore_ascii_case(".jj")
                || part.eq_ignore_ascii_case(".git")
        })
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
