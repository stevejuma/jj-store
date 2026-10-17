//! Request-local synthetic merge blobs. Never store or publish derived content.
use super::*;
use futures::{AsyncRead, AsyncReadExt};
use jj_lib::backend::FileId;
use std::{
    collections::BTreeMap,
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
};

pub(crate) enum Hash {
    Git,
    #[cfg(feature = "native")]
    Native,
}
struct File {
    bytes: Vec<u8>,
    _memory: memory::Lease,
}
struct Held {
    bytes: usize,
    files: BTreeMap<FileId, Arc<File>>,
}
pub(crate) struct Files {
    hash: Hash,
    max_file: usize,
    max_total: usize,
    charge: Box<dyn Fn(usize) -> Result<()> + Send + Sync>,
    held: Mutex<Held>,
}
impl Files {
    pub fn new(
        hash: Hash,
        max_file: usize,
        max_total: usize,
        charge: impl Fn(usize) -> Result<()> + Send + Sync + 'static,
    ) -> Arc<Self> {
        Arc::new(Self {
            hash,
            max_file,
            max_total,
            charge: Box::new(charge),
            held: Mutex::new(Held {
                bytes: 0,
                files: BTreeMap::new(),
            }),
        })
    }
    pub async fn write(&self, input: &mut (dyn AsyncRead + Send + Unpin)) -> Result<FileId> {
        (self.charge)(0)?;
        let _scratch = memory::Lease::reserve((self.max_file + 1) as u64)?;
        // Reserve before allocating, including the byte used to detect overflow.
        // Fixed capacity avoids read_to_end's geometric growth exceeding admission.
        let mut bytes = Vec::with_capacity(self.max_file + 1);
        input
            .take(self.max_file as u64 + 1)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| Error::Unavailable)?;
        if bytes.len() > self.max_file {
            return Err(Error::Limit);
        }
        (self.charge)(bytes.len())?;
        bytes.shrink_to_fit();
        let id = match self.hash {
            Hash::Git => FileId::from_bytes(
                gix::objs::compute_hash(gix::hash::Kind::Sha1, gix::objs::Kind::Blob, &bytes)
                    .map_err(|_| Error::Corrupt)?
                    .as_bytes(),
            ),
            #[cfg(feature = "native")]
            Hash::Native => {
                use blake2::{Blake2b512, Digest};
                FileId::from_bytes(&Blake2b512::digest(&bytes))
            }
        };
        let mut held = self.held.lock().unwrap();
        if held.files.contains_key(&id) {
            return Ok(id);
        }
        let total = held
            .bytes
            .checked_add(bytes.len())
            .filter(|n| *n <= self.max_total)
            .ok_or(Error::Limit)?;
        if held.files.len() >= 4096 {
            return Err(Error::Limit);
        }
        let lease = memory::Lease::reserve((bytes.capacity() + 256) as u64)?;
        held.files.insert(
            id.clone(),
            Arc::new(File {
                bytes,
                _memory: lease,
            }),
        );
        held.bytes = total;
        Ok(id)
    }
    pub fn read(&self, id: &FileId) -> Option<Pin<Box<dyn AsyncRead + Send>>> {
        self.held
            .lock()
            .unwrap()
            .files
            .get(id)
            .cloned()
            .map(|file| Box::pin(Reader { file, position: 0 }) as _)
    }
}
struct Reader {
    file: Arc<File>,
    position: usize,
}
impl AsyncRead for Reader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        output: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let n = output.len().min(self.file.bytes.len() - self.position);
        output[..n].copy_from_slice(&self.file.bytes[self.position..self.position + n]);
        self.position += n;
        Poll::Ready(Ok(n))
    }
}
