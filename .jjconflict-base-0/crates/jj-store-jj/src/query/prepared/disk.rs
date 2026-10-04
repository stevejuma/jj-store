//! Disposable ordered scratch tables with bounded caches and pre-growth admission.
use super::*;
use redb::{
    Database, Durability, ReadableTable, StorageBackend, TableDefinition, WriteTransaction,
};
use std::{fs::File, io, ops::Bound, sync::Mutex};

pub(super) type Table = TableDefinition<'static, &'static [u8], &'static [u8]>;
type Row = (Vec<u8>, Vec<u8>);

#[derive(Debug)]
pub(super) struct Space {
    used: Mutex<u64>,
    limit: u64,
}
impl Space {
    pub fn new(limit: u64) -> Arc<Self> {
        Arc::new(Self {
            used: Mutex::new(0),
            limit,
        })
    }
    pub fn reserve(&self, bytes: u64) -> io::Result<()> {
        let mut used = self.used.lock().unwrap();
        *used = used
            .checked_add(bytes)
            .filter(|n| *n <= self.limit)
            .ok_or_else(|| io::Error::other(Error::Limit))?;
        Ok(())
    }
    fn release(&self, bytes: u64) {
        *self.used.lock().unwrap() -= bytes;
    }
}

#[derive(Debug)]
struct BoundedFile {
    inner: redb::backends::FileBackend,
    size: Mutex<u64>,
    space: Arc<Space>,
}
impl StorageBackend for BoundedFile {
    fn len(&self) -> io::Result<u64> {
        self.inner.len()
    }
    fn read(&self, offset: u64, out: &mut [u8]) -> io::Result<()> {
        self.inner.read(offset, out)
    }
    fn set_len(&self, len: u64) -> io::Result<()> {
        let mut size = self.size.lock().unwrap();
        let growth = len.saturating_sub(*size);
        self.space.reserve(growth)?;
        if let Err(e) = self.inner.set_len(len) {
            self.space.release(growth);
            return Err(e);
        }
        self.space.release(size.saturating_sub(len));
        *size = len;
        Ok(())
    }
    fn sync_data(&self) -> io::Result<()> {
        self.inner.sync_data()
    }
    fn write(&self, offset: u64, data: &[u8]) -> io::Result<()> {
        let mut size = self.size.lock().unwrap();
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or_else(|| io::Error::other(Error::Limit))?;
        self.space.reserve(end.saturating_sub(*size))?;
        // Keep the reservation on partial IO failure: the disposable database
        // is discarded, and no retry may spend the same bytes twice.
        *size = (*size).max(end);
        self.inner.write(offset, data)
    }
    fn close(&self) -> io::Result<()> {
        self.inner.close()
    }
    fn try_lock_range(
        &self,
        a: Bound<u64>,
        b: Bound<u64>,
    ) -> std::result::Result<bool, redb::BackendError> {
        self.inner.try_lock_range(a, b)
    }
    fn try_lock_shared_range(
        &self,
        a: Bound<u64>,
        b: Bound<u64>,
    ) -> std::result::Result<bool, redb::BackendError> {
        self.inner.try_lock_shared_range(a, b)
    }
    fn lock_range(
        &self,
        a: Bound<u64>,
        b: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.lock_range(a, b)
    }
    fn lock_shared_range(
        &self,
        a: Bound<u64>,
        b: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.lock_shared_range(a, b)
    }
    fn unlock_range(
        &self,
        a: Bound<u64>,
        b: Bound<u64>,
    ) -> std::result::Result<(), redb::BackendError> {
        self.inner.unlock_range(a, b)
    }
}

pub(super) struct Disk {
    transaction: Option<WriteTransaction>,
    db: Database,
    writes: usize,
    batches: usize,
}
impl Disk {
    pub fn new(path: &Path, space: Arc<Space>) -> Result<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(io_error)?;
        let mut builder = Database::builder();
        builder.set_cache_size(8 * 1024 * 1024);
        let db = builder
            .create_with_backend(BoundedFile {
                inner: redb::backends::FileBackend::new(file).map_err(db_error)?,
                size: Mutex::new(0),
                space,
            })
            .map_err(db_error)?;
        let mut disk = Self {
            transaction: None,
            db,
            writes: 0,
            batches: 0,
        };
        disk.begin()?;
        Ok(disk)
    }
    fn begin(&mut self) -> Result<()> {
        let mut txn = self.db.begin_write().map_err(db_error)?;
        // Scratch is never recovered or published. No fsync per traversal batch.
        txn.set_durability(Durability::None).map_err(db_error)?;
        self.transaction = Some(txn);
        Ok(())
    }
    fn table(&self, name: Table) -> Result<redb::Table<'_, &'static [u8], &'static [u8]>> {
        self.transaction
            .as_ref()
            .unwrap()
            .open_table(name)
            .map_err(db_error)
    }
    pub fn get(&self, table: Table, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self
            .table(table)?
            .get(key)
            .map_err(db_error)?
            .map(|v| v.value().to_vec()))
    }
    pub fn len(&self, table: Table, key: &[u8]) -> Result<Option<usize>> {
        Ok(self
            .table(table)?
            .get(key)
            .map_err(db_error)?
            .map(|v| v.value().len()))
    }
    pub fn finish(&mut self) -> Result<()> {
        let mut transaction = self.transaction.take().unwrap();
        transaction
            .set_durability(Durability::Immediate)
            .map_err(db_error)?;
        transaction.commit().map_err(db_error)
    }
    pub fn put(&mut self, table: Table, key: &[u8], value: &[u8]) -> Result<()> {
        self.table(table)?.insert(key, value).map_err(db_error)?;
        self.wrote()
    }
    pub fn remove(&mut self, table: Table, key: &[u8]) -> Result<()> {
        self.table(table)?.remove(key).map_err(db_error)?;
        self.wrote()
    }
    fn wrote(&mut self) -> Result<()> {
        self.writes += 1;
        if self.writes == 512 {
            let mut transaction = self.transaction.take().unwrap();
            self.batches += 1;
            if self.batches.is_multiple_of(16) {
                // Bound pending non-durable commit and freed-page tracking.
                transaction
                    .set_durability(Durability::Immediate)
                    .map_err(db_error)?;
            }
            transaction.commit().map_err(db_error)?;
            self.begin()?;
            self.writes = 0;
        }
        Ok(())
    }
    pub fn first(&self, table: Table) -> Result<Option<Row>> {
        Ok(self
            .table(table)?
            .first()
            .map_err(db_error)?
            .map(|(k, v)| (k.value().to_vec(), v.value().to_vec())))
    }
    pub fn last(&self, table: Table) -> Result<Option<Row>> {
        Ok(self
            .table(table)?
            .last()
            .map_err(db_error)?
            .map(|(k, v)| (k.value().to_vec(), v.value().to_vec())))
    }
    pub fn after(&self, table: Table, key: &[u8]) -> Result<Option<Row>> {
        self.table(table)?
            .range::<&[u8]>((Bound::Excluded(key), Bound::Unbounded))
            .map_err(db_error)?
            .next()
            .transpose()
            .map_err(db_error)
            .map(|r| r.map(|(k, v)| (k.value().to_vec(), v.value().to_vec())))
    }
}

pub(super) fn db_error(e: impl Into<redb::Error>) -> Error {
    match e.into() {
        redb::Error::Io(e) => io_error(e),
        redb::Error::ValueTooLarge(_) => Error::Limit,
        redb::Error::Corrupted(_) => Error::Corrupt,
        _ => Error::Unavailable,
    }
}
pub(super) fn io_error(e: io::Error) -> Error {
    io_error_ref(&e)
}
fn io_error_ref(e: &io::Error) -> Error {
    e.get_ref()
        .and_then(|e| e.downcast_ref::<Error>())
        .cloned()
        .unwrap_or(Error::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scratch_file_refuses_growth_before_touching_the_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("scratch");
        let file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let space = Space::new(4096);
        let backend = BoundedFile {
            inner: redb::backends::FileBackend::new(file).unwrap(),
            size: Mutex::new(0),
            space: space.clone(),
        };
        backend.set_len(4000).unwrap();
        backend.write(3999, &[1]).unwrap();
        assert_eq!(io_error(backend.set_len(4097).unwrap_err()), Error::Limit);
        assert_eq!(
            io_error(backend.write(4096, &[1]).unwrap_err()),
            Error::Limit
        );
        assert_eq!(std::fs::metadata(&path).unwrap().len(), 4000);
        // Pending segment bytes share scratch's allowance, including a buffer.
        space.reserve(96).unwrap();
        assert_eq!(io_error(space.reserve(1).unwrap_err()), Error::Limit);
        backend.set_len(3000).unwrap();
        space.reserve(1000).unwrap();
        assert_eq!(*space.used.lock().unwrap(), 4096);
    }

    #[test]
    fn scratch_tables_survive_batch_boundaries_and_refuse_quota_overflow() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("scratch.redb");
        let space = Space::new(16 * 1024 * 1024);
        let mut disk = Disk::new(&path, space.clone()).unwrap();
        let table = TableDefinition::new("ordered");
        for n in (0u32..2000).rev() {
            disk.put(table, &n.to_be_bytes(), &n.to_be_bytes()).unwrap();
        }
        for n in 0u32..2000 {
            let (key, value) = disk.first(table).unwrap().unwrap();
            assert_eq!(key, n.to_be_bytes());
            assert_eq!(value, key);
            disk.remove(table, &key).unwrap();
        }
        assert!(disk.first(table).unwrap().is_none());
        drop(disk);
        assert!(std::fs::metadata(path).unwrap().len() <= space.limit);
        let tiny = directory.path().join("tiny.redb");
        assert!(matches!(
            Disk::new(&tiny, Space::new(4096)),
            Err(Error::Limit)
        ));
        assert!(std::fs::metadata(tiny).unwrap().len() <= 4096);
    }
}
