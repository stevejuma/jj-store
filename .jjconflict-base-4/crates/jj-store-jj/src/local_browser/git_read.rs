//! Bounded source reads without GitBackend extra import or table reconciliation.
pub(super) mod commit;
pub(crate) mod extras;
use super::*;

pub(crate) struct Reader {
    extras: extras::Extras,
    shallow: BTreeSet<gix::ObjectId>,
}
impl Reader {
    pub fn open(backend: &dyn b::Backend, metadata: &Path, budget: &Budget) -> Result<Self> {
        let git = backend
            .downcast_ref::<jj_lib::git_backend::GitBackend>()
            .ok_or(Error::Invalid)?
            .git_repo();
        Ok(Self {
            shallow: shallow_roots(&git, budget)?,
            extras: extras::Extras::open_point(metadata, budget)?,
        })
    }
    pub fn commit(
        &self,
        backend: &dyn b::Backend,
        id: &b::CommitId,
        budget: &Budget,
    ) -> Result<b::Commit> {
        let git = backend
            .downcast_ref::<jj_lib::git_backend::GitBackend>()
            .ok_or(Error::Invalid)?
            .git_repo();
        commit::read(backend, &git, &self.extras, &self.shallow, id, budget)
    }
    /// Recovery must preserve original JJ extras, never infer a synthetic
    /// change identity when the original record has disappeared.
    pub(crate) fn exact_commit(
        &self,
        backend: &dyn b::Backend,
        id: &b::CommitId,
        budget: &Budget,
    ) -> Result<b::Commit> {
        if id != backend.root_commit_id() {
            self.extras.require(id.as_bytes(), budget)?;
        }
        self.commit(backend, id, budget)
    }
}

pub(super) fn shallow_roots(
    git: &gix::Repository,
    budget: &Budget,
) -> Result<BTreeSet<gix::ObjectId>> {
    let path = git.shallow_file();
    let info = match std::fs::symlink_metadata(&path) {
        Ok(info) => info,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        Err(_) => return Err(Error::Incomplete),
    };
    if !info.is_file() || info.len() > 1024 * 1024 {
        return Err(Error::Limit);
    }
    budget.read_bytes(info.len() as usize)?;
    // A bounded read also prevents a concurrently growing file from allocating
    // more than admitted before parsing the captured shallow-root set.
    use std::io::Read as _;
    let mut bytes = vec![];
    std::fs::File::open(path)
        .map_err(|_| Error::Incomplete)?
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::Incomplete)?;
    if bytes.len() as u64 != info.len() {
        return Err(Error::Incomplete);
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| Error::Corrupt)?;
    text.lines()
        .map(|line| {
            budget.work(1)?;
            if line.len() != 40 {
                return Err(Error::Corrupt);
            }
            gix::ObjectId::from_hex(line.as_bytes()).map_err(|_| Error::Corrupt)
        })
        .collect()
}
