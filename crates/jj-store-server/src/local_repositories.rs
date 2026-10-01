//! Private device bindings and durable registration requests, never code authority.
use crate::{installation::Identity, policy};
use jj_store_model::{RepositoryId, RequestId};
use jj_store_service::{
    local_browser::*,
    local_repository::{LocalAgentOutcome, LocalRepository},
    registry::RepositoryRegistry,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write as _, path::PathBuf, sync::Arc};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedBinding {
    folder: LocalFolder,
    name: String,
    generation: u64,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    request: RegisterFolder,
    folder: LocalFolder,
    proposed_repository: RepositoryId,
    result: Option<RegistrationOutcome>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    version: u32,
    installation: String,
    revision: u64,
    bindings: Vec<SavedBinding>,
    requests: Vec<Journal>,
}

#[derive(Clone)]
pub struct DeviceRepositories {
    path: PathBuf,
    identity: Identity,
    provider: Arc<dyn FolderProvider>,
    native: Arc<dyn RepositoryRegistry>,
}
impl DeviceRepositories {
    pub fn new(
        state: PathBuf,
        identity: Identity,
        provider: Arc<dyn FolderProvider>,
        native: Arc<dyn RepositoryRegistry>,
    ) -> crate::Result<Self> {
        let state = policy::outside_checkout(&state)?;
        if !state.is_dir() {
            return Err(crate::Error::Config("local repository registry directory"));
        }
        Ok(Self {
            path: state.join("local-repositories.json"),
            identity,
            provider,
            native,
        })
    }
    async fn register_inner(&self, request: RegisterFolder) -> Result<RegistrationOutcome> {
        if request.version != 1
            || !valid_path(&request.path)
            || !crate::registry::valid_name(&request.name)
        {
            return Err(Error::Invalid);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut registry = self.read()?;
        if let Some(index) = registry
            .requests
            .iter()
            .position(|j| j.request.request_id == request.request_id)
        {
            if registry.requests[index].request != request {
                return Err(Error::Conflict);
            }
            return self.apply(&mut registry, index).await;
        }
        if registry.revision != request.expected_revision {
            return Err(Error::Stale);
        }
        if registry.bindings.len() >= 256 || registry.requests.len() >= 4096 {
            return Err(Error::Limit);
        }
        let folder = self.provider.inspect(&request.path)?;
        let native = self
            .native
            .snapshot(None)
            .await
            .map_err(|_| Error::Unavailable)?;
        if native
            .repositories
            .iter()
            .any(|r| r.name == request.name || Some(r.id) == folder.repository)
            || registry.bindings.iter().any(|b| {
                (b.name == request.name && b.folder.repository != folder.repository)
                    || (b.folder.repository == folder.repository
                        && b.folder.metadata != folder.metadata)
            })
        {
            return Err(Error::Conflict);
        }
        let proposed_repository = match folder.repository {
            Some(id) => id,
            None => {
                let mut bytes = [0; 16];
                getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
                RepositoryId::new(bytes)
            }
        };
        registry.requests.push(Journal {
            request,
            folder,
            proposed_repository,
            result: None,
        });
        let index = registry.requests.len() - 1;
        self.persist(&registry)?;
        self.apply(&mut registry, index).await
    }
    async fn status_inner(&self, request: RequestId) -> Result<RegistrationOutcome> {
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut registry = self.read()?;
        match registry
            .requests
            .iter()
            .position(|j| j.request.request_id == request)
        {
            Some(index) => self.apply(&mut registry, index).await,
            None => Ok(RegistrationOutcome::NotKnown {
                request_id: request,
            }),
        }
    }
    fn read(&self) -> Result<Registry> {
        match fs::symlink_metadata(&self.path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Registry {
                    version: 1,
                    installation: self.identity.installation.clone(),
                    revision: 0,
                    bindings: vec![],
                    requests: vec![],
                });
            }
            Err(_) => return Err(Error::Unavailable),
            Ok(metadata) => {
                if !metadata.is_file() || metadata.file_type().is_symlink() {
                    return Err(Error::Unavailable);
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt as _;
                    if metadata.permissions().mode() & 0o077 != 0 {
                        return Err(Error::Unavailable);
                    }
                }
            }
        }
        let bytes =
            policy::read_regular(&self.path, 4 * 1024 * 1024).map_err(|_| Error::Unavailable)?;
        let value: Registry = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if value.version != 1
            || value.installation != self.identity.installation
            || value.bindings.len() > 256
            || value.requests.len() > 4096
            || value.bindings.iter().any(|b| {
                b.folder.repository.is_none()
                    || !valid_path(&b.folder.path)
                    || !valid_path(&b.folder.metadata)
                    || !crate::registry::valid_name(&b.name)
            })
        {
            return Err(Error::Unavailable);
        }
        let mut ids = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::new();
        let mut metadata_paths = std::collections::BTreeSet::new();
        let mut generations = std::collections::BTreeSet::new();
        if value.bindings.iter().any(|b| {
            !ids.insert(b.folder.repository)
                || !names.insert(&b.name)
                || !metadata_paths.insert(&b.folder.metadata)
                || !generations.insert(b.generation)
        }) {
            return Err(Error::Unavailable);
        }
        let mut request_ids = std::collections::BTreeSet::new();
        if value
            .bindings
            .iter()
            .any(|b| b.generation == 0 || b.generation > value.revision)
            || value.requests.iter().any(|j| {
                !request_ids.insert(j.request.request_id)
                    || j.request.version != 1
                    || j.request.expected_revision > value.revision
                    || !valid_path(&j.request.path)
                    || !valid_path(&j.folder.path)
                    || !valid_path(&j.folder.metadata)
                    || !crate::registry::valid_name(&j.request.name)
                    || j.folder
                        .repository
                        .is_some_and(|id| id != j.proposed_repository)
                    || j.result.as_ref().is_some_and(|result| match result {
                        RegistrationOutcome::Committed {
                            request_id,
                            binding,
                            revision,
                        } => {
                            *request_id != j.request.request_id
                                || binding.repository != j.proposed_repository
                                || binding.name != j.request.name
                                || !valid_path(&binding.path)
                                || binding.generation == 0
                                || binding.generation > *revision
                                || *revision > value.revision
                        }
                        _ => true,
                    })
            })
        {
            return Err(Error::Unavailable);
        }
        Ok(value)
    }
    fn persist(&self, registry: &Registry) -> Result<()> {
        let bytes = serde_json::to_vec(registry).map_err(|_| Error::Unavailable)?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let parent = self.path.parent().ok_or(Error::Unavailable)?;
        let mut staging =
            tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        staging.write_all(&bytes).map_err(|_| Error::Unavailable)?;
        staging
            .as_file()
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        staging
            .persist(&self.path)
            .map_err(|_| Error::Unavailable)?;
        fs::File::open(parent)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)
    }
    fn binding(&self, binding: &SavedBinding) -> LocalBinding {
        let availability = match self.provider.inspect(&binding.folder.path) {
            Ok(folder) if folder == binding.folder => Availability::Ready,
            Ok(_) => Availability::IdentityChanged,
            Err(_) => Availability::Unavailable,
        };
        LocalBinding {
            repository: binding.folder.repository.expect("validated binding"),
            path: binding.folder.path.clone(),
            name: binding.name.clone(),
            generation: binding.generation,
            availability,
        }
    }
    fn snapshot(&self, registry: &Registry) -> LocalBindings {
        LocalBindings {
            version: 1,
            revision: registry.revision,
            bindings: registry.bindings.iter().map(|b| self.binding(b)).collect(),
        }
    }
    async fn apply(&self, registry: &mut Registry, index: usize) -> Result<RegistrationOutcome> {
        if let Some(result) = &registry.requests[index].result {
            return Ok(result.clone());
        }
        let journal = registry.requests[index].clone();
        let request_id = journal.request.request_id;
        let current = self.provider.inspect(&journal.folder.path)?;
        if current.metadata != journal.folder.metadata
            || journal
                .folder
                .repository
                .is_some_and(|id| current.repository != Some(id))
        {
            return Err(Error::IdentityChanged);
        }
        let repo = self.provider.open(&current)?;
        // After an interrupted first initialization, replay the original receipt.
        if journal.folder.repository.is_none() {
            let result = repo
                .initialize_for_browser(request_id, journal.proposed_repository)
                .await
                .map_err(|error| match error {
                    jj_store_service::local_repository::LocalError::Stale => Error::IdentityChanged,
                    _ => Error::Unavailable,
                })?;
            if !matches!(result, LocalAgentOutcome::Committed { .. }) {
                return Ok(RegistrationOutcome::OutcomeUnknown { request_id });
            }
        }
        let folder = self.provider.inspect(&journal.folder.path)?;
        let repository = folder.repository.ok_or(Error::Unavailable)?;
        if repository != journal.proposed_repository {
            return Err(Error::IdentityChanged);
        }
        let native = self
            .native
            .snapshot(None)
            .await
            .map_err(|_| Error::Unavailable)?;
        if native
            .repositories
            .iter()
            .any(|r| r.id == repository || r.name == journal.request.name)
        {
            return Err(Error::Conflict);
        }
        if let Some(binding) = registry
            .bindings
            .iter()
            .find(|b| b.folder.repository == Some(repository))
        {
            if binding.folder.metadata != folder.metadata || binding.name != journal.request.name {
                return Err(Error::Conflict);
            }
            let result = RegistrationOutcome::Committed {
                request_id,
                binding: self.binding(binding),
                revision: registry.revision,
            };
            registry.requests[index].result = Some(result.clone());
            self.persist(registry)?;
            return Ok(result);
        }
        if registry
            .bindings
            .iter()
            .any(|b| b.name == journal.request.name)
        {
            return Err(Error::Conflict);
        }
        registry.revision = registry.revision.checked_add(1).ok_or(Error::Limit)?;
        let binding = SavedBinding {
            folder,
            name: journal.request.name,
            generation: registry.revision,
        };
        let result = RegistrationOutcome::Committed {
            request_id,
            binding: self.binding(&binding),
            revision: registry.revision,
        };
        registry.bindings.push(binding);
        registry.bindings.sort_by_key(|b| b.folder.repository);
        registry.requests[index].result = Some(result.clone());
        if self.persist(registry).is_err() {
            return Ok(RegistrationOutcome::OutcomeUnknown { request_id });
        }
        Ok(result)
    }
}
fn valid_path(path: &str) -> bool {
    path.len() <= 4096 && !path.contains('\0') && std::path::Path::new(path).is_absolute()
}

#[async_trait::async_trait]
impl LocalRepositories for DeviceRepositories {
    fn inspect(&self, path: &str) -> Result<LocalFolder> {
        self.provider.inspect(path)
    }
    fn list(&self) -> Result<LocalBindings> {
        Ok(self.snapshot(&self.read()?))
    }
    fn folder(&self, repository: RepositoryId) -> Result<(LocalFolder, u64)> {
        let registry = self.read()?;
        let binding = registry
            .bindings
            .iter()
            .find(|b| b.folder.repository == Some(repository))
            .ok_or(Error::Unavailable)?;
        let folder = self.provider.inspect(&binding.folder.path)?;
        if folder != binding.folder {
            return Err(Error::IdentityChanged);
        }
        Ok((folder, binding.generation))
    }
    fn open(&self, repository: RepositoryId) -> Result<LocalRepository> {
        Ok(self
            .provider
            .open(&self.folder(repository)?.0)?
            .bound_to(repository))
    }
    async fn register(&self, request: RegisterFolder) -> Result<RegistrationOutcome> {
        let manager = self.clone();
        let runtime = tokio::runtime::Handle::current();
        tokio::task::spawn_blocking(move || runtime.block_on(manager.register_inner(request)))
            .await
            .map_err(|_| Error::Unavailable)?
    }
    async fn status(&self, request: RequestId) -> Result<RegistrationOutcome> {
        let manager = self.clone();
        let runtime = tokio::runtime::Handle::current();
        tokio::task::spawn_blocking(move || runtime.block_on(manager.status_inner(request)))
            .await
            .map_err(|_| Error::Unavailable)?
    }
    fn reconnect(
        &self,
        repository: RepositoryId,
        request: ReconnectFolder,
    ) -> Result<LocalBindings> {
        if request.version != 1 || !valid_path(&request.path) {
            return Err(Error::Invalid);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut registry = self.read()?;
        if request.expected_revision != registry.revision {
            return Err(Error::Stale);
        }
        let index = registry
            .bindings
            .iter()
            .position(|b| b.folder.repository == Some(repository))
            .ok_or(Error::Unavailable)?;
        let folder = self.provider.inspect(&request.path)?;
        if folder.repository != Some(repository) {
            return Err(Error::IdentityChanged);
        }
        let old = &registry.bindings[index];
        if self.binding(old).availability == Availability::Ready
            && folder.metadata != old.folder.metadata
        {
            return Err(Error::Conflict);
        }
        if registry
            .bindings
            .iter()
            .enumerate()
            .any(|(i, b)| i != index && b.folder.metadata == folder.metadata)
        {
            return Err(Error::Conflict);
        }
        if old.folder == folder {
            return Ok(self.snapshot(&registry));
        }
        registry.revision = registry.revision.checked_add(1).ok_or(Error::Limit)?;
        registry.bindings[index].folder = folder;
        registry.bindings[index].generation = registry.revision;
        self.persist(&registry)?;
        Ok(self.snapshot(&registry))
    }
    fn remove(&self, repository: RepositoryId, request: RemoveBinding) -> Result<LocalBindings> {
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut registry = self.read()?;
        if request.version != 1 {
            return Err(Error::Invalid);
        }
        if request.expected_revision != registry.revision {
            return Err(Error::Stale);
        }
        registry
            .bindings
            .retain(|b| b.folder.repository != Some(repository));
        registry.revision = registry.revision.checked_add(1).ok_or(Error::Limit)?;
        self.persist(&registry)?;
        Ok(self.snapshot(&registry))
    }
}
