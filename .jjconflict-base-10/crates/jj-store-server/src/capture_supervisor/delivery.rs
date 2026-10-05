use super::*;
use jj_store_capture::{
    agent_capture::PendingCaptureV1,
    agent_publication::{CaptureDrainPartition, CaptureDrainScheduler, drain_capture_spool_entry},
    codex_transcript::{CodexBatchRecovery, CodexRecoveryPoll, RecoveredCodexDirectory},
};
use std::collections::BTreeMap;
struct Frame {
    entry: PendingCaptureV1,
    source: DeliverySource,
    probe: CodexBatchRecovery,
}
pub(super) struct DeliveryCommand {
    pub sources: Vec<DeliverySource>,
    pub epoch: u64,
}
pub(super) struct DeliveryOutput {
    pub capacity_refusal: Option<jj_store_capture::agent_capture::CaptureCapacityRefusal>,
    pub pending: usize,
    pub epoch: u64,
    pub frames: usize,
    pub unavailable: bool,
}
pub(super) struct DeliveryWork {
    resources: CodexSupervisorResources,
    frames: BTreeMap<CaptureDrainPartition, Frame>,
    limit: usize,
    scheduler: CaptureDrainScheduler,
    candidate_after: usize,
    binding: Option<jj_store_capture::CaptureSpoolBinding>,
}
impl DeliveryWork {
    pub fn new(resources: CodexSupervisorResources, limit: usize) -> Self {
        Self {
            resources,
            frames: BTreeMap::new(),
            limit,
            scheduler: Default::default(),
            candidate_after: 0,
            binding: None,
        }
    }
    pub fn frames(&self) -> usize {
        self.frames.len()
    }
    #[cfg(test)]
    pub(super) fn admission_for_test(&mut self, admission: CaptureAdmission) {
        self.resources.admission = admission.for_owned_worker();
    }
    async fn advance(&mut self, sources: &[DeliverySource]) -> Result<usize> {
        self.check_spool()?;
        let pending =
            self.resources.spool.pending().map_err(|error| {
                Error::capture_spool(error, "capture delivery queue unavailable")
            })?;
        self.frames
            .retain(|_, frame| pending.iter().any(|e| e == &frame.entry));
        let available: Vec<_> = pending
            .iter()
            .filter(|entry| {
                matches!(
                    entry.cursor.integration,
                    jj_store_capture::opencode_ingress::IntegrationId::Codex
                        | jj_store_capture::opencode_ingress::IntegrationId::Cursor
                ) && (self.frames.len() < self.limit
                    || self.frames.contains_key(&CaptureDrainPartition::of(entry)))
            })
            .cloned()
            .collect();
        let selected = self
            .scheduler
            .select(&available)
            .map_err(|_| Error::Service("capture delivery selection unavailable"))?
            .cloned();
        drop(available);
        let count = pending.len();
        drop(pending);
        let Some(entry) = selected else {
            return Ok(count);
        };
        let partition = CaptureDrainPartition::of(&entry);
        if !self.frames.contains_key(&partition) {
            let mut selected = None;
            for offset in 0..sources.len() {
                let index = (self.candidate_after + offset) % sources.len();
                let source = &sources[index];
                if source.qualified.profile().integration() != entry.cursor.integration {
                    continue;
                }
                if !source.ordinary.load(std::sync::atomic::Ordering::SeqCst) {
                    continue;
                }
                if self
                    .resources
                    .sources
                    .lookup_admitted(&source.qualified)
                    .ok()
                    .flatten()
                    != Some(partition.source)
                {
                    continue;
                }
                self.candidate_after = (index + 1) % sources.len();
                if let Ok(probe) = source.qualified.recover_batch(partition.source, &entry) {
                    selected = Some(Frame {
                        entry,
                        source: source.clone(),
                        probe,
                    });
                }
                break;
            }
            let Some(frame) = selected else {
                return Ok(count);
            };
            self.frames.insert(partition, frame);
        }
        let result = self.deliver(partition).await;
        // Complete proof bodies have already been discarded by recovery. A ready
        // withheld partition yields its slot so other partitions can be verified.
        if !matches!(result, Ok(false)) {
            self.frames.remove(&partition);
        }
        result?;
        self.check_spool()?;
        self.resources
            .spool
            .pending()
            .map(|v| v.len())
            .map_err(|_| Error::Service("capture delivery queue unavailable"))
    }
    async fn deliver(&mut self, partition: CaptureDrainPartition) -> Result<bool> {
        self.check_spool()?;
        let frame = self.frames.get_mut(&partition).expect("selected frame");
        ordinary(&frame.source)?;
        if self
            .resources
            .sources
            .lookup_admitted(&frame.source.qualified)
            .map_err(|_| Error::Service("capture delivery eligibility unavailable"))?
            != Some(partition.source)
        {
            return Ok(true);
        }
        let directory = match frame
            .probe
            .poll()
            .map_err(|_| Error::Service("capture delivery proof unavailable"))?
        {
            CodexRecoveryPoll::Verifying | CodexRecoveryPoll::ChangedDuringRead => {
                return Ok(false);
            }
            CodexRecoveryPoll::Ready(directory) => directory,
        };
        let Some(proof) = admission(
            &self.resources.admission,
            &directory,
            &frame.source,
            frame.entry.repository,
        )
        .await?
        else {
            return Ok(true);
        };
        self.resources
            .authority
            .bind(&self.resources.host, &self.resources.admission, &proof)
            .await?;
        ordinary(&frame.source)?;
        if !frame
            .probe
            .revalidate()
            .map_err(|_| Error::Service("capture delivery proof changed"))?
            || self
                .resources
                .sources
                .lookup_admitted(&frame.source.qualified)
                .map_err(|_| Error::Service("capture delivery eligibility unavailable"))?
                != Some(partition.source)
        {
            return Ok(true);
        }
        let publisher = self.resources.authority.publisher(
            &self.resources.host,
            &self.resources.admission,
            &proof,
        );
        if self
            .resources
            .spool
            .binding()
            .map_err(|_| Error::Service("capture delivery spool unavailable"))?
            != *self.binding.as_ref().expect("bound delivery spool")
        {
            return Err(Error::Service("capture delivery spool changed"));
        }
        drain_capture_spool_entry(
            &mut self.resources.spool,
            &frame.entry,
            &publisher.with_guard(&|| ordinary(&frame.source)),
        )
        .await
        .map_err(|error| Error::capture_spool(error, "capture delivery unavailable"))?;
        Ok(true)
    }
    fn check_spool(&mut self) -> Result<()> {
        let current = self
            .resources
            .spool
            .binding()
            .map_err(|_| Error::Service("capture delivery spool unavailable"))?;
        if self.binding.as_ref().is_some_and(|old| old != &current) {
            return Err(Error::Service("capture delivery spool changed"));
        }
        self.binding = Some(current);
        Ok(())
    }
}
fn ordinary(source: &DeliverySource) -> Result<()> {
    if source.ordinary.load(std::sync::atomic::Ordering::SeqCst) {
        Ok(())
    } else {
        Err(Error::Service(
            "capture delivery transferred to import owner",
        ))
    }
}
async fn admission(
    admission: &CaptureAdmission,
    directory: &RecoveredCodexDirectory,
    source: &DeliverySource,
    repository: jj_store_model::RepositoryId,
) -> Result<Option<VerifiedCaptureWorkspace>> {
    let mut candidates = vec![directory.directory()];
    if directory.omission_only() {
        candidates.extend([
            directory.omission_anchor(),
            Some(source.qualified.directory()),
            source
                .anchor
                .as_ref()
                .map(|p| std::path::Path::new(&p.directory)),
        ]);
    }
    for path in candidates.into_iter().flatten() {
        let Some(path) = path.to_str() else {
            continue;
        };
        match admission.inspect(path).await {
            Ok(proof) if proof.workspace.repository == repository => return Ok(Some(proof)),
            Ok(_)
            | Err(
                crate::capture_admission::Error::Unbound | crate::capture_admission::Error::Limit,
            ) => {}
            Err(_) => return Err(Error::Service("capture delivery admission unavailable")),
        }
    }
    Ok(None)
}
#[async_trait::async_trait]
impl OwnedWork for DeliveryWork {
    type Command = DeliveryCommand;
    type Output = DeliveryOutput;
    async fn step(&mut self, command: Self::Command) -> Self::Output {
        let result = self.advance(&command.sources).await;
        DeliveryOutput {
            capacity_refusal: result.as_ref().err().and_then(Error::capacity_refusal),
            pending: result.as_ref().copied().unwrap_or(0),
            epoch: command.epoch,
            frames: self.frames.len(),
            unavailable: result.is_err(),
        }
    }
}
