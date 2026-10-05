//! Exact retained tree effects, physical preflight and provider-owned installation.
pub(super) mod completion;
mod effects;
mod installation;
mod observation;
mod plan;
use super::*;
use crate::local_browser::working_tree::{self as tree, current::decode};
pub use completion::VerifiedTreeCheckout;
pub use installation::InstalledTreeFiles;
pub use plan::TreeCheckoutPlanV1;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

impl RecoveredHistory {
    /// Capture only. Composition must durably select this exact plan before any
    /// future physical installation. No publication or completion proof is made.
    pub async fn prepare_tree_checkout(
        self,
        physical: Arc<dyn PhysicalGuard>,
        expected: m::RecordDigest,
    ) -> Result<(Self, TreeCheckoutPlanV1)> {
        self.tree_checkout(None, physical, expected).await
    }
    /// Original-request data revalidation before installation; cannot accept
    /// changed effects, stamps or a replacement preparation under an old digest.
    pub async fn revalidate_tree_checkout(
        self,
        plan: TreeCheckoutPlanV1,
        physical: Arc<dyn PhysicalGuard>,
    ) -> Result<(Self, TreeCheckoutPlanV1)> {
        let expected = plan.physical();
        self.tree_checkout(Some(plan), physical, expected).await
    }
    async fn tree_checkout(
        self,
        saved: Option<TreeCheckoutPlanV1>,
        physical: Arc<dyn PhysicalGuard>,
        expected: m::RecordDigest,
    ) -> Result<(Self, TreeCheckoutPlanV1)> {
        if self.checkout.is_some() || expected == m::RecordDigest::new([0; 32]) {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                check(&self, &physical, expected, &budget).await?;
                let plan = plan::capture(&self, expected, &budget)?;
                if let Some(saved) = saved {
                    saved.encode()?;
                    if saved != plan {
                        return Err(Error::EpochChanged);
                    }
                }
                // Capture all per-path and ancestor identities twice, in
                // addition to the complete composition-owned physical guard.
                observation::validate(&self.root, &plan, &budget)?;
                check(&self, &physical, expected, &budget).await?;
                Ok((self, plan))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
async fn check(
    recovery: &RecoveredHistory,
    physical: &Arc<dyn PhysicalGuard>,
    expected: m::RecordDigest,
    budget: &Budget,
) -> Result<()> {
    recovery.check(budget).await?;
    budget
        .until(physical.validate(&recovery.witness.binding(), expected))
        .await??;
    recovery.check(budget).await
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileValue {
    object: m::GitSha1Id,
    content: m::RecordDigest,
    size: u64,
    executable: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Change {
    path: String,
    before: Option<FileValue>,
    after: Option<FileValue>,
    observed: Option<m::RecordDigest>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Directory {
    path: String,
    identity: Option<m::RecordDigest>,
}
fn safe_path(path: &str) -> Result<()> {
    tree::safe_path(path)?;
    // This first installation profile avoids Unicode normalization and
    // case-folding aliases until filesystem-specific collision proof exists.
    if !path.is_ascii()
        || path.split('/').any(|name| {
            name.starts_with(".jjconflict")
                || name == jj_lib::git_backend::JJ_CONFLICT_README_FILE_NAME
        })
        || path.len() > 4096
        || path.split('/').count() > 64
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
