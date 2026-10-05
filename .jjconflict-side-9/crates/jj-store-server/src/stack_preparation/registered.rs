//! Server-resolved authorization/physical context; never client paths or setup.
use super::*;
use jj_store_service::{auth::Permission, local_browser as registration, working_tree};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActiveCombination {
    pub selection: s::CombinationSelection,
    /// Exact selected merge/WIP operation, independently checked against disk.
    pub code: s::CodeSelection,
    /// Configured target/immutable boundaries at this exact combination.
    pub immutable_heads: Vec<m::GitSha1Id>,
}
/// Installation-owned durable combination reader. Absence never initializes
/// state. This seam is separate from registration and must qualify before wiring.
#[async_trait::async_trait]
pub trait CombinationAuthority: Send + Sync {
    async fn current(
        &self,
        repository: m::RepositoryId,
        physical: s::PhysicalWorkspaceId,
        workspace: &str,
    ) -> Result<Option<ActiveCombination>>;
}
#[derive(Clone)]
pub struct RegisteredAuthority {
    principal: m::PrincipalId,
    repositories: Arc<dyn registration::LocalRepositories>,
    combinations: Arc<dyn CombinationAuthority>,
    observer: working_tree::Observer,
}
impl RegisteredAuthority {
    pub fn new(
        access: &jj_store_http::Access,
        repositories: Arc<dyn registration::LocalRepositories>,
        combinations: Arc<dyn CombinationAuthority>,
        observer: working_tree::Observer,
    ) -> Result<Self> {
        let jj_store_http::Access::LocalTrusted {
            principal,
            permission: Permission::Admin,
        } = access
        else {
            return Err(Error::Forbidden);
        };
        if principal.as_bytes() == &[0; 16] {
            return Err(Error::Forbidden);
        }
        Ok(Self {
            principal: *principal,
            repositories,
            combinations,
            observer,
        })
    }
    async fn binding(
        &self,
        principal: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> Result<Arc<Binding>> {
        if principal != self.principal {
            return Err(Error::Forbidden);
        }
        if !matches!(
            (&request.selection.code, &request.command),
            (
                s::CodeSelection::GitSha1Jj045 { .. },
                s::Command::Reword { .. }
            )
        ) {
            return Err(Error::Unsupported);
        }
        let manager = self.repositories.clone();
        let repository = request.selection.repository;
        let (folder, generation) = tokio::task::spawn_blocking(move || manager.folder(repository))
            .await
            .map_err(|_| Error::Forbidden)?
            .map_err(|_| Error::Forbidden)?;
        if folder.repository != Some(repository)
            || !std::path::Path::new(&folder.path).is_absolute()
            || !std::path::Path::new(&folder.metadata).is_absolute()
        {
            return Err(Error::Forbidden);
        }
        Ok(Arc::new(Binding {
            repositories: self.repositories.clone(),
            folder,
            generation,
        }))
    }
}
struct Binding {
    repositories: Arc<dyn registration::LocalRepositories>,
    folder: registration::LocalFolder,
    generation: u64,
}
#[async_trait::async_trait]
impl q::Access for Binding {
    fn repository(&self) -> m::RepositoryId {
        self.folder.repository.expect("verified registration")
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.folder.metadata)
    }
    async fn authorize(&self) -> q::Result<()> {
        let repositories = self.repositories.clone();
        let repository = self.repository();
        let (folder, generation) =
            tokio::task::spawn_blocking(move || repositories.folder(repository))
                .await
                .map_err(|_| q::Error::Denied)?
                .map_err(|_| q::Error::Denied)?;
        if folder != self.folder || generation != self.generation {
            return Err(q::Error::Denied);
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl Authority for RegisteredAuthority {
    async fn authorize(
        &self,
        principal: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> Result<()> {
        self.binding(principal, request).await?.authorize().await?;
        Ok(())
    }
    async fn context(
        &self,
        principal: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> Result<Context> {
        let access = self.binding(principal, request).await?;
        let root = PathBuf::from(&access.folder.path);
        let s::CodeSelection::GitSha1Jj045 { operation, .. } = request.selection.code else {
            return Err(Error::Unsupported);
        };
        let observation = self
            .observer
            .observe(access.clone(), root.clone(), operation)
            .await
            .map_err(|error| match error {
                working_tree::Error::Denied => Error::Forbidden,
                working_tree::Error::Stale => Error::Stale,
                working_tree::Error::Unsupported | working_tree::Error::Invalid => {
                    Error::Unsupported
                }
                working_tree::Error::Busy => Error::Provider(q::Error::Busy),
                working_tree::Error::Cancelled => Error::Provider(q::Error::Canceled),
                working_tree::Error::Limit => Error::Provider(q::Error::Limit),
                working_tree::Error::Unavailable => Error::Provider(q::Error::Unavailable),
            })?;
        let current = observation.current;
        let combination = self
            .combinations
            .current(
                access.repository(),
                s::PhysicalWorkspaceId(current.physical_workspace),
                &current.checkout.workspace,
            )
            .await?
            .ok_or(Error::Unsupported)?;
        if combination.code
            != (s::CodeSelection::GitSha1Jj045 {
                operation: current.baseline.operation,
                view: current.baseline.view,
                working_copy: current.baseline.commit,
            })
        {
            return Err(Error::Stale);
        }
        if combination.immutable_heads.is_empty()
            || combination.immutable_heads.len() > 64
            || combination
                .immutable_heads
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != combination.immutable_heads.len()
        {
            return Err(Error::Unsupported);
        }
        access.authorize().await?;
        Ok(Context {
            access,
            root,
            current,
            working_tree: observation.projection.physical.fingerprint,
            combination: combination.selection,
            immutable_heads: combination.immutable_heads,
        })
    }
    async fn recovery(
        &self,
        principal: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> Result<RecoveryContext> {
        let access = self.binding(principal, request).await?;
        let root = PathBuf::from(&access.folder.path);
        access.authorize().await?;
        Ok(RecoveryContext { access, root })
    }
}
