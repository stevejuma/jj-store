//! Admission of JJ 0.45.1 index framing before its allocating decoder.
//! Layout follows jj-lib default_index readonly/changed_path.rs (Apache-2.0).
use super::*;

fn word(file: &mut fs::File) -> Result<u64> {
    let mut bytes = [0; 4];
    file.read_exact(&mut bytes)
        .map_err(|_| Error::Unavailable)?;
    Ok(u32::from_le_bytes(bytes).into())
}
fn name(value: &str) -> bool {
    value.len() == 128
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(crate) fn weight(
    path: &Path,
    commit_len: usize,
    change_len: usize,
    max_commits: usize,
    check: &mut impl FnMut() -> Result<()>,
) -> Result<u64> {
    let mut weight = 512 * 1024u64;
    let mut files = 0;
    let mut parents = BTreeMap::new();
    for (directory, changed_paths) in [("segments", false), ("changed_paths", true)] {
        let root = path.join(directory);
        let entries = match fs::read_dir(&root) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(Error::Unavailable),
        };
        if !fs::symlink_metadata(&root)
            .map_err(|_| Error::Unavailable)?
            .is_dir()
        {
            return Err(Error::Unavailable);
        }
        for entry in entries {
            check()?;
            files += 1;
            if files > 4096 {
                return Err(Error::Limit);
            }
            let entry = entry.map_err(|_| Error::Unavailable)?;
            let id = entry
                .file_name()
                .into_string()
                .map_err(|_| Error::Unavailable)?;
            if !name(&id) || !entry.file_type().map_err(|_| Error::Unavailable)?.is_file() {
                return Err(Error::Unavailable);
            }
            let size = entry.metadata().map_err(|_| Error::Unavailable)?.len();
            if size > LIMIT {
                return Err(Error::Limit);
            }
            let mut file = fs::File::open(entry.path()).map_err(|_| Error::Unavailable)?;
            let format = word(&mut file)?;
            let expected = if changed_paths {
                if format != 0 {
                    return Err(Error::Unavailable);
                }
                let commits = word(&mut file)?;
                let changed = word(&mut file)?;
                let paths = word(&mut file)?;
                let bytes = word(&mut file)?;
                if commits > max_commits as u64 {
                    return Err(Error::IndexBudget);
                }
                20 + (commits + 1 + changed + paths + 1) * 4 + bytes
            } else {
                if format != 6 {
                    return Err(Error::Unavailable);
                }
                let parent_len = word(&mut file)?;
                let parent = match parent_len {
                    0 => None,
                    128 => {
                        let mut bytes = [0; 128];
                        file.read_exact(&mut bytes)
                            .map_err(|_| Error::Unavailable)?;
                        let value =
                            String::from_utf8(bytes.to_vec()).map_err(|_| Error::Unavailable)?;
                        if !name(&value) {
                            return Err(Error::Unavailable);
                        }
                        Some(value)
                    }
                    _ => return Err(Error::Unavailable),
                };
                let commits = word(&mut file)?;
                let changes = word(&mut file)?;
                let parent_overflow = word(&mut file)?;
                let change_overflow = word(&mut file)?;
                if commits > max_commits as u64 {
                    return Err(Error::IndexBudget);
                }
                if changes > commits {
                    return Err(Error::Unavailable);
                }
                parents.insert(id, (parent, commits));
                24 + parent_len
                    + commits * (20 + commit_len as u64)
                    + changes * (4 + change_len as u64)
                    + (parent_overflow + change_overflow) * 4
            };
            if expected != size {
                return Err(Error::Unavailable);
            }
            weight = weight
                .checked_add(size * 2 + 8192)
                .filter(|n| *n <= LIMIT)
                .ok_or(Error::Limit)?;
        }
    }
    for id in parents.keys() {
        let mut next = Some(id);
        let mut depth = 0;
        let mut commits = 0;
        while let Some(id) = next {
            check()?;
            depth += 1;
            if depth > 128 {
                return Err(Error::Limit);
            }
            let (parent, count) = parents.get(id).ok_or(Error::Unavailable)?;
            commits += count;
            if commits > max_commits as u64 {
                return Err(Error::IndexBudget);
            }
            next = parent.as_ref();
        }
    }
    Ok(weight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;
    #[test]
    fn rejects_allocating_parent_headers_cycles_and_impossible_lengths() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join("segments")).unwrap();
        let id = "ab".repeat(64);
        let path = root.path().join("segments").join(&id);
        let test = || weight(root.path(), 64, 16, 4_000_000, &mut || Ok(()));
        let write = |words: &[u32], parent: &[u8]| {
            let mut file = fs::File::create(&path).unwrap();
            for word in &words[..2] {
                file.write_all(&word.to_le_bytes()).unwrap();
            }
            file.write_all(parent).unwrap();
            for word in &words[2..] {
                file.write_all(&word.to_le_bytes()).unwrap();
            }
        };
        write(&[6, u32::MAX], &[]);
        assert!(matches!(test(), Err(Error::Unavailable)));
        write(&[6, 128, 0, 0, 0, 0], id.as_bytes());
        assert!(matches!(test(), Err(Error::Limit)));
        write(&[6, 0, 1, 1, 0, 0], &[]);
        assert!(matches!(test(), Err(Error::Unavailable)));
        write(&[6, 0, 4_000_001, 0, 0, 0], &[]);
        assert!(matches!(test(), Err(Error::IndexBudget)));
        write(&[6, 0, 0, 0, 0, 0], &[]);
        assert_eq!(test().unwrap(), 512 * 1024 + 8192 + 48);
    }
}
