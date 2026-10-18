//! Private operation-pinned WC baseline; neither a disk snapshot nor a wire query.
pub(crate) mod current;
mod ignores;
mod status;
use super::*;
pub use current::{Checkout, CurrentState, MAX_CHECKOUT_BYTES};
use futures::AsyncReadExt as _;
pub use ignores::{IgnoreFile, IgnoreRules, SparseRules};
use jj_lib::matchers::EverythingMatcher;
use sha2::{Digest as _, Sha256};
pub use status::{LineCounts, Status};

pub const MAX_ENTRIES: usize = 4096;
pub const MAX_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_FILE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Kind {
    File { executable: bool },
    Symlink,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Entry {
    pub path: String,
    pub kind: Kind,
    pub bytes: u64,
    pub content: m::RecordDigest,
    /// Bounded private bytes; never included in serialized baseline frames.
    #[serde(skip)]
    pub data: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Baseline {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub generation: u64,
    pub operation: m::OperationId,
    pub view: m::ViewId,
    pub workspace: String,
    pub commit: m::GitSha1Id,
    pub trees: Vec<m::GitSha1Id>,
    pub labels: Vec<String>,
    pub entries: Vec<Entry>,
}
/// Same raw-byte domain as the private physical scanner; no timestamp heuristic.
pub fn content_digest(bytes: &[u8]) -> m::RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/physical-content/v1\0");
    hash.update(bytes);
    m::RecordDigest::new(hash.finalize().into())
}
pub(crate) fn safe_path(value: &str) -> Result<jj_lib::repo_path::RepoPathBuf> {
    if value.is_empty()
        || value.contains('\\')
        || value.chars().any(char::is_control)
        || value.split('/').any(|v| {
            v.is_empty() || v.eq_ignore_ascii_case(".jj") || v.eq_ignore_ascii_case(".git")
        })
    {
        return Err(Error::Invalid);
    }
    native::path(value)
}
impl Executor {
    /// Only trusted composition supplies the workspace name and access owner.
    /// Historical selection is allowed: caller must prove active physical state separately.
    pub async fn working_tree_baseline(
        &self,
        access: Arc<dyn Access>,
        operation: m::OperationId,
        workspace: String,
    ) -> Result<Baseline> {
        let limits = self.limits;
        self.worker(access, move |access, budget| {
            Box::pin(async move {
                if workspace.is_empty()
                    || workspace.len() > 256
                    || workspace.chars().any(char::is_control)
                {
                    return Err(Error::Invalid);
                }
                let context =
                    Context::open(access, operation, budget, limits, true, vec![], false).await?;
                read(&context, operation, workspace).await
            })
        })
        .await
    }
}
async fn read(context: &Context, operation: m::OperationId, workspace: String) -> Result<Baseline> {
    let view_id = context.operation(operation).await?.view_id;
    let view = context.view(view_id).await?;
    let commit_id = *view.wc_commit_ids.get(&workspace).ok_or(Error::NotFound)?;
    read_commit(context, operation, view_id, workspace, commit_id).await
}
pub(super) async fn read_commit(
    context: &Context,
    operation: m::OperationId,
    view_id: m::ViewId,
    workspace: String,
    commit_id: m::GitSha1Id,
) -> Result<Baseline> {
    let commit = context.commit(&context.store, commit_id).await?;
    let mut entries = Vec::new();
    let mut bytes = 0usize;
    for (path, value) in commit.tree().entries_matching(&EverythingMatcher) {
        context.budget.authorize(&context.access).await?;
        context.budget.work(1)?;
        if entries.len() >= MAX_ENTRIES {
            return Err(Error::Limit);
        }
        let value = value.map_err(native::map_backend)?;
        let Some(value) = value.as_resolved() else {
            return Err(Error::Invalid);
        };
        let Some(value) = value else { continue };
        let name = path.as_internal_file_string();
        safe_path(name)?;
        let (kind, id) = match value {
            b::TreeValue::File { id, executable, .. } => (
                Kind::File {
                    executable: *executable,
                },
                git_id(id)?,
            ),
            b::TreeValue::Symlink(id) => (Kind::Symlink, git_id(id)?),
            _ => return Err(Error::Invalid),
        };
        let data = raw_bytes(context, &path, id, kind, MAX_BYTES - bytes).await?;
        bytes = bytes.checked_add(data.len()).ok_or(Error::Limit)?;
        if bytes > MAX_BYTES {
            return Err(Error::Limit);
        }
        let entry = Entry {
            path: name.into(),
            kind,
            bytes: data.len() as u64,
            content: content_digest(&data),
            data,
        };
        context.budget.output(&entry)?;
        entries.push(entry);
    }
    // JJ's traversal order is lexical, but preserve an explicit owned invariant.
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    if entries.windows(2).any(|w| w[0].path == w[1].path) {
        return Err(Error::Corrupt);
    }
    Ok(Baseline {
        version: 1,
        repository: context.access.repository(),
        generation: context.access.generation(),
        operation,
        view: view_id,
        workspace,
        commit: commit_id,
        trees: commit
            .tree_ids()
            .iter()
            .map(git_id)
            .collect::<Result<_>>()?,
        entries,
        labels: commit.tree().labels().as_slice().to_vec(),
    })
}
async fn raw_bytes(
    context: &Context,
    path: &jj_lib::repo_path::RepoPath,
    id: m::GitSha1Id,
    kind: Kind,
    remaining: usize,
) -> Result<Vec<u8>> {
    let source = &context.scoped.as_ref().ok_or(Error::Unavailable)?.source;
    let git = source
        .backend()
        .downcast_ref::<jj_lib::git_backend::GitBackend>()
        .ok_or(Error::Invalid)?
        .git_repo();
    let header = git
        .find_header(gix::ObjectId::from_bytes_or_panic(id.as_bytes()))
        .map_err(|_| Error::Incomplete)?;
    if header.kind() != gix::object::Kind::Blob {
        return Err(Error::Corrupt);
    }
    if header.size() > MAX_FILE_BYTES.min(remaining) as u64 {
        return Err(Error::Limit);
    }
    let bytes = match kind {
        Kind::File { .. } => {
            let mut input = context
                .store
                .backend()
                .read_file(path, &b::FileId::from_bytes(id.as_bytes()))
                .await
                .map_err(native::map_backend)?;
            let mut bytes = Vec::new();
            (&mut input)
                .take(MAX_FILE_BYTES as u64 + 1)
                .read_to_end(&mut bytes)
                .await
                .map_err(|_| Error::Unavailable)?;
            bytes
        }
        Kind::Symlink => context
            .store
            .backend()
            .read_symlink(path, &b::SymlinkId::from_bytes(id.as_bytes()))
            .await
            .map_err(native::map_backend)?
            .into_bytes(),
    };
    context.budget.read_bytes(bytes.len())?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err(Error::Limit);
    }
    if bytes.len() as u64 != header.size() {
        return Err(Error::Corrupt);
    }
    Ok(bytes)
}
