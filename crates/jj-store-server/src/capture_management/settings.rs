//! Bounded local settings replacement; no worker, journal or agent execution.
use super::*;
use crate::{
    capture_config::{AgentCaptureConfig, HarnessConfig},
    config::Config,
    policy,
};
use jj_store_capture::profile::CaptureProfile;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

pub(super) struct SettingsFile {
    path: PathBuf,
    effective: AgentCaptureConfig,
}
fn profile(value: CaptureProfile) -> Result<CaptureHarnessProfile, Error> {
    Ok(match value {
        CaptureProfile::CodexCliJsonlV1 => CaptureHarnessProfile::CodexCliJsonlV1,
        CaptureProfile::CodexDesktopJsonlV1 => CaptureHarnessProfile::CodexDesktopJsonlV1,
        CaptureProfile::CursorDesktopTranscriptV1 => {
            CaptureHarnessProfile::CursorDesktopTranscriptV1
        }
        CaptureProfile::CursorCliTranscriptV1 => CaptureHarnessProfile::CursorCliTranscriptV1,
        CaptureProfile::OpencodeReadOnlyApiV1 => CaptureHarnessProfile::OpencodeReadOnlyApiV1,
        _ => return Err(Error::Invalid),
    })
}
fn wire(config: &AgentCaptureConfig) -> Result<CaptureConfiguration, Error> {
    let mut config = config.clone();
    config.harnesses.sort_by_key(|h| h.profile);
    Ok(CaptureConfiguration {
        enabled: config.enabled,
        max_readers: config.max_readers,
        rescan_seconds: config.rescan_seconds,
        max_sources: config.max_sources,
        max_pending_bytes: config.max_pending_bytes,
        harnesses: config
            .harnesses
            .iter()
            .map(|h| {
                Ok(CaptureHarnessSetting {
                    profile: profile(h.profile)?,
                    product_version: h.product_version.clone(),
                    endpoint: h.endpoint.clone(),
                    roots: h
                        .roots
                        .iter()
                        .map(|p| {
                            p.to_str()
                                .map(|s| CaptureRootPath(s.into()))
                                .ok_or(Error::Invalid)
                        })
                        .collect::<Result<_, _>>()?,
                })
            })
            .collect::<Result<_, _>>()?,
    })
}
fn native(value: &CaptureConfiguration) -> Result<AgentCaptureConfig, Error> {
    let config = AgentCaptureConfig {
        enabled: value.enabled,
        max_readers: value.max_readers,
        rescan_seconds: value.rescan_seconds,
        max_sources: value.max_sources,
        max_pending_bytes: value.max_pending_bytes,
        harnesses: value
            .harnesses
            .iter()
            .map(|h| {
                let profile = match h.profile {
                    CaptureHarnessProfile::CodexCliJsonlV1 => CaptureProfile::CodexCliJsonlV1,
                    CaptureHarnessProfile::CodexDesktopJsonlV1 => {
                        CaptureProfile::CodexDesktopJsonlV1
                    }
                    CaptureHarnessProfile::CursorDesktopTranscriptV1 => {
                        CaptureProfile::CursorDesktopTranscriptV1
                    }
                    CaptureHarnessProfile::CursorCliTranscriptV1 => {
                        CaptureProfile::CursorCliTranscriptV1
                    }
                    CaptureHarnessProfile::OpencodeReadOnlyApiV1 => {
                        CaptureProfile::OpencodeReadOnlyApiV1
                    }
                    _ => return Err(Error::Invalid),
                };
                for root in &h.roots {
                    let p = Path::new(&root.0);
                    if p.parent().is_none()
                        || root.0.chars().any(char::is_control)
                        || p.components().any(|c| {
                            matches!(
                                c,
                                std::path::Component::ParentDir | std::path::Component::CurDir
                            )
                        })
                    {
                        return Err(Error::Invalid);
                    }
                }
                Ok(HarnessConfig {
                    profile,
                    product_version: h.product_version.clone(),
                    roots: h.roots.iter().map(|r| PathBuf::from(&r.0)).collect(),
                    endpoint: h.endpoint.clone(),
                })
            })
            .collect::<Result<_, _>>()?,
    };
    Ok(config)
}
impl SettingsFile {
    pub(super) fn new(path: &Path, effective: &AgentCaptureConfig) -> Result<Self, Error> {
        Ok(Self {
            path: policy::outside_checkout(path).map_err(|_| Error::Unavailable)?,
            effective: effective.clone(),
        })
    }
    fn bytes(&self) -> Result<Vec<u8>, Error> {
        policy::read_regular(&self.path, 64 * 1024).map_err(|_| Error::Unavailable)
    }
    fn read(&self) -> Result<(String, Config), Error> {
        let bytes = self.bytes()?;
        let mut config: Config =
            toml::from_str(std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?)
                .map_err(|_| Error::Invalid)?;
        config
            .resolve_paths(self.path.parent().ok_or(Error::Invalid)?)
            .map_err(|_| Error::Invalid)?;
        config.validate().map_err(|_| Error::Invalid)?;
        Ok((hex::encode(Sha256::digest(&bytes)), config))
    }
    fn status(&self) -> Result<LocalCaptureSettings, Error> {
        let (revision, config) = self.read()?;
        let home = std::env::var_os("HOME").map(PathBuf::from);
        let codex_home = std::env::var_os("CODEX_HOME").map(PathBuf::from);
        let defaults = [
            CaptureProfile::CodexCliJsonlV1,
            CaptureProfile::CodexDesktopJsonlV1,
            CaptureProfile::CursorDesktopTranscriptV1,
            CaptureProfile::CursorCliTranscriptV1,
            CaptureProfile::OpencodeReadOnlyApiV1,
        ]
        .into_iter()
        .map(|p| {
            let h = HarnessConfig {
                profile: p,
                product_version: p.product_version().into(),
                roots: vec![],
                endpoint: None,
            };
            let roots = h
                .roots(home.as_deref(), codex_home.as_deref())
                .map_err(|_| Error::Unavailable)?;
            Ok(CaptureHarnessSetting {
                endpoint: None,
                profile: profile(p)?,
                product_version: h.product_version,
                roots: roots
                    .into_iter()
                    .map(|p| {
                        p.to_str()
                            .map(|s| CaptureRootPath(s.into()))
                            .ok_or(Error::Unavailable)
                    })
                    .collect::<Result<_, _>>()?,
            })
        })
        .collect::<Result<_, _>>()?;
        let desired = wire(&config.agent_capture)?;
        let effective = wire(&self.effective)?;
        let restart_required = desired != effective;
        Ok(LocalCaptureSettings {
            version: 1,
            revision,
            desired,
            effective,
            restart_required,
            defaults,
        })
    }
    fn save(
        &self,
        request: &SaveCaptureSettings,
        check: impl Fn() -> Result<(), Error>,
    ) -> Result<(), Error> {
        if request.version != 1
            || request.expected_revision.len() != 64
            || !request
                .expected_revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::Invalid);
        }
        let desired = native(&request.configuration)?;
        let mut lock_path = self.path.as_os_str().to_owned();
        lock_path.push(".settings.lock");
        let _lock = policy::exclusive_lock(Path::new(&lock_path)).map_err(|_| Error::Busy)?;
        check()?;
        let (revision, mut config) = self.read()?;
        if revision != request.expected_revision {
            return Err(Error::Stale);
        }
        config.agent_capture = desired.clone();
        config.validate().map_err(|_| Error::Invalid)?;
        let bytes = self.bytes()?;
        if hex::encode(Sha256::digest(&bytes)) != revision {
            return Err(Error::Stale);
        }
        let mut doc: toml::Value =
            toml::from_str(std::str::from_utf8(&bytes).map_err(|_| Error::Invalid)?)
                .map_err(|_| Error::Invalid)?;
        doc.as_table_mut().ok_or(Error::Invalid)?.insert(
            "agent_capture".into(),
            toml::Value::try_from(desired).map_err(|_| Error::Invalid)?,
        );
        let output = toml::to_string(&doc).map_err(|_| Error::Invalid)?;
        if output.len() > 64 * 1024 {
            return Err(Error::Invalid);
        }
        let parent = self.path.parent().ok_or(Error::Invalid)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        temp.write_all(output.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|_| Error::Unavailable)?;
        check()?;
        if self.read()?.0 != revision {
            return Err(Error::Stale);
        }
        temp.persist(&self.path).map_err(|_| Error::Unknown)?;
        File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)?;
        Ok(())
    }
}
impl Manager {
    pub(super) async fn read_settings(
        &self,
        actor: PrincipalId,
    ) -> Result<LocalCaptureSettings, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            this.0.authority.configuration(actor)?;
            let status = this.0.settings.status()?;
            this.0.authority.configuration(actor)?;
            Ok(status)
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
    pub(super) async fn write_settings(
        &self,
        actor: PrincipalId,
        request: SaveCaptureSettings,
    ) -> Result<LocalCaptureSettings, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            this.0.authority.configuration(actor)?;
            this.0.settings.save(&request, || {
                this.0.authority.configuration(actor).map(|_| ())
            })?;
            this.0.changes.notify(
                None,
                jj_store_service::freshness::Domain::Configuration,
                None,
            );
            this.0
                .authority
                .configuration(actor)
                .map_err(|_| Error::Unknown)?;
            this.0.settings.status().map_err(|_| Error::Unknown)
        })
        .await
        .map_err(|_| Error::Unknown)?
    }
}
