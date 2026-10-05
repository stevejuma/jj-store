//! Captured ignore bytes only. Never follows a disk link or implicitly reads user config.
use super::*;
use jj_lib::matchers::{Matcher as _, PrefixMatcher};
use jj_lib::{
    gitignore::GitIgnoreFile,
    repo_path::{RepoPath, RepoPathBuf},
};
pub struct SparseRules(PrefixMatcher);
impl SparseRules {
    pub fn new(prefixes: Vec<String>) -> Result<Self> {
        if prefixes.len() > 512 {
            return Err(Error::Limit);
        }
        let paths = prefixes
            .iter()
            .map(|p| {
                if p.is_empty() {
                    Ok(RepoPathBuf::root())
                } else {
                    safe_path(p)
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(PrefixMatcher::new(paths)))
    }
    pub fn matches(&self, path: &str) -> Result<bool> {
        Ok(self.0.matches(&safe_path(path)?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IgnoreFile {
    pub path: String,
    pub bytes: Vec<u8>,
}
pub struct IgnoreRules {
    directories: BTreeMap<String, (Arc<GitIgnoreFile>, bool)>,
}
impl IgnoreRules {
    /// `base` is the explicitly captured global/excludes sequence in JJ priority order.
    /// Empty means verified absent policy, never a fallback for a failed read.
    pub fn new(base: Vec<Vec<u8>>, files: Vec<IgnoreFile>) -> Result<Self> {
        Self::with_guard(base, files, || Ok(()))
    }
    pub fn with_guard(
        base: Vec<Vec<u8>>,
        files: Vec<IgnoreFile>,
        mut guard: impl FnMut() -> Result<()>,
    ) -> Result<Self> {
        guard()?;
        if base.len() > 2 || files.len() > MAX_ENTRIES {
            return Err(Error::Limit);
        }
        let mut bytes = 0usize;
        let mut rules = GitIgnoreFile::empty();
        for input in base {
            guard()?;
            admit(&mut bytes, &input)?;
            rules = rules
                .chain(RepoPath::root(), Path::new("base-ignore"), &input)
                .map_err(|_| Error::Invalid)?;
        }
        let mut captured = BTreeMap::new();
        for file in files {
            guard()?;
            safe_path(&file.path)?;
            if file.path.rsplit('/').next() != Some(".gitignore") {
                return Err(Error::Invalid);
            }
            admit(&mut bytes, &file.bytes)?;
            let parent = file.path.rsplit_once('/').map_or("", |(p, _)| p).to_owned();
            if captured.insert(parent, file).is_some() {
                return Err(Error::Invalid);
            }
        }
        if let Some(file) = captured.remove("") {
            guard()?;
            rules = rules
                .chain(RepoPath::root(), Path::new(&file.path), &file.bytes)
                .map_err(|_| Error::Invalid)?;
        }
        let mut directories = BTreeMap::from([(String::new(), (rules, false))]);
        for (dir, file) in captured {
            guard()?;
            let (parent, ignored) = inherited(&directories, &dir)?;
            let path = safe_path(&dir)?;
            let ignored = ignored || ignored_ancestors(&parent, &path) || parent.matches_dir(&path);
            let rules = if ignored {
                parent
            } else {
                parent
                    .chain(&path, Path::new(&file.path), &file.bytes)
                    .map_err(|_| Error::Invalid)?
            };
            directories.insert(dir, (rules, ignored));
        }
        guard()?;
        Ok(Self { directories })
    }
    pub fn ignored(&self, name: &str) -> Result<bool> {
        let path = safe_path(name)?;
        let parent = name.rsplit_once('/').map_or("", |(p, _)| p);
        let (rules, ignored) = inherited(&self.directories, parent)?;
        Ok(ignored || ignored_ancestors(&rules, &path) || rules.matches_file(&path))
    }
}
fn admit(total: &mut usize, bytes: &[u8]) -> Result<()> {
    *total = total.checked_add(bytes.len()).ok_or(Error::Limit)?;
    if bytes.len() > MAX_FILE_BYTES || *total > MAX_BYTES {
        return Err(Error::Limit);
    }
    Ok(())
}
fn inherited(
    map: &BTreeMap<String, (Arc<GitIgnoreFile>, bool)>,
    dir: &str,
) -> Result<(Arc<GitIgnoreFile>, bool)> {
    let mut current = dir;
    loop {
        if let Some((rules, ignored)) = map.get(current) {
            return Ok((rules.clone(), *ignored));
        }
        current = current.rsplit_once('/').map_or("", |(parent, _)| parent);
    }
}
fn ignored_ancestors(rules: &GitIgnoreFile, path: &RepoPath) -> bool {
    let mut parent = RepoPathBuf::root();
    let mut parts = path.components().peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            break;
        }
        parent = parent.join(part);
        if rules.matches_dir(&parent) {
            return true;
        }
    }
    false
}
