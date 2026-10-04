//! Read-only device binding for a private spool; never persisted or debug-rendered.
use crate::{CaptureResult, agent_capture::AgentCaptureSpool};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq)]
pub struct CaptureSpoolBinding {
    root: PathBuf,
    device: u64,
    inode: u64,
    owner: u32,
}
impl AgentCaptureSpool {
    pub fn binding(&self) -> CaptureResult<CaptureSpoolBinding> {
        #[cfg(unix)]
        {
            use std::{fs, os::unix::fs::MetadataExt as _};
            let root = self.state_root();
            let metadata = fs::symlink_metadata(root)?;
            if !metadata.is_dir()
                || metadata.file_type().is_symlink()
                || fs::canonicalize(root)? != root
                || metadata.mode() & 0o077 != 0
            {
                return Err("capture spool binding unavailable".into());
            }
            Ok(CaptureSpoolBinding {
                root: root.to_owned(),
                device: metadata.dev(),
                inode: metadata.ino(),
                owner: metadata.uid(),
            })
        }
        #[cfg(not(unix))]
        {
            Err("capture spool physical identity unavailable".into())
        }
    }
}
