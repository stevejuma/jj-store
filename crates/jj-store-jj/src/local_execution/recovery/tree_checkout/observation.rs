use super::*;
use crate::local_browser::working_tree::current::physical;
use rustix::fs::{self, AtFlags, FileType};
use std::{fs::File, path::Path};
fn identity(dir: &File) -> Result<m::RecordDigest> {
    let info = fs::fstat(dir).map_err(|_| Error::Unavailable)?;
    let mut hash = Sha256::new_with_prefix(b"jj-store/tree-checkout-directory/v1\0");
    hash.update((i128::from(info.st_dev) as u64).to_be_bytes());
    hash.update(info.st_ino.to_be_bytes());
    Ok(m::RecordDigest::new(hash.finalize().into()))
}
fn record(
    dirs: &mut BTreeMap<String, Option<m::RecordDigest>>,
    path: String,
    id: Option<m::RecordDigest>,
) -> Result<()> {
    if dirs.insert(path, id).is_some_and(|previous| previous != id) {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
fn observe(root: &Path, changes: &mut [Change], budget: &Budget) -> Result<Vec<Directory>> {
    let root_dir = physical::directory(root)?;
    let mut dirs = BTreeMap::from([(String::new(), Some(identity(&root_dir)?))]);
    for change in changes {
        budget.work(1)?;
        safe_path(&change.path)?;
        let mut parent = Some(root_dir.try_clone().map_err(|_| Error::Unavailable)?);
        let parts: Vec<_> = change.path.split('/').collect();
        let mut path = String::new();
        for name in &parts[..parts.len() - 1] {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(name);
            if let Some(dir) = &parent {
                budget.work(1)?;
                match fs::statat(dir, *name, AtFlags::SYMLINK_NOFOLLOW) {
                    Ok(stat) if FileType::from_raw_mode(stat.st_mode) == FileType::Directory => {
                        let child = physical::child(dir, name)?;
                        let actual = fs::fstat(&child).map_err(|_| Error::Unavailable)?;
                        if (actual.st_dev, actual.st_ino) != (stat.st_dev, stat.st_ino) {
                            return Err(Error::EpochChanged);
                        }
                        record(&mut dirs, path.clone(), Some(identity(&child)?))?;
                        parent = Some(child);
                    }
                    Err(e) if e == rustix::io::Errno::NOENT => {
                        record(&mut dirs, path.clone(), None)?;
                        parent = None;
                    }
                    _ => return Err(Error::Invalid),
                }
            } else {
                record(&mut dirs, path.clone(), None)?;
            }
        }
        if let Some(dir) = parent {
            let name = parts[parts.len() - 1];
            let data = physical::read(&dir, name, tree::MAX_FILE_BYTES, true, budget)?;
            match (&change.before, data) {
                (Some(before), Some(data)) => {
                    let stat = fs::statat(&dir, name, AtFlags::SYMLINK_NOFOLLOW)
                        .map_err(|_| Error::EpochChanged)?;
                    if tree::content_digest(&data.bytes) != before.content
                        || data.bytes.len() as u64 != before.size
                        || (stat.st_mode & 0o111 != 0) != before.executable
                        || physical::read(&dir, name, tree::MAX_FILE_BYTES, false, budget)?.as_ref()
                            != Some(&data)
                    {
                        return Err(Error::EpochChanged);
                    }
                    change.observed = Some(data.fingerprint);
                }
                (None, None) => change.observed = None,
                _ => return Err(Error::EpochChanged),
            }
        } else if change.before.is_some() {
            return Err(Error::EpochChanged);
        } else {
            change.observed = None;
        }
    }
    Ok(dirs
        .into_iter()
        .map(|(path, identity)| Directory { path, identity })
        .collect())
}
pub(super) fn capture(
    root: &Path,
    changes: &mut [Change],
    budget: &Budget,
) -> Result<Vec<Directory>> {
    observe(root, changes, budget)
}
pub(super) fn validate(root: &Path, plan: &TreeCheckoutPlanV1, budget: &Budget) -> Result<()> {
    let mut changes = plan.changes.clone();
    let dirs = observe(root, &mut changes, budget)?;
    if changes != plan.changes || dirs != plan.directories {
        return Err(Error::EpochChanged);
    }
    Ok(())
}
