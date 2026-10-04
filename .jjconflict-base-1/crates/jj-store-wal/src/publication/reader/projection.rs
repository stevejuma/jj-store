//! Data-only identities and copy edges, retained at first descriptor admission.
use super::*;
use crate::catalog::ScanBudget;
use jj_store_model::CopyId;

#[derive(Clone, Copy)]
pub struct InventoryProjectionLimits {
    pub descriptors: usize,
    pub copy_edges: usize,
}
impl Default for InventoryProjectionLimits {
    fn default() -> Self {
        Self {
            descriptors: 256,
            copy_edges: 65_536,
        }
    }
}
impl InventoryProjectionLimits {
    fn validate(self) -> Result<()> {
        if !(1..=256).contains(&self.descriptors) || !(1..=65_536).contains(&self.copy_edges) {
            return Err(Error::Limit("inventory projection window"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyParents {
    pub id: CopyId,
    pub parents: BTreeSet<CopyId>,
}
pub struct InventoryProjectionPage {
    pub operations: Vec<OperationId>,
    pub copies: Vec<CopyParents>,
    /// Processed descriptors, including unrelated kinds discarded after reading.
    pub scanned: usize,
    pub next: Option<RecordDigest>,
    /// Charged record attempts, including an admission-limited lookahead.
    pub record_reads: usize,
    /// Encoded bytes from successfully admitted catalog and descriptor records.
    pub record_bytes: u64,
}

pub(in crate::publication) async fn inventory_projection_page(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    root: Option<RecordRef>,
    after: Option<RecordDigest>,
    limits: InventoryProjectionLimits,
) -> Result<InventoryProjectionPage> {
    limits.validate()?;
    if root.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let mut paths = ScanBudget::new(768, 8 * 1024 * 1024);
    let page = Catalog::new(records.clone())
        .page(root, after, limits.descriptors, &mut paths)
        .await?;
    window(&records, page, paths, limits).await
}
pub(in crate::publication) async fn inventory_projection_additions_page(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    before: Option<RecordRef>,
    after: Option<RecordRef>,
    anchor: Option<RecordDigest>,
    limits: InventoryProjectionLimits,
) -> Result<InventoryProjectionPage> {
    limits.validate()?;
    if before.is_none() || after.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let mut paths = ScanBudget::new(768, 8 * 1024 * 1024);
    let page = Catalog::new(records.clone())
        .additions_page(before, after, anchor, limits.descriptors, &mut paths)
        .await?;
    window(&records, page, paths, limits).await
}
fn finish(
    mut out: InventoryProjectionPage,
    budget: &ScanBudget,
    descriptor_bytes: u64,
    next: Option<RecordDigest>,
) -> InventoryProjectionPage {
    out.next = next;
    out.record_reads += 256 - budget.remaining_reads();
    out.record_bytes += descriptor_bytes - budget.remaining_bytes();
    out
}
async fn window(
    records: &Records,
    page: crate::catalog::Page,
    paths: ScanBudget,
    limits: InventoryProjectionLimits,
) -> Result<InventoryProjectionPage> {
    let path_bytes = 8 * 1024 * 1024 - paths.remaining_bytes();
    let descriptor_bytes = 32 * 1024 * 1024 - path_bytes;
    let mut budget = ScanBudget::new(256, descriptor_bytes);
    let mut out = InventoryProjectionPage {
        operations: Vec::new(),
        copies: Vec::new(),
        scanned: 0,
        next: None,
        record_reads: 768 - paths.remaining_reads(),
        record_bytes: path_bytes,
    };
    let mut edges = 0usize;
    let mut last = None;
    for (key, reference) in page.entries {
        let node: VerifiedNode = match budget.get(records, reference, RecordKind::Inventory).await {
            Ok(node) => node,
            Err(error) if ScanBudget::admission(&error) && out.scanned != 0 => {
                return Ok(finish(out, &budget, descriptor_bytes, last));
            }
            Err(error) => return Err(error),
        };
        if object_key(node.key) != key {
            return Err(Error::Corrupt("inventory catalog key"));
        }
        match node.key {
            ObjectKey::Operation(id) => out.operations.push(id),
            ObjectKey::Copy(id) => {
                if node
                    .dependencies
                    .iter()
                    .any(|key| !matches!(key, ObjectKey::Copy(_)))
                {
                    return Err(Error::Corrupt("copy inventory dependency kind"));
                }
                if node.dependencies.len() > limits.copy_edges - edges {
                    if out.scanned == 0 {
                        return Err(Error::Limit("inventory projection edges"));
                    }
                    return Ok(finish(out, &budget, descriptor_bytes, last));
                }
                edges += node.dependencies.len();
                let parents = node
                    .dependencies
                    .into_iter()
                    .map(|key| match key {
                        ObjectKey::Copy(id) => id,
                        _ => unreachable!("checked copy dependency kind"),
                    })
                    .collect();
                out.copies.push(CopyParents { id, parents });
            }
            _ => {}
        }
        out.scanned += 1;
        last = Some(key);
    }
    Ok(finish(out, &budget, descriptor_bytes, page.next))
}
