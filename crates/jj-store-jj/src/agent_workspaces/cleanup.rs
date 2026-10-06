//! Explicit cleanup retains a revision and refuses physical files absent from it.
use super::{Provider, Result, Selection};
use std::{collections::BTreeSet, fs, path::Path};
impl Provider {
    pub async fn cleanup(&self, workspace: &Selection) -> Result<()> {
        if !workspace.name.starts_with("agent-") || workspace.root.parent().is_none() {
            return Err("invalid_workspace");
        }
        let selected = self.snapshot(workspace).await?;
        let tracked = self
            .run(
                &selected.root,
                &[
                    "--ignore-working-copy",
                    "--at-op",
                    &selected.operation.to_string(),
                    "file",
                    "list",
                    "-r",
                    &selected.commit,
                    "-T",
                    "path ++ \"\\0\"",
                ],
            )
            .await?;
        let tracked: BTreeSet<_> = tracked
            .split('\0')
            .filter(|p| !p.is_empty())
            .map(str::to_owned)
            .collect();
        let before = inventory(&selected.root)?;
        if before.keys().cloned().collect::<BTreeSet<_>>() != tracked {
            return Err("cleanup_has_untracked_files");
        }
        let bookmark = format!("retained-{}", workspace.name);
        self.run(
            &selected.root,
            &[
                "--ignore-working-copy",
                "bookmark",
                "set",
                &bookmark,
                "-r",
                &selected.commit,
            ],
        )
        .await?;
        let retained = self
            .run(
                &selected.root,
                &[
                    "--ignore-working-copy",
                    "log",
                    "-r",
                    &bookmark,
                    "--no-graph",
                    "-T",
                    "commit_id",
                ],
            )
            .await?;
        if retained.trim() != selected.commit || inventory(&selected.root)? != before {
            return Err("workspace_changed");
        }
        self.run(
            &selected.root,
            &[
                "--ignore-working-copy",
                "workspace",
                "forget",
                &workspace.name,
            ],
        )
        .await?;
        // A forgotten workspace still has its physical data and retained bookmark.
        // Refuse deletion if anything changed while JJ retained the revision.
        if inventory(&selected.root)? != before {
            return Err("workspace_changed");
        }
        fs::remove_dir_all(&selected.root).map_err(|_| "cleanup_outcome_unknown")
    }
}
fn inventory(root: &Path) -> Result<std::collections::BTreeMap<String, [u8; 32]>> {
    use sha2::{Digest as _, Sha256};
    use std::io::Read as _;
    let mut found = std::collections::BTreeMap::new();
    let mut directories = vec![root.to_path_buf()];
    let mut bytes = 0u64;
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(&directory).map_err(|_| "workspace_unavailable")? {
            let entry = entry.map_err(|_| "workspace_unavailable")?;
            let path = entry.path();
            if path == root.join(".jj") {
                continue;
            }
            let stat = fs::symlink_metadata(&path).map_err(|_| "workspace_unavailable")?;
            if stat.file_type().is_symlink() || (!stat.is_file() && !stat.is_dir()) {
                return Err("cleanup_has_special_files");
            }
            if stat.is_dir() {
                directories.push(path);
                continue;
            }
            bytes = bytes.checked_add(stat.len()).ok_or("workspace_limit")?;
            if bytes > 256 * 1024 * 1024 || found.len() >= 4096 {
                return Err("workspace_limit");
            }
            let mut hash = Sha256::new();
            let mut file = fs::File::open(&path).map_err(|_| "workspace_unavailable")?;
            let mut buffer = [0; 16384];
            loop {
                let n = file
                    .read(&mut buffer)
                    .map_err(|_| "workspace_unavailable")?;
                if n == 0 {
                    break;
                }
                hash.update(&buffer[..n]);
            }
            let key = path
                .strip_prefix(root)
                .ok()
                .and_then(Path::to_str)
                .ok_or("invalid_workspace")?;
            found.insert(key.to_owned(), hash.finalize().into());
        }
    }
    Ok(found)
}
