//! Read-only physical JJ workspace discovery for installation-local settings.
use crate::binding::{CONNECTION_FILE, Connection};
use jj_lib::workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _};
use jj_store_server::policy;
use jj_store_service::local_management::{
    Error, MAX_PATH_BYTES, VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile,
    WorkspaceVerifier,
};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};

pub struct NativeWorkspaceVerifier {
    socket: PathBuf,
}
impl NativeWorkspaceVerifier {
    pub fn new(socket: &Path) -> Result<Self, Error> {
        Ok(Self {
            socket: policy::outside_checkout(socket).map_err(|_| Error::Invalid)?,
        })
    }
}
impl WorkspaceVerifier for NativeWorkspaceVerifier {
    fn inspect(&self, path: &str) -> Result<VerifiedWorkspace, Error> {
        let requested = Path::new(path);
        if path.len() > MAX_PATH_BYTES || !requested.is_absolute() || path.contains('\0') {
            return Err(Error::Invalid);
        }
        let root = requested.canonicalize().map_err(|_| Error::Invalid)?;
        let jj = root.join(".jj");
        let metadata = std::fs::symlink_metadata(&jj).map_err(|_| Error::Invalid)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(Error::Invalid);
        }
        let working_copy =
            std::fs::symlink_metadata(jj.join("working_copy")).map_err(|_| Error::Invalid)?;
        if !working_copy.is_dir() || working_copy.file_type().is_symlink() {
            return Err(Error::Invalid);
        }
        let repo = jj.join("repo");
        let metadata = std::fs::symlink_metadata(&repo).map_err(|_| Error::Invalid)?;
        if metadata.file_type().is_symlink() {
            return Err(Error::Invalid);
        }
        if metadata.is_file() {
            policy::read_regular(&repo, MAX_PATH_BYTES as u64).map_err(|_| Error::Invalid)?;
        } else if !metadata.is_dir() {
            return Err(Error::Invalid);
        }
        let loader = DefaultWorkspaceLoaderFactory
            .create(&root)
            .map_err(|_| Error::Invalid)?;
        let repo = loader
            .repo_path()
            .canonicalize()
            .map_err(|_| Error::Invalid)?;
        let kind =
            policy::read_regular(&repo.join("store/type"), 128).map_err(|_| Error::Invalid)?;
        if kind == b"git" || kind == b"git\n" {
            return Err(Error::UnsupportedProfile);
        }
        if kind != b"jj-store" && kind != b"jj-store\n" {
            return Err(Error::Invalid);
        }
        // Check regular-file bounds before using the existing native binding validator.
        policy::read_regular(&repo.join(CONNECTION_FILE), 64 * 1024).map_err(|_| Error::Invalid)?;
        for component in ["store", "op_store", "op_heads"] {
            policy::read_regular(&repo.join(component).join("native-profile"), 1024)
                .map_err(|_| Error::Invalid)?;
        }
        let connection = Connection::read(&repo).map_err(|_| Error::Invalid)?;
        let socket = connection.unix_socket.as_ref().ok_or(Error::Invalid)?;
        if policy::outside_checkout(socket).map_err(|_| Error::Invalid)? != self.socket {
            return Err(Error::Invalid);
        }
        let string = |path: &Path| -> Result<String, Error> {
            path.to_str()
                .filter(|value| value.len() <= MAX_PATH_BYTES)
                .map(str::to_owned)
                .ok_or(Error::Invalid)
        };
        Ok(VerifiedWorkspace {
            path: string(&root)?,
            metadata: string(&repo)?,
            repository: connection.repository,
            profile: WorkspaceProfile::Native,
        })
    }
    fn inspect_capture_directory(&self, path: &str) -> Result<VerifiedCaptureWorkspace, Error> {
        let requested = Path::new(path);
        if path.len() > MAX_PATH_BYTES || !requested.is_absolute() || path.contains('\0') {
            return Err(Error::Invalid);
        }
        let directory = requested.canonicalize().map_err(|_| Error::Invalid)?;
        if !directory.is_dir()
            || directory
                .to_str()
                .is_none_or(|path| path.len() > MAX_PATH_BYTES)
            || directory.components().any(|part| {
                part.as_os_str().to_str().is_some_and(|name| {
                    name.eq_ignore_ascii_case(".jj") || name.eq_ignore_ascii_case(".git")
                })
            })
        {
            return Err(Error::Invalid);
        }
        let mut ancestors = directory.ancestors();
        for _ in 0..64 {
            let root = ancestors.next().ok_or(Error::Invalid)?;
            if repository_boundary(&root.join(".jj"))? {
                let workspace = self.inspect(root.to_str().ok_or(Error::Invalid)?)?;
                let digest = capture_binding_digest(&workspace)?;
                if self.inspect(&workspace.path)? != workspace {
                    return Err(Error::Invalid);
                }
                return Ok(VerifiedCaptureWorkspace {
                    directory: path.into(),
                    workspace,
                    binding_digest: digest,
                });
            }
            if repository_boundary(&root.join(".git"))? {
                return Err(Error::UnsupportedProfile);
            }
        }
        Err(Error::Limit)
    }
}

fn repository_boundary(path: &Path) -> Result<bool, Error> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(Error::Unavailable),
    }
}

fn capture_binding_digest(workspace: &VerifiedWorkspace) -> Result<[u8; 32], Error> {
    let root = Path::new(&workspace.path);
    let metadata = Path::new(&workspace.metadata);
    let mut digest = Sha256::new_with_prefix(b"jj-store/capture-workspace-binding/v1\0");
    for path in [
        root,
        &root.join(".jj"),
        &root.join(".jj/working_copy"),
        metadata,
    ] {
        let value = std::fs::symlink_metadata(path).map_err(|_| Error::Invalid)?;
        if !value.is_dir() || value.file_type().is_symlink() {
            return Err(Error::Invalid);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            digest.update(value.dev().to_be_bytes());
            digest.update(value.ino().to_be_bytes());
        }
        #[cfg(not(unix))]
        {
            return Err(Error::UnsupportedProfile);
        }
        let bytes = path.to_str().ok_or(Error::Invalid)?.as_bytes();
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    let pointer = root.join(".jj/repo");
    if std::fs::symlink_metadata(&pointer)
        .map_err(|_| Error::Invalid)?
        .is_file()
    {
        let bytes =
            policy::read_regular(&pointer, MAX_PATH_BYTES as u64).map_err(|_| Error::Invalid)?;
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    } else {
        digest.update(0u64.to_be_bytes());
    }
    for path in [
        metadata.join(CONNECTION_FILE),
        metadata.join("store/native-profile"),
        metadata.join("op_store/native-profile"),
        metadata.join("op_heads/native-profile"),
        metadata.join("store/type"),
    ] {
        let bytes = policy::read_regular(&path, 64 * 1024).map_err(|_| Error::Invalid)?;
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(digest.finalize().into())
}
