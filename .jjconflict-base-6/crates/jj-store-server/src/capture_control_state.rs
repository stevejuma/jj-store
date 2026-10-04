//! Installation-private policy selection; no runtime or repository authority.
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
use std::path::Path;
mod catalog;
mod pinned;
use crate::capture_journal_files as files;
use catalog::Catalog;
use files::Root;
pub(crate) use pinned::CaptureControlPinned;
pub const MAX_RECEIPTS: usize = 4096;
pub const MAX_BYTES: usize = 1024 * 1024;
const LAYOUT: files::Layout = files::Layout {
    lock: "control.lock",
    temporary: ".capture-control-",
    max_bytes: MAX_BYTES,
};
impl From<files::Error> for Error {
    fn from(error: files::Error) -> Self {
        match error {
            files::Error::Unavailable => Self::Unavailable,
            files::Error::Capacity => Self::Capacity,
            files::Error::OutcomeUnknown => Self::OutcomeUnknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid capture control selection")]
    Invalid,
    #[error("capture control state unavailable")]
    Unavailable,
    #[error("capture control request identity conflicts")]
    Conflict,
    #[error("capture control revision changed")]
    StaleRevision,
    #[error("capture control generation changed")]
    ForeignGeneration,
    #[error("capture control receipt capacity exhausted")]
    Capacity,
    #[error("capture control selection outcome unknown")]
    OutcomeUnknown,
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureControlSnapshot {
    pub generation: [u8; 16],
    pub revision: u64,
    pub paused: bool,
}
/// The caller supplies the authenticated actor; this is not an inbound DTO.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureControlChange {
    pub generation: [u8; 16],
    pub principal: PrincipalId,
    pub request: RequestId,
    pub expected_revision: u64,
    pub paused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureControlReceipt {
    pub principal: PrincipalId,
    pub request: RequestId,
    pub expected_revision: u64,
    pub revision: u64,
    pub paused: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureControlSelection {
    pub current: CaptureControlSnapshot,
    pub receipt: CaptureControlReceipt,
    pub replayed: bool,
}
/// Holds a private physical binding, not a runtime task, lease or credential.
#[derive(Clone)]
pub struct CaptureControlJournal {
    root: Root,
    installation: [u8; 16],
}
impl CaptureControlJournal {
    pub fn create(path: &Path, installation: [u8; 16]) -> Result<Self> {
        if installation == [0; 16] {
            return Err(Error::Invalid);
        }
        let mut generation = [0; 16];
        getrandom::fill(&mut generation).map_err(|_| Error::Unavailable)?;
        if generation == [0; 16] {
            return Err(Error::Unavailable);
        }
        let root = Root::create(path, LAYOUT)?;
        let gate = root.gate()?;
        let catalog = Catalog::new(installation, generation);
        root.persist(&gate, &catalog.encode()?, None)?;
        drop(gate);
        Ok(Self { root, installation })
    }
    /// Missing state is unavailable, including a missing directory; never initializes.
    pub fn open(path: &Path, installation: [u8; 16]) -> Result<Self> {
        if installation == [0; 16] {
            return Err(Error::Invalid);
        }
        let journal = Self {
            root: Root::open(path, LAYOUT)?,
            installation,
        };
        journal.current()?;
        Ok(journal)
    }
    pub fn current(&self) -> Result<CaptureControlSnapshot> {
        self.read_then(|selected, _| selected)
    }
    /// Retains the physical gate across an authenticated transient operation.
    pub(crate) fn read_then<T>(
        &self,
        apply: impl FnOnce(CaptureControlSnapshot, &dyn Fn() -> Result<()>) -> T,
    ) -> Result<T> {
        let gate = self.root.gate()?;
        let (bytes, _) = self.root.read(&gate)?;
        let selected = Catalog::decode(&bytes, self.installation)?.snapshot()?;
        let check = || {
            let (bytes, _) = self.root.read(&gate)?;
            if Catalog::decode(&bytes, self.installation)?.snapshot()? != selected {
                return Err(Error::Unavailable);
            }
            Ok(())
        };
        Ok(apply(selected, &check))
    }
    /// Import admission pins the selected control file as well as its policy snapshot.
    pub(crate) fn read_bound_then<T>(
        &self,
        apply: impl FnOnce(CaptureControlSnapshot, &dyn Fn() -> Result<()>) -> T,
    ) -> Result<T> {
        let pinned = self.pin_selected()?;
        Ok(apply(pinned.selected(), &|| pinned.check()))
    }
    /// Selects device policy only. Authenticate and apply its latest state separately.
    pub fn record(&self, change: &CaptureControlChange) -> Result<CaptureControlSelection> {
        self.record_then(change, |_, _| ())
            .map(|(selected, ())| selected)
    }
    /// The bounded server callback retains coordination through current-policy application.
    pub(crate) fn record_then<T>(
        &self,
        change: &CaptureControlChange,
        apply: impl FnOnce(&CaptureControlSelection, &dyn Fn() -> Result<()>) -> T,
    ) -> Result<(CaptureControlSelection, T)> {
        if change.principal.as_bytes() == &[0; 16] || change.request.as_bytes() == &[0; 16] {
            return Err(Error::Invalid);
        }
        let gate = self.root.gate()?;
        let (bytes, stamp) = self.root.read(&gate)?;
        let mut catalog = Catalog::decode(&bytes, self.installation)?;
        if catalog.snapshot()?.generation != change.generation {
            return Err(Error::ForeignGeneration);
        }
        if let Some(receipt) = catalog
            .receipts
            .iter()
            .find(|r| r.request == change.request)
        {
            if receipt.principal != change.principal
                || receipt.expected_revision != change.expected_revision
                || receipt.paused != change.paused
            {
                return Err(Error::Conflict);
            }
            self.root.sync_selected(&gate, stamp)?;
            let selected = CaptureControlSelection {
                current: catalog.snapshot()?,
                receipt: *receipt,
                replayed: true,
            };
            return self.apply_selected(&gate, selected, apply);
        }
        if change.expected_revision != catalog.revision {
            return Err(Error::StaleRevision);
        }
        if catalog.receipts.len() == MAX_RECEIPTS {
            return Err(Error::Capacity);
        }
        let receipt = CaptureControlReceipt {
            principal: change.principal,
            request: change.request,
            expected_revision: change.expected_revision,
            revision: catalog.revision + 1,
            paused: change.paused,
        };
        catalog.revision = receipt.revision;
        catalog.paused = receipt.paused;
        catalog.receipts.push(receipt);
        self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
        let selected = CaptureControlSelection {
            current: catalog.snapshot()?,
            receipt,
            replayed: false,
        };
        self.apply_selected(&gate, selected, apply)
    }
    fn apply_selected<T>(
        &self,
        gate: &files::Gate,
        selected: CaptureControlSelection,
        apply: impl FnOnce(&CaptureControlSelection, &dyn Fn() -> Result<()>) -> T,
    ) -> Result<(CaptureControlSelection, T)> {
        let check = || {
            let (bytes, _) = self.root.read(gate)?;
            let current = Catalog::decode(&bytes, self.installation)?;
            if current.snapshot()? != selected.current {
                return Err(Error::Unavailable);
            }
            Ok(())
        };
        let applied = apply(&selected, &check);
        Ok((selected, applied))
    }
}

#[cfg(all(test, unix))]
mod tests;
