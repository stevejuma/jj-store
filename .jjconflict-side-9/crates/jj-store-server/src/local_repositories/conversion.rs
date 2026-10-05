//! Installation-owned independent conversion; accepted retries never reread the source.
use super::*;
use jj_store_model::EpochId;
use jj_store_service::{
    auth::Permission,
    conversion::{RepositoryArchiveV1, contracts::*},
};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};
mod journal;
mod prefix;
mod preparation;
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Conversion {
    version: u32,
    installation: String,
    request: StartConversion,
    repository: RepositoryId,
    epoch: EpochId,
    destination: Option<PathBuf>,
    attempt: Option<String>,
    prepared: bool,
    prepared_digest: Option<String>,
    publication_attempted: bool,
    committed: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Prepared {
    report: ConversionReport,
    native: bool,
    metadata: Option<jj_store_jj::local_metadata::GitOid>,
}
fn digest(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn fresh() -> Result<[u8; 16]> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
    Ok(bytes)
}
impl DeviceRepositories {
    pub(super) fn conversion_name_reserved(&self, name: &str) -> Result<bool> {
        for entry in fs::read_dir(self.path.parent().unwrap()).map_err(|_| Error::Unavailable)? {
            let entry = entry.map_err(|_| Error::Unavailable)?;
            if let Some(id) = entry
                .file_name()
                .to_str()
                .and_then(|s| s.strip_prefix("conversion-"))
                .and_then(|s| s.strip_suffix(".json"))
            {
                let id = id.parse().map_err(|_| Error::Unavailable)?;
                if self
                    .read_conversion(id)?
                    .is_some_and(|v| !v.committed && v.request.intent.name == name)
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    fn validate_conversion(&self, intent: &ConversionIntent) -> Result<Option<PathBuf>> {
        if self.host.is_none() {
            return Err(Error::Unsupported);
        }
        if intent.version != 1
            || intent.source_profile == intent.destination_profile
            || !crate::registry::valid_name(&intent.name)
        {
            return Err(Error::Invalid);
        }
        match (intent.destination_profile, &intent.path) {
            (ConversionProfile::Native, None) => Ok(None),
            (ConversionProfile::LocalJjGit, Some(path)) if valid_path(path) => {
                let path = Path::new(path);
                let parent = path
                    .parent()
                    .ok_or(Error::Invalid)?
                    .canonicalize()
                    .map_err(|_| Error::Invalid)?;
                if !parent.is_dir() {
                    return Err(Error::Invalid);
                }
                let destination = parent.join(path.file_name().ok_or(Error::Invalid)?);
                match fs::symlink_metadata(&destination) {
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Some(destination)),
                    Ok(_) => Err(Error::Conflict),
                    Err(_) => Err(Error::Unavailable),
                }
            }
            _ => Err(Error::Invalid),
        }
    }
    async fn source_archive(
        &self,
        intent: &ConversionIntent,
    ) -> jj_store_service::conversion::Result<RepositoryArchiveV1> {
        match intent.source_profile {
            ConversionProfile::LocalJjGit => {
                self.open(intent.source)?
                    .conversion_archive(intent.operation)
                    .await
            }
            ConversionProfile::Native => {
                if self
                    .read()?
                    .bindings
                    .iter()
                    .any(|b| b.folder.repository == Some(intent.source))
                {
                    return Err("source profile changed".into());
                }
                self.host
                    .as_ref()
                    .ok_or("native storage unavailable")?
                    .endpoint(intent.source)?
                    .bind_local_trusted(
                        self.identity.principal(b"loopback-http"),
                        Permission::Admin,
                    )?
                    .into_session()
                    .conversion_archive(intent.operation)
                    .await
            }
        }
    }
    pub(super) async fn preview_conversion_inner(
        &self,
        intent: ConversionIntent,
    ) -> Result<ConversionPreview> {
        self.validate_conversion(&intent)?;
        let preview = async {
            let archive = self.source_archive(&intent).await?;
            let bytes = archive.encode()?;
            // Exercise the full destination translator and provider audits before acceptance.
            self.preflight(&archive, intent.destination_profile).await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>((
                digest(&bytes),
                bytes.len(),
                archive.collaboration.len(),
            ))
        }
        .await;
        Ok(match preview {
            Ok((source, source_bytes, history_selections)) => ConversionPreview {
                version: 1,
                prepared_source: Some(source),
                source_bytes,
                history_selections,
                blocker: None,
            },
            Err(error) => ConversionPreview {
                version: 1,
                prepared_source: None,
                source_bytes: 0,
                history_selections: 0,
                blocker: Some(error.to_string().chars().take(512).collect()),
            },
        })
    }
    pub(super) async fn start_conversion_inner(
        &self,
        request: StartConversion,
    ) -> Result<ConversionOutcome> {
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        if let Some(mut value) = self.read_conversion(request.request_id)? {
            if value.request != request {
                return Err(Error::Conflict);
            }
            return self.finish_conversion(&mut value, true).await;
        }
        let destination = self.validate_conversion(&request.intent)?;
        if request.prepared_source.len() != 64 || hex::decode(&request.prepared_source).is_err() {
            return Err(Error::Invalid);
        }
        let name = &request.intent.name;
        let local = self.read()?;
        let native = self
            .native
            .snapshot(None)
            .await
            .map_err(|_| Error::Unavailable)?;
        if native.repositories.iter().any(|r| r.name == *name) {
            return Err(Error::Conflict);
        }
        if local.bindings.iter().any(|b| b.name == *name)
            || local
                .requests
                .iter()
                .any(|j| j.result.is_none() && j.request.name == *name)
            || self.creation_name_reserved(name)?
            || self.conversion_name_reserved(name)?
        {
            return Err(Error::Conflict);
        }
        if fs::read_dir(self.path.parent().unwrap())
            .map_err(|_| Error::Unavailable)?
            .filter_map(std::result::Result::ok)
            .filter(|e| {
                e.file_name().to_string_lossy().starts_with("conversion-")
                    && e.path().extension().is_some_and(|e| e == "json")
            })
            .count()
            >= 4096
        {
            return Err(Error::Limit);
        }
        let archive = self
            .source_archive(&request.intent)
            .await
            .map_err(|_| Error::Unsupported)?;
        let bytes = archive.encode().map_err(|_| Error::Limit)?;
        if digest(&bytes) != request.prepared_source {
            return Err(Error::Stale);
        }
        self.preflight(&archive, request.intent.destination_profile)
            .await
            .map_err(|_| Error::Unsupported)?;
        let mut value = Conversion {
            version: 1,
            installation: self.identity.installation.clone(),
            request,
            repository: RepositoryId::new(fresh()?),
            epoch: EpochId::new(fresh()?),
            destination,
            attempt: None,
            prepared: false,
            prepared_digest: None,
            publication_attempted: false,
            committed: false,
        };
        self.write_conversion_file(
            &self.conversion_file(value.request.request_id, "archive"),
            &bytes,
        )?;
        self.save_conversion(&value)?;
        self.finish_conversion(&mut value, true).await
    }
    pub(super) async fn conversion_status_inner(
        &self,
        request: RequestId,
    ) -> Result<ConversionOutcome> {
        if self.host.is_none() {
            return Err(Error::Unsupported);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let Some(mut value) = self.read_conversion(request)? else {
            return Ok(ConversionOutcome::NotKnown {
                request_id: request,
            });
        };
        self.finish_conversion(&mut value, false).await
    }
    async fn finish_conversion(
        &self,
        value: &mut Conversion,
        retry: bool,
    ) -> Result<ConversionOutcome> {
        let request_id = value.request.request_id;
        let unknown = || ConversionOutcome::OutcomeUnknown { request_id };
        if value.committed {
            return Ok(ConversionOutcome::Committed {
                report: Box::new(self.read_prepared(value)?.report),
            });
        }
        if !retry && !value.prepared && value.attempt.is_some() {
            return Ok(unknown());
        }
        if value.request.intent.destination_profile == ConversionProfile::Native {
            self.native
                .reserve(value.repository, value.request.intent.name.clone())
                .await
                .map_err(|_| Error::Conflict)?;
        }
        if !value.prepared {
            let bytes = self.read_conversion_file(
                &self.conversion_file(value.request.request_id, "archive"),
                16 * 1024 * 1024,
            )?;
            if digest(&bytes) != value.request.prepared_source {
                return Err(Error::IdentityChanged);
            }
            let archive = RepositoryArchiveV1::decode(&bytes).map_err(|_| Error::Unavailable)?;
            if archive.repository != value.request.intent.source {
                return Err(Error::IdentityChanged);
            }
            match (&archive.code, value.request.intent.source_profile) {
                (
                    jj_store_jj::conversion::CodeArchiveV1::Git { capsule, .. },
                    ConversionProfile::LocalJjGit,
                ) if capsule.operation_id == value.request.intent.operation => {}
                (
                    jj_store_jj::conversion::CodeArchiveV1::Native { operation, .. },
                    ConversionProfile::Native,
                ) if *operation == value.request.intent.operation => {}
                _ => return Err(Error::IdentityChanged),
            }
            // An unfinished attempt is unpublished. Start fresh from the accepted immutable archive.
            value.attempt = Some(hex::encode(fresh()?));
            self.save_conversion(value)?;
            let prepared = match self.prepare_conversion(value, &archive).await {
                Ok(v) => v,
                Err(_) => return Ok(unknown()),
            };
            let bytes = serde_json::to_vec(&prepared).map_err(|_| Error::Unavailable)?;
            self.write_conversion_file(
                &self.conversion_file(value.request.request_id, "prepared"),
                &bytes,
            )?;
            value.prepared_digest = Some(digest(&bytes));
            value.prepared = true;
            self.save_conversion(value)?;
        }
        let prepared = self.read_prepared(value)?;
        if value.committed {
            return Ok(ConversionOutcome::Committed {
                report: Box::new(prepared.report),
            });
        }
        value.publication_attempted = true;
        self.save_conversion(value)?;
        match self.publish_conversion(value, &prepared).await {
            Ok(()) => {}
            Err(Error::Conflict | Error::IdentityChanged) => return Err(Error::Conflict),
            Err(_) => return Ok(unknown()),
        }
        value.committed = true;
        self.save_conversion(value)?;
        Ok(ConversionOutcome::Committed {
            report: Box::new(prepared.report),
        })
    }
}
