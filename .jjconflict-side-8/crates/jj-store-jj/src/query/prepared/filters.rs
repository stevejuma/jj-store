//! Disposable local substring projection. No source bodies survive in memory.
//! Byte grams are generated after whole-string Unicode lowercasing. Intersected
//! postings are candidates only: exact text and pinned visible targets decide.
use super::*;
use crate::query::memory::Lease;
use redb::{ReadOnlyDatabase, ReadableDatabase as _, ReadableTableMetadata as _};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    io::{Read as _, Write as _},
    ops::Bound,
};

pub(crate) const NAME: &str = "local-reference-filter";
const MAGIC: &[u8; 8] = b"JSTLRF1\0";
const TEXT: Table = TableDefinition::new("text");
const SIZES: Table = TableDefinition::new("sizes");
const KEYS: Table = TableDefinition::new("keys");
const RANKS: Table = TableDefinition::new("ranks");
const POSTINGS: Table = TableDefinition::new("postings");
const COUNTS: Table = TableDefinition::new("counts");
const META: Table = TableDefinition::new("meta");
const CACHE_MEMORY: u64 = 16 * 1024 * 1024;

const BLOCK: usize = 4096;
const WORDS: usize = BLOCK / 64;
const PENDING_MAX: usize = 65536;
const PENDING_MEMORY: u64 = 64 * 1024 * 1024;
type Mask = [u64; WORDS];
fn gram(bytes: &[u8]) -> [u8; 4] {
    let mut value = [0; 4];
    value[0] = bytes.len() as u8;
    value[1..1 + bytes.len()].copy_from_slice(bytes);
    value
}
fn key(gram: [u8; 4], block: u32) -> [u8; 8] {
    let mut key = [0; 8];
    key[..4].copy_from_slice(&gram);
    key[4..].copy_from_slice(&block.to_be_bytes());
    key
}
fn decode(bytes: &[u8]) -> Result<Mask> {
    let mut mask = [0; WORDS];
    match bytes.split_first() {
        Some((0, sparse)) if sparse.len() <= BLOCK / 8 && sparse.len().is_multiple_of(2) => {
            let mut prior = None;
            for bytes in sparse.chunks_exact(2) {
                let bit = u16::from_le_bytes(bytes.try_into().unwrap());
                if bit as usize >= BLOCK {
                    return Err(Error::Corrupt);
                }
                if prior.is_some_and(|prior| prior >= bit) {
                    return Err(Error::Corrupt);
                }
                mask[bit as usize / 64] |= 1 << (bit % 64);
                prior = Some(bit);
            }
        }
        Some((1, dense)) if dense.len() == BLOCK / 8 => {
            for (word, bytes) in mask.iter_mut().zip(dense.chunks_exact(8)) {
                *word = u64::from_le_bytes(bytes.try_into().unwrap());
            }
        }
        _ => return Err(Error::Corrupt),
    }
    Ok(mask)
}
fn encode(mask: Mask) -> Vec<u8> {
    if mask.iter().map(|w| w.count_ones()).sum::<u32>() <= (BLOCK / 16) as u32 {
        let mut bytes = vec![0];
        for bit in 0..BLOCK {
            if mask[bit / 64] & (1 << (bit % 64)) != 0 {
                bytes.extend((bit as u16).to_le_bytes());
            }
        }
        bytes
    } else {
        let mut bytes = vec![1];
        for word in mask {
            bytes.extend(word.to_le_bytes());
        }
        bytes
    }
}

pub(super) struct Builder {
    disk: Disk,
    rows: u32,
    pending: BTreeMap<[u8; 4], Mask>,
    block: u32,
    _memory: Lease,
}
impl Builder {
    pub fn new(path: &Path, space: Arc<Space>) -> Result<Self> {
        let memory = Lease::reserve(CACHE_MEMORY + PENDING_MEMORY)?;
        let mut disk = Disk::new(&path.join(NAME), space)?;
        // Empty tables must exist even for a root-only or description-free view.
        for table in [TEXT, SIZES, KEYS, RANKS, POSTINGS, COUNTS, META] {
            disk.put(table, &[], &[])?;
            disk.remove(table, &[])?;
        }
        Ok(Self {
            disk,
            rows: 0,
            pending: BTreeMap::new(),
            block: 0,
            _memory: memory,
        })
    }
    pub fn commit(&mut self, id: &[u8], description: &str, source: &impl Source) -> Result<()> {
        let _memory = Lease::reserve(4096 + description.len() as u64 * 8)?;
        source.work(1 + description.len().div_ceil(64))?;
        let text = description.to_lowercase();
        self.disk.put(
            SIZES,
            id,
            &(u32::try_from(text.len()).map_err(|_| Error::Limit)?).to_be_bytes(),
        )?;
        self.disk.put(TEXT, id, text.as_bytes())
    }
    fn grams(&mut self, rank: u32, text: &str, source: &impl Source) -> Result<()> {
        // Admit the entire bounded per-record dedup vector before growth.
        let _memory = Lease::reserve(4096 + text.len() as u64 * 16)?;
        source.work(1 + text.len().div_ceil(64) * text.len().max(1).ilog2() as usize)?;
        let mut grams = Vec::with_capacity(text.len().saturating_mul(3));
        for width in 1..=3 {
            grams.extend(text.as_bytes().windows(width).map(gram));
        }
        grams.sort_unstable();
        grams.dedup();
        for gram in grams {
            source.work(1)?;
            if self.pending.len() == PENDING_MAX {
                self.flush(source)?;
            }
            let mask = self.pending.entry(gram).or_insert([0; WORDS]);
            mask[rank as usize % BLOCK / 64] |= 1 << (rank % 64);
        }
        Ok(())
    }
    fn flush(&mut self, source: &impl Source) -> Result<()> {
        for (gram, pending) in std::mem::take(&mut self.pending) {
            source.work(1 + WORDS / 8)?;
            let key = key(gram, self.block);
            let mut mask = self
                .disk
                .get(POSTINGS, &key)?
                .map_or(Ok([0; WORDS]), |b| decode(&b))?;
            let mut added = 0;
            for (word, new) in mask.iter_mut().zip(pending) {
                added += (new & !*word).count_ones();
                *word |= new;
            }
            if added == 0 {
                continue;
            }
            self.disk.put(POSTINGS, &key, &encode(mask))?;
            let count = self
                .disk
                .get(COUNTS, &gram)?
                .map_or(Ok(0), |b| number(&b))?;
            self.disk.put(
                COUNTS,
                &gram,
                &count.checked_add(added).ok_or(Error::Limit)?.to_be_bytes(),
            )?;
        }
        Ok(())
    }
    pub fn reference(&mut self, key: &str, source: &impl Source) -> Result<u32> {
        let _memory = Lease::reserve(4096 + key.len() as u64 * 8)?;
        let rank = self.rows;
        if rank / BLOCK as u32 != self.block {
            self.flush(source)?;
            self.block = rank / BLOCK as u32;
        }
        self.rows = self.rows.checked_add(1).ok_or(Error::Limit)?;
        self.disk.put(KEYS, key.as_bytes(), &rank.to_be_bytes())?;
        self.disk.put(RANKS, &rank.to_be_bytes(), key.as_bytes())?;
        self.grams(rank, &key.to_lowercase(), source)?;
        Ok(rank)
    }
    pub fn target(&mut self, rank: u32, id: &[u8], source: &impl Source) -> Result<()> {
        let length = self.disk.len(TEXT, id)?.ok_or(Error::Incomplete)?;
        let _memory = Lease::reserve(4096 + length as u64 * 2)?;
        let text = self.disk.get(TEXT, id)?.ok_or(Error::Incomplete)?;
        let text = std::str::from_utf8(&text).map_err(|_| Error::Corrupt)?;
        self.grams(rank, text, source)
    }
    pub fn finish(
        mut self,
        path: &Path,
        selected: &jj_lib::op_store::OperationId,
        source: &impl Source,
        space: Arc<Space>,
    ) -> Result<()> {
        self.flush(source)?;
        let mut header = MAGIC.to_vec();
        header.extend_from_slice(selected.as_bytes());
        header.extend(self.rows.to_be_bytes());
        self.disk.put(META, b"selection", &header)?;
        self.disk.finish()?;
        drop(self.disk);
        let mut file = std::fs::File::open(path.join(NAME)).map_err(io_error)?;
        let mut digest = Sha256::new();
        let mut bytes = [0; 64 * 1024];
        loop {
            source.work(1)?;
            let n = file.read(&mut bytes).map_err(io_error)?;
            if n == 0 {
                break;
            }
            digest.update(&bytes[..n]);
        }
        space.reserve(32).map_err(io_error)?;
        let mut checksum = std::fs::File::options()
            .write(true)
            .create_new(true)
            .open(path.join(format!("{NAME}.sha256")))
            .map_err(io_error)?;
        checksum.write_all(&digest.finalize()).map_err(io_error)?;
        checksum.sync_all().map_err(io_error)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct SourceOnly;
    impl Source for SourceOnly {
        fn check(&self) -> Result<()> {
            Ok(())
        }
        fn work(&self, _: usize) -> Result<()> {
            Ok(())
        }
        fn max_commits(&self) -> usize {
            10000
        }
        fn max_parents(&self) -> usize {
            1
        }
        fn commit_len(&self) -> usize {
            20
        }
        fn change_len(&self) -> usize {
            16
        }
        async fn operation(&self, _: &jj_lib::op_store::OperationId) -> Result<Operation> {
            unreachable!()
        }
        async fn view(&self, _: &Operation) -> Result<jj_lib::view::View> {
            unreachable!()
        }
        async fn commit(&self, _: &backend::CommitId) -> Result<CommitFields> {
            unreachable!()
        }
    }
    #[test]
    fn filter_masks_preserve_sparse_dense_and_reject_invalid_encodings() {
        for bits in [vec![], vec![0, 7, 255], (0..BLOCK).collect()] {
            let mut mask = [0; WORDS];
            for bit in bits {
                mask[bit / 64] |= 1 << (bit % 64);
            }
            assert_eq!(decode(&encode(mask)).unwrap(), mask);
        }
        for bytes in [
            vec![],
            vec![2],
            vec![0, 1],
            vec![0, 1, 0, 1, 0],
            vec![0, 2, 0, 1, 0],
            vec![0, 0, 16],
            vec![1, 0],
        ] {
            assert_eq!(decode(&bytes).unwrap_err(), Error::Corrupt);
        }
    }
    #[test]
    fn filter_seeks_compressed_windows_and_validates_selection_checksum_and_budgets() {
        let directory = tempfile::tempdir().unwrap();
        let space = Space::new(32 * 1024 * 1024);
        let selected = jj_lib::op_store::OperationId::from_bytes(&[3; 64]);
        let mut builder = Builder::new(directory.path(), space.clone()).unwrap();
        for n in 0u32..9000 {
            let id = n.to_be_bytes();
            let text = if n % 4097 == 0 {
                "ΟΣ İSTANBUL needle"
            } else {
                "ordinary unrelated text"
            };
            builder.commit(&id, text, &SourceOnly).unwrap();
            let rank = builder
                .reference(&format!("change:{n:016}"), &SourceOnly)
                .unwrap();
            builder.target(rank, &id, &SourceOnly).unwrap();
        }
        builder
            .finish(directory.path(), &selected, &SourceOnly, space)
            .unwrap();
        let pin = jj_store_model::OperationId::new([3; 64]);
        let reader = Reader::load(directory.path(), pin, 9000, |_| Ok(()), |_| Ok(())).unwrap();
        let mut work = 0;
        let values = reader
            .search(
                "needle",
                Some("change:0000000000000099"),
                false,
                2,
                |_| Ok(true),
                |n| {
                    work += n;
                    Ok(())
                },
                |_| Ok(()),
            )
            .unwrap();
        assert_eq!(
            values,
            ["change:0000000000004097", "change:0000000000008194"]
        );
        assert!(work < 200, "{work}");
        assert_eq!(
            reader
                .search(
                    "needle",
                    Some(&values[1]),
                    true,
                    2,
                    |_| Ok(true),
                    |_| Ok(()),
                    |_| Ok(())
                )
                .unwrap(),
            ["change:0000000000004097", "change:0000000000000000"]
        );
        assert_eq!(
            reader
                .search("e", None, false, 2, |_| Ok(true), |_| Ok(()), |_| Ok(()))
                .unwrap(),
            ["change:0000000000000000", "change:0000000000000001"]
        );
        assert!(
            reader
                .contains(&4097u32.to_be_bytes(), "ος", |_| Ok(()), |_| Ok(()))
                .unwrap()
        );
        assert!(
            reader
                .contains(
                    &4097u32.to_be_bytes(),
                    "i\u{307}stanbul",
                    |_| Ok(()),
                    |_| Ok(())
                )
                .unwrap()
        );
        assert_eq!(
            reader
                .search(
                    "needle",
                    None,
                    false,
                    1,
                    |_| Ok(false),
                    |_| Ok(()),
                    |_| Ok(())
                )
                .unwrap(),
            Vec::<String>::new()
        );
        assert_eq!(
            reader
                .search(
                    "needle",
                    None,
                    false,
                    1,
                    |_| Ok(true),
                    |_| Err(Error::Canceled),
                    |_| Ok(())
                )
                .unwrap_err(),
            Error::Canceled
        );
        assert_eq!(
            reader
                .search(
                    "needle",
                    None,
                    false,
                    1,
                    |_| Ok(true),
                    |_| Ok(()),
                    |_| Err(Error::Limit)
                )
                .unwrap_err(),
            Error::Limit
        );
        assert_eq!(
            reader
                .contains(
                    &4097u32.to_be_bytes(),
                    "needle",
                    |_| Ok(()),
                    |_| Err(Error::Limit)
                )
                .unwrap_err(),
            Error::Limit
        );
        drop(reader);
        assert!(matches!(
            Reader::load(directory.path(), pin, 10, |_| Ok(()), |_| Ok(())),
            Err(Error::Limit)
        ));
        assert!(matches!(
            Reader::load(
                directory.path(),
                jj_store_model::OperationId::new([4; 64]),
                9000,
                |_| Ok(()),
                |_| Ok(())
            ),
            Err(Error::Corrupt)
        ));
        let mut bytes = std::fs::read(directory.path().join(NAME)).unwrap();
        bytes[100] ^= 1;
        std::fs::write(directory.path().join(NAME), bytes).unwrap();
        assert!(matches!(
            Reader::load(directory.path(), pin, 9000, |_| Ok(()), |_| Ok(())),
            Err(Error::Corrupt)
        ));
    }
    #[test]
    fn filter_database_refuses_unreserved_growth() {
        let directory = tempfile::tempdir().unwrap();
        assert!(matches!(
            Builder::new(directory.path(), Space::new(4096)),
            Err(Error::Limit)
        ));
        assert!(
            std::fs::metadata(directory.path().join(NAME))
                .unwrap()
                .len()
                <= 4096
        );
    }
}

pub(crate) struct Reader {
    db: ReadOnlyDatabase,
    rows: u32,
    _memory: Lease,
}
impl Reader {
    pub(crate) fn load(
        path: &Path,
        selected: jj_store_model::OperationId,
        maximum: usize,
        mut work: impl FnMut(usize) -> Result<()>,
        mut read: impl FnMut(usize) -> Result<()>,
    ) -> Result<Self> {
        let memory = Lease::reserve(CACHE_MEMORY + 64 * 1024)?;
        let path = path.join(NAME);
        let metadata = std::fs::symlink_metadata(&path).map_err(|_| Error::Incomplete)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(Error::Corrupt);
        }
        read(usize::try_from(metadata.len()).map_err(|_| Error::Limit)?)?;
        let mut file = std::fs::File::open(&path).map_err(io_error)?;
        let mut digest = Sha256::new();
        let mut bytes = [0; 64 * 1024];
        loop {
            work(1)?;
            let n = file.read(&mut bytes).map_err(io_error)?;
            if n == 0 {
                break;
            }
            digest.update(&bytes[..n]);
        }
        let checksum = path.with_file_name(format!("{NAME}.sha256"));
        let metadata = std::fs::symlink_metadata(&checksum).map_err(|_| Error::Incomplete)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() != 32 {
            return Err(Error::Corrupt);
        }
        read(32)?;
        if std::fs::read(checksum).map_err(io_error)? != digest.finalize().as_slice() {
            return Err(Error::Corrupt);
        }
        let db = redb::Database::builder()
            .set_cache_size(8 * 1024 * 1024)
            .open_read_only(&path)
            .map_err(disk::db_error)?;
        let txn = db.begin_read().map_err(disk::db_error)?;
        let table = txn.open_table(META).map_err(disk::db_error)?;
        let header = table
            .get(b"selection".as_slice())
            .map_err(disk::db_error)?
            .ok_or(Error::Corrupt)?;
        let header = header.value();
        if header.len() != 76 || &header[..8] != MAGIC || &header[8..72] != selected.as_bytes() {
            return Err(Error::Corrupt);
        }
        let rows = number(&header[72..])?;
        if rows as usize > maximum {
            return Err(Error::Limit);
        }
        work(2)?;
        for name in [KEYS, RANKS] {
            if txn
                .open_table(name)
                .map_err(disk::db_error)?
                .len()
                .map_err(disk::db_error)?
                != u64::from(rows)
            {
                return Err(Error::Corrupt);
            }
        }
        drop(table);
        drop(txn);
        Ok(Self {
            db,
            rows,
            _memory: memory,
        })
    }
    pub(crate) fn contains(
        &self,
        id: &[u8],
        needle: &str,
        mut work: impl FnMut(usize) -> Result<()>,
        mut read: impl FnMut(usize) -> Result<()>,
    ) -> Result<bool> {
        let txn = self.db.begin_read().map_err(disk::db_error)?;
        let sizes = txn.open_table(SIZES).map_err(disk::db_error)?;
        let size = sizes
            .get(id)
            .map_err(disk::db_error)?
            .ok_or(Error::Incomplete)?;
        let size = number(size.value())? as usize;
        work(2 + size.div_ceil(64))?;
        read(size)?;
        let _memory = Lease::reserve(4096 + size as u64 * 2)?;
        let table = txn.open_table(TEXT).map_err(disk::db_error)?;
        let text = table
            .get(id)
            .map_err(disk::db_error)?
            .ok_or(Error::Incomplete)?;
        let bytes = text.value();
        if bytes.len() != size {
            return Err(Error::Corrupt);
        }
        Ok(std::str::from_utf8(bytes)
            .map_err(|_| Error::Corrupt)?
            .contains(needle))
    }
    /// Seek compressed candidate blocks in lexical rank order; no all-match set.
    pub(crate) fn search(
        &self,
        needle: &str,
        anchor: Option<&str>,
        backward: bool,
        limit: usize,
        mut matches: impl FnMut(&str) -> Result<bool>,
        mut work: impl FnMut(usize) -> Result<()>,
        mut read: impl FnMut(usize) -> Result<()>,
    ) -> Result<Vec<String>> {
        if needle.is_empty() || limit == 0 {
            return Err(Error::Invalid);
        }
        let _memory = Lease::reserve(8192 + needle.len() as u64 * 32 + limit as u64 * 8192)?;
        let txn = self.db.begin_read().map_err(disk::db_error)?;
        let keys = txn.open_table(KEYS).map_err(disk::db_error)?;
        let ranks = txn.open_table(RANKS).map_err(disk::db_error)?;
        let counts = txn.open_table(COUNTS).map_err(disk::db_error)?;
        let postings = txn.open_table(POSTINGS).map_err(disk::db_error)?;
        work(2 + self.rows.max(1).ilog2() as usize)?;
        let bounds = match (backward, anchor) {
            (false, Some(a)) => (Bound::Excluded(a.as_bytes()), Bound::Unbounded),
            (true, Some(a)) => (Bound::Unbounded, Bound::Excluded(a.as_bytes())),
            _ => (Bound::Unbounded, Bound::Unbounded),
        };
        let mut range = keys.range::<&[u8]>(bounds).map_err(disk::db_error)?;
        let start = if backward {
            range.next_back()
        } else {
            range.next()
        };
        let Some((_, start)) = start.transpose().map_err(disk::db_error)? else {
            return Ok(vec![]);
        };
        let start = number(start.value())?;
        let width = needle.len().min(3);
        let mut grams: Vec<_> = needle.as_bytes().windows(width).map(gram).collect();
        grams.sort_unstable();
        grams.dedup();
        let mut order = vec![];
        for gram in grams {
            work(1 + self.rows.max(1).ilog2() as usize)?;
            let Some(count) = counts.get(gram.as_slice()).map_err(disk::db_error)? else {
                return Ok(vec![]);
            };
            order.push((number(count.value())?, gram));
        }
        order.sort_unstable();
        let rare = order[0].1;
        let low = key(rare, if backward { 0 } else { start / BLOCK as u32 });
        let high = key(
            rare,
            if backward {
                start / BLOCK as u32
            } else {
                u32::MAX
            },
        );
        let mut blocks = postings
            .range::<&[u8]>(low.as_slice()..=high.as_slice())
            .map_err(disk::db_error)?;
        let mut result = Vec::with_capacity(limit);
        loop {
            work(1)?;
            let row = if backward {
                blocks.next_back()
            } else {
                blocks.next()
            };
            let Some((block, bytes)) = row.transpose().map_err(disk::db_error)? else {
                break;
            };
            if block.value().len() != 8 {
                return Err(Error::Corrupt);
            }
            let block = number(&block.value()[4..])?;
            read(bytes.value().len())?;
            let mut mask = decode(bytes.value())?;
            if block == start / BLOCK as u32 {
                let word = start as usize % BLOCK / 64;
                if backward {
                    mask[word] &= u64::MAX >> (63 - start % 64);
                    mask[word + 1..].fill(0);
                } else {
                    mask[word] &= u64::MAX << (start % 64);
                    mask[..word].fill(0);
                }
            }
            // Do not read intersecting postings for an already excluded block.
            if mask == [0; WORDS] {
                continue;
            }
            for (_, gram) in &order[1..] {
                work(1 + self.rows.max(1).ilog2() as usize)?;
                let key = key(*gram, block);
                let other =
                    if let Some(bytes) = postings.get(key.as_slice()).map_err(disk::db_error)? {
                        read(bytes.value().len())?;
                        decode(bytes.value())?
                    } else {
                        [0; WORDS]
                    };
                for (a, b) in mask.iter_mut().zip(other) {
                    *a &= b;
                }
                if mask == [0; WORDS] {
                    break;
                }
            }
            for offset in 0..WORDS {
                let word = if backward { WORDS - 1 - offset } else { offset };
                let mut bits = mask[word];
                while bits != 0 {
                    let bit = if backward {
                        63 - bits.leading_zeros()
                    } else {
                        bits.trailing_zeros()
                    };
                    bits &= !(1 << bit);
                    let rank = block
                        .checked_mul(BLOCK as u32)
                        .and_then(|n| n.checked_add(word as u32 * 64 + bit))
                        .ok_or(Error::Corrupt)?;
                    if rank >= self.rows
                        || (backward && rank > start)
                        || (!backward && rank < start)
                    {
                        continue;
                    }
                    work(1 + self.rows.max(1).ilog2() as usize)?;
                    let rank = rank.to_be_bytes();
                    let value = ranks
                        .get(rank.as_slice())
                        .map_err(disk::db_error)?
                        .ok_or(Error::Corrupt)?;
                    read(value.value().len())?;
                    let value = std::str::from_utf8(value.value()).map_err(|_| Error::Corrupt)?;
                    if matches(value)? {
                        result.push(value.to_owned());
                        if result.len() == limit {
                            return Ok(result);
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}
