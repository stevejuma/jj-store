//! Standard JJ graph only. No custom visibility, operation or filter projection.
pub(crate) mod merge_files;
mod native;
pub(crate) mod path_history;
mod path_index;
use super::{Error, Result, memory};
use jj_lib::{
    default_index::{DefaultIndexStore, DefaultReadonlyIndex},
    index::{IndexStore, IndexStoreError, IndexStoreResult, MutableIndex, ReadonlyIndex},
    operation::Operation,
    store::Store,
};
pub use native::Prepared;
use std::{
    fs,
    io::{Read as _, Write as _},
    path::Path,
    sync::Arc,
};

/// Owned evictable cache retention registered with the existing process memory pool.
pub struct Retained<T>(Arc<memory::Retained<T>>);
impl<T: Send + Sync + 'static> Retained<T> {
    pub fn new(value: Arc<T>) -> Self {
        Self(memory::Retained::new(value))
    }
    pub fn get(&self) -> Option<Arc<T>> {
        self.0.get()
    }
    pub fn present(&self) -> bool {
        self.0.present()
    }
}

pub(crate) struct Index {
    pub raw: DefaultReadonlyIndex,
    pub history: Arc<path_history::History>,
    operation: jj_lib::op_store::OperationId,
    _memory: Arc<memory::Lease>,
}
impl std::fmt::Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ManagedCodeIndex")
    }
}
impl Index {
    pub async fn load(
        path: &Path,
        operation: &Operation,
        store: &Arc<Store>,
        max_commits: usize,
        mut check: impl FnMut() -> Result<()>,
    ) -> Result<Self> {
        let weight = memory::framing::weight(
            path,
            store.commit_id_length(),
            store.change_id_length(),
            max_commits,
            &mut check,
        )?;
        let memory = Arc::new(memory::Lease::reserve(weight)?);
        check()?;
        let index = DefaultIndexStore::init(path)
            .map_err(|_| Error::Unavailable)?
            .get_index_at_op(operation, store)
            .await
            .map_err(|error| {
                let mut source: &(dyn std::error::Error + 'static) = &error;
                loop {
                    if let Some(error) = source.downcast_ref::<Error>() {
                        break error.clone();
                    }
                    match source.source() {
                        Some(next) => source = next,
                        None => break Error::Unavailable,
                    }
                }
            })?;
        let raw = index
            .downcast_ref::<DefaultReadonlyIndex>()
            .ok_or(Error::Unavailable)?
            .clone();
        if raw.num_commits() as usize > max_commits {
            return Err(Error::IndexBudget);
        }
        check()?;
        // Construction can add segments. Admit their resident allocations too.
        let final_weight = memory::framing::weight(
            path,
            store.commit_id_length(),
            store.change_id_length(),
            max_commits,
            &mut check,
        )?;
        let memory = if final_weight > weight {
            drop(memory);
            Arc::new(memory::Lease::reserve(final_weight)?)
        } else {
            memory
        };
        Ok(Self {
            raw,
            history: Arc::default(),
            operation: operation.id().clone(),
            _memory: memory,
        })
    }
}
#[async_trait::async_trait(?Send)]
impl IndexStore for Index {
    fn name(&self) -> &str {
        "default"
    }
    async fn get_index_at_op(
        &self,
        operation: &Operation,
        _: &Arc<Store>,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        if operation.id() != &self.operation {
            return Err(IndexStoreError::Read(Box::new(Error::NotCommitted)));
        }
        Ok(Box::new(self.raw.clone()))
    }
    fn write_index(
        &self,
        _: Box<dyn MutableIndex>,
        _: &Operation,
    ) -> IndexStoreResult<Box<dyn ReadonlyIndex>> {
        Err(IndexStoreError::Write(Box::new(Error::Denied)))
    }
}
fn hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
/// Copy the nearest indexed ancestor association and only its segment chain.
/// Avoid enumerating every source operation or unrelated index generation.
pub(crate) async fn seed(
    source: &Path,
    target: &Path,
    selected: &Operation,
    max_bytes: u64,
    mut charge: impl FnMut(usize) -> Result<()>,
) -> Result<()> {
    use futures::StreamExt as _;
    use jj_lib::object_id::ObjectId as _;
    if !source
        .join("op_links")
        .try_exists()
        .map_err(|_| Error::Unavailable)?
    {
        return Ok(());
    }
    for name in ["op_links", "segments"] {
        if !fs::symlink_metadata(source.join(name))
            .map_err(|_| Error::Unavailable)?
            .is_dir()
        {
            return Err(Error::Corrupt);
        }
        fs::create_dir_all(target.join(name)).map_err(|_| Error::Unavailable)?;
    }
    let mut walk = std::pin::pin!(jj_lib::op_walk::walk_ancestors(std::slice::from_ref(
        selected
    )));
    let mut count = 0;
    let (operation, link) = loop {
        charge(0)?;
        count += 1;
        if count > 4096 {
            return Err(Error::Limit);
        }
        let Some(operation) = walk.next().await else {
            return Ok(());
        };
        let operation = operation.map_err(|_| Error::Unavailable)?;
        let path = source.join("op_links").join(operation.id().hex());
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => return Err(Error::Unavailable),
        };
        if !metadata.is_file() || metadata.len() > 16 * 1024 {
            return Err(Error::Corrupt);
        }
        let mut link = vec![];
        fs::File::open(path)
            .map_err(|_| Error::Unavailable)?
            .take(16 * 1024 + 1)
            .read_to_end(&mut link)
            .map_err(|_| Error::Unavailable)?;
        charge(link.len())?;
        if link.len() > 16 * 1024 {
            return Err(Error::Corrupt);
        }
        break (operation.id().hex(), link);
    };
    use prost::Message as _;
    let control = SegmentControl::decode(link.as_slice()).map_err(|_| Error::Corrupt)?;
    if control.commit_segment_id.len() != 64
        || control.changed_path_segment_ids.len() > 128
        || control
            .changed_path_segment_ids
            .iter()
            .any(|id| id.len() != 64)
        || control.changed_path_start_commit_pos.is_none()
            != control.changed_path_segment_ids.is_empty()
    {
        return Err(Error::Corrupt);
    }
    let mut next = Some(bytes_hex(&control.commit_segment_id));
    let mut seen = std::collections::BTreeSet::new();
    let mut total = link.len() as u64;
    while let Some(name) = next {
        charge(0)?;
        if !hex(&name, 128) || !seen.insert(name.clone()) {
            return Err(Error::Corrupt);
        }
        if seen.len() > 128 {
            return Err(Error::Limit);
        }
        let path = source.join("segments").join(&name);
        let metadata = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
        if !metadata.is_file() {
            return Err(Error::Corrupt);
        }
        total = total
            .checked_add(metadata.len())
            .filter(|n| *n <= max_bytes)
            .ok_or(Error::Limit)?;
        let parent = copy_segment(
            &path,
            &target.join("segments").join(name),
            metadata.len(),
            &mut charge,
        )?;
        next = parent;
    }
    if !control.changed_path_segment_ids.is_empty() {
        fs::create_dir_all(target.join("changed_paths")).map_err(|_| Error::Unavailable)?;
        for id in control.changed_path_segment_ids {
            charge(0)?;
            let name = bytes_hex(&id);
            let path = source.join("changed_paths").join(&name);
            let metadata = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
            if !metadata.is_file() {
                return Err(Error::Corrupt);
            }
            total = total
                .checked_add(metadata.len())
                .filter(|n| *n <= max_bytes)
                .ok_or(Error::Limit)?;
            copy_segment(
                &path,
                &target.join("changed_paths").join(name),
                metadata.len(),
                &mut charge,
            )?;
        }
    }
    fs::write(target.join("op_links").join(operation), link).map_err(|_| Error::Unavailable)?;
    Ok(())
}
fn copy_segment(
    source: &Path,
    target: &Path,
    size: u64,
    charge: &mut impl FnMut(usize) -> Result<()>,
) -> Result<Option<String>> {
    use blake2::{Blake2b512, Digest as _};
    let mut input = fs::File::open(source).map_err(|_| Error::Unavailable)?;
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)
        .map_err(|_| Error::Unavailable)?;
    let mut hash = Blake2b512::new();
    let mut header = Vec::new();
    let mut read = 0u64;
    let mut buffer = [0; 64 * 1024];
    loop {
        charge(0)?;
        let count = input.read(&mut buffer).map_err(|_| Error::Unavailable)?;
        if count == 0 {
            break;
        }
        read += count as u64;
        if read > size {
            return Err(Error::Corrupt);
        }
        charge(count)?;
        hash.update(&buffer[..count]);
        if header.len() < 136 {
            header.extend_from_slice(&buffer[..count.min(136 - header.len())]);
        }
        output
            .write_all(&buffer[..count])
            .map_err(|_| Error::Unavailable)?;
    }
    if read != size
        || target.file_name().and_then(|n| n.to_str()) != Some(&format!("{:x}", hash.finalize()))
    {
        return Err(Error::Corrupt);
    }
    if header.len() >= 20 && header[..4] == 0u32.to_le_bytes() {
        return Ok(None);
    }
    if header.len() < 8 || header[..4] != 6u32.to_le_bytes() {
        return Err(Error::Corrupt);
    }
    match u32::from_le_bytes(header[4..8].try_into().unwrap()) {
        0 => Ok(None),
        128 if header.len() == 136 => {
            let parent = std::str::from_utf8(&header[8..]).map_err(|_| Error::Corrupt)?;
            if !hex(parent, 128) {
                return Err(Error::Corrupt);
            }
            Ok(Some(parent.to_owned()))
        }
        _ => Err(Error::Corrupt),
    }
}

// JJ 0.45.1 default_index.proto (Apache-2.0); read-only source-copy framing.
#[derive(prost::Message)]
struct SegmentControl {
    #[prost(bytes = "vec", tag = "1")]
    commit_segment_id: Vec<u8>,
    #[prost(uint32, optional, tag = "2")]
    changed_path_start_commit_pos: Option<u32>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    changed_path_segment_ids: Vec<Vec<u8>>,
}
fn bytes_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
