//! Signed windows for operation groups and exact comparison path summaries.
use super::{browsing, *};
use jj_store_jj::query::content::{ContentReader, Direction, diff_summary, operation_changes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupRequest {
    pub operation: m::OperationId,
    pub parent: m::OperationId,
    pub limit: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffPathRequest {
    pub from: m::CommitId,
    pub to: m::CommitId,
    pub path: String,
    pub limit: usize,
}
pub struct OperationBrowserPager {
    reader: ContentReader,
    cursors: Arc<SnapshotCursors>,
}
impl OperationBrowserPager {
    pub fn with_cursors(reader: ContentReader, cursors: Arc<SnapshotCursors>) -> Self {
        Self { reader, cursors }
    }
    pub async fn groups(
        &self,
        session: Arc<NativeSession>,
        request: GroupRequest,
        cursor: Option<&str>,
        anchor: Option<String>,
        direction: Direction,
    ) -> Result<browsing::Start<operation_changes::Page>> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        self.cursors.authorized(&session).await?;
        let fingerprint = request.fingerprint()?;
        let (read, position) = self
            .open(
                session,
                request.operation,
                cursor,
                fingerprint,
                request.limit,
                anchor,
                direction,
                true,
            )
            .await?;
        let data = read
            .operation_changes(
                &self.reader,
                operation_changes::Request {
                    operation: request.operation,
                    parent: request.parent,
                    limit: request.limit,
                    anchor: position.as_ref().map(|p| p.0.clone()),
                    direction: position.map_or(direction, |p| p.1),
                },
            )
            .await?;
        Ok(browsing::Start::Window(Box::new(
            self.finish(read, fingerprint, request.limit, data, true)
                .await?,
        )))
    }
    pub async fn paths(
        &self,
        session: Arc<NativeSession>,
        operation: m::OperationId,
        request: DiffPathRequest,
        cursor: Option<&str>,
        anchor: Option<String>,
        direction: Direction,
    ) -> Result<browsing::Start<diff_summary::Page>> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        self.cursors.authorized(&session).await?;
        let fingerprint = request.fingerprint()?;
        let (read, position) = self
            .open(
                session,
                operation,
                cursor,
                fingerprint,
                request.limit,
                anchor,
                direction,
                false,
            )
            .await?;
        let data = read
            .diff_summaries(
                &self.reader,
                diff_summary::Request {
                    from: request.from,
                    to: request.to,
                    path: request.path,
                    limit: request.limit,
                    anchor: position.as_ref().map(|p| p.0.clone()),
                    direction: position.map_or(direction, |p| p.1),
                },
            )
            .await?;
        Ok(browsing::Start::Window(Box::new(
            self.finish(read, fingerprint, request.limit, data, false)
                .await?,
        )))
    }
    #[allow(clippy::too_many_arguments)]
    async fn open(
        &self,
        session: Arc<NativeSession>,
        operation: m::OperationId,
        cursor: Option<&str>,
        fingerprint: [u8; 32],
        limit: usize,
        anchor: Option<String>,
        direction: Direction,
        groups: bool,
    ) -> Result<(Arc<reads::CommittedRead>, Option<(String, Direction)>)> {
        if let Some(cursor) = cursor {
            let (claims, entry) = self.cursors.lookup(
                session.repository,
                cursor,
                Some(operation),
                fingerprint,
                limit,
            )?;
            let position = match (&claims.after, groups) {
                (Continuation::OperationGroups { anchor, direction }, true)
                | (Continuation::DiffPaths { anchor, direction }, false) => {
                    (anchor.clone(), *direction)
                }
                _ => return Err(Error::InvalidCursor),
            };
            Ok((
                entry.pin.resume(session, self.cursors.limits.read).await?,
                Some(position),
            ))
        } else {
            match session
                .open_read(Some(operation), self.cursors.limits.read)
                .await?
            {
                OpenRead::Pinned(read) => Ok((Arc::new(read), anchor.map(|a| (a, direction)))),
                OpenRead::SelectHead(_) => Err(Error::Unavailable),
            }
        }
    }
    async fn finish<T: Bounds>(
        &self,
        read: Arc<reads::CommittedRead>,
        fingerprint: [u8; 32],
        limit: usize,
        mut data: T,
        groups: bool,
    ) -> Result<browsing::Window<T>> {
        let pin = read.pin().await?;
        let snapshot = pin.snapshot.clone();
        let previous_cursor = data
            .previous()
            .map(|anchor| {
                self.cursors.retain(
                    pin.clone(),
                    fingerprint,
                    limit,
                    continuation(anchor, Direction::Backward, groups),
                )
            })
            .transpose()?;
        let next_cursor = data
            .next()
            .map(|anchor| {
                self.cursors.retain(
                    pin.clone(),
                    fingerprint,
                    limit,
                    continuation(anchor, Direction::Forward, groups),
                )
            })
            .transpose()?;
        data.clear();
        Ok(browsing::Window {
            snapshot,
            data,
            previous_cursor,
            next_cursor,
        })
    }
}
fn continuation(anchor: String, direction: Direction, groups: bool) -> Continuation {
    if groups {
        Continuation::OperationGroups { anchor, direction }
    } else {
        Continuation::DiffPaths { anchor, direction }
    }
}
trait Bounds {
    fn previous(&self) -> Option<String>;
    fn next(&self) -> Option<String>;
    fn clear(&mut self);
}
impl Bounds for operation_changes::Page {
    fn previous(&self) -> Option<String> {
        self.previous.clone()
    }
    fn next(&self) -> Option<String> {
        self.next.clone()
    }
    fn clear(&mut self) {
        self.previous = None;
        self.next = None;
    }
}
impl Bounds for diff_summary::Page {
    fn previous(&self) -> Option<String> {
        self.previous.clone()
    }
    fn next(&self) -> Option<String> {
        self.next.clone()
    }
    fn clear(&mut self) {
        self.previous = None;
        self.next = None;
    }
}
impl GroupRequest {
    fn fingerprint(&self) -> Result<[u8; 32]> {
        validate(self.limit, "")?;
        cursors::fingerprint(b"operation-groups\0", self)
    }
}
impl DiffPathRequest {
    fn fingerprint(&self) -> Result<[u8; 32]> {
        validate(self.limit, &self.path)?;
        cursors::fingerprint(b"diff-paths\0", self)
    }
}
fn validate(limit: usize, path: &str) -> Result<()> {
    if !(1..=10_000).contains(&limit) || path.len() > 4096 {
        Err(Error::Limit)
    } else {
        Ok(())
    }
}
