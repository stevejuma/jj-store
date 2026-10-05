//! Signed bidirectional windows for browser reference and recursive-path rows.
use super::*;
use jj_store_jj::query::content::{ContentReader, Direction, paths, references};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceRequest {
    pub query: String,
    pub exact: Option<String>,
    pub kind: references::Kind,
    pub limit: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathRequest {
    pub commit: m::CommitId,
    pub path: String,
    pub recursive: bool,
    pub include_last_change: bool,
    pub limit: usize,
}
#[derive(Clone, Debug, Serialize, schemars::JsonSchema)]
pub struct Window<T> {
    pub snapshot: ReadSnapshot,
    pub data: T,
    pub previous_cursor: Option<String>,
    pub next_cursor: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Start<T> {
    SelectHead { snapshot: HeadSnapshot },
    Window(Box<Window<T>>),
}
pub struct BrowserPager {
    reader: ContentReader,
    cursors: Arc<SnapshotCursors>,
}
struct OpenRequest<'a> {
    session: Arc<NativeSession>,
    operation: Option<m::OperationId>,
    cursor: Option<&'a str>,
    fingerprint: [u8; 32],
    limit: usize,
    anchor: Option<String>,
    direction: Direction,
    references: bool,
}
impl BrowserPager {
    pub fn with_cursors(reader: ContentReader, cursors: Arc<SnapshotCursors>) -> Self {
        Self { reader, cursors }
    }
    pub async fn references(
        &self,
        session: Arc<NativeSession>,
        operation: Option<m::OperationId>,
        request: ReferenceRequest,
        cursor: Option<&str>,
        anchor: Option<String>,
        direction: Direction,
    ) -> Result<Start<references::Page>> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        self.cursors.authorized(&session).await?;
        let fingerprint = request.fingerprint()?;
        let (read, position, retained) = self
            .open(OpenRequest {
                session,
                operation,
                cursor,
                fingerprint,
                limit: request.limit,
                anchor,
                direction,
                references: true,
            })
            .await?;
        let Some(read) = read else {
            return Ok(Start::SelectHead {
                snapshot: retained.unwrap(),
            });
        };
        let data = match read
            .references(
                &self.reader,
                references::Request {
                    operation: read.snapshot().await?.operation_id,
                    query: request.query,
                    exact: request.exact,
                    kind: request.kind,
                    limit: request.limit,
                    anchor: position.as_ref().map(|p| p.0.clone()),
                    direction: position.map_or(direction, |p| p.1),
                },
            )
            .await
        {
            Ok(page) => page,
            Err(query::Error::IndexRebuildRequired) => {
                return Err(Error::IndexRebuildRequired {
                    snapshot: Box::new(read.snapshot().await?.clone()),
                });
            }
            Err(error) => return Err(error.into()),
        };
        Ok(Start::Window(Box::new(
            self.finish(read, fingerprint, request.limit, data, true)
                .await?,
        )))
    }
    pub async fn paths(
        &self,
        session: Arc<NativeSession>,
        operation: Option<m::OperationId>,
        request: PathRequest,
        cursor: Option<&str>,
        anchor: Option<String>,
        direction: Direction,
    ) -> Result<Start<paths::Page>> {
        let _permit = self
            .cursors
            .permits
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        self.cursors.authorized(&session).await?;
        let fingerprint = request.fingerprint()?;
        let (read, position, retained) = self
            .open(OpenRequest {
                session,
                operation,
                cursor,
                fingerprint,
                limit: request.limit,
                anchor,
                direction,
                references: false,
            })
            .await?;
        let Some(read) = read else {
            return Ok(Start::SelectHead {
                snapshot: retained.unwrap(),
            });
        };
        let data = read
            .paths(
                &self.reader,
                paths::Request {
                    include_last_change: request.include_last_change,
                    commit: request.commit,
                    path: request.path,
                    recursive: request.recursive,
                    limit: request.limit,
                    anchor: position.as_ref().map(|p| p.0.clone()),
                    direction: position.map_or(direction, |p| p.1),
                },
            )
            .await?;
        Ok(Start::Window(Box::new(
            self.finish(read, fingerprint, request.limit, data, false)
                .await?,
        )))
    }
    async fn open(
        &self,
        request: OpenRequest<'_>,
    ) -> Result<(
        Option<Arc<reads::CommittedRead>>,
        Option<(String, Direction)>,
        Option<HeadSnapshot>,
    )> {
        if let Some(cursor) = request.cursor {
            let (claims, entry) = self.cursors.lookup(
                request.session.repository,
                cursor,
                request.operation,
                request.fingerprint,
                request.limit,
            )?;
            let position = match (&claims.after, request.references) {
                (Continuation::References { anchor, direction }, true)
                | (Continuation::Paths { anchor, direction }, false) => {
                    (anchor.clone(), *direction)
                }
                _ => return Err(Error::InvalidCursor),
            };
            let read = entry
                .pin
                .resume(request.session, self.cursors.limits.read)
                .await?;
            Ok((Some(read), Some(position), None))
        } else {
            match request
                .session
                .open_read(request.operation, self.cursors.limits.read)
                .await?
            {
                OpenRead::SelectHead(snapshot) => Ok((None, None, Some(snapshot))),
                OpenRead::Pinned(read) => Ok((
                    Some(Arc::new(read)),
                    request.anchor.map(|anchor| (anchor, request.direction)),
                    None,
                )),
            }
        }
    }
    async fn finish<T: Bounds>(
        &self,
        read: Arc<reads::CommittedRead>,
        fingerprint: [u8; 32],
        limit: usize,
        mut data: T,
        references: bool,
    ) -> Result<Window<T>> {
        let pin = read.pin().await?;
        let snapshot = pin.snapshot.clone();
        let previous_cursor = data
            .previous()
            .map(|anchor| {
                self.cursors.retain(
                    pin.clone(),
                    fingerprint,
                    limit,
                    continuation(anchor, Direction::Backward, references),
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
                    continuation(anchor, Direction::Forward, references),
                )
            })
            .transpose()?;
        data.clear_continuations();
        Ok(Window {
            snapshot,
            data,
            previous_cursor,
            next_cursor,
        })
    }
}
fn continuation(anchor: String, direction: Direction, references: bool) -> Continuation {
    if references {
        Continuation::References { anchor, direction }
    } else {
        Continuation::Paths { anchor, direction }
    }
}
trait Bounds {
    fn previous(&self) -> Option<String>;
    fn next(&self) -> Option<String>;
    fn clear_continuations(&mut self);
}
impl Bounds for references::Page {
    fn previous(&self) -> Option<String> {
        self.previous.clone()
    }
    fn next(&self) -> Option<String> {
        self.next.clone()
    }
    fn clear_continuations(&mut self) {
        self.previous = None;
        self.next = None;
    }
}
impl Bounds for paths::Page {
    fn previous(&self) -> Option<String> {
        self.previous.clone()
    }
    fn next(&self) -> Option<String> {
        self.next.clone()
    }
    fn clear_continuations(&mut self) {
        self.previous = None;
        self.next = None;
    }
}
impl ReferenceRequest {
    fn fingerprint(&self) -> Result<[u8; 32]> {
        validate(self.limit, &self.query)?;
        cursors::fingerprint(b"references\0", self)
    }
}
impl PathRequest {
    fn fingerprint(&self) -> Result<[u8; 32]> {
        validate(self.limit, &self.path)?;
        cursors::fingerprint(b"paths\0", self)
    }
}
fn validate(limit: usize, value: &str) -> Result<()> {
    if !(1..=10_000).contains(&limit) || value.len() > 4096 {
        Err(Error::Limit)
    } else {
        Ok(())
    }
}
