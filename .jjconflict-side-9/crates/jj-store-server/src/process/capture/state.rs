//! Fresh-only initialization; an existing root never grants permission to repair.
use super::*;
use crate::capture_import_state::CaptureImportJournal;

pub(super) fn prepare(state: &Path, installation: [u8; 16], initialize: bool) -> Result<PathBuf> {
    let logical = state.join("agent-capture");
    let root = policy::outside_checkout(&logical)?;
    match fs::symlink_metadata(&logical) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && initialize => {
            // The caller retains authoritative storage and the whole-server state lock.
            // Exclusive creation leaves interrupted initialization unavailable on retry.
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt as _;
                builder.mode(0o700);
            }
            builder.create(&root)?;
            let control = CaptureControlJournal::create(&root.join("control"), installation)
                .map_err(|_| Error::Service("capture control initialization unavailable"))?;
            let selected = control
                .current()
                .map_err(|_| Error::Service("capture control initialization unavailable"))?;
            CaptureImportJournal::create(&root.join("imports"), installation, selected.generation)
                .map_err(|_| Error::Service("capture import initialization unavailable"))?;
            File::open(&root)?.sync_all()?;
            File::open(root.parent().ok_or(Error::Config("capture state parent"))?)?.sync_all()?;
        }
        Ok(metadata) if !metadata.is_dir() || metadata.file_type().is_symlink() => {
            return Err(Error::Config("capture state must be private"));
        }
        Ok(_) => {}
        Err(error) => return Err(Error::Io(error)),
    }
    let metadata = fs::symlink_metadata(&root)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(Error::Config("capture state must be private"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        let owner = tempfile::tempfile()?.metadata()?.uid();
        if metadata.mode() & 0o077 != 0 || metadata.uid() != owner {
            return Err(Error::Config("capture state must be private"));
        }
    }
    Ok(root)
}
