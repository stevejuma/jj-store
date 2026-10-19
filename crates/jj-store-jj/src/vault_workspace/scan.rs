//! Complete capability-contained backup tree, independent of Git tracking rules.
use super::*;
use cap_std::{ambient_authority, fs::Dir};
use jj_lib::{
    backend::TreeValue, merge::Merge, merged_tree::MergedTree,
    merged_tree_builder::MergedTreeBuilder, repo_path::RepoPathBuf, store::Store,
};
use std::{
    collections::BTreeMap,
    io::Read as _,
    time::{Duration, Instant, SystemTime},
};
const MAX_ENTRIES: usize = 100_000;
const MAX_FILE: u64 = 64 * 1024 * 1024;
const MAX_TOTAL: u64 = 512 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Stamp {
    directory: bool,
    symlink: bool,
    bytes: u64,
    modified: Option<SystemTime>,
}
pub(super) fn excluded(path: &str, exclusions: &[String]) -> bool {
    path.split('/')
        .any(|part| matches!(part, ".git" | ".jj" | ".lapis" | ".DS_Store"))
        || exclusions
            .iter()
            .any(|prefix| path == prefix || path.starts_with(&format!("{prefix}/")))
}
fn check(deadline: Instant) -> Result<()> {
    if Instant::now() > deadline {
        return Err(Error::Jj("Vault capture exceeded its work limit".into()));
    }
    Ok(())
}
fn inventory(
    root: &Dir,
    exclusions: &[String],
    deadline: Instant,
) -> Result<BTreeMap<String, Stamp>> {
    let mut pending = vec![String::new()];
    let mut result = BTreeMap::new();
    let mut total = 0u64;
    while let Some(directory) = pending.pop() {
        check(deadline)?;
        for entry in root.read_dir(if directory.is_empty() {
            "."
        } else {
            &directory
        })? {
            let entry = entry?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| Error::Binding)?;
            let path = if directory.is_empty() {
                name
            } else {
                format!("{directory}/{name}")
            };
            if excluded(&path, exclusions) {
                continue;
            }
            if path.len() > 4096 || result.len() >= MAX_ENTRIES {
                return Err(Error::Binding);
            }
            let info = root.symlink_metadata(&path)?;
            if !(info.is_file() || info.is_dir() || info.is_symlink()) {
                return Err(Error::Binding);
            }
            if info.is_file() {
                total = total.checked_add(info.len()).ok_or(Error::Binding)?;
                if info.len() > MAX_FILE || total > MAX_TOTAL {
                    return Err(Error::Jj("Vault capture exceeds its byte limit".into()));
                }
            }
            result.insert(
                path.clone(),
                Stamp {
                    directory: info.is_dir(),
                    symlink: info.is_symlink(),
                    bytes: info.len(),
                    modified: info.modified().ok().map(|t| t.into_std()),
                },
            );
            if info.is_dir() {
                pending.push(path);
            }
        }
    }
    Ok(result)
}
pub(super) async fn capture(
    store: &Arc<Store>,
    path: &Path,
    exclusions: &[String],
) -> Result<MergedTree> {
    if exclusions.len() > 256
        || exclusions.iter().any(|p| {
            p.is_empty()
                || p.len() > 4096
                || p.contains('\\')
                || p.split('/').any(|v| matches!(v, "" | "." | ".."))
        })
    {
        return Err(Error::Binding);
    }
    let root = Dir::open_ambient_dir(path, ambient_authority())?;
    let deadline = Instant::now() + Duration::from_secs(30);
    let before = inventory(&root, exclusions, deadline)?;
    let mut builder = MergedTreeBuilder::new(store.empty_merged_tree());
    for (path, stamp) in &before {
        check(deadline)?;
        if stamp.directory {
            continue;
        }
        let repo_path = RepoPathBuf::from_internal_string(path.clone()).map_err(jj)?;
        let value = if stamp.symlink {
            let target = root.read_link(path)?;
            let target = target.to_str().ok_or(Error::Binding)?;
            TreeValue::Symlink(store.write_symlink(&repo_path, target).await.map_err(jj)?)
        } else {
            let mut file = root.open(path)?;
            #[cfg(unix)]
            let executable = {
                use cap_std::fs::PermissionsExt as _;
                file.metadata()?.permissions().mode() & 0o111 != 0
            };
            #[cfg(not(unix))]
            let executable = false;
            let mut bytes = Vec::new();
            (&mut file).take(MAX_FILE + 1).read_to_end(&mut bytes)?;
            if bytes.len() as u64 != stamp.bytes || bytes.len() as u64 > MAX_FILE {
                return Err(Error::Binding);
            }
            let id = store
                .write_file(&repo_path, &mut futures::io::Cursor::new(bytes))
                .await
                .map_err(jj)?;
            TreeValue::File {
                id,
                executable,
                copy_id: jj_lib::backend::CopyId::placeholder(),
            }
        };
        builder.set_or_remove(repo_path, Merge::resolved(Some(value)));
    }
    if before != inventory(&root, exclusions, deadline)? {
        return Err(Error::Jj(
            "Vault files changed during capture; retrying is required".into(),
        ));
    }
    builder.write_tree().await.map_err(jj)
}
