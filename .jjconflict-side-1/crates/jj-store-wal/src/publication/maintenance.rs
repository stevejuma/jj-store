use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckpointCoverage {
    NotCovered,
    Selected,
    Covered,
}

impl Coordinator {
    pub async fn checkpoint_target(&self, target: &Checkpoint) -> Result<ManifestSnapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let captured = self.restore_maintenance(target).await?;
            let checkpoint = self
                .records
                .put(
                    RecordKind::Checkpoint,
                    None,
                    &Checkpoint {
                        captured: captured.manifest.clone(),
                        captured_digest: captured.digest,
                    },
                )
                .await?;
            for _ in 0..self.limits.cas_attempts {
                let base = self.read().await?;
                if base.manifest.epoch != captured.manifest.epoch {
                    return Err(Error::Stale);
                }
                // Do not replace a checkpoint already ahead of the captured one.
                if let Some(existing) = base.manifest.checkpoint {
                    let existing: Checkpoint =
                        self.records.get(existing, RecordKind::Checkpoint).await?;
                    if existing.captured.published >= captured.manifest.published {
                        return Ok(base);
                    }
                }
                let manifest = Manifest {
                    revision: base
                        .manifest
                        .revision
                        .checked_add(1)
                        .ok_or(Error::Limit("manifest revision"))?,
                    checkpoint: Some(checkpoint),
                    ..base.manifest
                };
                let bytes = records::encode(&manifest, MAX_MANIFEST)?;
                let upload = StagedUpload::from_bytes(&bytes, MAX_MANIFEST).await?;
                match self
                    .records
                    .store
                    .put(
                        &self.manifest_key(),
                        Condition::Matches(base.version),
                        &upload,
                    )
                    .await?
                {
                    WriteOutcome::Applied(version) => {
                        return Ok(ManifestSnapshot {
                            manifest,
                            digest: records::digest(&bytes),
                            version,
                        });
                    }
                    WriteOutcome::PreconditionFailed => continue,
                    WriteOutcome::Unknown => {
                        let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                        if latest.manifest.checkpoint == Some(checkpoint) {
                            return Ok(latest);
                        }
                        return Err(Error::OutcomeUnknown);
                    }
                }
            }
            Err(Error::Contended)
        })
        .await
        .map_err(|_| Error::OutcomeUnknown)?
    }
    pub async fn checkpoint(&self) -> Result<ManifestSnapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            self.checkpoint_target(&self.capture_maintenance().await?)
                .await
        })
        .await
        .map_err(|_| Error::OutcomeUnknown)?
    }
    pub async fn capture_maintenance(&self) -> Result<Checkpoint> {
        let snapshot = self.read().await?;
        Ok(Checkpoint {
            captured: snapshot.manifest,
            captured_digest: snapshot.digest,
        })
    }
    // Restored captures are read pins, never CAS bases. Publication always reads
    // a fresh base and retains its newer tail.
    async fn restore_maintenance(&self, target: &Checkpoint) -> Result<ManifestSnapshot> {
        target.captured.validate(self.records.repository)?;
        if records::digest(&records::encode(&target.captured, MAX_MANIFEST)?)
            != target.captured_digest
        {
            return Err(Error::Corrupt("maintenance capture digest"));
        }
        let current = self.read().await?;
        if current.manifest.epoch != target.captured.epoch
            || current.manifest.published < target.captured.published
            || current.manifest.revision < target.captured.revision
            || current.manifest.initial_settings != target.captured.initial_settings
        {
            return Err(Error::Stale);
        }
        let tail = if target.captured.published.0 == 0 {
            None
        } else {
            self.catalog
                .get(
                    current.manifest.history,
                    sequence_key(target.captured.published),
                )
                .await?
        };
        if tail != target.captured.tail {
            return Err(Error::Corrupt("maintenance capture tail"));
        }
        Ok(ManifestSnapshot {
            manifest: target.captured.clone(),
            digest: target.captured_digest,
            version: current.version,
        })
    }
    pub async fn recover_target(
        &self,
        target: &Checkpoint,
        max_entries: usize,
    ) -> Result<ManifestSnapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let snapshot = self.restore_maintenance(target).await?;
            self.recover_snapshot(snapshot, max_entries).await
        })
        .await
        .map_err(|_| Error::Limit("recovery time"))?
    }
    pub async fn checkpoint_coverage(&self, target: &Checkpoint) -> Result<CheckpointCoverage> {
        self.restore_maintenance(target).await?;
        let current = self.read().await?;
        let Some(reference) = current.manifest.checkpoint else {
            return Ok(CheckpointCoverage::NotCovered);
        };
        let selected: Checkpoint = self.records.get(reference, RecordKind::Checkpoint).await?;
        if selected == *target {
            return Ok(CheckpointCoverage::Selected);
        }
        if selected.captured.epoch == target.captured.epoch
            && selected.captured.published >= target.captured.published
        {
            return Ok(CheckpointCoverage::Covered);
        }
        Ok(CheckpointCoverage::NotCovered)
    }
    pub async fn selected_checkpoint(
        &self,
        snapshot: &ManifestSnapshot,
    ) -> Result<Option<Checkpoint>> {
        snapshot.manifest.validate(self.records.repository)?;
        match snapshot.manifest.checkpoint {
            Some(reference) => self
                .records
                .get(reference, RecordKind::Checkpoint)
                .await
                .map(Some),
            None => Ok(None),
        }
    }
}
