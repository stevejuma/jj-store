//! Non-destructive handoff of an existing native workspace from an
//! authenticated remote connection to a complete LocalTrusted replica.
use crate::{attachment, binding, remote, replication_endpoint::RpcEndpoint};
use jj_lib::{
    default_backend_factories::{default_backend_factories, default_working_copy_factories},
    object_id::ObjectId as _,
    ref_name::WorkspaceNameBuf,
    settings::UserSettings,
    simple_workspace_store::SimpleWorkspaceStore,
    workspace::Workspace,
    workspace_store::WorkspaceStore as _,
};
use jj_store_jj::{
    backend::BackendLimits,
    operation_stores::{AuthorityResult, OperationLimits},
};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_replication::{
    Direction, PeerCheckpointStore, ReplicaCapabilitiesV1, ReplicationWorker, WorkerLimits,
};
use jj_store_rpc::client::NativeClient;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
};
use tonic::Code;

const JOURNAL_FORMAT: u32 = 1;
const MAX_JOURNAL: u64 = 128 * 1024;
const MAX_PULL_STEPS: usize = 1_000_000;
pub const BACKUP_NAME: &str = "repo.jj-store-remote-v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    format: u32,
    repository: m::RepositoryId,
    workspace_root: PathBuf,
    workspace: String,
    working_copy_operation: m::OperationId,
    old_cache: PathBuf,
    old_connection: binding::Connection,
    repo_descriptor_digest: [u8; 32],
    local_cache: PathBuf,
    local_connection: binding::Connection,
    local_epoch: m::EpochId,
    discovery_peer: m::ReplicaId,
    hydrated: bool,
    switched: bool,
}

pub struct Migration {
    state: PathBuf,
    record: Record,
    old_client: Option<NativeClient>,
    local_client: NativeClient,
    _lock: File,
}

#[derive(Clone, Debug, Serialize)]
pub struct Migrated {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub workspace: String,
    pub path: PathBuf,
    pub cache: PathBuf,
    pub backup: PathBuf,
    pub operation: m::OperationId,
}

impl Migration {
    pub async fn open(
        settings: &UserSettings,
        workspace_root: PathBuf,
        current_cache: PathBuf,
        local_connection: binding::Connection,
        local_cache: Option<PathBuf>,
        state: Option<PathBuf>,
    ) -> AuthorityResult<Self> {
        local_connection.validate()?;
        if local_connection.unix_socket.is_none() {
            return Err("workspace migration requires a LocalTrusted destination".into());
        }
        let workspace_root = attachment::target(&workspace_root)?;
        let current_cache = attachment::target(&current_cache)?;
        let current_connection = binding::Connection::read(&current_cache)?;
        if current_connection.repository != local_connection.repository {
            return Err("migration connections select different repositories".into());
        }
        let descriptor = descriptor(&workspace_root, &current_cache)?;
        let mut factories = default_backend_factories();
        factories.merge(binding::lazy_factories());
        let workspace = Workspace::load(
            settings,
            &workspace_root,
            &factories,
            &default_working_copy_factories(),
        )?;
        if workspace.repo_path() != current_cache {
            return Err("workspace repository descriptor changed during migration".into());
        }
        let workspace_name = workspace.workspace_name().as_str().to_owned();
        let operation =
            m::OperationId::from_slice(workspace.working_copy().operation_id().as_bytes())?;
        let state = migration_state(&workspace_root, state)?;
        if state.starts_with(&workspace_root) || workspace_root.starts_with(&state) {
            return Err("migration state must be separate from the physical checkout".into());
        }
        fs::create_dir_all(&state)?;
        let lock = jj_store_server::policy::exclusive_lock(&state.join("migration.lock"))?;
        let journal = state.join("migration.json");
        let requested_cache = local_cache
            .map(|path| attachment::target(&path))
            .transpose()?
            .unwrap_or_else(|| state.join("cache"));
        let record = if journal.exists() {
            let bytes = binding::bounded_read(&journal, MAX_JOURNAL)?;
            let record: Record = serde_json::from_slice(&bytes)
                .map_err(|_| "invalid workspace migration journal")?;
            if record.format != JOURNAL_FORMAT
                || record.repository != current_connection.repository
                || record.workspace_root != workspace_root
                || record.workspace != workspace_name
                || record.working_copy_operation != operation
                || (current_cache != record.old_cache && current_cache != record.local_cache)
                || record.local_cache != requested_cache
                || serde_json::to_value(&record.local_connection)?
                    != serde_json::to_value(&local_connection)?
            {
                return Err("workspace migration retry differs from its durable intent".into());
            }
            if current_cache == record.old_cache
                && (serde_json::to_value(&record.old_connection)?
                    != serde_json::to_value(&current_connection)?
                    || record.repo_descriptor_digest
                        != <[u8; 32]>::from(Sha256::digest(&descriptor)))
            {
                return Err("workspace migration source descriptor changed".into());
            }
            if current_cache == record.local_cache
                && serde_json::to_value(&record.local_connection)?
                    != serde_json::to_value(&current_connection)?
            {
                return Err("workspace migration destination descriptor changed".into());
            }
            record
        } else {
            if current_connection.unix_socket.is_some() {
                return Err("migration source must be an authenticated connection".into());
            }
            if requested_cache.starts_with(&workspace_root)
                || workspace_root.starts_with(&requested_cache)
                || requested_cache == current_cache
            {
                return Err(
                    "migration cache must be separate from the checkout and source cache".into(),
                );
            }
            Record {
                format: JOURNAL_FORMAT,
                repository: current_connection.repository,
                workspace_root,
                workspace: workspace_name,
                working_copy_operation: operation,
                old_cache: current_cache,
                old_connection: current_connection,
                repo_descriptor_digest: Sha256::digest(&descriptor).into(),
                local_cache: requested_cache,
                local_connection,
                local_epoch: random_id(m::EpochId::new)?,
                discovery_peer: random_id(m::ReplicaId::new)?,
                hydrated: false,
                switched: false,
            }
        };
        let old_client = if record.hydrated {
            None
        } else {
            let client = record.old_connection.connect().await?;
            client.authorize(true).await?;
            Some(client)
        };
        let local_client = record.local_connection.connect().await?;
        let this = Self {
            state,
            record,
            old_client,
            local_client,
            _lock: lock,
        };
        this.save()?;
        Ok(this)
    }

    pub async fn run(&mut self, settings: &UserSettings) -> AuthorityResult<Migrated> {
        if !self.record.hydrated {
            self.ensure_local_repository().await?;
            self.pull_complete_frontier().await?;
            self.prepare_local_cache(settings).await?;
            self.record.hydrated = true;
            self.save()?;
        } else {
            self.verify_local_cache(settings).await?;
        }
        self.switch_descriptor()?;
        self.verify_switched_workspace(settings).await?;
        self.record.switched = true;
        self.save()?;
        Ok(Migrated {
            version: 1,
            repository: self.record.repository,
            workspace: self.record.workspace.clone(),
            path: self.record.workspace_root.clone(),
            cache: self.record.local_cache.clone(),
            backup: self.backup_path(),
            operation: self.record.working_copy_operation,
        })
    }

    async fn ensure_local_repository(&self) -> AuthorityResult<()> {
        match self.local_client.authorize(true).await {
            Ok(()) => Ok(()),
            Err(status) if matches!(status.code(), Code::NotFound | Code::PermissionDenied) => {
                self.local_client
                    .create_repository_local_named(
                        self.record.local_epoch,
                        &format!("local-{}", self.record.repository),
                    )
                    .await?;
                self.local_client.authorize(true).await?;
                Ok(())
            }
            Err(status) => Err(status.into()),
        }
    }

    async fn pull_complete_frontier(&self) -> AuthorityResult<()> {
        let capabilities = ReplicaCapabilitiesV1::default();
        let local = self
            .local_client
            .negotiate_replica(&jj_store_replication::NegotiateReplicaV1 {
                schema_version: 1,
                repository: self.record.repository,
                peer: self.record.discovery_peer,
                capabilities: capabilities.clone(),
            })
            .await?;
        let remote = self
            .old_client
            .as_ref()
            .ok_or("migration source is unavailable before hydration")?
            .negotiate_replica(&jj_store_replication::NegotiateReplicaV1 {
                schema_version: 1,
                repository: self.record.repository,
                peer: local.replica,
                capabilities,
            })
            .await?;
        if local.replica == remote.replica {
            return Err(
                "migration source and destination share a writable replica identity".into(),
            );
        }
        let checkpoint_store: Arc<dyn jj_store_object::ObjectStore> =
            Arc::new(MemoryStore::default());
        let checkpoints = PeerCheckpointStore::new(
            checkpoint_store,
            self.record.repository,
            local.replica,
            remote.replica,
        )?;
        let mut worker = ReplicationWorker::new(
            self.record.repository,
            local.replica,
            remote.replica,
            RpcEndpoint(self.local_client.clone()),
            RpcEndpoint(
                self.old_client
                    .clone()
                    .ok_or("migration source is unavailable before hydration")?,
            ),
            checkpoints,
            WorkerLimits::default(),
        )?;
        for _ in 0..MAX_PULL_STEPS {
            if worker.step_direction(Direction::Pull).await?.caught_up {
                return Ok(());
            }
        }
        Err("migration exceeded the bounded source feed page count".into())
    }

    async fn prepare_local_cache(&self, settings: &UserSettings) -> AuthorityResult<()> {
        let cache = &self.record.local_cache;
        if cache.join(binding::CONNECTION_FILE).exists() {
            return self.verify_local_cache(settings).await;
        }
        attachment::empty(cache)?;
        let parent = cache.parent().ok_or("migration cache parent")?;
        fs::create_dir_all(parent)?;
        let stage = tempfile::tempdir_in(parent)?;
        let stores = local_stores(self.local_client.clone(), &self.record.local_connection)?;
        stores.initialize_cache(settings, stage.path()).await?;
        self.record.local_connection.write(stage.path())?;
        let workspace: WorkspaceNameBuf = self.record.workspace.clone().into();
        SimpleWorkspaceStore::load(&stage.path().join("workspace_store"))?
            .add(&workspace, &self.record.workspace_root)?;
        stores
            .load_at(
                &stores.load_cache(settings, stage.path())?,
                self.record.working_copy_operation,
            )
            .await?;
        sync_tree(stage.path())?;
        if cache.exists() {
            fs::remove_dir(cache)?;
        }
        fs::rename(stage.path(), cache)?;
        sync_dir(parent)?;
        self.verify_local_cache(settings).await
    }

    async fn verify_local_cache(&self, settings: &UserSettings) -> AuthorityResult<()> {
        if serde_json::to_value(binding::Connection::read(&self.record.local_cache)?)?
            != serde_json::to_value(&self.record.local_connection)?
        {
            return Err("migration local cache binding changed".into());
        }
        let stores = local_stores(self.local_client.clone(), &self.record.local_connection)?;
        stores
            .load_at(
                &stores.load_cache(settings, &self.record.local_cache)?,
                self.record.working_copy_operation,
            )
            .await?;
        Ok(())
    }

    fn switch_descriptor(&self) -> AuthorityResult<()> {
        let path = self.record.workspace_root.join(".jj/repo");
        let current = binding::bounded_read(&path, 4096)?;
        let expected_old = descriptor(&self.record.workspace_root, &self.record.old_cache)?;
        let expected_new = jj_lib::file_util::path_to_bytes(&self.record.local_cache)?.to_vec();
        if current == expected_new {
            return verify_backup(&self.backup_path(), &expected_old);
        }
        if current != expected_old
            || <[u8; 32]>::from(Sha256::digest(&current)) != self.record.repo_descriptor_digest
        {
            return Err("workspace repository descriptor changed during migration".into());
        }
        write_backup(&self.backup_path(), &expected_old)?;
        let parent = path.parent().ok_or("workspace metadata parent")?;
        let mut replacement = tempfile::NamedTempFile::new_in(parent)?;
        replacement.write_all(&expected_new)?;
        replacement.as_file().sync_all()?;
        replacement.persist(&path)?;
        sync_dir(parent)
    }

    async fn verify_switched_workspace(&self, settings: &UserSettings) -> AuthorityResult<()> {
        let mut factories = default_backend_factories();
        factories.merge(binding::lazy_factories());
        let workspace = Workspace::load(
            settings,
            &self.record.workspace_root,
            &factories,
            &default_working_copy_factories(),
        )?;
        if workspace.repo_path() != self.record.local_cache
            || workspace.workspace_name().as_str() != self.record.workspace
            || workspace.working_copy().operation_id().as_bytes()
                != self.record.working_copy_operation.as_bytes()
        {
            return Err("migrated workspace identity or operation changed".into());
        }
        self.local_client.authorize(true).await?;
        Ok(())
    }

    fn backup_path(&self) -> PathBuf {
        self.record.workspace_root.join(".jj").join(BACKUP_NAME)
    }

    fn save(&self) -> AuthorityResult<()> {
        let mut file = tempfile::NamedTempFile::new_in(&self.state)?;
        file.write_all(&serde_json::to_vec_pretty(&self.record)?)?;
        file.as_file().sync_all()?;
        file.persist(self.state.join("migration.json"))?;
        sync_dir(&self.state)
    }
}

fn local_stores(
    client: NativeClient,
    connection: &binding::Connection,
) -> AuthorityResult<jj_store_jj::factories::NativeStores> {
    remote::stores(
        client,
        connection.max_blob_bytes,
        BackendLimits::default(),
        OperationLimits::default(),
    )
}

fn migration_state(root: &Path, state: Option<PathBuf>) -> AuthorityResult<PathBuf> {
    if let Some(state) = state {
        return Ok(jj_store_server::policy::outside_checkout(
            &attachment::target(&state)?,
        )?);
    }
    let key = hex::encode(Sha256::digest(root.as_os_str().as_encoded_bytes()));
    Ok(dirs::data_local_dir()
        .ok_or("client data directory unavailable")?
        .join("jj-store/migrations")
        .join(key))
}

fn descriptor(root: &Path, cache: &Path) -> AuthorityResult<Vec<u8>> {
    let path = root.join(".jj/repo");
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("workspace repository descriptor must be a regular pointer file".into());
    }
    let bytes = binding::bounded_read(&path, 4096)?;
    let decoded = jj_lib::file_util::path_from_bytes(&bytes)?;
    let resolved = attachment::target(&root.join(".jj").join(decoded))?;
    if resolved != cache {
        return Err("workspace repository descriptor does not select the source cache".into());
    }
    Ok(bytes)
}

fn write_backup(path: &Path, expected: &[u8]) -> AuthorityResult<()> {
    if path.exists() {
        return verify_backup(path, expected);
    }
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(expected)?;
    file.sync_all()?;
    sync_dir(path.parent().ok_or("workspace metadata parent")?)
}

fn verify_backup(path: &Path, expected: &[u8]) -> AuthorityResult<()> {
    if binding::bounded_read(path, 4096)? != expected {
        return Err("workspace migration backup does not match the original descriptor".into());
    }
    Ok(())
}

fn random_id<T>(construct: impl FnOnce([u8; 16]) -> T) -> AuthorityResult<T> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes)?;
    Ok(construct(bytes))
}

fn sync_tree(path: &Path) -> AuthorityResult<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            sync_tree(&entry.path())?;
        } else {
            File::open(entry.path())?.sync_all()?;
        }
    }
    sync_dir(path)
}

fn sync_dir(path: &Path) -> AuthorityResult<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
