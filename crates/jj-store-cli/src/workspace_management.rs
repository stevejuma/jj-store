//! Read-only physical JJ workspace discovery for installation-local settings.
use crate::binding::{CONNECTION_FILE, Connection};
use jj_lib::workspace::{DefaultWorkspaceLoaderFactory, WorkspaceLoaderFactory as _};
use jj_store_server::policy;
use jj_store_service::local_management::{
    Error, MAX_PATH_BYTES, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
};
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
}
