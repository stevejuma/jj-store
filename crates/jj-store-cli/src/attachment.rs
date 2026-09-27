//! Durable, secret-free client attachment intent. Only an acknowledged native
//! publication permits physical JJ metadata to become visible.
use crate::binding::{self, Connection};
use jj_lib::{
    default_backend_factories::{
        default_backend_factories, default_working_copy_factories, default_working_copy_factory,
    },
    object_id::ObjectId as _,
    repo::Repo as _,
    settings::UserSettings,
    workspace::Workspace,
    workspace_store::{SimpleWorkspaceStore, WorkspaceStore as _},
};
use jj_store_jj::{factories::NativeStores, operation_stores::AuthorityResult};
use jj_store_model as m;
use jj_store_rpc::client::NativeClient;
use jj_store_service::auth::BearerToken;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::Write as _,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pending {
    pub old: BTreeSet<m::OperationId>,
    pub new: m::OperationId,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    format: u32,
    connection: Connection,
    destination: PathBuf,
    cache: PathBuf,
    workspace: String,
    token: m::TokenId,
    pending: Option<Pending>,
    published: bool,
    complete: bool,
}
pub struct Attachment {
    state: PathBuf,
    record: Record,
    client: NativeClient,
    stores: NativeStores,
    _lock: File,
}
#[derive(Serialize)]
pub struct Attached {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub workspace: String,
    pub path: PathBuf,
    pub cache: PathBuf,
    pub operation: m::OperationId,
}
fn random_name() -> AuthorityResult<String> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes)?;
    Ok(hex::encode(bytes))
}
/// Canonicalize existing ancestors without creating a target. Reject ambiguous
/// parent traversal and final symlinks; resumed physical workspaces are allowed.
fn target(path: &Path) -> AuthorityResult<PathBuf> {
    if !path.is_absolute()
        || path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
    {
        return Err("attachment paths must be absolute without parent traversal".into());
    }
    if fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("attachment target must not be a symlink".into());
    }
    let mut parent = path;
    let mut suffix = Vec::new();
    while !parent.exists() {
        suffix.push(parent.file_name().ok_or("attachment path")?.to_owned());
        parent = parent.parent().ok_or("attachment path")?;
    }
    let mut resolved = parent.canonicalize()?;
    for part in suffix.into_iter().rev() {
        resolved.push(part);
    }
    Ok(resolved)
}
fn empty(path: &Path) -> AuthorityResult<()> {
    if path.exists() && fs::read_dir(path)?.next().is_some() {
        return Err("native attachment requires an empty destination and cache".into());
    }
    Ok(())
}
fn sync_dir(path: &Path) -> AuthorityResult<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}
impl Attachment {
    pub async fn open(
        connection: Connection,
        token: BearerToken,
        destination: PathBuf,
        cache: Option<PathBuf>,
        workspace: Option<String>,
        state: Option<PathBuf>,
    ) -> AuthorityResult<Self> {
        connection.validate()?;
        // Even a completed retry checks current access before revealing metadata.
        let client = connection.connect_with(token.clone()).await?;
        client.authorize(true).await?;
        let destination = target(&destination)?;
        let key = hex::encode(Sha256::digest(destination.as_os_str().as_encoded_bytes()));
        let state = match state {
            Some(state) => state,
            None => match &cache {
                Some(cache) => target(cache)?
                    .parent()
                    .ok_or("cache parent")?
                    .join(".jj-store-attachments")
                    .join(key),
                None => dirs::data_local_dir()
                    .ok_or("client data directory unavailable")?
                    .join("jj-store/attachments")
                    .join(key),
            },
        };
        let state = jj_store_server::policy::outside_checkout(&state)?;
        if state.starts_with(&destination) || destination.starts_with(&state) {
            return Err("attachment state must be separate from the physical checkout".into());
        }
        fs::create_dir_all(&state)?;
        let lock = jj_store_server::policy::exclusive_lock(&state.join("attachment.lock"))?;
        let journal = state.join("attachment.json");
        let record = if journal.exists() {
            let record: Record = serde_json::from_slice(&jj_store_server::policy::read_regular(
                &journal,
                64 * 1024,
            )?)
            .map_err(|_| "invalid attachment journal")?;
            record.connection.validate()?;
            if record.format != 1
                || record.destination != destination
                || record.token != token.id()
                || serde_json::to_value(&record.connection)? != serde_json::to_value(&connection)?
                || cache
                    .as_ref()
                    .map(|p| target(p))
                    .transpose()?
                    .is_some_and(|p| p != record.cache)
                || workspace.as_ref().is_some_and(|w| w != &record.workspace)
                || record.published && record.pending.is_none()
                || record.complete && !record.published
            {
                return Err("attachment retry differs from its durable intent".into());
            }
            record
        } else {
            empty(&destination)?;
            let cache = target(&cache.unwrap_or_else(|| state.join("cache")))?;
            empty(&cache)?;
            Record {
                format: 1,
                connection,
                destination,
                cache,
                workspace: workspace.unwrap_or(format!("client-{}", random_name()?)),
                token: token.id(),
                pending: None,
                published: false,
                complete: false,
            }
        };
        if record.workspace.is_empty()
            || record.workspace.len() > 128
            || record
                .workspace
                .bytes()
                .any(|c| !c.is_ascii_alphanumeric() && !b"-_".contains(&c))
            || record.cache.starts_with(&record.destination)
            || record.destination.starts_with(&record.cache)
            || target(&record.cache)? != record.cache
        {
            return Err("invalid workspace name or overlapping attachment paths".into());
        }
        let stores = crate::remote::stores(
            client.clone(),
            record.connection.max_blob_bytes,
            jj_store_jj::backend::BackendLimits::default(),
            jj_store_jj::operation_stores::OperationLimits::default(),
        )?;
        let this = Self {
            state,
            record,
            client,
            stores,
            _lock: lock,
        };
        this.save()?;
        Ok(this)
    }
    fn save(&self) -> AuthorityResult<()> {
        let mut file = tempfile::NamedTempFile::new_in(&self.state)?;
        file.write_all(&serde_json::to_vec_pretty(&self.record)?)?;
        file.as_file().sync_all()?;
        file.persist(self.state.join("attachment.json"))?;
        sync_dir(&self.state)
    }
    pub fn destination(&self) -> &Path {
        &self.record.destination
    }
    pub fn pending(&self) -> Option<&Pending> {
        self.record.pending.as_ref()
    }
    pub async fn prepare(&mut self, settings: &UserSettings) -> AuthorityResult<()> {
        let cache = &self.record.cache;
        if cache.join(binding::CONNECTION_FILE).exists() {
            if serde_json::to_value(Connection::read(cache)?)?
                != serde_json::to_value(&self.record.connection)?
            {
                return Err("attachment cache binding changed".into());
            }
        } else {
            empty(cache)?;
            let parent = cache.parent().ok_or("cache parent")?;
            fs::create_dir_all(parent)?;
            let staging = tempfile::tempdir_in(parent)?;
            self.stores
                .initialize_cache(settings, staging.path())
                .await?;
            self.record.connection.write(staging.path())?;
            if cache.exists() {
                fs::remove_dir(cache)?;
            }
            fs::rename(staging.path(), cache)?;
            sync_dir(parent)?;
        }
        if self.record.pending.is_none() {
            let loader = self.stores.load_cache(settings, cache)?;
            let repo = self.stores.load_for_mutation(&loader).await?;
            let workspace = self.record.workspace.clone().into();
            if repo.view().wc_commit_ids().contains_key(&workspace) {
                return Err("logical workspace name already exists".into());
            }
            let mut tx = repo.start_transaction();
            tx.repo_mut()
                .check_out(workspace, &repo.store().root_commit())
                .await?;
            let staged = tx
                .write(format!("add workspace '{}'", self.record.workspace))
                .await?
                .leave_unpublished();
            self.record.pending = Some(Pending {
                old: staged
                    .operation()
                    .parent_ids()
                    .iter()
                    .map(|id| m::OperationId::from_slice(id.as_bytes()))
                    .collect::<Result<_, _>>()?,
                new: m::OperationId::from_slice(staged.op_id().as_bytes())?,
            });
            self.save()?;
        }
        Ok(())
    }
    pub async fn finish(&mut self, settings: &UserSettings) -> AuthorityResult<Attached> {
        let pending = self
            .record
            .pending
            .clone()
            .ok_or("attachment not prepared")?;
        if !self.record.published {
            // Always retry these exact IDs. A timeout never clears the journal.
            self.client
                .publish(pending.old.clone(), pending.new)
                .await?;
            self.record.published = true;
            self.save()?;
        }
        let loader = self.stores.load_cache(settings, &self.record.cache)?;
        let repo = self.stores.load_at(&loader, pending.new).await?;
        let destination = &self.record.destination;
        let metadata = destination.join(".jj");
        if !metadata.exists() {
            if self.record.complete {
                return Err("completed workspace metadata is missing".into());
            }
            fs::create_dir_all(destination)?;
            // A private, journal-owned stage. No physical source file is removed.
            let stage = destination.join(".jj-store-bootstrap");
            if stage.exists() {
                if fs::symlink_metadata(&stage)?.file_type().is_symlink() {
                    return Err("workspace metadata stage is a symlink".into());
                }
                let owner = fs::read(stage.join("attachment-owner"))?;
                if owner != pending.new.to_string().as_bytes() {
                    return Err("workspace metadata stage is not owned by this attachment".into());
                }
                fs::remove_dir_all(&stage)?;
            }
            fs::create_dir(&stage)?;
            fs::write(stage.join("attachment-owner"), pending.new.to_string())?;
            let state = stage.join("working_copy");
            fs::create_dir(&state)?;
            let working_copy = default_working_copy_factory().init_working_copy(
                repo.store().clone(),
                destination.clone(),
                state.clone(),
                repo.op_id().clone(),
                self.record.workspace.clone().into(),
                settings,
            )?;
            fs::write(state.join("type"), working_copy.name())?;
            fs::write(
                stage.join("repo"),
                jj_lib::file_util::path_to_bytes(&self.record.cache)?,
            )?;
            // Sync the small local metadata tree before atomically exposing it.
            sync_tree(&stage)?;
            fs::rename(&stage, &metadata)?;
            sync_dir(destination)?;
        }
        let mut factories = default_backend_factories();
        factories.merge(self.stores.factories());
        let workspace = Workspace::load(
            settings,
            destination,
            &factories,
            &default_working_copy_factories(),
        )?;
        if workspace.repo_path() != self.record.cache
            || workspace.workspace_name().as_str() != self.record.workspace
        {
            return Err("physical workspace does not match attachment intent".into());
        }
        SimpleWorkspaceStore::load(&self.record.cache)?
            .add(workspace.workspace_name(), destination)?;
        self.record.complete = true;
        self.save()?;
        Ok(Attached {
            version: 1,
            repository: self.record.connection.repository,
            workspace: self.record.workspace.clone(),
            path: self.record.destination.clone(),
            cache: self.record.cache.clone(),
            operation: pending.new,
        })
    }
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
