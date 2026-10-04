//! Disposable fixed-width retained-operation identities, never source authority.
use super::*;
use crate::query::memory::Lease;
use jj_lib::object_id::{HexPrefix, PrefixResolution};
use jj_store_model::OperationId;
use sha2::{Digest as _, Sha256};
use std::io::{BufReader, BufWriter, Read as _, Write as _};

const MAGIC: &[u8; 8] = b"JSTLOP2\0";
const FRAMING: u64 = 8 + 64 + 32;
const RECORD: u64 = 128;
struct Record {
    id: OperationId,
    digest: [u8; 32],
    view_digest: [u8; 32],
}
pub(crate) struct Lookup {
    ids: Vec<Record>,
    _memory: Lease,
}

pub(super) fn write(
    path: &Path,
    selected: &jj_lib::op_store::OperationId,
    disk: &Tables,
    source: &impl Source,
    space: Arc<Space>,
) -> Result<()> {
    if selected.as_bytes().len() != 64 {
        return Err(Error::Corrupt);
    }
    let file = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(path.join("local-operations"))
        .map_err(io_error)?;
    let mut file = BufWriter::new(file);
    let mut digest = Sha256::new();
    let mut output = |bytes: &[u8]| -> Result<()> {
        source.work(1)?;
        space.reserve(bytes.len() as u64).map_err(io_error)?;
        file.write_all(bytes).map_err(io_error)?;
        digest.update(bytes);
        Ok(())
    };
    output(MAGIC)?;
    output(selected.as_bytes())?;
    let mut row = disk.first(SEEN_OPS)?;
    while let Some((id, fingerprint)) = row {
        if id.len() != 64 || fingerprint.len() != 64 {
            return Err(Error::Corrupt);
        }
        output(&id)?;
        output(&fingerprint)?;
        row = disk.after(SEEN_OPS, &id)?;
    }
    space.reserve(32).map_err(io_error)?;
    file.write_all(&digest.finalize()).map_err(io_error)?;
    file.flush().map_err(io_error)?;
    file.get_ref().sync_all().map_err(io_error)?;
    Ok(())
}
impl Lookup {
    pub(crate) fn load(
        path: &Path,
        selected: OperationId,
        maximum: u64,
        mut work: impl FnMut(usize) -> Result<()>,
        mut read_bytes: impl FnMut(usize) -> Result<()>,
    ) -> Result<Self> {
        let path = path.join("local-operations");
        let metadata = std::fs::symlink_metadata(&path).map_err(|_| Error::Incomplete)?;
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() < FRAMING + RECORD
            || !(metadata.len() - FRAMING).is_multiple_of(RECORD)
        {
            return Err(Error::Corrupt);
        }
        read_bytes(usize::try_from(metadata.len()).map_err(|_| Error::Limit)?)?;
        let count = (metadata.len() - FRAMING) / RECORD;
        if count > maximum {
            return Err(Error::Limit);
        }
        // Admit the vector, buffer and framing before allocation. Each identity
        // is fixed-width; no operation bodies, views or reference names survive.
        let memory = Lease::reserve(
            count
                .checked_mul(256)
                .and_then(|n| n.checked_add(64 * 1024))
                .ok_or(Error::Limit)?,
        )?;
        let mut file = BufReader::new(std::fs::File::open(path).map_err(|_| Error::Incomplete)?);
        let mut digest = Sha256::new();
        let mut header = [0; 72];
        file.read_exact(&mut header).map_err(|_| Error::Corrupt)?;
        if &header[..8] != MAGIC || &header[8..] != selected.as_bytes() {
            return Err(Error::Corrupt);
        }
        digest.update(header);
        let mut ids = Vec::with_capacity(usize::try_from(count).map_err(|_| Error::Limit)?);
        for _ in 0..count {
            work(1)?;
            let mut bytes = [0; 128];
            file.read_exact(&mut bytes).map_err(|_| Error::Corrupt)?;
            digest.update(bytes);
            let id = OperationId::from_slice(&bytes[..64]).map_err(|_| Error::Corrupt)?;
            if ids.last().is_some_and(|prior: &Record| prior.id >= id) {
                return Err(Error::Corrupt);
            }
            ids.push(Record {
                id,
                digest: bytes[64..96].try_into().unwrap(),
                view_digest: bytes[96..].try_into().unwrap(),
            });
        }
        let mut checksum = [0; 32];
        file.read_exact(&mut checksum).map_err(|_| Error::Corrupt)?;
        if checksum != <[u8; 32]>::from(digest.finalize())
            || file.read(&mut [0; 1]).map_err(|_| Error::Corrupt)? != 0
            || ids
                .binary_search_by_key(&selected, |record| record.id)
                .is_err()
        {
            return Err(Error::Corrupt);
        }
        Ok(Self {
            ids,
            _memory: memory,
        })
    }
    pub(crate) fn fingerprint(
        &self,
        id: OperationId,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<[u8; 32]>> {
        work(self.ids.len().max(1).ilog2() as usize + 2)?;
        Ok(self
            .ids
            .binary_search_by_key(&id, |record| record.id)
            .ok()
            .map(|index| self.ids[index].digest))
    }
    pub(crate) fn resolve(
        &self,
        prefix: &HexPrefix,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<PrefixResolution<jj_lib::op_store::OperationId>> {
        work((self.ids.len().max(1).ilog2() as usize + 1) * 2 + 3)?;
        let start = self
            .ids
            .partition_point(|id| id.id.as_bytes().as_slice() < prefix.min_prefix_bytes());
        let id = |index: usize| {
            self.ids
                .get(index)
                .map(|id| jj_lib::op_store::OperationId::from_bytes(id.id.as_bytes()))
                .filter(|id| prefix.matches(id))
        };
        Ok(match (id(start), id(start + 1)) {
            (None, _) => PrefixResolution::NoMatch,
            (Some(_), Some(_)) => PrefixResolution::AmbiguousMatch,
            (Some(id), None) => PrefixResolution::SingleMatch(id),
        })
    }
    pub(crate) fn view_fingerprint(
        &self,
        id: OperationId,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<[u8; 32]>> {
        work(self.ids.len().max(1).ilog2() as usize + 2)?;
        Ok(self
            .ids
            .binary_search_by_key(&id, |record| record.id)
            .ok()
            .map(|index| self.ids[index].view_digest))
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
            1
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
    fn operation_identity_writer_refuses_growth_before_exceeding_reserved_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let scratch = directory.path().join("scratch");
        let mut disk = Tables::new(&scratch, Space::new(1024 * 1024), true).unwrap();
        disk.put(SEEN_OPS, &[1; 64], &[4; 64]).unwrap();
        disk.put(SEEN_OPS, &[2; 64], &[4; 64]).unwrap();
        let selected = jj_lib::op_store::OperationId::from_bytes(&[2; 64]);
        let limit = FRAMING + RECORD * 2 - 1;
        assert_eq!(
            write(
                directory.path(),
                &selected,
                &disk,
                &SourceOnly,
                Space::new(limit)
            )
            .unwrap_err(),
            Error::Limit
        );
        assert!(
            std::fs::metadata(directory.path().join("local-operations"))
                .unwrap()
                .len()
                <= limit
        );
    }
    fn artifact(selected: OperationId, ids: &[OperationId]) -> Vec<u8> {
        let mut bytes = MAGIC.to_vec();
        bytes.extend_from_slice(selected.as_bytes());
        for id in ids {
            bytes.extend_from_slice(id.as_bytes());
            bytes.extend_from_slice(&[4; 64]);
        }
        let checksum = Sha256::digest(&bytes);
        bytes.extend_from_slice(&checksum);
        bytes
    }
    #[test]
    fn lookup_preserves_exact_odd_missing_and_ambiguous_prefixes_under_admission() {
        let directory = tempfile::tempdir().unwrap();
        let ids = [
            OperationId::new([1; 64]),
            OperationId::new([2; 64]),
            OperationId::new([3; 64]),
        ];
        let bytes = artifact(ids[1], &ids);
        std::fs::write(directory.path().join("local-operations"), &bytes).unwrap();
        let mut admitted = 0;
        let lookup = Lookup::load(
            directory.path(),
            ids[1],
            3,
            |_| Ok(()),
            |n| {
                admitted += n;
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(admitted, bytes.len());
        assert!(lookup.fingerprint(ids[0], |_| Ok(())).unwrap().is_some());
        assert_eq!(
            lookup.view_fingerprint(ids[0], |_| Ok(())).unwrap(),
            Some([4; 32])
        );
        assert!(
            lookup
                .view_fingerprint(OperationId::new([4; 64]), |_| Ok(()))
                .unwrap()
                .is_none()
        );
        assert!(
            lookup
                .fingerprint(OperationId::new([4; 64]), |_| Ok(()))
                .unwrap()
                .is_none()
        );
        for prefix in ["02", "020", &ids[1].to_string()] {
            let prefix = HexPrefix::try_from_hex(prefix).unwrap();
            let resolved = lookup.resolve(&prefix, |_| Ok(())).unwrap();
            assert!(
                matches!(resolved, PrefixResolution::SingleMatch(id) if id.as_bytes() == ids[1].as_bytes())
            );
        }
        assert!(matches!(
            lookup
                .resolve(&HexPrefix::try_from_hex("0").unwrap(), |_| Ok(()))
                .unwrap(),
            PrefixResolution::AmbiguousMatch
        ));
        assert!(matches!(
            lookup
                .resolve(&HexPrefix::try_from_hex("f").unwrap(), |_| Ok(()))
                .unwrap(),
            PrefixResolution::NoMatch
        ));
        assert!(matches!(
            lookup.resolve(&HexPrefix::try_from_hex("02").unwrap(), |_| Err(
                Error::Canceled
            )),
            Err(Error::Canceled)
        ));
        assert!(matches!(
            Lookup::load(directory.path(), ids[1], 2, |_| Ok(()), |_| Ok(())),
            Err(Error::Limit)
        ));
        assert!(matches!(
            Lookup::load(
                directory.path(),
                ids[1],
                3,
                |_| Ok(()),
                |_| Err(Error::Limit)
            ),
            Err(Error::Limit)
        ));
    }
    #[test]
    fn lookup_rejects_corrupt_framing_checksum_order_membership_and_source_selection() {
        let directory = tempfile::tempdir().unwrap();
        let a = OperationId::new([1; 64]);
        let b = OperationId::new([2; 64]);
        let mut corrupt = artifact(b, &[a, b]);
        corrupt[80] ^= 1;
        let mut magic = artifact(b, &[a, b]);
        magic[0] ^= 1;
        let valid = artifact(b, &[a, b]);
        for bytes in [
            corrupt,
            magic,
            valid[..valid.len() - 1].to_vec(),
            artifact(b, &[b, a]),
            artifact(b, &[a, a]),
            artifact(b, &[a]),
            artifact(a, &[a, b]),
        ] {
            std::fs::write(directory.path().join("local-operations"), bytes).unwrap();
            assert!(matches!(
                Lookup::load(directory.path(), b, 3, |_| Ok(()), |_| Ok(())),
                Err(Error::Corrupt)
            ));
        }
    }
}
