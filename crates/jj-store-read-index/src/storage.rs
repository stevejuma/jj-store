use crate::{Error, Index, MAX_BYTES, Result, Row, SCHEMA};
use redb::{ReadableDatabase, TableDefinition};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

const DATA: TableDefinition<&str, &[u8]> = TableDefinition::new("derived-index-v1");
const DISK_BYTES: u64 = 2 * 1024 * 1024 * 1024;
const FILE_BYTES: u64 = 128 * 1024 * 1024;
const FILES: usize = 64;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Key {
    /// Stable server-owned source namespace; never a credential or endpoint URL.
    pub namespace: String,
    pub repository: String,
    pub epoch: String,
    pub root: String,
    pub projection: String,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub restored: u64,
    pub built: u64,
    pub failed: u64,
}
pub struct Store {
    path: PathBuf,
    writer: Mutex<()>,
    previous: Mutex<BTreeMap<String, Arc<Index>>>,
    restored: AtomicU64,
    built: AtomicU64,
    failed: AtomicU64,
}
impl Store {
    pub fn new(path: PathBuf) -> Result<Self> {
        if let Ok(meta) = fs::symlink_metadata(&path)
            && (!meta.is_dir() || meta.file_type().is_symlink())
        {
            return Err(Error::Unavailable);
        }
        fs::create_dir_all(&path).map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
                .map_err(|_| Error::Unavailable)?;
        }
        Ok(Self {
            path,
            writer: Mutex::new(()),
            previous: Mutex::new(BTreeMap::new()),
            restored: AtomicU64::new(0),
            built: AtomicU64::new(0),
            failed: AtomicU64::new(0),
        })
    }
    pub fn stats(&self) -> Stats {
        Stats {
            restored: self.restored.load(Ordering::Relaxed),
            built: self.built.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
        }
    }
    /// The caller supplies facts from a freshly admitted, fully audited source.
    /// Disk data must agree with every supplied row before it can accelerate a read.
    pub fn prepare(&self, key: &Key, rows: Vec<Row>) -> Result<Index> {
        let source = Index::fingerprint(&rows)?;
        let series = serde_json::to_string(&(
            SCHEMA,
            &key.namespace,
            &key.repository,
            &key.epoch,
            &key.projection,
        ))
        .map_err(|_| Error::Invalid)?;
        let key_bytes = serde_json::to_vec(&(SCHEMA, key)).map_err(|_| Error::Invalid)?;
        let path = self
            .path
            .join(format!("{}.redb", hex::encode(Sha256::digest(&key_bytes))));
        if let Ok(index) = restore(&path, &key_bytes, source) {
            self.restored.fetch_add(1, Ordering::Relaxed);
            self.remember(series, &index);
            return Ok(index);
        }
        let previous = self
            .previous
            .lock()
            .map_err(|_| Error::Unavailable)?
            .get(&series)
            .cloned();
        let index = match previous {
            Some(previous) => {
                Index::advance(previous, rows.clone()).or_else(|_| Index::build(rows))?
            }
            None => Index::build(rows)?,
        };
        self.built.fetch_add(1, Ordering::Relaxed);
        // Cache maintenance never holds publication or request authorization locks.
        if let Ok(_writer) = self.writer.try_lock() {
            if self.save(&path, &key_bytes, source, &index).is_err() {
                self.failed.fetch_add(1, Ordering::Relaxed);
            }
        } else {
            self.failed.fetch_add(1, Ordering::Relaxed);
        }
        self.remember(series, &index);
        Ok(index)
    }
    fn remember(&self, series: String, index: &Index) {
        if let Ok(mut previous) = self.previous.lock() {
            previous.insert(series, Arc::new(index.clone()));
            while previous.values().map(|i| i.bytes()).sum::<usize>() > 64 * 1024 * 1024
                || previous.len() > 16
            {
                let Some(key) = previous.keys().next().cloned() else {
                    break;
                };
                previous.remove(&key);
            }
        }
    }
    fn save(&self, path: &Path, key: &[u8], source: [u8; 32], index: &Index) -> Result<()> {
        let bytes = crate::codec::encode(index)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Limit);
        }
        let mut existing = vec![];
        let mut total = 0u64;
        for item in fs::read_dir(&self.path).map_err(|_| Error::Unavailable)? {
            let item = item.map_err(|_| Error::Unavailable)?;
            let p = item.path();
            if !owned_file(&p) {
                continue;
            }
            let meta = fs::symlink_metadata(&p).map_err(|_| Error::Unavailable)?;
            if !meta.is_file() || meta.file_type().is_symlink() {
                return Err(Error::Unavailable);
            }
            total = total.saturating_add(meta.len());
            existing.push((meta.modified().ok(), p, meta.len()));
        }
        existing.sort();
        while existing.len() >= FILES || total.saturating_add(FILE_BYTES) > DISK_BYTES {
            if existing.is_empty() {
                return Err(Error::Limit);
            }
            let (_, old, size) = existing.remove(0);
            fs::remove_file(old).map_err(|_| Error::Unavailable)?;
            total = total.saturating_sub(size);
        }
        let temp = tempfile::NamedTempFile::new_in(&self.path).map_err(|_| Error::Unavailable)?;
        let db = redb::Database::builder()
            .set_cache_size(4 * 1024 * 1024)
            .create(temp.path())
            .map_err(|_| Error::Unavailable)?;
        let txn = db.begin_write().map_err(|_| Error::Unavailable)?;
        {
            let mut table = txn.open_table(DATA).map_err(|_| Error::Unavailable)?;
            for (name, value) in [
                ("key", key),
                ("source", source.as_slice()),
                ("checksum", Sha256::digest(&bytes).as_slice()),
                ("index", bytes.as_slice()),
            ] {
                table.insert(name, value).map_err(|_| Error::Unavailable)?;
            }
        }
        txn.commit().map_err(|_| Error::Unavailable)?;
        drop(db);
        if temp
            .as_file()
            .metadata()
            .map_err(|_| Error::Unavailable)?
            .len()
            > FILE_BYTES
        {
            return Err(Error::Limit);
        }
        temp.persist(path).map_err(|_| Error::Unavailable)?;
        Ok(())
    }
}
fn owned_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|p| p.to_str())
        .is_some_and(|name| {
            name.len() == 69
                && name.ends_with(".redb")
                && name[..64].bytes().all(|b| b.is_ascii_hexdigit())
        })
}
fn restore(path: &Path, key: &[u8], source: [u8; 32]) -> Result<Index> {
    let meta = fs::symlink_metadata(path).map_err(|_| Error::Unavailable)?;
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > FILE_BYTES {
        return Err(Error::Invalid);
    }
    let db = redb::Database::builder()
        .set_cache_size(4 * 1024 * 1024)
        .open_read_only(path)
        .map_err(|_| Error::Invalid)?;
    let txn = db.begin_read().map_err(|_| Error::Invalid)?;
    let table = txn.open_table(DATA).map_err(|_| Error::Invalid)?;
    let get = |name| {
        let value = table
            .get(name)
            .map_err(|_| Error::Invalid)?
            .ok_or(Error::Invalid)?;
        if value.value().len() > MAX_BYTES {
            return Err(Error::Limit);
        }
        Ok(value.value().to_vec())
    };
    if get("key")? != key || get("source")? != source {
        return Err(Error::Invalid);
    }
    let bytes = get("index")?;
    if bytes.len() > MAX_BYTES || get("checksum")? != Sha256::digest(&bytes).as_slice() {
        return Err(Error::Invalid);
    }
    let index: Index = serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)?;
    index.validate()?;
    if index.bytes() > MAX_BYTES || Index::fingerprint(&index.all_rows())? != source {
        return Err(Error::Invalid);
    }
    Ok(index)
}
