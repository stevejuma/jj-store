//! One guarded foreground slot; paths and preparation failures stay device-private.
use super::*;
use crate::{
    capture_admission::CaptureAdmission,
    capture_authority::CaptureAuthority,
    capture_control_state::CaptureControlJournal,
    capture_runtime::RunningCodexCapture,
    capture_source::CapturePrivacyRoots,
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorResources,
    },
};
use jj_store_capture::{
    agent_capture::AgentCaptureSpool, profile::CaptureProfile, source_registry::SourceRegistry,
};
use jj_store_service::administration::RepositoryHost;
use std::path::{Component, Path, PathBuf};
mod selection;
mod state;
use selection::ReadyCapture;

pub(super) struct CaptureContext<'a> {
    pub identity: Option<&'a Identity>,
    pub host: Option<&'a RepositoryHost>,
    pub storage: Option<&'a AuthoritativeStorage>,
    pub admission: Option<CaptureAdmission>,
}
pub(super) struct CaptureLocations {
    pub home: PathBuf,
    pub codex_home: Option<PathBuf>,
    pub temporary: PathBuf,
}
impl CaptureLocations {
    pub(super) fn environment() -> Result<Self> {
        Ok(Self {
            home: std::env::var_os("HOME")
                .map(PathBuf::from)
                .ok_or(Error::Config("capture home unavailable"))?,
            codex_home: std::env::var_os("CODEX_HOME").map(PathBuf::from),
            temporary: std::env::temp_dir(),
        })
    }
    fn validate(&self) -> Result<()> {
        valid_path(&self.home)?;
        valid_path(&self.temporary)?;
        if self.home == self.temporary {
            return Err(Error::Config("capture privacy roots must be distinct"));
        }
        if let Some(path) = &self.codex_home {
            valid_path(path)?;
        }
        Ok(())
    }
}

pub(super) enum PreparedCapture {
    Disabled,
    Unavailable,
    Ready(Box<ReadyCapture>),
}
impl PreparedCapture {
    #[cfg(test)]
    pub(super) fn prepare(
        config: &Config,
        context: CaptureContext<'_>,
        locations: impl FnOnce() -> Result<CaptureLocations>,
    ) -> Self {
        Self::select(config, context, locations, false)
    }
    /// Only whole-server composition may initialize absent policy under its state lock.
    pub(super) fn prepare_server(
        config: &Config,
        context: CaptureContext<'_>,
        locations: impl FnOnce() -> Result<CaptureLocations>,
    ) -> Self {
        Self::select(config, context, locations, true)
    }
    fn select(
        config: &Config,
        context: CaptureContext<'_>,
        locations: impl FnOnce() -> Result<CaptureLocations>,
        initialize: bool,
    ) -> Self {
        if !config.agent_capture.enabled {
            return Self::Disabled;
        }
        match prepare_enabled(config, context, locations, initialize) {
            Ok(supervisor) => Self::Ready(Box::new(supervisor)),
            // No path, body, credential or source identity enters health state.
            Err(_) => Self::Unavailable,
        }
    }
    pub(super) fn start(self) -> RunningCapture {
        match self {
            Self::Disabled => RunningCapture::Disabled,
            Self::Unavailable => RunningCapture::Unavailable,
            Self::Ready(ready) => match ready.start() {
                Ok(runtime) => RunningCapture::Running(runtime),
                Err(_) => RunningCapture::Unavailable,
            },
        }
    }
    pub(super) fn management(
        &self,
        path: &Path,
        config: &Config,
        identity: &Identity,
    ) -> Result<crate::capture_management::Manager> {
        let binding = match self {
            Self::Ready(ready) => Some(ready.management()),
            _ => None,
        };
        let manager = crate::capture_management::Manager::new(path, config, identity, binding)
            .map_err(|_| Error::Service("capture management unavailable"))?;
        if let Self::Ready(ready) = self {
            ready.bind_recovery(&manager);
        }
        Ok(manager)
    }
}
pub(super) enum RunningCapture {
    Disabled,
    Unavailable,
    Running(RunningCodexCapture),
}
impl RunningCapture {
    pub(super) async fn shutdown(&mut self) -> Result<()> {
        match self {
            Self::Running(runtime) => runtime.shutdown().await,
            Self::Disabled | Self::Unavailable => Ok(()),
        }
    }
}

fn prepare_enabled(
    config: &Config,
    context: CaptureContext<'_>,
    locations: impl FnOnce() -> Result<CaptureLocations>,
    initialize: bool,
) -> Result<ReadyCapture> {
    config.validate()?;
    supported(config)?;
    let identity = context
        .identity
        .ok_or(Error::Config("capture identity unavailable"))?;
    let host = context
        .host
        .ok_or(Error::Config("capture native host unavailable"))?;
    let storage = context
        .storage
        .ok_or(Error::Config("capture storage unavailable"))?;
    let admission = context
        .admission
        .ok_or(Error::Config("capture verifier unavailable"))?;
    let Access::LocalTrusted { installation, .. } = &config.access else {
        return Err(Error::Config("capture access unavailable"));
    };
    if Identity::load(installation)? != *identity {
        return Err(Error::Config("capture installation changed"));
    }
    let identity_path = installation.clone();
    let installation: [u8; 16] = hex::decode(&identity.installation)
        .map_err(|_| Error::Config("capture installation unavailable"))?
        .try_into()
        .map_err(|_| Error::Config("capture installation unavailable"))?;
    if installation == [0; 16] {
        return Err(Error::Config("capture installation unavailable"));
    }
    let locations = locations()?;
    locations.validate()?;
    for harness in &config.agent_capture.harnesses {
        for root in harness.roots(Some(&locations.home), locations.codex_home.as_deref())? {
            valid_path(&root)?;
        }
    }
    let root = state::prepare(&config.state_dir, installation, initialize)?;
    let journal = CaptureControlJournal::open(&root.join("control"), installation)
        .map_err(|_| Error::Service("capture control state unavailable"))?;
    let selected = journal
        .current()
        .map_err(|_| Error::Service("capture control state unavailable"))?;
    let sources = SourceRegistry::open(
        &root.join("sources"),
        installation,
        config.agent_capture.max_sources,
    )
    .map_err(|_| Error::Service("capture sources unavailable"))?;
    let spool =
        AgentCaptureSpool::open(&root.join("spool"), config.agent_capture.max_pending_bytes)
            .map_err(|_| Error::Service("capture spool unavailable"))?;
    // Missing or unqualified import metadata withholds import management only.
    // Existing state is never repaired; inspection shares the existing spool group.
    let imports = crate::capture_import_state::CaptureImportJournal::open(
        &root.join("imports"),
        installation,
        selected.generation,
    )
    .ok()
    .and_then(|journal| {
        crate::capture_management::CaptureImportManagement::new(journal, &spool).ok()
    })
    .map(Arc::new);
    let mut supervisor = CodexCaptureSupervisor::prepare(
        config.agent_capture.clone(),
        CodexSupervisorLocations {
            home: Some(locations.home.clone()),
            codex_home: locations.codex_home,
        },
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::from_identity(identity)),
            host: host.clone(),
            admission,
            sources: Arc::new(sources),
            spool,
            privacy: CapturePrivacyRoots {
                home: locations.home,
                temporary: locations.temporary,
                cache: root.join("kingfisher"),
            },
        },
        Duration::from_secs(config.limits.request_seconds.min(300)),
    )?
    .ok_or(Error::Config("capture unexpectedly disabled"))?;
    supervisor.retain_storage(storage.capture_lifetime())?;
    Ok(ReadyCapture::new(
        supervisor,
        journal,
        selected,
        identity_path,
        identity.clone(),
        imports,
    ))
}
pub(super) fn supported(config: &Config) -> Result<()> {
    if config.agent_capture.enabled
        && config.agent_capture.harnesses.iter().any(|h| {
            !matches!(
                h.profile,
                CaptureProfile::CodexCliJsonlV1
                    | CaptureProfile::CodexDesktopJsonlV1
                    | CaptureProfile::OpencodeReadOnlyApiV1
                    | CaptureProfile::CursorDesktopTranscriptV1
                    | CaptureProfile::CursorCliTranscriptV1
            )
        })
    {
        return Err(Error::Config("capture reader unavailable"));
    }
    Ok(())
}
fn valid_path(path: &Path) -> Result<()> {
    if !path.is_absolute()
        || path.parent().is_none()
        || path
            .to_str()
            .is_none_or(|p| p.len() > 4096 || p.chars().any(char::is_control))
        || path
            .components()
            .any(|p| matches!(p, Component::ParentDir | Component::CurDir))
    {
        return Err(Error::Config("capture location unavailable"));
    }
    Ok(())
}
