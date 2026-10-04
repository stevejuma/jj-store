//! Bounded installation-owned links. Repository authority and checkout files
//! remain unchanged; discovery is supplied by the CLI composition boundary.
use crate::{installation::Identity, policy};
use jj_store_model::RepositoryId;
use jj_store_service::local_management::{
    Error, LinkWorkspace, MAX_PATH_BYTES, MAX_WORKSPACES, VerifiedWorkspace, WorkspaceAvailability,
    WorkspaceLink, WorkspaceLinks, WorkspaceManager, WorkspaceProfile, WorkspaceVerifier,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{fs, io::Write as _, path::PathBuf, sync::Arc};

const MAX_BYTES: u64 = 1024 * 1024;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    version: u32,
    installation: String,
    revision: u64,
    links: Vec<VerifiedWorkspace>,
}

#[derive(Clone)]
pub struct Workspaces {
    changes: jj_store_service::freshness::Signals,
    path: PathBuf,
    identity: Identity,
    verifier: Arc<dyn WorkspaceVerifier>,
}

impl Workspaces {
    pub fn new(
        state: PathBuf,
        identity: Identity,
        verifier: Arc<dyn WorkspaceVerifier>,
    ) -> crate::Result<Self> {
        let state = policy::outside_checkout(&state)?;
        if !state.is_dir() {
            return Err(crate::Error::Config("workspace registry state directory"));
        }
        Ok(Self {
            changes: Default::default(),
            path: state.join("workspace-links.json"),
            identity,
            verifier,
        })
    }

    fn read(&self) -> Result<Registry, Error> {
        match fs::symlink_metadata(&self.path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Registry {
                    version: 1,
                    installation: self.identity.installation.clone(),
                    revision: 0,
                    links: Vec::new(),
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
        let bytes = policy::read_regular(&self.path, MAX_BYTES).map_err(|_| Error::Unavailable)?;
        let value: Registry = serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?;
        if value.version != 1
            || value.installation != self.identity.installation
            || value.links.len() > MAX_WORKSPACES
            || value.links.iter().any(|link| !valid(link))
            || value
                .links
                .windows(2)
                .any(|pair| pair[0].path >= pair[1].path)
        {
            return Err(Error::Unavailable);
        }
        Ok(value)
    }

    fn snapshot(&self, value: &Registry, repository: RepositoryId) -> WorkspaceLinks {
        WorkspaceLinks {
            version: 1,
            revision: value.revision,
            links: value
                .links
                .iter()
                .filter(|link| link.repository == repository)
                .map(|link| {
                    let availability = match self.verifier.inspect(&link.path) {
                        Ok(current) if current == *link => WorkspaceAvailability::Ready,
                        _ => WorkspaceAvailability::Unavailable,
                    };
                    let mut digest = Sha256::new_with_prefix(b"jj-store/device-workspace/v1");
                    digest.update(self.identity.installation.as_bytes());
                    digest.update(repository.to_string().as_bytes());
                    digest.update(link.path.as_bytes());
                    WorkspaceLink {
                        id: hex::encode(digest.finalize()),
                        path: link.path.clone(),
                        profile: link.profile,
                        availability,
                    }
                })
                .collect(),
        }
    }

    fn persist(&self, value: &Registry) -> Result<(), Error> {
        let bytes = serde_json::to_vec(value).map_err(|_| Error::Unavailable)?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err(Error::Limit);
        }
        let parent = self.path.parent().ok_or(Error::Unavailable)?;
        let mut temporary =
            tempfile::NamedTempFile::new_in(parent).map_err(|_| Error::Unavailable)?;
        // NamedTempFile creates an owner-only regular file on Unix.
        temporary
            .write_all(&bytes)
            .map_err(|_| Error::Unavailable)?;
        temporary
            .as_file()
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        temporary
            .persist(&self.path)
            .map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        fs::File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|_| Error::Unavailable)?;
        Ok(())
    }

    fn link_sync(
        &self,
        repository: RepositoryId,
        request: LinkWorkspace,
    ) -> Result<WorkspaceLinks, Error> {
        if request.version != 1 || !valid_path(&request.path) {
            return Err(Error::Invalid);
        }
        let _lock = policy::exclusive_lock(&self.path.with_extension("lock"))
            .map_err(|_| Error::Unavailable)?;
        let mut value = self.read()?;
        let verified = self.verifier.inspect(&request.path)?;
        if !valid(&verified) || verified.repository != repository {
            return Err(Error::Invalid);
        }
        if let Some(previous) = value.links.iter().find(|link| link.path == verified.path) {
            if *previous != verified {
                return Err(Error::Invalid);
            }
            return Ok(self.snapshot(&value, repository));
        }
        if value.revision != request.expected_revision {
            return Err(Error::StaleRevision);
        }
        if value.links.len() == MAX_WORKSPACES {
            return Err(Error::Limit);
        }
        value.revision = value.revision.checked_add(1).ok_or(Error::Limit)?;
        value.links.push(verified);
        value.links.sort_by(|a, b| a.path.cmp(&b.path));
        self.persist(&value)?;
        Ok(self.snapshot(&value, repository))
    }
}

fn valid_path(path: &str) -> bool {
    path.len() <= MAX_PATH_BYTES && !path.contains('\0') && std::path::Path::new(path).is_absolute()
}
fn valid(link: &VerifiedWorkspace) -> bool {
    valid_path(&link.path) && valid_path(&link.metadata) && link.profile == WorkspaceProfile::Native
}

#[async_trait::async_trait]
impl WorkspaceManager for Workspaces {
    fn freshness_witness(&self) -> Option<String> {
        let value = match self.read() {
            Ok(value) => value,
            Err(_) => return Some("unavailable".into()),
        };
        let availability = value
            .links
            .iter()
            .map(|link| self.verifier.inspect(&link.path).as_ref() == Ok(link))
            .collect::<Vec<_>>();
        Some(hex::encode(Sha256::digest(
            serde_json::to_vec(&(value, availability)).ok()?,
        )))
    }
    fn freshness(
        &self,
    ) -> Option<tokio::sync::broadcast::Receiver<jj_store_service::freshness::Change>> {
        Some(self.changes.subscribe())
    }
    async fn list(&self, repository: RepositoryId) -> Result<WorkspaceLinks, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || Ok(this.snapshot(&this.read()?, repository)))
            .await
            .map_err(|_| Error::Unavailable)?
    }
    async fn link(
        &self,
        repository: RepositoryId,
        request: LinkWorkspace,
    ) -> Result<WorkspaceLinks, Error> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
            let result = this.link_sync(repository, request)?;
            this.changes.notify(
                Some(repository),
                jj_store_service::freshness::Domain::Workspaces,
                None,
            );
            Ok(result)
        })
        .await
        .map_err(|_| Error::Unavailable)?
    }
}
