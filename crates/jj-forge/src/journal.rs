//! Private device receipts: external intent only, never repository authority.
use crate::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    #[serde(default = "version")]
    version: u32,
    connection: Option<Connection>,
    receipts: Vec<Receipt>,
    operations: Vec<Operation>,
    #[serde(default)]
    restacks: Vec<RestackReceipt>,
}
fn version() -> u32 {
    1
}
impl Default for State {
    fn default() -> Self {
        Self {
            version: 1,
            connection: None,
            receipts: vec![],
            operations: vec![],
            restacks: vec![],
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "ForgeOperation")]
pub struct Operation {
    pub id: String,
    pub number: u64,
    pub action: PrAction,
    pub outcome: Outcome,
    pub merge: Option<MergeResult>,
}
pub struct Device {
    path: PathBuf,
    _lock: File,
    state: State,
}
impl Device {
    pub fn open(metadata: &Path) -> Result<Self> {
        let root = std::env::var_os("JJ_STORE_FORGE_STATE_DIR")
            .map(PathBuf::from)
            .or_else(|| dirs::data_local_dir().map(|d| d.join("jj-store/forges")))
            .ok_or(Error::State)?;
        Self::at(&root, metadata)
    }
    pub fn at(root: &Path, metadata: &Path) -> Result<Self> {
        if root
            .ancestors()
            .any(|p| p.join(".jj").exists() || p.join(".git").exists())
        {
            return Err(Error::Invalid);
        }
        fs::create_dir_all(root).map_err(|_| Error::State)?;
        let root = root.canonicalize().map_err(|_| Error::State)?;
        if root
            .ancestors()
            .any(|p| p.join(".jj").exists() || p.join(".git").exists())
        {
            return Err(Error::Invalid);
        }
        let key = hex::encode(Sha256::digest(metadata.as_os_str().as_encoded_bytes()));
        let path = root.join(format!("{key}.json"));
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.with_extension("lock"))
            .map_err(|_| Error::State)?;
        lock.try_lock().map_err(|_| Error::State)?;
        let state: State = match fs::read(&path) {
            Ok(b) if b.len() <= 8 * 1024 * 1024 => {
                serde_json::from_slice(&b).map_err(|_| Error::State)?
            }
            Ok(_) => return Err(Error::Limit),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => State::default(),
            Err(_) => return Err(Error::State),
        };
        if state.version != 1 {
            return Err(Error::State);
        }
        Ok(Self {
            path,
            _lock: lock,
            state,
        })
    }
    pub fn connection(&self) -> Option<Connection> {
        self.state.connection.clone()
    }
    pub fn configure(&mut self, c: Connection) -> Result<()> {
        c.validate()?;
        if self.state.connection.as_ref().is_some_and(|old| old != &c)
            && (!self.state.receipts.is_empty()
                || !self.state.operations.is_empty()
                || !self.state.restacks.is_empty())
        {
            return Err(Error::Stale);
        }
        self.state.connection = Some(c);
        self.save()
    }
    fn save(&self) -> Result<()> {
        let bytes = serde_json::to_vec(&self.state).map_err(|_| Error::State)?;
        if bytes.len() > 8 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let root = self.path.parent().ok_or(Error::State)?;
        let mut temp = tempfile::NamedTempFile::new_in(root).map_err(|_| Error::State)?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| Error::State)?;
        temp.persist(&self.path).map_err(|_| Error::State)?;
        File::open(root)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unknown)
    }
    pub fn receipt(&self, id: &str) -> Option<Receipt> {
        self.state
            .receipts
            .iter()
            .find(|r| r.plan.request_id == id)
            .cloned()
    }
    pub fn receipts(&self) -> Vec<Receipt> {
        self.state.receipts.clone()
    }
    pub fn pending_restack(&self) -> Option<RestackReceipt> {
        self.state
            .restacks
            .iter()
            .rev()
            .find(|r| r.operation.is_none())
            .cloned()
    }
    pub fn put(&mut self, r: Receipt) -> Result<()> {
        if self.connection().as_ref() != Some(&r.plan.connection) {
            return Err(Error::Stale);
        }
        if let Some(old) = self
            .state
            .receipts
            .iter_mut()
            .find(|v| v.plan.request_id == r.plan.request_id)
        {
            if serde_json::to_vec(&old.plan).ok() != serde_json::to_vec(&r.plan).ok() {
                return Err(Error::Stale);
            }
            *old = r;
        } else {
            if self.state.receipts.len() >= 128 {
                return Err(Error::Limit);
            }
            if self
                .state
                .receipts
                .iter()
                .any(|v| v.outcome != Outcome::Complete && v.outcome != Outcome::Rejected)
            {
                return Err(Error::Unknown);
            }
            self.state.receipts.push(r);
        }
        self.save()
    }
    pub fn begin_operation(
        &mut self,
        id: &str,
        number: u64,
        action: &PrAction,
    ) -> Result<Option<Operation>> {
        if !safe_name(id) {
            return Err(Error::Invalid);
        }
        if let Some(old) = self.state.operations.iter().find(|o| o.id == id) {
            if old.number != number
                || serde_json::to_vec(&old.action).ok() != serde_json::to_vec(action).ok()
            {
                return Err(Error::Stale);
            }
            return Ok(Some(old.clone()));
        }
        if self.unresolved_operation(Some(number)).is_some() {
            return Err(Error::Unknown);
        }
        if self.state.operations.len() >= 1024 {
            return Err(Error::Limit);
        }
        self.state.operations.push(Operation {
            id: id.into(),
            number,
            action: action.clone(),
            outcome: Outcome::Running,
            merge: None,
        });
        self.save()?;
        Ok(None)
    }
    pub fn restack(&self, id: &str) -> Option<RestackReceipt> {
        self.state
            .restacks
            .iter()
            .find(|r| r.plan.request_id == id)
            .cloned()
    }
    pub fn put_restack(&mut self, r: RestackReceipt) -> Result<()> {
        if self.connection().as_ref() != Some(&r.plan.connection) {
            return Err(Error::Stale);
        }
        if let Some(old) = self
            .state
            .restacks
            .iter_mut()
            .find(|v| v.plan.request_id == r.plan.request_id)
        {
            if serde_json::to_vec(&old.plan).ok() != serde_json::to_vec(&r.plan).ok() {
                return Err(Error::Stale);
            }
            *old = r;
        } else {
            if self.state.restacks.len() >= 128 {
                return Err(Error::Limit);
            }
            self.state.restacks.push(r);
        }
        self.save()
    }
    pub fn unresolved_operation(&self, number: Option<u64>) -> Option<Operation> {
        self.state
            .operations
            .iter()
            .rev()
            .find(|o| {
                number.is_none_or(|n| n == o.number)
                    && matches!(o.outcome, Outcome::Running | Outcome::Unknown)
            })
            .cloned()
    }
    pub fn restart_rejected(&mut self, id: &str) -> Result<()> {
        let op = self
            .state
            .operations
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or(Error::State)?;
        if op.outcome != Outcome::Rejected {
            return Err(Error::Unknown);
        }
        op.outcome = Outcome::Running;
        self.save()
    }
    pub fn merge_results(&self) -> std::collections::BTreeMap<u64, MergeResult> {
        self.state
            .operations
            .iter()
            .filter_map(|op| op.merge.clone().map(|m| (op.number, m)))
            .collect()
    }
    pub fn record_merge_status(
        &mut self,
        number: u64,
        receipt: &str,
        result: MergeResult,
    ) -> Result<()> {
        if let Some(op) = self.state.operations.iter_mut().rev().find(|op| {
            op.number == number
                && op.merge.as_ref().and_then(|m| m.receipt.as_deref()) == Some(receipt)
        }) {
            op.merge = Some(result);
            self.save()?;
        }
        Ok(())
    }
    pub fn operation(&self, id: &str) -> Option<Operation> {
        self.state.operations.iter().find(|o| o.id == id).cloned()
    }
    pub fn finish_operation(
        &mut self,
        id: &str,
        result: Result<Option<MergeResult>>,
    ) -> Result<Option<MergeResult>> {
        let op = self
            .state
            .operations
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or(Error::State)?;
        match result {
            Ok(v) => {
                op.outcome = Outcome::Complete;
                op.merge = v.clone();
                self.save()?;
                Ok(v)
            }
            Err(e) => {
                op.outcome = if matches!(e, Error::Unknown) {
                    Outcome::Unknown
                } else {
                    Outcome::Rejected
                };
                self.save()?;
                Err(e)
            }
        }
    }
}
