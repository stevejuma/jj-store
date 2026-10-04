//! Runtime-private qualified slots; observation pages contain none of these bindings.
use super::*;
use jj_store_capture::source_registry::QualifiedCodexSource;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ImportTargets {
    pub generation: [u8; 16],
    pub slots: Vec<(u64, QualifiedCodexSource)>,
}
impl CodexCaptureSupervisor {
    pub(crate) fn import_targets(&self) -> ImportTargets {
        ImportTargets {
            generation: self.source_generation(),
            slots: self.retained_import_slots(),
        }
    }
}
impl CaptureRuntimeControl {
    pub(crate) fn import_targets(&self) -> Result<Arc<ImportTargets>> {
        self.admit_import()?;
        if self.targets.has_changed().is_err() {
            return Err(Error::Service("capture import targets unavailable"));
        }
        Ok(self.targets.borrow().clone())
    }
}
#[cfg(test)]
pub(super) fn empty() -> watch::Receiver<Arc<ImportTargets>> {
    watch::channel(Arc::new(ImportTargets {
        generation: [1; 16],
        slots: vec![],
    }))
    .1
}
