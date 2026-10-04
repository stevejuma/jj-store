//! Publish a fully prepared new local folder without replacing any destination.
use super::Result;
use std::{fs, path::Path};
fn sync_tree(path: &Path) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            sync_tree(&entry.path())?;
        } else if kind.is_file() {
            fs::File::open(entry.path())?.sync_all()?;
        } else if !kind.is_symlink() {
            return Err("unsupported destination entry".into());
        }
    }
    fs::File::open(path)?.sync_all()?;
    Ok(())
}
pub fn publish_directory(staging: &Path, destination: &Path) -> Result<()> {
    if staging.parent() != destination.parent() {
        return Err("conversion destination must share staging filesystem".into());
    }
    sync_tree(staging)?;
    rustix::fs::renameat_with(
        rustix::fs::CWD,
        staging,
        rustix::fs::CWD,
        destination,
        rustix::fs::RenameFlags::NOREPLACE,
    )?;
    fs::File::open(destination.parent().ok_or("destination parent")?)?.sync_all()?;
    Ok(())
}
