//! Bounded provider preparation and native manifest-last/local no-replace publication.
use super::*;
use jj_store_jj::{
    conversion::IdentityMappingsV1,
    local_metadata::{Limits as MetadataLimits, MetadataStore},
    objects::NativeObjects,
};
use jj_store_object::{ObjectStore, memory::MemoryStore};
use jj_store_service::conversion::{CollaborationMappingsV1, publish_directory};
use std::collections::BTreeMap;
type PreparedState = (
    jj_store_model::OperationId,
    IdentityMappingsV1,
    CollaborationMappingsV1,
);
impl DeviceRepositories {
    pub(super) async fn preflight(
        &self,
        archive: &RepositoryArchiveV1,
        profile: ConversionProfile,
    ) -> jj_store_service::conversion::Result<()> {
        let root = tempfile::tempdir()?;
        let repository = RepositoryId::new(fresh()?);
        match profile {
            ConversionProfile::Native => {
                let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
                let host = self
                    .host
                    .as_ref()
                    .ok_or("native host unavailable")?
                    .with_conversion_store(store.clone());
                self.prepare_native(
                    &host,
                    archive,
                    repository,
                    EpochId::new(fresh()?),
                    RequestId::new(fresh()?),
                    root.path(),
                )
                .await?;
            }
            ConversionProfile::LocalJjGit => {
                self.prepare_local(
                    archive,
                    repository,
                    RequestId::new(fresh()?),
                    root.path(),
                    &root.path().join("destination"),
                )
                .await?;
            }
        }
        Ok(())
    }
    fn staging(&self, value: &Conversion) -> Result<PathBuf> {
        let destination = value.destination.as_ref().ok_or(Error::Invalid)?;
        Ok(destination.parent().ok_or(Error::Invalid)?.join(format!(
            ".jj-store-conversion-{}-{}",
            value.request.request_id,
            value.attempt.as_ref().ok_or(Error::Unavailable)?
        )))
    }
    fn staging_host(
        &self,
        value: &Conversion,
    ) -> Result<(
        jj_store_service::administration::RepositoryHost,
        Arc<dyn ObjectStore>,
    )> {
        let host = self.host.as_ref().ok_or(Error::Unsupported)?;
        let store: Arc<dyn ObjectStore> = Arc::new(prefix::Prefix {
            store: host.conversion_store(),
            prefix: format!(
                "conversion-staging/{}/{}",
                value.request.request_id,
                value.attempt.as_ref().ok_or(Error::Unavailable)?
            ),
        });
        Ok((host.with_conversion_store(store.clone()), store))
    }
    async fn prepare_native(
        &self,
        host: &jj_store_service::administration::RepositoryHost,
        archive: &RepositoryArchiveV1,
        repository: RepositoryId,
        epoch: EpochId,
        request: RequestId,
        scratch: &Path,
    ) -> jj_store_service::conversion::Result<PreparedState> {
        host.create_local_trusted(repository, epoch, Permission::Admin)
            .await?;
        let session = host
            .endpoint(repository)?
            .bind_local_trusted(self.identity.principal(b"loopback-http"), Permission::Admin)?
            .into_session();
        let collaboration = session.import_conversion_history(archive, request).await?;
        let objects = Arc::new(NativeObjects::new(
            host.conversion_store(),
            repository,
            16 * 1024 * 1024,
        )?);
        let (operation, code) =
            jj_store_jj::conversion::to_native(&archive.code, scratch, objects).await?;
        if !session
            .publish_conversion_code(request, operation, &digest(&archive.encode()?))
            .await?
        {
            return Err("private code publication unknown".into());
        }
        // Full cold-copy audit covers current and historical provider closures.
        session
            .copy_conversion_to(Arc::new(MemoryStore::default()))
            .await?;
        if serde_json::to_vec(&(operation, &code, &collaboration))?.len() > 2 * 1024 * 1024 {
            return Err("conversion mapping report limit".into());
        }
        Ok((operation, code, collaboration))
    }
    async fn prepare_local(
        &self,
        archive: &RepositoryArchiveV1,
        repository: RepositoryId,
        request: RequestId,
        scratch: &Path,
        destination: &Path,
    ) -> jj_store_service::conversion::Result<PreparedState> {
        let (operation, code) =
            jj_store_jj::conversion::to_local(&archive.code, scratch, destination).await?;
        let local = LocalRepository::new(
            &destination.join(".jj/repo"),
            MetadataStore::open(&destination.join(".git"), MetadataLimits::default())?,
        );
        let collaboration = local
            .import_conversion_history(
                archive,
                repository,
                self.identity.principal(b"loopback-http"),
                request,
            )
            .await?;
        if local.repository()? != Some(repository) {
            return Err("conversion destination identity".into());
        }
        if serde_json::to_vec(&(operation, &code, &collaboration))?.len() > 2 * 1024 * 1024 {
            return Err("conversion mapping report limit".into());
        }
        Ok((operation, code, collaboration))
    }
    pub(super) async fn prepare_conversion(
        &self,
        value: &Conversion,
        archive: &RepositoryArchiveV1,
    ) -> jj_store_service::conversion::Result<Prepared> {
        let scratch = tempfile::tempdir()?;
        let (state, metadata) = match value.request.intent.destination_profile {
            ConversionProfile::Native => {
                let (host, _) = self.staging_host(value)?;
                (
                    self.prepare_native(
                        &host,
                        archive,
                        value.repository,
                        value.epoch,
                        value.request.request_id,
                        scratch.path(),
                    )
                    .await?,
                    None,
                )
            }
            ConversionProfile::LocalJjGit => {
                let staging = self.staging(value)?;
                let state = self
                    .prepare_local(
                        archive,
                        value.repository,
                        value.request.request_id,
                        scratch.path(),
                        &staging,
                    )
                    .await?;
                let local = LocalRepository::new(
                    &staging.join(".jj/repo"),
                    MetadataStore::open(&staging.join(".git"), MetadataLimits::default())?,
                );
                (
                    state,
                    Some(
                        local
                            .metadata_store()
                            .read()?
                            .ok_or("destination metadata")?
                            .commit(),
                    ),
                )
            }
        };
        let (operation, code, collaboration) = state;
        let code_mappings = BTreeMap::from([
            ("operations".into(), code.operations),
            ("views".into(), code.views),
            ("commits".into(), code.commits),
            ("trees".into(), code.trees),
            ("files".into(), code.files),
            ("symlinks".into(), code.symlinks),
        ]);
        Ok(Prepared {
            native: value.request.intent.destination_profile == ConversionProfile::Native,
            metadata,
            report: ConversionReport {
                version: 1,
                request_id: value.request.request_id,
                source: value.request.intent.source,
                destination: value.repository,
                destination_profile: value.request.intent.destination_profile,
                name: value.request.intent.name.clone(),
                operation,
                archive_digest: value.request.prepared_source.clone(),
                code_mappings,
                collaboration,
            },
        })
    }
    pub(super) async fn publish_conversion(
        &self,
        value: &Conversion,
        prepared: &Prepared,
    ) -> Result<()> {
        let host = self.host.as_ref().ok_or(Error::Unsupported)?;
        if prepared.native {
            let store = host.conversion_store();
            let key =
                jj_store_object::Key::new(format!("repos/{}/manifest.json", value.repository))
                    .map_err(|_| Error::Invalid)?;
            match store.head(&key).await {
                Ok(_) => {
                    let session = host
                        .endpoint(value.repository)
                        .map_err(|_| Error::Unavailable)?
                        .bind_local_trusted(
                            self.identity.principal(b"loopback-http"),
                            Permission::Admin,
                        )
                        .map_err(|_| Error::Unavailable)?
                        .into_session();
                    if session
                        .snapshot()
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .manifest()
                        .epoch
                        != value.epoch
                        || !session
                            .conversion_code_receipt(
                                value.request.request_id,
                                prepared.report.operation,
                                &prepared.report.archive_digest,
                            )
                            .await
                            .map_err(|_| Error::Unavailable)?
                    {
                        return Err(Error::IdentityChanged);
                    }
                }
                Err(jj_store_object::Error::NotFound) => {
                    let (staged, _) = self.staging_host(value)?;
                    staged
                        .endpoint(value.repository)
                        .map_err(|_| Error::Unavailable)?
                        .bind_local_trusted(
                            self.identity.principal(b"loopback-http"),
                            Permission::Admin,
                        )
                        .map_err(|_| Error::Unavailable)?
                        .into_session()
                        .copy_conversion_to(store)
                        .await
                        .map_err(|_| Error::Unavailable)?;
                }
                Err(_) => return Err(Error::Unavailable),
            }
            self.native
                .register(value.repository, value.request.intent.name.clone(), None)
                .await
                .map_err(|_| Error::Unavailable)?;
        } else {
            let destination = value.destination.as_ref().ok_or(Error::Invalid)?;
            if destination
                .parent()
                .ok_or(Error::Invalid)?
                .canonicalize()
                .map_err(|_| Error::Unavailable)?
                != destination.parent().unwrap()
            {
                return Err(Error::IdentityChanged);
            }
            match fs::symlink_metadata(destination) {
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    publish_directory(&self.staging(value)?, destination)
                        .map_err(|_| Error::Unavailable)?
                }
                Ok(info) if info.is_dir() && !info.file_type().is_symlink() => {}
                Ok(_) => return Err(Error::Conflict),
                Err(_) => return Err(Error::Unavailable),
            }
            let folder = self
                .provider
                .inspect(destination.to_str().ok_or(Error::Invalid)?)?;
            if folder.repository != Some(value.repository) {
                return Err(Error::IdentityChanged);
            }
            let local = self.provider.open(&folder)?;
            let selected = local
                .metadata_store()
                .read()
                .map_err(|_| Error::Unavailable)?
                .ok_or(Error::IdentityChanged)?;
            if !local
                .metadata_store()
                .history(&selected)
                .map_err(|_| Error::Unavailable)?
                .iter()
                .any(|s| Some(s.commit()) == prepared.metadata)
            {
                return Err(Error::IdentityChanged);
            }
            let mut registry = self.read()?;
            if !registry
                .bindings
                .iter()
                .any(|b| b.folder.repository == Some(value.repository))
            {
                if registry
                    .bindings
                    .iter()
                    .any(|b| b.name == value.request.intent.name)
                    || registry.bindings.len() >= 256
                {
                    return Err(Error::Conflict);
                }
                registry.revision = registry.revision.checked_add(1).ok_or(Error::Limit)?;
                registry.bindings.push(SavedBinding {
                    folder,
                    name: value.request.intent.name.clone(),
                    generation: registry.revision,
                });
                registry
                    .bindings
                    .sort_by_key(|binding| binding.folder.repository);
                self.persist(&registry)?;
            }
        }
        Ok(())
    }
}
