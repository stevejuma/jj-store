//! Directory pages bind commit/path queries to signed, retained read snapshots.
use super::*;
use jj_store_jj::query::content::{ContentReader, TreePage, TreeRequest};
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub commit: m::CommitId,
    /// Empty means repository root; nonempty paths are canonical native paths.
    pub path: String,
    pub limit: usize,
}
impl Request {
    fn fingerprint(&self) -> Result<[u8; 32]> {
        if self.path.len() > 4096 || !(1..=10_000).contains(&self.limit) {
            return Err(Error::Limit);
        }
        cursors::fingerprint(b"directory\0", self)
    }
    fn native(self, after: Option<String>) -> TreeRequest {
        TreeRequest {
            commit: self.commit,
            path: self.path,
            limit: self.limit,
            after,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Page {
    pub snapshot: ReadSnapshot,
    /// Only next_cursor crosses the application boundary; tree.next is removed.
    pub tree: TreePage,
    pub next_cursor: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Start {
    SelectHead { snapshot: HeadSnapshot },
    Page(Box<Page>),
}
pub struct DirectoryPager {
    reader: ContentReader,
    cursors: Arc<SnapshotCursors>,
}
impl std::fmt::Debug for DirectoryPager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectoryPager")
            .field("cursors", &self.cursors)
            .finish_non_exhaustive()
    }
}
impl DirectoryPager {
    pub fn new(reader: ContentReader, limits: Limits, clock: Arc<dyn Clock>) -> Result<Self> {
        Ok(Self::with_cursors(
            reader,
            Arc::new(SnapshotCursors::new(limits, clock)?),
        ))
    }
    /// Share this registry with graph pages to enforce aggregate retention and
    /// request bounds across every repository served by the instance.
    pub fn with_cursors(reader: ContentReader, cursors: Arc<SnapshotCursors>) -> Self {
        Self { reader, cursors }
    }
    pub async fn begin(
        &self,
        session: Arc<NativeSession>,
        operation: Option<m::OperationId>,
        request: Request,
    ) -> Result<Start> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        within_timeout(self.cursors.limits.read.timeout, async {
            self.cursors.authorized(&session).await?;
            let fingerprint = request.fingerprint()?;
            let page_size = request.limit;
            let read = match session
                .open_read(operation, self.cursors.limits.read)
                .await?
            {
                OpenRead::SelectHead(snapshot) => return Ok(Start::SelectHead { snapshot }),
                OpenRead::Pinned(read) => Arc::new(read),
            };
            let mut tree = read.tree(&self.reader, request.native(None)).await?;
            let pin = read.pin().await?;
            let snapshot = pin.snapshot.clone();
            let next_cursor = tree
                .next
                .take()
                .map(|after| {
                    self.cursors
                        .retain(pin, fingerprint, page_size, Continuation::Directory(after))
                })
                .transpose()?;
            Ok(Start::Page(Box::new(Page {
                snapshot,
                tree,
                next_cursor,
            })))
        })
        .await
    }
    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        operation: Option<m::OperationId>,
        request: Request,
    ) -> Result<Page> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        within_timeout(self.cursors.limits.read.timeout, async {
            self.cursors.authorized(&session).await?;
            let (claims, entry) = self.cursors.lookup(
                session.repository,
                cursor,
                operation,
                request.fingerprint()?,
                request.limit,
            )?;
            let Continuation::Directory(after) = &claims.after else {
                return Err(Error::InvalidCursor);
            };
            let read = entry.pin.resume(session, self.cursors.limits.read).await?;
            let mut tree = read
                .tree(&self.reader, request.native(Some(after.clone())))
                .await?;
            let next_cursor = self.cursors.next(
                &claims,
                &entry,
                tree.next.take().map(Continuation::Directory),
            )?;
            Ok(Page {
                snapshot: entry.pin.snapshot,
                tree,
                next_cursor,
            })
        })
        .await
    }
}
