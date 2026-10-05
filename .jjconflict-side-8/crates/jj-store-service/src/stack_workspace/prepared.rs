use super::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCommitRewrite")]
#[serde(deny_unknown_fields)]
pub struct Rewrite {
    pub change: m::ChangeId,
    pub before: Vec<CommitIdentity>,
    pub after: Vec<CommitIdentity>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackBookmarkEffect")]
#[serde(deny_unknown_fields)]
pub struct BookmarkEffect {
    pub name: String,
    pub before: Vec<Option<CommitIdentity>>,
    pub after: Vec<Option<CommitIdentity>>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackConflictEffect")]
#[serde(deny_unknown_fields)]
pub struct ConflictEffect {
    pub path: String,
    pub fingerprint: m::ConflictFingerprint,
    /// Content and ordered native terms use the existing conflict read contract.
    pub term_count: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackCheckoutEffect")]
#[serde(deny_unknown_fields)]
pub struct CheckoutEffect {
    pub before: TreeSelection,
    pub after: TreeSelection,
    pub paths: Vec<String>,
}
/// Exact server preparation, not an executable client proposal. The provider
/// retains the unpublished operation and all objects referenced by this report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPreparedPlan")]
#[serde(deny_unknown_fields)]
pub struct PreparedPlan {
    pub version: u32,
    pub request: PrepareRequest,
    pub operation: m::OperationId,
    pub view: m::ViewId,
    pub checkout: CheckoutEffect,
    pub combination: CombinationSelection,
    pub rewrites: Vec<Rewrite>,
    pub bookmarks: Vec<BookmarkEffect>,
    pub conflicts: Vec<ConflictEffect>,
}
impl PreparedPlan {
    pub fn validate(&self) -> Result<()> {
        if self.version != VERSION {
            return Err(Error::Invalid);
        }
        self.request.validate()?;
        let code = &self.request.selection.code;
        self.checkout.before.validate(code)?;
        self.checkout.after.validate(code)?;
        if self.checkout.paths.len() > 4096
            || self.rewrites.len() > 4096
            || self.bookmarks.len() > 1024
            || self.conflicts.len() > 256
        {
            return Err(Error::Limit);
        }
        for path in &self.checkout.paths {
            super::content::validate_path(path)?;
        }
        if self
            .checkout
            .paths
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != self.checkout.paths.len()
        {
            return Err(Error::Invalid);
        }
        if self
            .rewrites
            .iter()
            .map(|item| item.change)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != self.rewrites.len()
        {
            return Err(Error::Invalid);
        }
        for rewrite in &self.rewrites {
            if rewrite.before.is_empty() || rewrite.before.len() > 64 || rewrite.after.len() > 64 {
                return Err(Error::Invalid);
            }
            for commit in rewrite.before.iter().chain(&rewrite.after) {
                code.commit(*commit)?;
            }
        }
        for bookmark in &self.bookmarks {
            text(&bookmark.name, 1024)?;
            if bookmark.before.len() > 65 || bookmark.after.len() > 65 {
                return Err(Error::Limit);
            }
            for commit in bookmark.before.iter().chain(&bookmark.after).flatten() {
                code.commit(*commit)?;
            }
        }
        for conflict in &self.conflicts {
            super::content::validate_path(&conflict.path)?;
            if conflict.term_count < 3 || conflict.term_count > 65 || conflict.term_count % 2 == 0 {
                return Err(Error::Invalid);
            }
        }
        Ok(())
    }
    pub fn identity(&self) -> Result<PreparedIdentity> {
        self.validate()?;
        Ok(PreparedIdentity(digest(
            b"jj-store/stack-prepared-plan/v1\0",
            self,
        )?))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "StackPreparedPreview")]
#[serde(deny_unknown_fields)]
pub struct Preview {
    pub prepared: PreparedIdentity,
    pub plan: PreparedPlan,
}
impl Preview {
    pub fn validate(&self) -> Result<()> {
        if self.plan.identity()? != self.prepared {
            return Err(Error::IntentMismatch);
        }
        Ok(())
    }
}
impl Receipt {
    /// Exact output verification is separate from current-state validation. A
    /// historical receipt must remain recoverable after outside edits or undo.
    pub fn validate_prepared(&self, preview: &Preview) -> Result<()> {
        preview.validate()?;
        self.validate(&preview.plan.request)?;
        if self.prepared != Some(preview.prepared) {
            return Err(Error::IntentMismatch);
        }
        match &self.state {
            ReceiptState::Published {
                operation, view, ..
            } if *operation != preview.plan.operation || *view != preview.plan.view => {
                Err(Error::IntentMismatch)
            }
            ReceiptState::Retired { operation, .. } if *operation != preview.plan.operation => {
                Err(Error::IntentMismatch)
            }
            _ => Ok(()),
        }
    }
}
