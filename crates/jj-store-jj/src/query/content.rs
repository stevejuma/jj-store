//! Complete native metadata and verified binary reads through committed pins.
pub mod conflicts;
pub mod diff;
mod diff_copies;
mod native;
pub mod operations;
mod trees;
use super::{
    Error, ReadAccess, Result, Stats,
    access::{Budget, ReadStore},
};
use crate::{
    codec::{NativeCodec as _, NativeSymlink},
    native_dto as dto,
    objects::{self, NativeObjects},
    roots,
};
use jj_lib::{backend as b, op_store as o};
use jj_store_model as m;
use jj_store_object::{ByteRange, ObjectStore, StagedUpload};
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tokio::{
    io::AsyncReadExt as _,
    sync::{OwnedSemaphorePermit, Semaphore},
};

pub use trees::{TreeEntry, TreePage, TreeRequest};
const CHUNK: usize = 64 * 1024;
#[derive(Clone, Copy)]
pub struct Limits {
    pub metadata: super::admission::Limits,
    pub max_blob_bytes: u64,
    pub max_read_bytes: u64,
    pub max_output_bytes: usize,
    pub max_work: usize,
    pub max_history_visits: usize,
    pub max_changes: usize,
    pub max_text_bytes: usize,
    pub max_diff_cells: usize,
    pub max_page: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            metadata: super::admission::Limits::default(),
            max_blob_bytes: 256 * 1024 * 1024,
            max_read_bytes: 512 * 1024 * 1024,
            max_output_bytes: 16 * 1024 * 1024,
            max_work: 1_000_000,
            max_history_visits: 50_000,
            max_changes: 1_000,
            max_text_bytes: 256 * 1024,
            max_diff_cells: 250_000,
            max_page: 500,
            timeout: Duration::from_secs(30),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum ObjectData {
    Commit(dto::Commit),
    Tree(dto::Tree),
    Copy(dto::CopyHistory),
    View(dto::View),
    Operation(dto::Operation),
    Symlink { target: String },
    File { bytes: u64 },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Object {
    pub version: u32,
    pub profile: &'static str,
    pub key: m::ObjectKey,
    pub virtual_object: bool,
    pub data: ObjectData,
    pub stats: Stats,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FileInfo {
    pub version: u32,
    pub id: m::FileId,
    pub full_length: u64,
    pub start: u64,
    pub end: u64,
}
/// No raw reader, spool, or local path escapes this authorization boundary.
pub struct VerifiedFile {
    info: FileInfo,
    reader: jj_store_object::Reader,
    _spool: StagedUpload,
    _permit: OwnedSemaphorePermit,
    access: Arc<dyn ReadAccess>,
    budget: Arc<Budget>,
    remaining: u64,
    chunk_in_progress: bool,
}
impl std::fmt::Debug for VerifiedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifiedFile").finish_non_exhaustive()
    }
}
impl VerifiedFile {
    pub async fn info(&self) -> Result<FileInfo> {
        self.budget.authorize(self.access.as_ref()).await?;
        Ok(self.info.clone())
    }
    pub async fn next_chunk(&mut self) -> Result<Option<Vec<u8>>> {
        // A canceled call can have consumed local bytes before its final
        // authorization. Fail closed on reuse instead of returning a shifted
        // stream; callers can reopen an explicitly verified range.
        if self.chunk_in_progress {
            return Err(self.budget.fail(Error::Unavailable));
        }
        self.chunk_in_progress = true;
        self.budget.authorize(self.access.as_ref()).await?;
        if self.remaining == 0 {
            self.chunk_in_progress = false;
            return Ok(None);
        }
        let mut bytes = vec![0; self.remaining.min(CHUNK as u64) as usize];
        tokio::time::timeout_at(self.budget.deadline, self.reader.read_exact(&mut bytes))
            .await
            .map_err(|_| self.budget.fail(Error::Limit))?
            .map_err(|_| self.budget.fail(Error::Unavailable))?;
        self.budget.authorize(self.access.as_ref()).await?;
        self.remaining -= bytes.len() as u64;
        self.chunk_in_progress = false;
        Ok(Some(bytes))
    }
}
#[derive(Clone)]
pub struct ContentReader {
    limits: Limits,
    permits: Arc<Semaphore>,
}
struct Context {
    limits: Limits,
    budget: Arc<Budget>,
    access: Arc<dyn ReadAccess>,
    objects: Arc<NativeObjects>,
    permit: Option<OwnedSemaphorePermit>,
}
impl ContentReader {
    pub fn new(limits: Limits, concurrency: usize) -> Result<Self> {
        limits.metadata.validate()?;
        if !(1..=64).contains(&concurrency)
            || limits.max_read_bytes == 0
            || limits.max_blob_bytes
                > jj_store_object::staging::MAX_SINGLE_PUT - m::envelope::HEADER_LEN as u64
            || limits.max_output_bytes == 0
            || limits.max_work == 0
            || !(1..=1_000_000).contains(&limits.max_history_visits)
            || !(1..=10_000).contains(&limits.max_changes)
            || !(1..=1024 * 1024).contains(&limits.max_text_bytes)
            || !(1..=1_000_000).contains(&limits.max_diff_cells)
            || !(1..=10_000).contains(&limits.max_page)
            || limits.timeout.is_zero()
            || limits.timeout > Duration::from_secs(300)
        {
            return Err(Error::Limit);
        }
        Ok(Self {
            limits,
            permits: Arc::new(Semaphore::new(concurrency)),
        })
    }
    async fn execute<T: Send + 'static>(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        run: impl FnOnce(Context) -> futures::future::LocalBoxFuture<'static, Result<T>>
        + Send
        + 'static,
    ) -> Result<T> {
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Busy)?;
        let limits = self.limits;
        let budget = Arc::new(Budget::new(super::Limits {
            metadata: limits.metadata,
            max_object_bytes: limits.max_read_bytes,
            max_output_bytes: limits.max_output_bytes,
            max_work: limits.max_work,
            timeout: limits.timeout,
            ..super::Limits::default()
        })?);
        budget.authorize(access.as_ref()).await?;
        let cancellation = budget.cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let objects = Arc::new(
                    NativeObjects::new(
                        Arc::new(ReadStore::new(store, access.clone(), budget.clone())),
                        access.repository(),
                        limits.max_blob_bytes,
                    )
                    .map_err(map_object)?,
                );
                let context = Context {
                    limits,
                    budget: budget.clone(),
                    access: access.clone(),
                    objects,
                    permit: Some(permit),
                };
                let result = budget.until(run(context)).await?;
                budget.authorize(access.as_ref()).await?;
                result.map_err(|error| budget.failure().unwrap_or(error))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancellation.disarm();
        result
    }
    pub async fn object(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        key: m::ObjectKey,
    ) -> Result<Object> {
        self.execute(store, access, move |context| {
            Box::pin(async move { context.object(key).await })
        })
        .await
    }
    pub async fn file(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        id: m::FileId,
        range: Option<ByteRange>,
    ) -> Result<VerifiedFile> {
        self.execute(store, access, move |context| {
            Box::pin(async move { context.file(id, range).await })
        })
        .await
    }
    pub async fn diff(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: diff::Request,
    ) -> Result<diff::Response> {
        self.execute(store, access, move |context| {
            Box::pin(async move { diff::read(context, request).await })
        })
        .await
    }
    pub async fn conflicts(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: conflicts::Request,
    ) -> Result<conflicts::Response> {
        self.execute(store, access, move |context| {
            Box::pin(async move { conflicts::read(context, request).await })
        })
        .await
    }
    pub async fn operation_diff(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: operations::Request,
    ) -> Result<operations::Response> {
        self.execute(store, access, move |context| {
            Box::pin(async move { operations::read(context, request).await })
        })
        .await
    }
    pub async fn tree(
        &self,
        store: Arc<dyn ObjectStore>,
        access: Arc<dyn ReadAccess>,
        request: TreeRequest,
    ) -> Result<TreePage> {
        self.execute(store, access, move |context| {
            Box::pin(async move { trees::read(context, request).await })
        })
        .await
    }
}
impl Context {
    async fn member(&self, key: m::ObjectKey) -> Result<()> {
        self.budget.authorize(self.access.as_ref()).await?;
        if !self.access.inventory().contains(&key) {
            return Err(Error::NotCommitted);
        }
        Ok(())
    }
    async fn object(&self, key: m::ObjectKey) -> Result<Object> {
        self.member(key).await?;
        let mut virtual_object = false;
        macro_rules! metadata {
            ($native:ty, $dto:ty, $variant:ident, $root:expr) => {{
                let native = match $root {
                    Some(value) => {
                        virtual_object = true;
                        value
                    }
                    None => self.objects.get::<$native>(key).await.map_err(map_object)?,
                };
                ObjectData::$variant(<$dto>::try_from(&native).map_err(|_| Error::Corrupt)?)
            }};
        }
        let data = match key {
            m::ObjectKey::Commit(id) => metadata!(
                b::Commit,
                dto::Commit,
                Commit,
                (id == roots::ROOT_COMMIT).then(roots::commit)
            ),
            m::ObjectKey::Tree(_) => metadata!(
                b::Tree,
                dto::Tree,
                Tree,
                (key == b::Tree::default().key()).then(b::Tree::default)
            ),
            m::ObjectKey::Copy(_) => metadata!(b::CopyHistory, dto::CopyHistory, Copy, None),
            m::ObjectKey::View(id) => metadata!(
                o::View,
                dto::View,
                View,
                (id == roots::ROOT_VIEW).then(roots::view)
            ),
            m::ObjectKey::Operation(id) => metadata!(
                o::Operation,
                dto::Operation,
                Operation,
                (id == roots::ROOT_OPERATION).then(roots::operation)
            ),
            m::ObjectKey::Symlink(_) => ObjectData::Symlink {
                target: self
                    .objects
                    .get::<NativeSymlink>(key)
                    .await
                    .map_err(map_object)?
                    .0,
            },
            m::ObjectKey::File(id) => ObjectData::File {
                bytes: self.objects.get_file(id).await.map_err(map_object)?.len(),
            },
        };
        let result = Object {
            version: 1,
            profile: crate::factories::PROFILE,
            key,
            virtual_object,
            data,
            stats: self.budget.stats(),
        };
        self.budget.complete_output(&result)?;
        Ok(result)
    }
    async fn file(mut self, id: m::FileId, range: Option<ByteRange>) -> Result<VerifiedFile> {
        self.member(m::ObjectKey::File(id)).await?;
        let spool = self.objects.get_file(id).await.map_err(map_object)?;
        let (start, end, reader) = if let Some(range) = range {
            if range.end() > spool.len() {
                return Err(Error::Range);
            }
            (
                range.start(),
                range.end(),
                spool
                    .reader_range(range)
                    .await
                    .map_err(|_| Error::Unavailable)?,
            )
        } else {
            (
                0,
                spool.len(),
                spool.reader().map_err(|_| Error::Unavailable)?,
            )
        };
        Ok(VerifiedFile {
            info: FileInfo {
                version: 1,
                id,
                full_length: spool.len(),
                start,
                end,
            },
            reader,
            _spool: spool,
            _permit: self.permit.take().ok_or(Error::Unavailable)?,
            access: self.access,
            budget: self.budget,
            remaining: end - start,
            chunk_in_progress: false,
        })
    }
}
fn map_object(error: objects::Error) -> Error {
    use jj_store_object::Error as S;
    match error {
        objects::Error::Store(S::NotFound) => Error::Incomplete,
        objects::Error::Store(S::Limit) => Error::Limit,
        objects::Error::Envelope(m::envelope::EnvelopeError::Limit)
        | objects::Error::Codec(crate::codec::CodecError::Envelope(
            m::envelope::EnvelopeError::Limit,
        )) => Error::Limit,
        objects::Error::Io(ref error) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
            Error::Corrupt
        }
        objects::Error::Store(S::Response(_))
        | objects::Error::Codec(_)
        | objects::Error::Envelope(_)
        | objects::Error::Collision
        | objects::Error::Integrity => Error::Corrupt,
        _ => Error::Unavailable,
    }
}
