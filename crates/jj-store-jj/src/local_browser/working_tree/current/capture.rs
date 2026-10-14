use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CheckoutFiles {
    pub workspace: m::RecordDigest,
    pub binding: m::RecordDigest,
    pub kind: physical::Bytes,
    pub checkout: physical::Bytes,
    pub tree: physical::Bytes,
}
impl CheckoutFiles {
    pub fn read(root: &Path, metadata: &Path, budget: &Budget) -> Result<Self> {
        let dir = physical::directory(root)?;
        let jj = physical::child(&dir, ".jj")?;
        let wc = physical::child(&jj, "working_copy")?;
        let stat = rustix::fs::statat(&jj, "repo", rustix::fs::AtFlags::SYMLINK_NOFOLLOW)
            .map_err(|_| Error::Unavailable)?;
        let (path, repo_binding) = match rustix::fs::FileType::from_raw_mode(stat.st_mode) {
            rustix::fs::FileType::Directory => (
                root.join(".jj/repo"),
                content_digest(
                    &[i128::from(stat.st_dev) as u64, stat.st_ino]
                        .into_iter()
                        .flat_map(u64::to_be_bytes)
                        .collect::<Vec<_>>(),
                ),
            ),
            rustix::fs::FileType::RegularFile => {
                let data =
                    physical::read(&jj, "repo", 4096, false, budget)?.ok_or(Error::Unavailable)?;
                let path = std::str::from_utf8(&data.bytes).map_err(|_| Error::Invalid)?;
                if path.is_empty() || path.chars().any(char::is_control) {
                    return Err(Error::Invalid);
                }
                (root.join(".jj").join(path), data.fingerprint)
            }
            _ => return Err(Error::Invalid),
        };
        if path.canonicalize().map_err(|_| Error::Unavailable)?
            != metadata.canonicalize().map_err(|_| Error::Unavailable)?
        {
            return Err(Error::EpochChanged);
        }
        let kind = physical::read(&wc, "type", 64, false, budget)?.ok_or(Error::Unavailable)?;
        if kind.bytes != b"local" {
            return Err(Error::Invalid);
        }
        let mut binding = Sha256::new_with_prefix(b"jj-store/current-working-copy-binding/v1\0");
        binding.update(repo_binding.as_bytes());
        let wc_stat = rustix::fs::fstat(&wc).map_err(|_| Error::Unavailable)?;
        let metadata_stat =
            rustix::fs::fstat(physical::directory(metadata)?).map_err(|_| Error::Unavailable)?;
        for part in [
            i128::from(wc_stat.st_dev) as u64,
            wc_stat.st_ino,
            i128::from(metadata_stat.st_dev) as u64,
            metadata_stat.st_ino,
        ] {
            binding.update(part.to_be_bytes());
        }
        Ok(Self {
            workspace: physical::workspace(root, &dir, &jj)?,
            binding: m::RecordDigest::new(binding.finalize().into()),
            kind,
            checkout: physical::read(&wc, "checkout", 65536, false, budget)?
                .ok_or(Error::Unavailable)?,
            tree: physical::read(&wc, "tree_state", MAX_CHECKOUT_BYTES, false, budget)?
                .ok_or(Error::Unavailable)?,
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Policy {
    pub inputs: Vec<Vec<u8>>,
    pub fingerprint: m::RecordDigest,
}
pub(super) fn policy(root: &Path, metadata: &Path, budget: &Budget) -> Result<Policy> {
    budget.check()?;
    let source = loader(metadata)?.store().clone();
    let backend = source
        .backend()
        .downcast_ref::<jj_lib::git_backend::GitBackend>()
        .ok_or(Error::Invalid)?;
    let git = backend.git_repo();
    policy_at_git(root, &git, backend.git_repo_path(), budget)
}
/// Existing Git policy can be checked without loading a missing operation or index store.
pub(crate) fn policy_at_git(
    root: &Path,
    git: &gix::Repository,
    git_path: &Path,
    budget: &Budget,
) -> Result<Policy> {
    let configured = git
        .config_snapshot()
        .string("core.excludesFile")
        .map(|v| {
            std::str::from_utf8(&v)
                .map(jj_lib::file_util::expand_home_path)
                .map_err(|_| Error::Invalid)
        })
        .transpose()?;
    let global = match configured {
        Some(path) => root.join(path),
        None => match std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
            Some(path) => PathBuf::from(path).join("git/ignore"),
            None => etcetera::home_dir()
                .map_err(|_| Error::Unavailable)?
                .join(".config/git/ignore"),
        },
    };
    if !global.is_absolute() {
        return Err(Error::Invalid);
    }
    let paths = [global, git_path.join("info/exclude")];
    let mut hash = Sha256::new_with_prefix(b"jj-store/current-ignore-policy/v1\0");
    let mut inputs = Vec::new();
    for path in paths {
        budget.check()?;
        let bytes = path.as_os_str().as_encoded_bytes();
        hash.update((bytes.len() as u64).to_be_bytes());
        hash.update(bytes);
        if let Some(data) = physical::absolute(&path, budget)? {
            hash.update([1]);
            hash.update(data.fingerprint.as_bytes());
            inputs.push(data.bytes);
        } else {
            hash.update([0]);
            inputs.push(vec![]);
        }
    }
    Ok(Policy {
        inputs,
        fingerprint: m::RecordDigest::new(hash.finalize().into()),
    })
}
