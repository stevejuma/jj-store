//! Durable deletion admission precedes a storage fence and bounded namespace erasure.
use crate::{installation::Identity, policy, registry::Registry};
use jj_store_model::{RepositoryId, RequestId};
use jj_store_object::s3::S3Store;
use jj_store_service::{administration::RepositoryHost, auth::Permission, repository_deletion::*};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    sync::Arc,
};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    repository: RepositoryId,
    request: DeleteRepository,
    deleted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    version: u32,
    installation: String,
    entries: Vec<Entry>,
}
pub struct Manager {
    path: PathBuf,
    identity: Identity,
    store: Arc<S3Store>,
    host: RepositoryHost,
    registry: Arc<Registry>,
    lock: tokio::sync::Mutex<()>,
}
fn load(path: &Path, identity: &Identity) -> crate::Result<Journal> {
    if matches!(fs::symlink_metadata(path), Err(ref e) if e.kind() == std::io::ErrorKind::NotFound)
    {
        return Ok(Journal {
            version: 1,
            installation: identity.installation.clone(),
            entries: vec![],
        });
    }
    let bytes = crate::registry_persistence::read(path)?;
    let journal: Journal =
        serde_json::from_slice(&bytes).map_err(|_| crate::Error::Config("deletion journal"))?;
    if journal.version != 1
        || journal.installation != identity.installation
        || journal.entries.len() > 4096
        || journal.entries.iter().any(|e| {
            e.request.version != 1
                || e.request.expected_installation != identity.installation
                || e.request.request_id == RequestId::new([0; 16])
        })
        || journal.entries.iter().enumerate().any(|(i, e)| {
            journal.entries[..i].iter().any(|p| {
                p.repository == e.repository || p.request.request_id == e.request.request_id
            })
        })
    {
        return Err(crate::Error::Config("deletion journal identity"));
    }
    Ok(journal)
}
fn persist(path: &Path, journal: &Journal) -> crate::Result<()> {
    let bytes = serde_json::to_vec(journal)
        .map_err(|_| crate::Error::Config("deletion journal encoding"))?;
    if bytes.len() > 4 * 1024 * 1024 {
        return Err(crate::Error::Config("deletion journal size"));
    }
    let mut file = tempfile::NamedTempFile::new_in(path.parent().unwrap())?;
    file.write_all(&bytes)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|e| crate::Error::Io(e.error))?;
    fs::File::open(path.parent().unwrap())?.sync_all()?;
    Ok(())
}
impl Manager {
    pub async fn restore(
        state: &Path,
        identity: &Identity,
        store: &S3Store,
        registry: &Registry,
    ) -> crate::Result<()> {
        let journal = load(&state.join("native-deletions.json"), identity)?;
        for entry in journal.entries {
            store.fence_repository(entry.repository).await;
            registry.remove_repository(entry.repository)?;
        }
        Ok(())
    }
    pub fn new(
        state: &Path,
        identity: Identity,
        store: Arc<S3Store>,
        host: RepositoryHost,
        registry: Arc<Registry>,
    ) -> Self {
        Self {
            path: state.join("native-deletions.json"),
            identity,
            store,
            host,
            registry,
            lock: tokio::sync::Mutex::new(()),
        }
    }
    fn outcome(entry: &Entry) -> DeletionOutcome {
        DeletionOutcome {
            version: 1,
            repository: entry.repository,
            request_id: entry.request.request_id,
            state: if entry.deleted {
                DeletionState::Deleted
            } else {
                DeletionState::Pending
            },
        }
    }
}
#[async_trait::async_trait]
impl RepositoryDeletion for Manager {
    async fn delete(
        &self,
        repository: RepositoryId,
        request: DeleteRepository,
    ) -> Result<DeletionOutcome, Error> {
        if request.expected_installation != self.identity.installation {
            return Err(Error::Conflict);
        }
        if request.version != 1 || request.request_id == RequestId::new([0; 16]) {
            return Err(Error::Invalid);
        }
        let _serial = self.lock.lock().await;
        let _file = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut journal = load(&self.path, &self.identity).map_err(|_| Error::Unavailable)?;
        let index = if let Some(index) = journal
            .entries
            .iter()
            .position(|e| e.repository == repository)
        {
            if journal.entries[index].request != request {
                return Err(Error::Conflict);
            }
            index
        } else {
            if journal.entries.len() >= 4096
                || journal
                    .entries
                    .iter()
                    .any(|e| e.request.request_id == request.request_id)
            {
                return Err(Error::Conflict);
            }
            // Only fresh admitted repository authority can start erasure. Replays
            // use the installation journal after repository access is fenced.
            let session = self
                .host
                .endpoint(repository)
                .map_err(|_| Error::Unavailable)?
                .bind_local_trusted(
                    self.identity.principal(b"repository-deletion"),
                    Permission::Admin,
                )
                .map_err(|_| Error::Unavailable)?
                .into_session();
            let snapshot = session.snapshot().await.map_err(|_| Error::Unavailable)?;
            if snapshot.manifest().epoch != request.expected_epoch {
                return Err(Error::Stale);
            }
            self.store
                .check_repository_deletion()
                .await
                .map_err(|e| match e {
                    jj_store_object::Error::Invalid(_) => Error::Unsupported,
                    _ => Error::Unavailable,
                })?;
            journal.entries.push(Entry {
                repository,
                request,
                deleted: false,
            });
            persist(&self.path, &journal).map_err(|_| Error::Unavailable)?;
            journal.entries.len() - 1
        };
        self.store.fence_repository(repository).await;
        self.registry
            .remove_repository(repository)
            .map_err(|_| Error::Unavailable)?;
        if journal.entries[index].deleted {
            return Ok(Self::outcome(&journal.entries[index]));
        }
        if self.store.repository_has_unknown_writes(repository) {
            return Err(Error::Uncertain);
        }
        let empty = self
            .store
            .delete_repository_batch(repository)
            .await
            .map_err(|_| Error::Unavailable)?;
        if empty {
            journal.entries[index].deleted = true;
            persist(&self.path, &journal).map_err(|_| Error::Unavailable)?;
        }
        Ok(Self::outcome(&journal.entries[index]))
    }
    async fn status(
        &self,
        repository: RepositoryId,
        request: RequestId,
    ) -> Result<DeletionOutcome, Error> {
        let _serial = self.lock.lock().await;
        let journal = load(&self.path, &self.identity).map_err(|_| Error::Unavailable)?;
        journal
            .entries
            .iter()
            .find(|e| e.repository == repository && e.request.request_id == request)
            .map(Self::outcome)
            .ok_or(Error::Unavailable)
    }
}
