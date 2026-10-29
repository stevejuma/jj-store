use super::*;
use crate::local_browser::{budget::Budget, git_read::Reader, working_tree::current::physical};
use jj_lib::{
    default_backend_factories::default_backend_factories, op_store::OpStore as _, repo::RepoLoader,
    settings::UserSettings, simple_op_store::SimpleOpStore,
};
use std::{path::Path, time::Duration};
#[derive(Clone, Copy)]
pub struct OperationArchiveLimits {
    pub max_operations: usize,
    pub max_commits: usize,
    pub max_bytes: usize,
    pub timeout: Duration,
}
impl Default for OperationArchiveLimits {
    fn default() -> Self {
        Self {
            max_operations: MAX_OPERATIONS,
            max_commits: 100_000,
            max_bytes: MAX_ARCHIVE_BYTES,
            timeout: Duration::from_secs(30),
        }
    }
}
pub async fn capture_operation_archive(
    settings: &UserSettings,
    path: &Path,
    repository: m::RepositoryId,
    operation: m::OperationId,
    limits: OperationArchiveLimits,
) -> Result<OperationArchiveV1> {
    if limits.max_operations == 0
        || limits.max_operations > MAX_OPERATIONS
        || limits.max_commits == 0
        || limits.max_commits > 100_000
        || limits.max_bytes == 0
        || limits.max_bytes > MAX_ARCHIVE_BYTES
        || limits.timeout.is_zero()
        || limits.timeout > Duration::from_secs(30)
    {
        return Err(Error::Limit);
    }
    tokio::time::timeout(
        limits.timeout,
        capture(settings, path, repository, operation, limits),
    )
    .await
    .map_err(|_| Error::Limit)?
}
async fn capture(
    settings: &UserSettings,
    path: &Path,
    repository: m::RepositoryId,
    operation: m::OperationId,
    limits: OperationArchiveLimits,
) -> Result<OperationArchiveV1> {
    let budget = Budget::new(crate::query::content::Limits {
        timeout: limits.timeout,
        ..Default::default()
    });
    let directory =
        physical::directory(path).map_err(|_| Error::Invalid("source metadata directory"))?;
    let source = physical::child(&directory, "op_store")
        .map_err(|_| Error::Invalid("source operation directory"))?;
    let kind = physical::read(&source, "type", 64, false, &budget)
        .map_err(|_| Error::Invalid("source operation type"))?
        .ok_or(Error::Invalid("source operation type"))?;
    if kind.bytes != SimpleOpStore::NAME.as_bytes() {
        return Err(Error::Invalid("source operation profile"));
    }
    let operations =
        physical::child(&source, "operations").map_err(|_| Error::Invalid("source operations"))?;
    let views = physical::child(&source, "views").map_err(|_| Error::Invalid("source views"))?;
    let temp = tempfile::tempdir()?;
    std::fs::create_dir(temp.path().join("operations"))?;
    std::fs::create_dir(temp.path().join("views"))?;
    let store = SimpleOpStore::load(
        temp.path(),
        o::RootOperationData {
            root_commit_id: git_native_commit_id(m::GitSha1Id::new([0; 20])),
        },
    );
    let loader = RepoLoader::init_from_file_system(settings, path, &default_backend_factories())
        .map_err(Error::other)?;
    if loader
        .store()
        .backend()
        .downcast_ref::<jj_lib::git_backend::GitBackend>()
        .is_none()
    {
        return Err(Error::Invalid("source Git profile"));
    }
    // Upstream GitBackend::read_commit may import missing extras and create
    // no-GC refs. The owned reader must keep archive capture read-only.
    let reader = Reader::open(loader.store().backend(), path, &budget).map_err(read_error)?;
    let mut archive = OperationArchiveV1 {
        version: 1,
        profile: m::portable::CodeProfileV1::GitSha1Jj045,
        repository,
        operation,
        operations: BTreeMap::new(),
        views: BTreeMap::new(),
        commits: BTreeMap::new(),
    };
    let mut pending = BTreeSet::from([operation]);
    let mut commits = BTreeSet::new();
    let mut raw = 0usize;
    let mut encoded = 256usize;
    while let Some(id) = pending.pop_first() {
        budget.check().map_err(|_| Error::Limit)?;
        if archive.operations.contains_key(&id) {
            continue;
        }
        if archive.operations.len() >= limits.max_operations {
            return Err(Error::Limit);
        }
        copy(
            &operations,
            temp.path(),
            "operations",
            &id.to_string(),
            &budget,
            &mut raw,
            limits,
        )?;
        let data = store
            .read_operation(&native_operation_id(id))
            .await
            .map_err(Error::other)?;
        let op = GitOperationV1::try_from(&data)?;
        pending.extend(op.parents.iter().copied());
        if let std::collections::btree_map::Entry::Vacant(entry) = archive.views.entry(op.view_id) {
            copy(
                &views,
                temp.path(),
                "views",
                &op.view_id.to_string(),
                &budget,
                &mut raw,
                limits,
            )?;
            let view = store
                .read_view(&native_view_id(op.view_id))
                .await
                .map_err(Error::other)?;
            let view = GitViewV1::try_from(&view)?;
            charge(&mut encoded, &view, limits)?;
            entry.insert(view);
        }
        commits.extend(
            archive
                .context(id, &op, &archive.views[&op.view_id])
                .direct_commits()?,
        );
        charge(&mut encoded, &op, limits)?;
        archive.operations.insert(id, op);
    }
    while let Some(id) = commits.pop_first() {
        budget.check().map_err(|_| Error::Limit)?;
        if archive.commits.contains_key(&id) {
            continue;
        }
        if archive.commits.len() >= limits.max_commits {
            return Err(Error::Limit);
        }
        let commit = reader
            .exact_commit(loader.store().backend(), &git_native_commit_id(id), &budget)
            .map_err(read_error)?;
        let wire = GitCommitV1::try_from(&commit)?;
        if b::Commit::try_from(wire.clone())? != commit {
            return Err(Error::Invalid("lossy archived code"));
        }
        charge(&mut encoded, &wire, limits)?;
        commits.extend(wire.parents.iter().copied());
        commits.extend(wire.predecessors.iter().copied());
        archive.commits.insert(id, wire);
    }
    if archive.encode()?.len() > limits.max_bytes {
        return Err(Error::Limit);
    }
    Ok(archive)
}
fn read_error(error: crate::local_browser::Error) -> Error {
    match error {
        crate::local_browser::Error::Limit => Error::Limit,
        _ => Error::Invalid("incomplete or unsafe archive code"),
    }
}
fn charge(total: &mut usize, value: &impl Serialize, limits: OperationArchiveLimits) -> Result<()> {
    *total = total
        .checked_add(serde_json::to_vec(value).map_err(Error::other)?.len() + 160)
        .ok_or(Error::Limit)?;
    if *total > limits.max_bytes {
        return Err(Error::Limit);
    }
    Ok(())
}
fn copy(
    source: &std::fs::File,
    temp: &Path,
    directory: &str,
    id: &str,
    budget: &Budget,
    raw: &mut usize,
    limits: OperationArchiveLimits,
) -> Result<()> {
    if id.bytes().all(|b| b == b'0') {
        return Ok(());
    }
    let bytes = physical::read(source, id, 4 * 1024 * 1024, false, budget)
        .map_err(|_| Error::Invalid("unsafe or missing source operation/view"))?
        .ok_or(Error::Invalid("missing source operation/view"))?;
    *raw = raw.checked_add(bytes.bytes.len()).ok_or(Error::Limit)?;
    if *raw > limits.max_bytes {
        return Err(Error::Limit);
    }
    std::fs::write(temp.join(directory).join(id), bytes.bytes)?;
    Ok(())
}
