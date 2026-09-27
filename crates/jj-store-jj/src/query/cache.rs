//! Disposable bounded cache slots. Only verified native objects are authority.
use super::*;
use blake2::{Blake2b512, Digest as _};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read as _, Write as _},
    path::Path,
};

const RECEIPT: &str = "query-cache.json";
const RECEIPT_LIMIT: u64 = 1024 * 1024;
const REVISION: &str = "7c41cdeb16b6b321c64e789a966b6adf723816a5";

#[derive(Clone, Copy, Debug)]
pub struct Policy {
    pub max_entries: usize,
    pub max_entry_bytes: u64,
    pub max_total_bytes: u64,
    pub max_files: usize,
}
impl Default for Policy {
    fn default() -> Self {
        Self {
            max_entries: 16,
            max_entry_bytes: 64 * 1024 * 1024,
            max_total_bytes: 256 * 1024 * 1024,
            max_files: 4096,
        }
    }
}
impl Policy {
    pub(super) fn validate(self) -> Result<()> {
        if !(1..=64).contains(&self.max_entries)
            || !(1..=4096).contains(&self.max_files)
            || self.max_entry_bytes == 0
            || self.max_entry_bytes > self.max_total_bytes
            || self.max_total_bytes > 16 * 1024 * 1024 * 1024
        {
            return Err(Error::Limit);
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "CacheState")]
#[serde(rename_all = "snake_case")]
pub enum State {
    Built,
    Reused,
    Rebuilt,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "CacheReport")]
pub struct Report {
    pub state: State,
    pub evicted: usize,
    pub retained_entries: usize,
    pub retained_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    bytes: u64,
    digest: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    version: u32,
    repository: m::RepositoryId,
    epoch: m::EpochId,
    operation: m::OperationId,
    used: u64,
    files: BTreeMap<String, File>,
}
struct Slot {
    id: usize,
    receipt: Receipt,
    bytes: u64,
}
pub(super) struct Cache {
    root: PathBuf,
    _lock: fs::File,
    policy: Policy,
    slots: Vec<Slot>,
    evicted: usize,
    damaged: bool,
}
pub(super) struct Entry {
    pub path: PathBuf,
    pub initialize: bool,
    slot: usize,
    state: State,
    staging: Option<tempfile::TempDir>,
    retain: bool,
}
impl Drop for Entry {
    fn drop(&mut self) {
        if !self.retain && self.staging.is_none() {
            let _ = remove(&self.path);
        }
    }
}
fn plain_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(|_| Error::Unavailable)?;
    if !fs::symlink_metadata(path)
        .map_err(|_| Error::Unavailable)?
        .file_type()
        .is_dir()
    {
        return Err(Error::Unavailable);
    }
    Ok(())
}
fn remove(path: &Path) -> Result<()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(Error::Unavailable),
    };
    if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
    .map_err(|_| Error::Unavailable)
}
fn receipt(path: &Path, policy: Policy) -> Result<Receipt> {
    let path = path.join(RECEIPT);
    let metadata = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
    if !metadata.is_file() || metadata.len() > RECEIPT_LIMIT {
        return Err(Error::Unavailable);
    }
    let value: Receipt = serde_json::from_slice(&fs::read(path).map_err(|_| Error::Unavailable)?)
        .map_err(|_| Error::Unavailable)?;
    if value.version != 1 || value.files.len() > policy.max_files {
        return Err(Error::Unavailable);
    }
    Ok(value)
}
/// Scan a bounded tree without following links, hashing only when requested.
fn inventory(
    root: &Path,
    policy: Policy,
    budget: &Budget,
    hash: bool,
) -> Result<(BTreeMap<String, File>, u64)> {
    if !fs::symlink_metadata(root)
        .map_err(|_| Error::Unavailable)?
        .is_dir()
    {
        return Err(Error::Unavailable);
    }
    let mut pending = vec![(root.to_owned(), 0)];
    let mut count = 0usize;
    let mut total = 0u64;
    let mut files = BTreeMap::new();
    while let Some((path, depth)) = pending.pop() {
        for entry in fs::read_dir(path).map_err(|_| Error::Unavailable)? {
            budget.work(1)?;
            count += 1;
            if count > policy.max_files || depth > 16 {
                return Err(Error::Limit);
            }
            let entry = entry.map_err(|_| Error::Unavailable)?;
            let path = entry.path();
            let kind = entry.file_type().map_err(|_| Error::Unavailable)?;
            if kind.is_dir() {
                pending.push((path, depth + 1));
            } else if kind.is_file() {
                let bytes = entry.metadata().map_err(|_| Error::Unavailable)?.len();
                total = total.checked_add(bytes).ok_or(Error::Limit)?;
                if total > policy.max_entry_bytes {
                    return Err(Error::Limit);
                }
                let name = path
                    .strip_prefix(root)
                    .map_err(|_| Error::Unavailable)?
                    .to_str()
                    .ok_or(Error::Unavailable)?
                    .to_owned();
                if name == RECEIPT {
                    continue;
                }
                let digest = if hash {
                    let mut input = fs::File::open(&path).map_err(|_| Error::Unavailable)?;
                    let mut buffer = [0; 64 * 1024];
                    let mut digest = Blake2b512::new();
                    let mut read = 0u64;
                    loop {
                        budget.work(1)?;
                        let n = input.read(&mut buffer).map_err(|_| Error::Unavailable)?;
                        if n == 0 {
                            break;
                        }
                        read = read.checked_add(n as u64).ok_or(Error::Limit)?;
                        if read > bytes {
                            return Err(Error::Unavailable);
                        }
                        digest.update(&buffer[..n]);
                    }
                    if read != bytes {
                        return Err(Error::Unavailable);
                    }
                    format!("{:x}", digest.finalize())
                } else {
                    String::new()
                };
                files.insert(name, File { bytes, digest });
            } else {
                return Err(Error::Unavailable);
            }
        }
    }
    Ok((files, total))
}
impl Cache {
    pub fn open(root: &Path, policy: Policy, budget: &Budget) -> Result<Self> {
        let mut root = root.to_owned();
        plain_dir(&root)?;
        for component in [REVISION, PROFILE, "query-cache-v1"] {
            root.push(component);
            plain_dir(&root)?;
        }
        let lock_path = root.join("lock");
        if fs::symlink_metadata(&lock_path).is_ok_and(|m| !m.is_file()) {
            return Err(Error::Unavailable);
        }
        let lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|_| Error::Unavailable)?;
        lock.try_lock().map_err(|_| Error::Busy)?;
        let mut cache = Self {
            root,
            _lock: lock,
            policy,
            slots: vec![],
            evicted: 0,
            damaged: false,
        };
        let entries = fs::read_dir(&cache.root).map_err(|_| Error::Unavailable)?;
        for (count, entry) in entries.enumerate() {
            budget.work(1)?;
            if count >= 256 {
                return Err(Error::Limit);
            }
            let entry = entry.map_err(|_| Error::Unavailable)?;
            let name = entry.file_name();
            let name = name.to_str().ok_or(Error::Unavailable)?;
            if name == "lock" {
                continue;
            }
            if name.starts_with(".building-") {
                remove(&entry.path())?;
                cache.damaged = true;
                continue;
            }
            let id: usize = name
                .strip_prefix("slot-")
                .and_then(|s| s.parse().ok())
                .filter(|id| *id < 64 && name == format!("slot-{id}"))
                .ok_or(Error::Unavailable)?;
            let retained = (|| {
                if !entry.file_type().map_err(|_| Error::Unavailable)?.is_dir() {
                    return Err(Error::Unavailable);
                }
                let value = receipt(&entry.path(), policy)?;
                let (_, bytes) = inventory(&entry.path(), policy, budget, false)?;
                Ok::<_, Error>(Slot {
                    id,
                    receipt: value,
                    bytes,
                })
            })();
            match retained {
                Ok(slot) if id < policy.max_entries => cache.slots.push(slot),
                _ => {
                    // Deadline/work failures cannot be converted into recovery success.
                    budget.check()?;
                    remove(&entry.path())?;
                    cache.evicted += 1;
                    cache.damaged = true;
                }
            }
        }
        cache.slots.sort_by_key(|slot| (slot.receipt.used, slot.id));
        while cache.bytes() > policy.max_total_bytes {
            cache.evict()?;
        }
        Ok(cache)
    }
    fn path(&self, id: usize) -> PathBuf {
        self.root.join(format!("slot-{id}"))
    }
    fn bytes(&self) -> u64 {
        self.slots.iter().map(|slot| slot.bytes).sum()
    }
    fn evict(&mut self) -> Result<()> {
        if self.slots.is_empty() {
            return Err(Error::Limit);
        }
        let slot = self.slots.remove(0);
        remove(&self.path(slot.id))?;
        self.evicted += 1;
        Ok(())
    }
    pub fn prepare(
        &mut self,
        access: &dyn ReadAccess,
        stores: &NativeStores,
        budget: &Budget,
    ) -> Result<Entry> {
        if let Some(index) = self.slots.iter().position(|slot| {
            slot.receipt.repository == access.repository()
                && slot.receipt.epoch == access.epoch()
                && slot.receipt.operation == access.operation()
        }) {
            let slot = self.slots.remove(index);
            let path = self.path(slot.id);
            // Check native bindings before touching the untrusted derived index.
            stores
                .verify_cache_binding(&path)
                .map_err(|_| Error::Unavailable)?;
            let valid = inventory(&path, self.policy, budget, true)
                .is_ok_and(|(files, _)| files == slot.receipt.files);
            budget.check()?;
            if valid {
                return Ok(Entry {
                    path,
                    initialize: false,
                    slot: slot.id,
                    state: State::Reused,
                    staging: None,
                    retain: false,
                });
            }
            remove(&path)?;
            self.damaged = true;
        }
        while self.slots.len() >= self.policy.max_entries
            || self.bytes() + self.policy.max_entry_bytes > self.policy.max_total_bytes
        {
            self.evict()?;
        }
        let slot = (0..self.policy.max_entries)
            .find(|id| !self.slots.iter().any(|s| s.id == *id))
            .ok_or(Error::Limit)?;
        let staging = tempfile::Builder::new()
            .prefix(".building-")
            .tempdir_in(&self.root)
            .map_err(|_| Error::Unavailable)?;
        Ok(Entry {
            path: staging.path().to_owned(),
            initialize: true,
            slot,
            state: if self.damaged {
                State::Rebuilt
            } else {
                State::Built
            },
            staging: Some(staging),
            retain: false,
        })
    }
    pub fn finish(
        &mut self,
        mut entry: Entry,
        access: &dyn ReadAccess,
        budget: &Budget,
    ) -> Result<Report> {
        let (files, _) = inventory(&entry.path, self.policy, budget, true)?;
        let used = self
            .slots
            .iter()
            .map(|s| s.receipt.used)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or(Error::Limit)?;
        let receipt = Receipt {
            version: 1,
            repository: access.repository(),
            epoch: access.epoch(),
            operation: access.operation(),
            used,
            files,
        };
        let bytes = serde_json::to_vec(&receipt).map_err(|_| Error::Unavailable)?;
        if bytes.len() as u64 > RECEIPT_LIMIT {
            return Err(Error::Limit);
        }
        // Publish the complete local receipt; an interrupted temp file makes the
        // next checksum inventory rebuild rather than accepting partial state.
        let mut receipt_file =
            tempfile::NamedTempFile::new_in(&entry.path).map_err(|_| Error::Unavailable)?;
        receipt_file
            .write_all(&bytes)
            .map_err(|_| Error::Unavailable)?;
        receipt_file
            .as_file()
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        receipt_file
            .persist(entry.path.join(RECEIPT))
            .map_err(|_| Error::Unavailable)?;
        let (_, size) = inventory(&entry.path, self.policy, budget, false)?;
        while self.bytes() + size > self.policy.max_total_bytes {
            self.evict()?;
        }
        if entry.staging.is_some() {
            let path = self.path(entry.slot);
            fs::rename(&entry.path, &path).map_err(|_| Error::Unavailable)?;
            entry.path = path;
        }
        self.slots.push(Slot {
            id: entry.slot,
            receipt,
            bytes: size,
        });
        entry.retain = true;
        Ok(Report {
            state: entry.state,
            evicted: self.evicted,
            retained_entries: self.slots.len(),
            retained_bytes: self.bytes(),
        })
    }
}
