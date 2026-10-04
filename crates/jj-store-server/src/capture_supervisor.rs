//! Explicit bounded composition; process startup and management remain gated.
use crate::{
    Error, Result,
    capture_admission::CaptureAdmission,
    capture_authority::CaptureAuthority,
    capture_config::AgentCaptureConfig,
    capture_source::CapturePrivacyRoots,
    capture_worker::{
        CaptureWorkerPhase,
        executor::{Completion, OwnedWork, OwnedWorker},
    },
};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool,
    source_registry::{CodexSourceFingerprint, QualifiedCodexSource, SourceRegistry},
};
use jj_store_service::{
    administration::RepositoryHost, local_management::VerifiedCaptureWorkspace,
};
use std::{collections::BTreeSet, path::PathBuf, sync::Arc, time::Duration};
mod delivery;
mod discovery;
mod imports;
mod lifetime;
mod opencode;
mod recovery;
mod scheduling;
mod source;
mod sources;
mod status;
use delivery::{DeliveryCommand, DeliveryWork};
use discovery::DiscoveryWork;
use source::{SourceSnapshot, SourceWork};
use sources::SourceSelectors;
pub(crate) use sources::{CodexSourceCatalog, CodexSourceObservation};
pub use status::{CodexDiscoveryStatus, CodexHarnessStatus, CodexSupervisorStatus};

#[cfg(all(test, unix))]
mod tests;

// Device paths and proof-bearing resources have no Debug or serialization.
pub struct CodexSupervisorLocations {
    pub home: Option<PathBuf>,
    pub codex_home: Option<PathBuf>,
}
pub struct CodexSupervisorResources {
    pub authority: Arc<CaptureAuthority>,
    pub host: RepositoryHost,
    pub admission: CaptureAdmission,
    pub sources: Arc<SourceRegistry>,
    pub spool: AgentCaptureSpool,
    pub privacy: CapturePrivacyRoots,
}
impl CodexSupervisorResources {
    fn fork(&self) -> Self {
        Self {
            authority: self.authority.clone(),
            host: self.host.clone(),
            admission: self.admission.clone().for_owned_worker(),
            sources: self.sources.clone(),
            spool: self.spool.fork_handle(),
            privacy: self.privacy.clone(),
        }
    }
}
#[derive(Clone)]
struct DeliverySource {
    qualified: QualifiedCodexSource,
    anchor: Option<VerifiedCaptureWorkspace>,
    ordinary: Arc<std::sync::atomic::AtomicBool>,
}
struct SourceSlot {
    selector: u64,
    owner: OwnedWorker<SourceWork>,
    qualified: QualifiedCodexSource,
    snapshot: SourceSnapshot,
    raw: bool,
    occupied: bool,
    import: Option<crate::capture_management::CaptureImportDispatch>,
    import_installed: bool,
    ordinary: Arc<std::sync::atomic::AtomicBool>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodexSupervisorPoll {
    Idle,
    Running,
    Completed(Box<CodexSupervisorStatus>),
}
pub struct CodexCaptureSupervisor {
    selectors: SourceSelectors,
    config: AgentCaptureConfig,
    configured_roots: Arc<[(jj_store_capture::profile::CaptureProfile, PathBuf)]>,
    resources: CodexSupervisorResources,
    discovery: OwnedWorker<DiscoveryWork>,
    discovery_coverage: discovery::RootObservations,
    rescan_requested: bool,
    discard_rescan: bool,
    delivery: OwnedWorker<DeliveryWork>,
    recovery: Option<recovery::RecoveryLane>,
    slots: Vec<SourceSlot>,
    fingerprints: BTreeSet<CodexSourceFingerprint>,
    after: usize,
    deadline: Duration,
    paused: bool,
    closed: bool,
    epoch: u64,
    queue_epoch: Option<u64>,
    pending: usize,
    delivery_frames: usize,
    delivery_capacity_refusal: Option<jj_store_capture::agent_capture::CaptureCapacityRefusal>,
    unavailable: usize,
    limit_reached: bool,
    dispatched: bool,
    storage: Option<crate::storage::CaptureStorageLifetime>,
    api: Option<opencode::ApiLane>,
    api_first: bool,
}
impl CodexCaptureSupervisor {
    pub(crate) fn configured_roots(
        &self,
    ) -> Arc<[(jj_store_capture::profile::CaptureProfile, PathBuf)]> {
        self.configured_roots.clone()
    }
    /// Pure configuration selection; root/file operations occur only in owned steps.
    pub fn prepare(
        config: AgentCaptureConfig,
        locations: CodexSupervisorLocations,
        resources: CodexSupervisorResources,
        deadline: Duration,
    ) -> Result<Option<Self>> {
        if !config.enabled {
            return Ok(None);
        }
        if !(1..=4).contains(&config.max_readers)
            || !(1..=4096).contains(&config.max_sources)
            || !(1..=30).contains(&config.rescan_seconds)
            || !(1..=64 * 1024 * 1024).contains(&config.max_pending_bytes)
            || config.harnesses.is_empty()
            || config.harnesses.len() > 16
            || config.max_pending_bytes != resources.spool.pending_capacity()
        {
            return Err(Error::Config("Codex supervisor resource bounds"));
        }
        let mut roots = Vec::new();
        let mut profiles = BTreeSet::new();
        let mut api = None;
        for harness in &config.harnesses {
            if !profiles.insert(harness.profile) {
                return Err(Error::Config("duplicate Codex profile"));
            }
            if harness.profile == jj_store_capture::profile::CaptureProfile::OpencodeReadOnlyApiV1 {
                if harness.product_version != harness.profile.product_version()
                    || !harness.roots.is_empty()
                {
                    return Err(Error::Config("OpenCode capture profile unavailable"));
                }
                api = Some(opencode::ApiLane::new(
                    crate::opencode_read_api::OpenCodeReadApi::new(
                        harness
                            .endpoint
                            .as_deref()
                            .ok_or(Error::Config("OpenCode capture endpoint unavailable"))?,
                    )?,
                    &resources,
                    Duration::from_secs(config.rescan_seconds),
                    deadline,
                )?);
                continue;
            }
            for path in
                harness.codex_roots(locations.home.as_deref(), locations.codex_home.as_deref())?
            {
                roots.push((harness.profile, path));
            }
        }
        let discovery_coverage =
            discovery::RootObservations::configured(roots.iter().map(|(profile, _)| *profile));
        let configured_roots = Arc::from(roots.clone());
        let discovery = OwnedWorker::new(
            DiscoveryWork::new(roots, Duration::from_secs(config.rescan_seconds)),
            deadline,
        )?;
        let delivery = OwnedWorker::new(
            DeliveryWork::new(resources.fork(), config.max_readers * 2),
            deadline,
        )?;
        Ok(Some(Self {
            selectors: SourceSelectors::new()?,
            config,
            configured_roots,
            resources,
            discovery,
            discovery_coverage,
            rescan_requested: false,
            discard_rescan: false,
            delivery,
            recovery: None,
            slots: Vec::new(),
            fingerprints: BTreeSet::new(),
            after: 0,
            deadline,
            paused: false,
            closed: false,
            epoch: 0,
            queue_epoch: None,
            pending: 0,
            delivery_frames: 0,
            delivery_capacity_refusal: None,
            unavailable: 0,
            limit_reached: false,
            dispatched: false,
            storage: None,
            api,
            api_first: false,
        }))
    }
    pub fn pause(&mut self) {
        self.rescan_requested = false;
        self.discard_rescan = true;
        self.paused = true;
        self.discovery.pause();
        self.delivery.pause();
        if let Some(api) = &mut self.api {
            api.pause();
        }
        if let Some(lane) = &mut self.recovery {
            lane.owner.pause();
        }
        for slot in &mut self.slots {
            slot.owner.pause();
        }
    }
    pub fn resume(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::Service("Codex supervisor stopped"));
        }
        self.discovery.resume()?;
        self.delivery.resume()?;
        if let Some(api) = &mut self.api {
            api.resume()?;
        }
        if let Some(lane) = &mut self.recovery
            && lane.owner.phase() != CaptureWorkerPhase::Stopped
        {
            lane.owner.resume()?;
        }
        for slot in &mut self.slots {
            if slot.owner.phase() != CaptureWorkerPhase::Stopped {
                slot.owner.resume()?;
            }
        }
        self.paused = false;
        Ok(())
    }
    pub(crate) fn stop(&mut self) {
        self.rescan_requested = false;
        self.discard_rescan = true;
        self.closed = true;
        self.discovery.stop();
        self.delivery.stop();
        if let Some(api) = &mut self.api {
            api.stop();
        }
        if let Some(lane) = &mut self.recovery {
            lane.owner.stop();
        }
        for slot in &mut self.slots {
            slot.owner.stop();
        }
    }
    pub fn rescan(&mut self) -> Result<()> {
        if self.paused || self.closed || self.discovery.phase() == CaptureWorkerPhase::Stopped {
            return Err(Error::Service("capture discovery paused or unavailable"));
        }
        self.rescan_requested = true;
        if let Some(api) = &mut self.api {
            api.rescan();
        }
        Ok(())
    }
    pub async fn shutdown(&mut self) -> Result<()> {
        self.stop();
        tokio::time::timeout(Duration::from_secs(5), async {
            while self.phase() != CaptureWorkerPhase::Stopped {
                self.poll().await;
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .map_err(|_| Error::Service("Codex supervisor is still draining"))
    }
}
