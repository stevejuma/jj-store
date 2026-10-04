//! Deterministic test storage; never use its results as RustFS conformance.
use crate::*;
use std::{
    collections::{BTreeMap, VecDeque},
    sync::Mutex,
};

#[derive(Clone, Copy, Debug)]
pub enum Fault {
    BeforeWrite,
    AfterWrite,
    ReadCorrupt,
    ReadMissing,
}
#[derive(Default)]
struct State {
    entries: BTreeMap<Key, (Vec<u8>, VersionToken)>,
    next: u64,
    faults: VecDeque<Fault>,
}
#[derive(Default)]
pub struct MemoryStore {
    state: Mutex<State>,
}
impl MemoryStore {
    pub fn inject(&self, fault: Fault) {
        self.state.lock().unwrap().faults.push_back(fault);
    }
}
#[async_trait]
impl ObjectStore for MemoryStore {
    async fn head(&self, key: &Key) -> Result<Metadata> {
        let state = self.state.lock().unwrap();
        let (bytes, version) = state.entries.get(key).ok_or(Error::NotFound)?;
        Ok(Metadata {
            len: bytes.len() as u64,
            version: version.clone(),
        })
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download> {
        let mut state = self.state.lock().unwrap();
        let fault = match state.faults.front() {
            Some(Fault::ReadCorrupt | Fault::ReadMissing) => state.faults.pop_front(),
            _ => None,
        };
        if matches!(fault, Some(Fault::ReadMissing)) {
            return Err(Error::NotFound);
        }
        let (bytes, version) = state.entries.get(key).ok_or(Error::NotFound)?;
        let mut bytes = if let Some(r) = range {
            bytes
                .get(
                    usize::try_from(r.start).map_err(|_| Error::Range)?
                        ..usize::try_from(r.end).map_err(|_| Error::Range)?,
                )
                .ok_or(Error::Range)?
                .to_vec()
        } else {
            bytes.clone()
        };
        if matches!(fault, Some(Fault::ReadCorrupt)) && !bytes.is_empty() {
            bytes[0] ^= 1;
        }
        Ok(Download {
            metadata: Metadata {
                len: bytes.len() as u64,
                version: version.clone(),
            },
            reader: Box::pin(std::io::Cursor::new(bytes)),
        })
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> Result<WriteOutcome> {
        let mut data = Vec::new();
        upload.reader()?.read_to_end(&mut data).await?;
        let mut state = self.state.lock().unwrap();
        let fault = match state.faults.front() {
            Some(Fault::BeforeWrite | Fault::AfterWrite) => state.faults.pop_front(),
            _ => None,
        };
        if matches!(fault, Some(Fault::BeforeWrite)) {
            return Ok(WriteOutcome::Unknown);
        }
        let matches = match condition {
            Condition::Absent => !state.entries.contains_key(key),
            Condition::Matches(token) => state
                .entries
                .get(key)
                .is_some_and(|(_, current)| *current == token),
        };
        if !matches {
            return Ok(WriteOutcome::PreconditionFailed);
        }
        state.next += 1;
        let version = VersionToken::new(format!("fake-{}", state.next))?;
        state.entries.insert(key.clone(), (data, version.clone()));
        if matches!(fault, Some(Fault::AfterWrite)) {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(WriteOutcome::Applied(version))
        }
    }
}
