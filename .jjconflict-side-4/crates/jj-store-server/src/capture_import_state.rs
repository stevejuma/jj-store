//! Durable device intent only. Fresh admission, scheduling and publication stay separate.
use crate::capture_journal_files::{self as files, Root};
use jj_store_model::{PrincipalId, RequestId};
use serde::{Deserialize, Serialize};
use std::path::Path;
mod catalog;
mod hex_codec;
mod reader;
mod selected;
mod types;
use catalog::Catalog;
pub use reader::CaptureImportReaderBinding;
pub(crate) use selected::CaptureImportSelected;
pub use types::*;
pub const MAX_RECEIPTS: usize = 4096;
pub const MAX_BYTES: usize = 2 * 1024 * 1024;
const LAYOUT: files::Layout = files::Layout {
    lock: "import.lock",
    temporary: ".capture-import-",
    max_bytes: MAX_BYTES,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid capture import intent")]
    Invalid,
    #[error("capture import state unavailable")]
    Unavailable,
    #[error("capture import request identity conflicts")]
    Conflict,
    #[error("capture import control generation changed")]
    ForeignGeneration,
    #[error("capture import intent capacity exhausted")]
    Capacity,
    #[error("capture import selection outcome unknown")]
    OutcomeUnknown,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<files::Error> for Error {
    fn from(error: files::Error) -> Self {
        match error {
            files::Error::Unavailable => Self::Unavailable,
            files::Error::Capacity => Self::Capacity,
            files::Error::OutcomeUnknown => Self::OutcomeUnknown,
        }
    }
}
/// Private file binding only: no source lease, directory proof or task owner.
#[derive(Clone)]
pub struct CaptureImportJournal {
    root: Root,
    installation: [u8; 16],
    generation: [u8; 16],
}
impl CaptureImportJournal {
    pub(crate) fn belongs_to(
        &self,
        path: &Path,
        installation: [u8; 16],
        generation: [u8; 16],
    ) -> bool {
        self.root.at(path) && self.installation == installation && self.generation == generation
    }
    /// Explicit initialization only. The caller separately verifies installation/control state.
    pub fn create(path: &Path, installation: [u8; 16], generation: [u8; 16]) -> Result<Self> {
        if installation == [0; 16] || generation == [0; 16] {
            return Err(Error::Invalid);
        }
        let root = Root::create(path, LAYOUT)?;
        let gate = root.gate()?;
        root.persist(
            &gate,
            &Catalog::new(installation, generation).encode()?,
            None,
        )?;
        drop(gate);
        Ok(Self {
            root,
            installation,
            generation,
        })
    }
    /// Never initialize, repair permissions, discard intent or regenerate state on open.
    pub fn open(path: &Path, installation: [u8; 16], generation: [u8; 16]) -> Result<Self> {
        if installation == [0; 16] || generation == [0; 16] {
            return Err(Error::Invalid);
        }
        let journal = Self {
            root: Root::open(path, LAYOUT)?,
            installation,
            generation,
        };
        journal.entries()?;
        Ok(journal)
    }
    fn decode(&self, bytes: &[u8]) -> Result<Catalog> {
        Catalog::decode(bytes, self.installation, self.generation)
    }
    fn validate(&self, intent: &CaptureImportIntent) -> Result<()> {
        intent.validate()?;
        if intent.generation != self.generation {
            return Err(Error::ForeignGeneration);
        }
        Ok(())
    }
    /// Bounded body-free metadata; caller admission remains independent.
    pub fn entries(&self) -> Result<Vec<CaptureImportReceipt>> {
        let gate = self.root.gate()?;
        let (bytes, _) = self.root.read(&gate)?;
        Ok(self.decode(&bytes)?.receipts)
    }
    /// Resolve exact original input before interpreting an ephemeral selector anew.
    pub fn resolve(&self, intent: &CaptureImportIntent) -> Result<Option<CaptureImportReceipt>> {
        self.validate(intent)?;
        let gate = self.root.gate()?;
        let (bytes, stamp) = self.root.read(&gate)?;
        let catalog = self.decode(&bytes)?;
        let receipt = catalog.find(intent)?.cloned();
        if receipt.is_some() {
            self.root.sync_selected(&gate, stamp)?;
        }
        Ok(receipt)
    }
    /// Records intent only. A known request returns its original binding, never the offered one.
    pub fn record(
        &self,
        intent: &CaptureImportIntent,
        source: &CaptureImportSourceBinding,
    ) -> Result<CaptureImportSelection> {
        self.record_then(intent, source, |_, _| ())
            .map(|(selected, ())| selected)
    }
    pub(crate) fn record_then<T>(
        &self,
        intent: &CaptureImportIntent,
        source: &CaptureImportSourceBinding,
        apply: impl FnOnce(&CaptureImportSelection, &dyn Fn() -> Result<()>) -> T,
    ) -> Result<(CaptureImportSelection, T)> {
        let selected = self.select_owned(intent, source)?;
        let applied = apply(selected.selection(), &|| selected.check());
        Ok((selected.selection().clone(), applied))
    }
}
#[cfg(all(test, unix))]
mod tests;
