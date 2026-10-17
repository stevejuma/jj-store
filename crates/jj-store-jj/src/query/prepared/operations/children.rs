//! Flat reverse edges for the same complete retained operation closure.
use super::*;
const MAGIC: &[u8; 8] = b"JSTLCH1\0";
const FILE: &str = super::CHILDREN_NAME;
const FRAMING: u64 = 8 + 64 + 32;
const RECORD: u64 = 128;

pub(super) fn write(
    path: &Path,
    selected: &jj_lib::op_store::OperationId,
    disk: &Tables,
    source: &impl Source,
    space: Arc<Space>,
) -> Result<()> {
    let file = std::fs::File::options()
        .write(true)
        .create_new(true)
        .open(path.join(FILE))
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
    let mut row = disk.first(OP_CHILDREN)?;
    while let Some((edge, value)) = row {
        if edge.len() != 128 || !value.is_empty() {
            return Err(Error::Corrupt);
        }
        output(&edge)?;
        row = disk.after(OP_CHILDREN, &edge)?;
    }
    space.reserve(32).map_err(io_error)?;
    file.write_all(&digest.finalize()).map_err(io_error)?;
    file.flush().map_err(io_error)?;
    file.get_ref().sync_all().map_err(io_error)?;
    Ok(())
}

pub(super) struct Lookup {
    edges: Vec<(OperationId, OperationId)>,
    _memory: Lease,
}
impl Lookup {
    #[cfg(feature = "native")]
    pub(super) fn weight(&self) -> u64 {
        self.edges.capacity() as u64 * 256 + 64 * 1024
    }

    pub(super) fn load(
        path: &Path,
        selected: OperationId,
        ids: &[super::Record],
        maximum: u64,
        work: &mut impl FnMut(usize) -> Result<()>,
        read_bytes: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<Self>> {
        let path = path.join(FILE);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(value) => value,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Incomplete),
        };
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() < FRAMING
            || !(metadata.len() - FRAMING).is_multiple_of(RECORD)
        {
            return Err(Error::Corrupt);
        }
        read_bytes(usize::try_from(metadata.len()).map_err(|_| Error::Limit)?)?;
        let count = (metadata.len() - FRAMING) / RECORD;
        if count > maximum.checked_mul(256).ok_or(Error::Limit)? {
            return Err(Error::Limit);
        }
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
        let mut edges = Vec::with_capacity(usize::try_from(count).map_err(|_| Error::Limit)?);
        for _ in 0..count {
            work((ids.len().max(1).ilog2() as usize + 2) * 2 + 1)?;
            let mut bytes = [0; 128];
            file.read_exact(&mut bytes).map_err(|_| Error::Corrupt)?;
            digest.update(bytes);
            let edge = (
                OperationId::from_slice(&bytes[..64]).map_err(|_| Error::Corrupt)?,
                OperationId::from_slice(&bytes[64..]).map_err(|_| Error::Corrupt)?,
            );
            if edge.0 == edge.1
                || edges.last().is_some_and(|prior| prior >= &edge)
                || [edge.0, edge.1]
                    .iter()
                    .any(|id| ids.binary_search_by_key(id, |record| record.id).is_err())
            {
                return Err(Error::Corrupt);
            }
            edges.push(edge);
        }
        let mut checksum = [0; 32];
        file.read_exact(&mut checksum).map_err(|_| Error::Corrupt)?;
        if checksum != <[u8; 32]>::from(digest.finalize())
            || file.read(&mut [0; 1]).map_err(|_| Error::Corrupt)? != 0
        {
            return Err(Error::Corrupt);
        }
        Ok(Some(Self {
            edges,
            _memory: memory,
        }))
    }
    pub(super) fn child(
        &self,
        parent: OperationId,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<PrefixResolution<jj_lib::op_store::OperationId>> {
        work(self.edges.len().max(1).ilog2() as usize + 4)?;
        let start = self.edges.partition_point(|edge| edge.0 < parent);
        let first = self.edges.get(start).filter(|edge| edge.0 == parent);
        let second = self.edges.get(start + 1).filter(|edge| edge.0 == parent);
        Ok(match (first, second) {
            (None, _) => PrefixResolution::NoMatch,
            (Some(edge), None) => PrefixResolution::SingleMatch(
                jj_lib::op_store::OperationId::from_bytes(edge.1.as_bytes()),
            ),
            (Some(_), Some(_)) => PrefixResolution::AmbiguousMatch,
        })
    }
}

#[cfg(test)]
mod tests;
