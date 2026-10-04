//! Credential-free local metadata, derived indexes and per-session factories.
use crate::{
    backend::{BackendAuthority, BackendLimits, NativeBackend},
    objects::NativeObjects,
    operation_stores::{
        AuthorityResult, NativeOpHeadsStore, NativeOpStore, OperationAuthority, OperationLimits,
    },
};
use jj_lib::{
    backend::BackendLoadError,
    default_backend_factories::{default_backend_factories, default_working_copy_factory},
    object_id::ObjectId as _,
    ref_name::WorkspaceNameBuf,
    repo::{ReadonlyRepo, RepoLoader, StoreFactories},
    settings::UserSettings,
    workspace::Workspace,
};
use jj_store_model as m;
use jj_store_wal::inventory::DependencySource as _;
use std::{
    fs::{self, OpenOptions},
    io::{Read as _, Write as _},
    path::Path,
    sync::Arc,
};

pub const PROFILE: &str = "jj-native-0.45.1-v1";
/// Shared, repository-bound authority, with independently implemented read,
/// staging and publication operations. The service supplies the authenticated implementation.
pub trait NativeAuthority: BackendAuthority + OperationAuthority {
    fn repository(&self) -> m::RepositoryId;
}
#[derive(Clone)]
pub struct NativeStores {
    objects: Arc<NativeObjects>,
    authority: Arc<dyn NativeAuthority>,
    backend_limits: BackendLimits,
    operation_limits: OperationLimits,
}
impl NativeStores {
    pub fn new(
        objects: Arc<NativeObjects>,
        authority: Arc<dyn NativeAuthority>,
        backend_limits: BackendLimits,
        operation_limits: OperationLimits,
    ) -> AuthorityResult<Self> {
        if objects.scope().repository() != authority.repository() {
            return Err("native factory repository mismatch".into());
        }
        if tokio::runtime::Handle::try_current()?.runtime_flavor()
            != tokio::runtime::RuntimeFlavor::MultiThread
        {
            return Err("native JJ factories require a running multithread Tokio runtime".into());
        }
        // Validate all limits before any local metadata creation.
        NativeBackend::new(objects.clone(), authority.clone(), backend_limits)?;
        NativeOpStore::new(objects.clone(), authority.clone(), operation_limits)?;
        Ok(Self {
            objects,
            authority,
            backend_limits,
            operation_limits,
        })
    }
    fn backend(&self) -> AuthorityResult<NativeBackend> {
        NativeBackend::new(
            self.objects.clone(),
            self.authority.clone(),
            self.backend_limits,
        )
    }
    fn operations(&self) -> AuthorityResult<NativeOpStore> {
        NativeOpStore::new(
            self.objects.clone(),
            self.authority.clone(),
            self.operation_limits,
        )
    }
    fn heads(&self) -> AuthorityResult<NativeOpHeadsStore> {
        NativeOpHeadsStore::new(self.authority.clone())
    }
    fn marker(&self) -> String {
        format!(
            "jj-store-client-cache-v1\n{PROFILE}\n{}\n",
            self.authority.repository()
        )
    }
    fn bind(&self, path: &Path) -> AuthorityResult<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.join("native-profile"))?;
        file.write_all(self.marker().as_bytes())?;
        file.sync_all()?;
        Ok(())
    }
    fn verify(&self, path: &Path) -> AuthorityResult<()> {
        let mut bytes = Vec::new();
        fs::File::open(path.join("native-profile"))?
            .take(1025)
            .read_to_end(&mut bytes)?;
        if bytes != self.marker().as_bytes() {
            return Err("native repository or profile binding mismatch".into());
        }
        Ok(())
    }
    /// Only the extension factories. Merge with upstream defaults when loading
    /// native derived indexes, submodule bookkeeping and physical working copies.
    pub fn factories(&self) -> StoreFactories {
        let mut factories = StoreFactories::empty();
        let this = self.clone();
        factories.add_backend(
            "jj-store",
            Box::new(move |_, path| {
                this.verify(path).map_err(BackendLoadError)?;
                Ok(Box::new(this.backend().map_err(BackendLoadError)?))
            }),
        );
        let this = self.clone();
        factories.add_op_store(
            "jj-store",
            Box::new(move |_, path, root| {
                this.verify(path).map_err(BackendLoadError)?;
                if root.root_commit_id.as_bytes() != crate::roots::ROOT_COMMIT.as_bytes() {
                    return Err(BackendLoadError("native root mismatch".into()));
                }
                Ok(Box::new(this.operations().map_err(BackendLoadError)?))
            }),
        );
        let this = self.clone();
        factories.add_op_heads_store(
            "jj-store",
            Box::new(move |_, path| {
                this.verify(path).map_err(BackendLoadError)?;
                Ok(Box::new(this.heads().map_err(BackendLoadError)?))
            }),
        );
        factories
    }
    /// Initialize an empty caller-owned local cache. This does not create remote
    /// authority or publish a workspace. The server can use the same cache API.
    pub async fn initialize_cache(
        &self,
        settings: &UserSettings,
        path: &Path,
    ) -> AuthorityResult<RepoLoader> {
        OperationAuthority::authorize_read(self.authority.as_ref()).await?;
        if fs::read_dir(path)?.next().is_some() {
            return Err("native cache directory must be empty".into());
        }
        // Local initialization must not invoke upstream root-read expect()
        // paths: authorization may be revoked at any remote read boundary.
        for name in ["store", "op_store", "op_heads"] {
            let component = path.join(name);
            fs::create_dir(&component)?;
            self.bind(&component)?;
            fs::write(component.join("type"), "jj-store")?;
        }
        let index_path = path.join("index");
        fs::create_dir(&index_path)?;
        let index = ReadonlyRepo::default_index_store_initializer()(settings, &index_path)?;
        fs::write(index_path.join("type"), index.name())?;
        let submodule_path = path.join("submodule_store");
        fs::create_dir(&submodule_path)?;
        let submodules =
            ReadonlyRepo::default_submodule_store_initializer()(settings, &submodule_path)?;
        fs::write(submodule_path.join("type"), submodules.name())?;
        self.load_cache(settings, path)
    }
    pub fn load_cache(&self, settings: &UserSettings, path: &Path) -> AuthorityResult<RepoLoader> {
        let mut factories = default_backend_factories();
        factories.merge(self.factories());
        self.verify_cache_binding(path)?;
        Ok(RepoLoader::init_from_file_system(
            settings, path, &factories,
        )?)
    }

    /// Query-only construction with an already verified resident index. No
    /// disposable cache path is opened, and the supplied authority still owns
    /// every backend/operation read and denies writes.
    pub(crate) fn resident_loader(
        &self,
        settings: &UserSettings,
        index: Arc<dyn jj_lib::index::IndexStore>,
    ) -> AuthorityResult<RepoLoader> {
        #[derive(Debug)]
        struct QuerySubmodules;
        impl jj_lib::submodule_store::SubmoduleStore for QuerySubmodules {
            fn name(&self) -> &str {
                "query-read-only"
            }
        }
        let store = jj_lib::store::Store::new(
            Box::new(self.backend()?),
            jj_lib::signing::Signer::from_settings(settings)?,
            jj_lib::tree_merge::MergeOptions::from_settings(settings)?,
        );
        Ok(RepoLoader::new(
            settings.clone(),
            store,
            Arc::new(self.operations()?),
            Arc::new(self.heads()?),
            index,
            Arc::new(QuerySubmodules),
        ))
    }
    pub(crate) fn verify_cache_binding(&self, path: &Path) -> AuthorityResult<()> {
        // Verify even when a modified type file would otherwise select a different backend.
        for component in ["store", "op_store", "op_heads"] {
            self.verify(&path.join(component))?;
            if fs::read(path.join(component).join("type"))? != b"jj-store" {
                return Err("native cache backend type mismatch".into());
            }
        }
        Ok(())
    }
    /// Explicit pinned read: never invokes native head reconciliation.
    pub async fn load_at(
        &self,
        loader: &RepoLoader,
        id: m::OperationId,
    ) -> AuthorityResult<Arc<ReadonlyRepo>> {
        OperationAuthority::authorize_read(self.authority.as_ref()).await?;
        let operation = loader
            .load_operation(&jj_lib::op_store::OperationId::from_bytes(id.as_bytes()))
            .await?;
        Ok(loader.load_at(&operation).await?)
    }
    /// A writer may reconcile sibling operations using upstream transactions.
    pub async fn load_for_mutation(
        &self,
        loader: &RepoLoader,
    ) -> AuthorityResult<Arc<ReadonlyRepo>> {
        OperationAuthority::authorize_write(self.authority.as_ref()).await?;
        Ok(loader.load_at_head().await?)
    }
    pub async fn attach_workspace(
        &self,
        settings: &UserSettings,
        workspace_path: &Path,
        cache_path: &Path,
        workspace_name: WorkspaceNameBuf,
    ) -> AuthorityResult<(Workspace, Arc<ReadonlyRepo>)> {
        OperationAuthority::authorize_write(self.authority.as_ref()).await?;
        let loader = self.load_cache(settings, cache_path)?;
        let repo = self.load_for_mutation(&loader).await?;
        if repo.view().wc_commit_ids().contains_key(&workspace_name) {
            return Err("logical workspace name already exists".into());
        }
        Ok(Workspace::init_workspace_with_existing_repo(
            workspace_path,
            cache_path,
            &repo,
            &*default_working_copy_factory(),
            workspace_name,
        )
        .await?)
    }
}
