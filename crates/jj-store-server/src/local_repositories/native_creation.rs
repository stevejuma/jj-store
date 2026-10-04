//! Durable installation creation identities precede name reservation and manifest CAS.
use super::*;
use jj_store_model::EpochId;
use jj_store_service::{administration::RepositoryHost, auth::Permission};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Creation {
    version: u32,
    installation: String,
    request: CreateNativeRepository,
    repository: RepositoryId,
    epoch: EpochId,
    committed: bool,
}
impl DeviceRepositories {
    pub fn with_native_host(mut self, host: Option<RepositoryHost>) -> Self {
        self.host = host;
        self
    }
    pub(super) fn creation_name_reserved(&self, name: &str) -> Result<bool> {
        for entry in fs::read_dir(self.path.parent().unwrap()).map_err(|_| Error::Unavailable)? {
            let entry = entry.map_err(|_| Error::Unavailable)?;
            let name_on_disk = entry.file_name();
            let text = name_on_disk.to_str().ok_or(Error::Unavailable)?;
            if let Some(id) = text
                .strip_prefix("native-create-")
                .and_then(|s| s.strip_suffix(".json"))
            {
                let request = id.parse::<RequestId>().map_err(|_| Error::Unavailable)?;
                if self
                    .read_creation(request)?
                    .is_some_and(|v| v.request.name == name)
                {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    fn local_name_reserved(&self, name: &str) -> Result<bool> {
        let registry = self.read()?;
        Ok(registry.bindings.iter().any(|b| b.name == name)
            || registry
                .requests
                .iter()
                .any(|j| j.result.is_none() && j.request.name == name))
    }
    fn creation_path(&self, request: RequestId) -> PathBuf {
        self.path
            .parent()
            .unwrap()
            .join(format!("native-create-{request}.json"))
    }
    fn read_creation(&self, request: RequestId) -> Result<Option<Creation>> {
        let path = self.creation_path(request);
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Unavailable),
            Ok(info) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt as _;
                    if info.permissions().mode() & 0o077 != 0 {
                        return Err(Error::Unavailable);
                    }
                }
                if !info.is_file() || info.file_type().is_symlink() {
                    return Err(Error::Unavailable);
                }
            }
        }
        let bytes = policy::read_regular(&path, 8192).map_err(|_| Error::Unavailable)?;
        let value: Creation = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if value.version != 1
            || value.installation != self.identity.installation
            || value.request.request_id != request
            || value.request.version != 1
            || !crate::registry::valid_name(&value.request.name)
        {
            return Err(Error::Unavailable);
        }
        Ok(Some(value))
    }
    fn save_creation(&self, value: &Creation) -> Result<()> {
        let path = self.creation_path(value.request.request_id);
        let mut file = tempfile::NamedTempFile::new_in(path.parent().unwrap())
            .map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            file.as_file()
                .set_permissions(fs::Permissions::from_mode(0o600))
                .map_err(|_| Error::Unavailable)?;
        }
        file.write_all(&serde_json::to_vec(value).map_err(|_| Error::Unavailable)?)
            .map_err(|_| Error::Unavailable)?;
        file.as_file().sync_all().map_err(|_| Error::Unavailable)?;
        file.persist(&path).map_err(|_| Error::Unavailable)?;
        fs::File::open(path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)
    }
    pub(super) async fn create_native_inner(
        &self,
        request: CreateNativeRepository,
    ) -> Result<NativeCreationOutcome> {
        if self.host.is_none() {
            return Err(Error::Unsupported);
        }
        if request.version != 1 || !crate::registry::valid_name(&request.name) {
            return Err(Error::Invalid);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut value = match self.read_creation(request.request_id)? {
            Some(value) if value.request == request => value,
            Some(_) => return Err(Error::Conflict),
            None => {
                if self.local_name_reserved(&request.name)?
                    || self.conversion_name_reserved(&request.name)?
                {
                    return Err(Error::Conflict);
                }
                if fs::read_dir(self.path.parent().unwrap())
                    .map_err(|_| Error::Unavailable)?
                    .filter_map(std::result::Result::ok)
                    .filter(|e| {
                        e.file_name()
                            .to_string_lossy()
                            .starts_with("native-create-")
                    })
                    .count()
                    >= 4096
                {
                    return Err(Error::Limit);
                }
                let mut repository = [0; 16];
                let mut epoch = [0; 16];
                getrandom::fill(&mut repository).map_err(|_| Error::Unavailable)?;
                getrandom::fill(&mut epoch).map_err(|_| Error::Unavailable)?;
                let value = Creation {
                    version: 1,
                    installation: self.identity.installation.clone(),
                    request,
                    repository: RepositoryId::new(repository),
                    epoch: EpochId::new(epoch),
                    committed: false,
                };
                self.save_creation(&value)?;
                value
            }
        };
        self.finish_creation(&mut value).await
    }
    pub(super) async fn native_status_inner(
        &self,
        request: RequestId,
    ) -> Result<NativeCreationOutcome> {
        if self.host.is_none() {
            return Err(Error::Unsupported);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let Some(mut value) = self.read_creation(request)? else {
            return Ok(NativeCreationOutcome::NotKnown {
                request_id: request,
            });
        };
        self.finish_creation(&mut value).await
    }
    async fn finish_creation(&self, value: &mut Creation) -> Result<NativeCreationOutcome> {
        let host = self.host.as_ref().ok_or(Error::Unsupported)?;
        if self.local_name_reserved(&value.request.name)? {
            return Err(Error::Conflict);
        }
        self.native
            .reserve(value.repository, value.request.name.clone())
            .await
            .map_err(|e| match e {
                jj_store_service::registry::Error::Conflict => Error::Conflict,
                _ => Error::Unavailable,
            })?;
        if !value.committed {
            let snapshot = match host
                .create_local_trusted(value.repository, value.epoch, Permission::Admin)
                .await
            {
                Ok(snapshot) => Some(snapshot.epoch),
                Err(error) => match jj_store_service::transport::Error::from(error) {
                    jj_store_service::transport::Error::Conflict => None,
                    jj_store_service::transport::Error::Unknown => {
                        return Ok(NativeCreationOutcome::OutcomeUnknown {
                            request_id: value.request.request_id,
                        });
                    }
                    _ => return Err(Error::Unavailable),
                },
            };
            let epoch = match snapshot {
                Some(epoch) => epoch,
                None => {
                    host.endpoint(value.repository)
                        .map_err(|_| Error::Unavailable)?
                        .bind_local_trusted(
                            self.identity.principal(b"loopback-http"),
                            Permission::Admin,
                        )
                        .map_err(|_| Error::Unavailable)?
                        .into_session()
                        .snapshot()
                        .await
                        .map_err(|_| Error::Unavailable)?
                        .manifest()
                        .epoch
                }
            };
            if epoch != value.epoch {
                return Err(Error::IdentityChanged);
            }
        }
        self.native
            .register(value.repository, value.request.name.clone(), None)
            .await
            .map_err(|_| Error::Unavailable)?;
        value.committed = true;
        self.save_creation(value)?;
        Ok(NativeCreationOutcome::Committed {
            request_id: value.request.request_id,
            repository: value.repository,
            name: value.request.name.clone(),
        })
    }
}
