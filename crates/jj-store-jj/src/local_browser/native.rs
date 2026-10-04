use super::*;
use futures::AsyncReadExt as _;
use jj_lib::repo_path::RepoPathBuf;
pub(super) fn path(value: &str) -> Result<RepoPathBuf> {
    if value.len() > 4096
        || value.contains('\0')
        || value.split('/').any(|v| matches!(v, "." | ".."))
    {
        return Err(Error::Invalid);
    }
    let path = RepoPathBuf::from_internal_string(value).map_err(|_| Error::Invalid)?;
    if path.components().count() > 64 {
        return Err(Error::Limit);
    }
    Ok(path)
}
pub(super) fn terms(value: &b::MergedTreeValue) -> Result<Vec<Option<dto::TreeValue>>> {
    if value.iter().len() > 65 {
        return Err(Error::Limit);
    }
    value
        .iter()
        .map(|v| v.as_ref().map(dto::TreeValue::try_from).transpose())
        .collect()
}
pub(super) fn map_backend(error: b::BackendError) -> Error {
    match error {
        b::BackendError::ObjectNotFound { .. } => Error::Incomplete,
        _ => Error::Unavailable,
    }
}
pub(super) enum Text {
    Utf8(String),
    Binary,
    TooLarge,
}
impl Context {
    pub(super) async fn commit(
        &self,
        store: &Arc<jj_lib::store::Store>,
        id: m::GitSha1Id,
    ) -> Result<jj_lib::commit::Commit> {
        self.member(ObjectKey::Commit(id)).await?;
        store
            .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(map_backend)
    }
    pub(super) async fn file_content(&self, id: m::GitSha1Id) -> Result<FileContent> {
        self.member(ObjectKey::File(id)).await?;
        // GitBackend's file reader is an owned local byte stream. Never run a textconv or pager.
        let backend = self
            .store
            .backend()
            .downcast_ref::<jj_lib::git_backend::GitBackend>()
            .ok_or(Error::Unavailable)?;
        let git = backend.git_repo();
        let oid = gix::ObjectId::from_bytes_or_panic(id.as_bytes());
        let header = git.find_header(oid).map_err(|_| Error::Incomplete)?;
        if header.kind() != gix::object::Kind::Blob {
            return Err(Error::Corrupt);
        }
        let length: usize = header.size().try_into().map_err(|_| Error::Limit)?;
        if length > self.limits.max_text_bytes {
            return Ok(FileContent {
                bytes: length,
                text: None,
                binary: false,
                too_large: true,
            });
        }
        let reader = self
            .store
            .backend()
            .read_file(
                jj_lib::repo_path::RepoPath::root(),
                &b::FileId::from_bytes(id.as_bytes()),
            )
            .await
            .map_err(map_backend)?;
        let mut bytes = Vec::new();
        reader
            .take((self.limits.max_text_bytes + 1) as u64)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| Error::Unavailable)?;
        self.budget.read_bytes(bytes.len())?;
        let too_large = bytes.len() > self.limits.max_text_bytes;
        if length != bytes.len() {
            return Err(Error::Corrupt);
        }
        let binary = bytes.contains(&0) || std::str::from_utf8(&bytes).is_err();
        let text = if too_large || binary {
            None
        } else {
            Some(String::from_utf8(bytes).map_err(|_| Error::Corrupt)?)
        };
        Ok(FileContent {
            bytes: length,
            text,
            binary,
            too_large,
        })
    }
    pub(super) async fn text(&self, id: m::GitSha1Id) -> Result<Text> {
        let value = self.file_content(id).await?;
        Ok(if value.too_large {
            Text::TooLarge
        } else if let Some(text) = value.text {
            Text::Utf8(text)
        } else {
            Text::Binary
        })
    }
}
/// Content predicates and cross-operation expressions are separately qualified.
/// A parsed expression cannot cause unbounded file reads or select arbitrary operations.
pub(super) fn admit_revset(
    expression: &jj_lib::revset::UserRevsetExpression,
    budget: &Budget,
) -> Result<()> {
    use jj_lib::revset::{RevsetExpression as E, RevsetFilterPredicate as F};
    let mut stack = vec![expression];
    while let Some(expression) = stack.pop() {
        budget.work(1)?;
        match expression {
            E::Filter(F::File(_) | F::DiffLines { .. }) | E::AtOperation { .. } => {
                return Err(crate::query::language::invalid(
                    "unsupported_predicate",
                    "This local repository does not support file predicates or at_operation().",
                ));
            }
            E::Ancestors { heads, .. } => stack.push(heads),
            E::Descendants { roots, .. } => stack.push(roots),
            E::Range { roots, heads, .. } | E::DagRange { roots, heads } => {
                stack.extend([roots.as_ref(), heads.as_ref()])
            }
            E::Reachable { sources, domain } => stack.extend([sources.as_ref(), domain.as_ref()]),
            E::HeadsRange {
                roots,
                heads,
                filter,
                ..
            } => stack.extend([roots.as_ref(), heads.as_ref(), filter.as_ref()]),
            E::Heads(a)
            | E::Roots(a)
            | E::ForkPoint(a)
            | E::MergePoint(a)
            | E::Bisect(a)
            | E::AsFilter(a)
            | E::Present(a)
            | E::NotIn(a) => stack.push(a),
            E::HasSize { candidates, .. }
            | E::Latest { candidates, .. }
            | E::WithinReference { candidates, .. }
            | E::WithinVisibility { candidates, .. } => stack.push(candidates),
            E::Coalesce(a, b) | E::Union(a, b) | E::Intersection(a, b) | E::Difference(a, b) => {
                stack.extend([a.as_ref(), b.as_ref()])
            }
            _ => {}
        }
    }
    Ok(())
}
