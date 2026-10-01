//! Bounded native repository copy without bucket listing or alternate authority.
use super::*;
use crate::{
    catalog::CatalogClosure,
    records::{MAX_RECORD_BYTES, RecordKind},
};

#[derive(Clone, Debug)]
pub struct NativeCopyOutcome {
    pub snapshot: ManifestSnapshot,
    pub native_objects: usize,
    pub control_records: usize,
    pub copied_bytes: u64,
}

struct NativeCopyPlan {
    snapshot: ManifestSnapshot,
    native: Vec<VerifiedNode>,
    control: BTreeSet<RecordRef>,
}

impl Coordinator {
    /// Copy one verified manifest-selected repository closure to an empty or
    /// byte-identical store. Every immutable dependency is installed before the
    /// exact manifest, and destination recovery verifies all three roots.
    pub async fn copy_native_to(
        &self,
        destination: Arc<dyn ObjectStore>,
        destination_source: Arc<dyn DependencySource>,
        max_entries: usize,
    ) -> Result<NativeCopyOutcome> {
        tokio::time::timeout(self.limits.timeout, async {
            let plan = self.native_copy_plan(max_entries).await?;
            let mut remaining = self.limits.inventory.max_new_bytes;
            let mut copied_bytes = 0u64;
            for node in &plan.native {
                let Representation::Stored { bytes, .. } = node.representation else {
                    continue;
                };
                let key = Key::new(node.key.storage_path(self.records.repository))?;
                copied_bytes = copied_bytes
                    .checked_add(
                        copy_exact(
                            &self.records.store,
                            &destination,
                            &key,
                            bytes,
                            &mut remaining,
                        )
                        .await?,
                    )
                    .ok_or(Error::Limit("native copy bytes"))?;
            }
            for reference in &plan.control {
                let key = reference.key(self.records.repository)?;
                copied_bytes = copied_bytes
                    .checked_add(
                        copy_exact(
                            &self.records.store,
                            &destination,
                            &key,
                            MAX_RECORD_BYTES,
                            &mut remaining,
                        )
                        .await?,
                    )
                    .ok_or(Error::Limit("native copy bytes"))?;
            }
            let manifest_bytes = records::encode(plan.snapshot.manifest(), MAX_MANIFEST)?;
            if records::digest(&manifest_bytes) != plan.snapshot.digest() {
                return Err(Error::Corrupt("native copy manifest digest"));
            }
            if manifest_bytes.len() as u64 > remaining {
                return Err(Error::Limit("native copy bytes"));
            }
            put_exact(
                &destination,
                &self.manifest_key(),
                &manifest_bytes,
                MAX_MANIFEST,
            )
            .await?;
            copied_bytes = copied_bytes
                .checked_add(manifest_bytes.len() as u64)
                .ok_or(Error::Limit("native copy bytes"))?;

            let restored = Coordinator::new_with_head_reducer(
                destination,
                self.records.repository,
                destination_source,
                self.guard.clone(),
                self.operation_heads.clone(),
                self.limits,
            )?;
            let snapshot = restored.recover(max_entries).await?;
            if snapshot.digest() != plan.snapshot.digest()
                || snapshot.manifest() != plan.snapshot.manifest()
            {
                return Err(Error::Corrupt("native copy selected snapshot"));
            }
            Ok(NativeCopyOutcome {
                snapshot,
                native_objects: plan
                    .native
                    .iter()
                    .filter(|node| matches!(node.representation, Representation::Stored { .. }))
                    .count(),
                control_records: plan.control.len(),
                copied_bytes,
            })
        })
        .await
        .map_err(|_| Error::Limit("native copy time"))?
    }

    async fn native_copy_plan(&self, max_entries: usize) -> Result<NativeCopyPlan> {
        let snapshot = self.recover(max_entries).await?;
        let native = self.inventory(&snapshot, max_entries).await?;
        let manifest = snapshot.manifest();
        let mut control = BTreeSet::new();
        include_manifest_records(manifest, &mut control);
        self.include_catalog(manifest.inventory, max_entries, &mut control)
            .await?;
        self.include_catalog(
            manifest.receipts,
            max_entries.saturating_mul(2),
            &mut control,
        )
        .await?;
        self.include_catalog(manifest.history, max_entries, &mut control)
            .await?;
        self.include_domains(manifest.domains(), &mut control)
            .await?;
        self.include_replication(manifest.replication_root, max_entries, &mut control)
            .await?;

        let mut after = WalPosition(0);
        while after < manifest.published {
            for transaction in self.committed(&snapshot, after, 1024).await? {
                control.insert(transaction.receipt);
                control.insert(transaction.settings);
                for origin in transaction.origins {
                    control.insert(origin);
                    let receipt: OriginReceiptV1 =
                        self.records.get(origin, RecordKind::OriginReceipt).await?;
                    control.insert(receipt.envelope);
                }
                control.extend(transaction.previous);
                self.include_catalog(transaction.inventory, max_entries, &mut control)
                    .await?;
                self.include_domains(transaction.before_domains, &mut control)
                    .await?;
                self.include_domains(transaction.after_domains, &mut control)
                    .await?;
                after = transaction.after.position;
            }
        }

        let mut checkpoint = manifest.checkpoint;
        for _ in 0..=max_entries {
            let Some(reference) = checkpoint else {
                return Ok(NativeCopyPlan {
                    snapshot,
                    native,
                    control,
                });
            };
            control.insert(reference);
            let value: Checkpoint = self.records.get(reference, RecordKind::Checkpoint).await?;
            include_manifest_records(&value.captured, &mut control);
            self.include_catalog(value.captured.inventory, max_entries, &mut control)
                .await?;
            self.include_catalog(
                value.captured.receipts,
                max_entries.saturating_mul(2),
                &mut control,
            )
            .await?;
            self.include_catalog(value.captured.history, max_entries, &mut control)
                .await?;
            self.include_domains(value.captured.domains(), &mut control)
                .await?;
            self.include_replication(value.captured.replication_root, max_entries, &mut control)
                .await?;
            checkpoint = value.captured.checkpoint;
        }
        Err(Error::Limit("native copy checkpoints"))
    }

    async fn include_catalog(
        &self,
        root: Option<RecordRef>,
        limit: usize,
        records: &mut BTreeSet<RecordRef>,
    ) -> Result<()> {
        let CatalogClosure { pages, entries } = self.catalog.closure(root, limit).await?;
        records.extend(pages);
        records.extend(entries.into_values());
        Ok(())
    }

    async fn include_domains(
        &self,
        domains: Option<RepositoryDomains>,
        records: &mut BTreeSet<RecordRef>,
    ) -> Result<()> {
        let Some(domains) = domains else {
            return Ok(());
        };
        if let Some(thread) = domains.thread {
            records.extend(self.native_thread_store(thread.root).await?.records);
        }
        if let Some(reaction) = domains.reaction {
            records.extend(self.native_reaction_store(reaction.root).await?.records);
        }
        if let Some(git) = domains.git {
            records.extend(self.git_store(git.root).await?.records);
        }
        if let Some(work) = domains.work {
            records.extend(self.work_store(work.root).await?.records);
        }
        records.insert(domains.agent.root);
        let (_, _, agent) = self.read_agent_root_closure(domains.agent.root).await?;
        records.extend(agent.into_iter().map(super::agent::agent_record));
        records.insert(domains.review.root);
        let (_, review) = self.read_review_root_closure(domains.review.root).await?;
        records.extend(review.objects.into_iter().map(super::review::review_record));
        records.extend(
            review
                .agent_objects
                .into_iter()
                .map(super::agent::agent_record),
        );
        Ok(())
    }

    async fn include_replication(
        &self,
        root: Option<RecordRef>,
        limit: usize,
        records: &mut BTreeSet<RecordRef>,
    ) -> Result<()> {
        let Some(root_ref) = root else {
            return Ok(());
        };
        records.insert(root_ref);
        let root = self.read_replication_root(root_ref).await?;
        let CatalogClosure { pages, entries } =
            self.catalog.closure(root.accepted_origins, limit).await?;
        records.extend(pages);
        for reference in entries.into_values() {
            records.insert(reference);
            let receipt: OriginReceiptV1 = self
                .records
                .get(reference, RecordKind::OriginReceipt)
                .await?;
            records.insert(receipt.envelope);
            if let Some(root) = self.origin_publication(receipt).await?.git_root {
                records.extend(self.git_store(root).await?.records);
            }
        }
        Ok(())
    }
}

fn include_manifest_records(manifest: &Manifest, records: &mut BTreeSet<RecordRef>) {
    records.insert(manifest.initial_settings);
    records.insert(manifest.settings);
    records.extend(manifest.tail);
    records.extend(manifest.inventory);
    records.extend(manifest.receipts);
    records.extend(manifest.history);
    records.extend(manifest.checkpoint);
    records.extend(manifest.agent_root);
    records.extend(manifest.review_root);
    records.extend(manifest.replication_root);
    records.extend(manifest.work_root);
    records.extend(manifest.git_root);
    records.extend(manifest.thread_root);
    records.extend(manifest.reaction_root);
}

async fn copy_exact(
    source: &Arc<dyn ObjectStore>,
    destination: &Arc<dyn ObjectStore>,
    key: &Key,
    limit: u64,
    remaining: &mut u64,
) -> Result<u64> {
    let bytes = source.get(key, None).await?.read_bounded(limit).await?;
    *remaining = remaining
        .checked_sub(bytes.len() as u64)
        .ok_or(Error::Limit("native copy bytes"))?;
    put_exact(destination, key, &bytes, limit).await?;
    Ok(bytes.len() as u64)
}

async fn put_exact(
    destination: &Arc<dyn ObjectStore>,
    key: &Key,
    bytes: &[u8],
    limit: u64,
) -> Result<()> {
    let upload = StagedUpload::from_bytes(bytes, limit).await?;
    match destination.put(key, Condition::Absent, &upload).await? {
        WriteOutcome::Applied(_) => {}
        WriteOutcome::PreconditionFailed | WriteOutcome::Unknown => {
            match destination.get(key, None).await {
                Ok(existing) => {
                    if existing.read_bounded(limit).await? != bytes {
                        return Err(Error::AlreadyExists);
                    }
                }
                Err(jj_store_object::Error::NotFound | jj_store_object::Error::Transport) => {
                    return Err(Error::OutcomeUnknown);
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
    Ok(())
}
