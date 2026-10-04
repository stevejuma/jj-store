//! Read-only seeks in JJ 0.45.1 stacked extra-metadata tables.
//! Format follows jj-lib/stacked_table.rs (Apache-2.0, Jujutsu Authors).
use super::*;
use blake2::{Blake2b512, Digest as _};
use std::{
    fs,
    io::{Read as _, Seek as _},
    sync::Mutex,
};

struct Segment {
    file: Mutex<fs::File>,
    index: u64,
    values: u64,
    size: u64,
    count: u32,
    parent: Option<String>,
    stamp: Stamp,
    digest: String,
}
#[derive(PartialEq, Eq)]
struct Stamp {
    size: u64,
    modified: std::time::SystemTime,
    #[cfg(unix)]
    identity: [u64; 4],
}
impl Stamp {
    fn precise(&self) -> bool {
        #[cfg(unix)]
        {
            self.identity[3] != 0
                && self
                    .modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .is_ok_and(|time| time.subsec_nanos() != 0)
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
    fn read(file: &fs::File) -> Result<Self> {
        let metadata = file.metadata().map_err(|_| Error::Incomplete)?;
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt as _;
        Ok(Self {
            size: metadata.len(),
            modified: metadata.modified().map_err(|_| Error::Unavailable)?,
            #[cfg(unix)]
            identity: [
                metadata.dev(),
                metadata.ino(),
                metadata.ctime() as u64,
                metadata.ctime_nsec() as u64,
            ],
        })
    }
}
pub(super) struct Extras {
    heads: Vec<String>,
    segments: BTreeMap<String, Segment>,
}
fn name(value: &str) -> Result<()> {
    if value.len() != 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(Error::Corrupt);
    }
    Ok(())
}
fn bytes(file: &mut fs::File, position: u64, out: &mut [u8]) -> Result<()> {
    file.seek(std::io::SeekFrom::Start(position))
        .map_err(|_| Error::Incomplete)?;
    file.read_exact(out).map_err(|_| Error::Incomplete)
}
impl Extras {
    pub fn open(metadata: &Path, budget: &Budget) -> Result<Self> {
        let root = metadata.join("store/extra");
        let mut heads = vec![];
        for entry in fs::read_dir(root.join("heads")).map_err(|_| Error::Incomplete)? {
            budget.work(1)?;
            if heads.len() >= 64 {
                return Err(Error::Limit);
            }
            let entry = entry.map_err(|_| Error::Incomplete)?;
            if !entry.file_type().map_err(|_| Error::Incomplete)?.is_file() {
                return Err(Error::Corrupt);
            }
            let id = entry
                .file_name()
                .into_string()
                .map_err(|_| Error::Corrupt)?;
            name(&id)?;
            heads.push(id);
        }
        let mut pending = heads.clone();
        let mut segments = BTreeMap::new();
        while let Some(id) = pending.pop() {
            budget.work(1)?;
            if segments.contains_key(&id) {
                continue;
            }
            if segments.len() >= 128 {
                return Err(Error::Limit);
            }
            let path = root.join(&id);
            let info = fs::symlink_metadata(&path).map_err(|_| Error::Incomplete)?;
            if !info.is_file() || info.len() > 2 * 1024 * 1024 * 1024 {
                return Err(Error::Limit);
            }
            let mut file = fs::File::open(path).map_err(|_| Error::Incomplete)?;
            let stamp = Stamp::read(&file)?;
            let mut digest = Blake2b512::new();
            let mut buffer = [0; 64 * 1024];
            let mut size = 0u64;
            loop {
                budget.work(1)?;
                let n = file.read(&mut buffer).map_err(|_| Error::Incomplete)?;
                if n == 0 {
                    break;
                }
                size += n as u64;
                if size > info.len() {
                    return Err(Error::Corrupt);
                }
                budget.read_bytes(n)?;
                digest.update(&buffer[..n]);
            }
            if size != info.len() || format!("{:x}", digest.finalize()) != id {
                return Err(Error::Corrupt);
            }
            if Stamp::read(&file)? != stamp {
                return Err(Error::Corrupt);
            }
            let mut word = [0; 4];
            bytes(&mut file, 0, &mut word)?;
            let parent_len = u32::from_le_bytes(word);
            let parent = match parent_len {
                0 => None,
                128 => {
                    let mut value = [0; 128];
                    bytes(&mut file, 4, &mut value)?;
                    let value = String::from_utf8(value.to_vec()).map_err(|_| Error::Corrupt)?;
                    name(&value)?;
                    pending.push(value.clone());
                    Some(value)
                }
                _ => return Err(Error::Corrupt),
            };
            bytes(&mut file, 4 + u64::from(parent_len), &mut word)?;
            let count = u32::from_le_bytes(word);
            let index = 8 + u64::from(parent_len);
            let values = index + u64::from(count) * 24;
            if values > size {
                return Err(Error::Corrupt);
            }
            segments.insert(
                id.clone(),
                Segment {
                    file: Mutex::new(file),
                    index,
                    values,
                    size,
                    count,
                    parent,
                    stamp,
                    digest: id.clone(),
                },
            );
        }
        Ok(Self { heads, segments })
    }
    pub fn change(&self, id: &[u8], budget: &Budget) -> Result<Option<b::ChangeId>> {
        Ok(self.record(id, budget)?.change)
    }
    pub fn record(&self, id: &[u8], budget: &Budget) -> Result<Record> {
        let mut result: Option<Vec<u8>> = None;
        for head in &self.heads {
            let mut next = Some(head);
            let mut visits = 0;
            while let Some(name) = next {
                budget.work(1)?;
                visits += 1;
                if visits > self.segments.len() {
                    return Err(Error::Corrupt);
                }
                let segment = self.segments.get(name).ok_or(Error::Corrupt)?;
                if let Some(value) = segment.get(id, budget)? {
                    if result.as_ref().is_some_and(|old| *old != value) {
                        return Err(Error::Corrupt);
                    }
                    result = Some(value);
                    break;
                }
                next = segment.parent.as_ref();
            }
        }
        result
            .map(|value| decode(&value))
            .transpose()
            .map(Option::unwrap_or_default)
    }
}
impl Segment {
    fn verify(&self, file: &mut fs::File, budget: &Budget) -> Result<()> {
        if Stamp::read(file)? != self.stamp {
            return Err(Error::Corrupt);
        }
        if !self.stamp.precise() {
            file.seek(std::io::SeekFrom::Start(0))
                .map_err(|_| Error::Incomplete)?;
            let mut digest = Blake2b512::new();
            let mut buffer = vec![0; 64 * 1024];
            let mut size = 0u64;
            loop {
                budget.work(1)?;
                let n = file.read(&mut buffer).map_err(|_| Error::Incomplete)?;
                if n == 0 {
                    break;
                }
                size += n as u64;
                if size > self.size {
                    return Err(Error::Corrupt);
                }
                budget.read_bytes(n)?;
                digest.update(&buffer[..n]);
            }
            if size != self.size
                || format!("{:x}", digest.finalize()) != self.digest
                || Stamp::read(file)? != self.stamp
            {
                return Err(Error::Corrupt);
            }
        }
        Ok(())
    }
    fn get(&self, id: &[u8], budget: &Budget) -> Result<Option<Vec<u8>>> {
        if id.len() != 20 {
            return Err(Error::Invalid);
        }
        let mut file = self.file.lock().unwrap();
        self.verify(&mut file, budget)?;
        let (mut lo, mut hi) = (0u32, self.count);
        while lo < hi {
            budget.work(1)?;
            let mid = lo + (hi - lo) / 2;
            let mut entry = [0; 24];
            budget.read_bytes(entry.len())?;
            bytes(&mut file, self.index + u64::from(mid) * 24, &mut entry)?;
            match id.cmp(&entry[..20]) {
                std::cmp::Ordering::Less => hi = mid,
                std::cmp::Ordering::Greater => lo = mid + 1,
                std::cmp::Ordering::Equal => {
                    let start = self.values
                        + u64::from(u32::from_le_bytes(entry[20..].try_into().unwrap()));
                    let end = if mid + 1 == self.count {
                        self.size
                    } else {
                        let mut offset = [0; 4];
                        budget.read_bytes(offset.len())?;
                        bytes(
                            &mut file,
                            self.index + u64::from(mid + 1) * 24 + 20,
                            &mut offset,
                        )?;
                        self.values + u64::from(u32::from_le_bytes(offset))
                    };
                    if start > end || end > self.size || end - start > 4 * 1024 * 1024 {
                        return Err(Error::Corrupt);
                    }
                    budget.read_bytes((end - start) as usize)?;
                    let mut value = vec![0; (end - start) as usize];
                    bytes(&mut file, start, &mut value)?;
                    self.verify(&mut file, budget)?;
                    return Ok(Some(value));
                }
            }
        }
        self.verify(&mut file, budget)?;
        Ok(None)
    }
}
fn varint(input: &mut &[u8]) -> Result<u64> {
    let mut value = 0;
    for shift in (0..70).step_by(7) {
        let (&byte, tail) = input.split_first().ok_or(Error::Corrupt)?;
        *input = tail;
        if shift == 63 && byte > 1 {
            return Err(Error::Corrupt);
        }
        value |= u64::from(byte & 127) << shift;
        if byte < 128 {
            return Ok(value);
        }
    }
    Err(Error::Corrupt)
}
#[derive(Default)]
pub(super) struct Record {
    pub change: Option<b::ChangeId>,
    pub predecessors: Vec<b::CommitId>,
    pub trees: Vec<b::TreeId>,
    pub tree_conflict_format: bool,
}
#[cfg(test)]
fn decode_change(input: &[u8]) -> Result<Option<b::ChangeId>> {
    Ok(decode(input)?.change)
}
fn decode(mut input: &[u8]) -> Result<Record> {
    let mut result = Record::default();
    while !input.is_empty() {
        let tag = varint(&mut input)?;
        let field = tag >> 3;
        let wire = tag & 7;
        if field == 0
            || (matches!(field, 1 | 2 | 4) && wire != 2)
            || (matches!(field, 8 | 9 | 10) && wire != 0)
        {
            return Err(Error::Corrupt);
        }
        let mut value = 0;
        let n = match wire {
            0 => {
                value = varint(&mut input)?;
                0
            }
            1 => 8,
            2 => usize::try_from(varint(&mut input)?).map_err(|_| Error::Corrupt)?,
            5 => 4,
            _ => return Err(Error::Corrupt),
        };
        if n > input.len() {
            return Err(Error::Corrupt);
        }
        match field {
            1 | 2 => {
                if n != 20 {
                    return Err(Error::Corrupt);
                }
                if field == 1 {
                    if result.trees.len() >= 65 {
                        return Err(Error::Limit);
                    }
                    result.trees.push(b::TreeId::from_bytes(&input[..n]));
                } else {
                    if result.predecessors.len() >= 256 {
                        return Err(Error::Limit);
                    }
                    result
                        .predecessors
                        .push(b::CommitId::from_bytes(&input[..n]));
                }
            }
            4 => {
                if n != 0 && n != 16 {
                    return Err(Error::Corrupt);
                }
                result.change = (n != 0).then(|| b::ChangeId::from_bytes(&input[..n]));
            }
            10 => result.tree_conflict_format = value != 0,
            _ => {}
        }
        input = &input[n..];
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_change_lookup_preserves_protobuf_replacement_and_legacy_fields() {
        let mut encoded = vec![18, 20]; // legacy predecessor
        encoded.extend([3; 20]);
        encoded.extend([34, 16]);
        encoded.extend([7; 16]);
        encoded.extend([80, 1]); // tree-conflict format
        assert_eq!(
            decode_change(&encoded).unwrap().unwrap().as_bytes(),
            &[7; 16]
        );
        encoded.extend([34, 0]);
        assert_eq!(decode_change(&encoded).unwrap(), None);
        encoded.extend([34, 16]);
        encoded.extend([9; 16]);
        assert_eq!(
            decode_change(&encoded).unwrap().unwrap().as_bytes(),
            &[9; 16]
        );
    }

    #[test]
    fn malformed_extra_framing_never_becomes_a_synthetic_change() {
        for input in [
            vec![0],
            vec![34, 16, 1],
            vec![32, 1],
            vec![34, 1, 7],
            vec![255; 10],
            vec![34, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2],
        ] {
            assert!(
                matches!(decode_change(&input), Err(Error::Corrupt)),
                "{input:?}"
            );
        }
    }
}
