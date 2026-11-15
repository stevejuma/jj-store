//! Existing checkout/ignore witnesses only; no WC loader, initialization or mutation.
pub(crate) mod capture;
pub(crate) mod decode;
pub(crate) mod physical;
use super::*;
use std::path::PathBuf;
pub const MAX_CHECKOUT_BYTES: usize = 4 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkout {
    pub operation: m::OperationId,
    pub workspace: String,
    pub trees: Vec<m::GitSha1Id>,
    pub labels: Vec<String>,
    pub sparse: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CurrentState {
    pub baseline: Baseline,
    pub checkout: Checkout,
    pub physical_workspace: m::RecordDigest,
    pub fingerprint: m::RecordDigest,
    pub base_ignores: Vec<Vec<u8>>,
    pub heads: Vec<m::OperationId>,
}
impl Executor {
    /// Stable directory identity for historical recovery admission. The caller
    /// supplies the registered canonical root; current code and file state do
    /// not participate. This grants no journal access or command authority.
    pub async fn physical_workspace_identity(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
    ) -> Result<m::RecordDigest> {
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                if root.as_os_str().as_encoded_bytes().len() > 4096 {
                    return Err(Error::Limit);
                }
                let sample = || {
                    budget.work(root.components().count())?;
                    let dir = physical::directory(&root)?;
                    let jj = physical::child(&dir, ".jj")?;
                    physical::workspace(&root, &dir, &jj)
                };
                let before = sample()?;
                budget.until(access.authorize()).await??;
                if before != sample()? {
                    return Err(Error::EpochChanged);
                }
                Ok(before)
            })
        })
        .await
    }

    /// Root comes from verified device registration, never a client folder argument.
    pub async fn working_tree_current(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        operation: m::OperationId,
    ) -> Result<CurrentState> {
        self.working_tree_witness(access, root, operation, false)
            .await
    }
    /// Status-only witness permits metadata descendants that retain the exact
    /// checked-out WC commit. It never grants a mutation/checkout precondition.
    pub async fn working_tree_observed(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        operation: m::OperationId,
    ) -> Result<CurrentState> {
        self.working_tree_witness(access, root, operation, true)
            .await
    }
    async fn working_tree_witness(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        operation: m::OperationId,
        metadata_descendant: bool,
    ) -> Result<CurrentState> {
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                let first = capture::CheckoutFiles::read(&root, access.metadata_path(), &budget)?;
                let checkout = decode::checkout(&first.checkout.bytes, &first.tree.bytes)?;
                if checkout.operation != operation && !metadata_descendant {
                    return Err(Error::EpochChanged);
                }
                let before_heads = heads(&access, &budget).await?;
                if !before_heads.contains(&operation) {
                    return Err(Error::EpochChanged);
                }
                if checkout.operation != operation && before_heads.as_slice() != [operation] {
                    return Err(Error::EpochChanged);
                }
                let context = Context::open(
                    access.clone(),
                    operation,
                    budget.clone(),
                    limits,
                    true,
                    vec![],
                    false,
                )
                .await?;
                let before_policy = capture::policy(&root, access.metadata_path(), &budget)?;
                let baseline = read(&context, operation, checkout.workspace.clone()).await?;
                if checkout.operation != operation {
                    same_checkout(&context, &checkout, &baseline).await?;
                }
                if baseline.trees != checkout.trees || baseline.labels != checkout.labels {
                    return Err(Error::EpochChanged);
                }
                budget.authorize(&context.access).await?;
                let after = capture::CheckoutFiles::read(&root, access.metadata_path(), &budget)?;
                let after_policy = capture::policy(&root, access.metadata_path(), &budget)?;
                let after_heads = heads(&access, &budget).await?;
                if first != after || before_policy != after_policy || before_heads != after_heads {
                    return Err(Error::EpochChanged);
                }
                let fingerprint = fingerprint(&access, &first, &before_policy, &before_heads);
                Ok(CurrentState {
                    baseline,
                    checkout,
                    physical_workspace: first.workspace,
                    fingerprint,
                    base_ignores: before_policy.inputs,
                    heads: before_heads,
                })
            })
        })
        .await
    }
}
async fn same_checkout(context: &Context, checkout: &Checkout, baseline: &Baseline) -> Result<()> {
    // Context::open proves membership in current operation ancestry under the
    // shared work/metadata budget. The caller requires one unchanged selected
    // head, so this cannot admit a checkout from an unrelated sibling.
    let original = Context::open(
        context.access.binding.clone(),
        checkout.operation,
        context.budget.clone(),
        context.limits,
        true,
        vec![],
        false,
    )
    .await?;
    let original = read(&original, checkout.operation, checkout.workspace.clone()).await?;
    if original.commit != baseline.commit
        || original.trees != baseline.trees
        || original.labels != baseline.labels
    {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
pub(crate) fn fingerprint(
    access: &Arc<dyn Access>,
    files: &capture::CheckoutFiles,
    policy: &capture::Policy,
    heads: &[m::OperationId],
) -> m::RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/current-working-tree/v1\0");
    hash.update(access.repository().as_bytes());
    hash.update(access.generation().to_be_bytes());
    hash.update(files.workspace.as_bytes());
    for digest in [
        &files.checkout.fingerprint,
        &files.tree.fingerprint,
        &files.kind.fingerprint,
        &files.binding,
        &policy.fingerprint,
    ] {
        hash.update(digest.as_bytes());
    }
    for head in heads {
        hash.update(head.as_bytes());
    }
    m::RecordDigest::new(hash.finalize().into())
}
async fn heads(access: &Arc<dyn Access>, budget: &Budget) -> Result<Vec<m::OperationId>> {
    budget.until(access.authorize()).await??;
    let heads = loader(access.metadata_path())?
        .op_heads_store()
        .get_op_heads()
        .await
        .map_err(|_| Error::Unavailable)?;
    budget.work(heads.len())?;
    if heads.is_empty() || heads.len() > 64 {
        return Err(Error::Limit);
    }
    let mut heads = heads.iter().map(operation_id).collect::<Result<Vec<_>>>()?;
    heads.sort();
    Ok(heads)
}
