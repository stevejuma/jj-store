use super::*;
use crate::capture_import_state::{
    CaptureImportIntent, CaptureImportJournal, CaptureImportReceipt,
};

/// Retained installation policy only. No source, raw frame, storage or task ownership.
/// Move this guard into owned work before dispatch; dropping a caller cannot release it.
pub struct CaptureImportPolicy {
    pub(super) manager: Manager,
    pub(super) intent: CaptureImportIntent,
    pub(super) journal: CaptureImportJournal,
    pub(super) selected: Option<CaptureImportReceipt>,
    _permit: tokio::sync::OwnedMutexGuard<()>,
}
impl Manager {
    /// Library preparation only. Runs on owned synchronous work, not an HTTP thread.
    /// Disabled/current-actor checks precede all private capture state lookup.
    pub fn begin_import(
        &self,
        intent: CaptureImportIntent,
        journal: CaptureImportJournal,
    ) -> Result<CaptureImportPolicy, Error> {
        intent.validate().map_err(import_failure)?;
        let permit = self
            .0
            .gate
            .clone()
            .try_lock_owned()
            .map_err(|_| Error::Busy)?;
        self.begin_import_with_permit(intent, journal, permit)
    }
    pub(super) async fn begin_import_owned(
        &self,
        intent: CaptureImportIntent,
        journal: CaptureImportJournal,
        admission: &crate::capture_admission::CaptureAdmission,
    ) -> Result<CaptureImportPolicy, Error> {
        intent.validate().map_err(import_failure)?;
        let permit = self.0.gate.clone().lock_owned().await;
        let manager = self.clone();
        admission
            .source_io(move || manager.begin_import_with_permit(intent, journal, permit))
            .await
            .map_err(|_| Error::Unavailable)?
    }
    pub(super) fn begin_import_with_permit(
        &self,
        intent: CaptureImportIntent,
        journal: CaptureImportJournal,
        permit: tokio::sync::OwnedMutexGuard<()>,
    ) -> Result<CaptureImportPolicy, Error> {
        intent.validate().map_err(import_failure)?;
        if !self.0.authority.admit(intent.principal)? {
            return Err(Error::Disabled);
        }
        let binding = self.binding()?;
        if intent.generation != binding.generation {
            return Err(Error::Stale);
        }
        let (path, installation) = self.0.authority.import_binding()?;
        if !journal.belongs_to(&path, installation, binding.generation) {
            return Err(Error::Unavailable);
        }
        // Original receipt first; revision and temporary selector rules apply only later.
        let selected = binding
            .journal
            .read_bound_then(|policy, check| {
                if policy.generation != binding.generation {
                    return Err(Error::Unavailable);
                }
                if !self.0.authority.admit(intent.principal)? {
                    return Err(Error::Disabled);
                }
                check().map_err(report::failure)?;
                let selected = journal.resolve(&intent).map_err(import_failure)?;
                check().map_err(report::failure)?;
                if !self.0.authority.admit(intent.principal)? {
                    return Err(Error::Disabled);
                }
                Ok(selected)
            })
            .map_err(report::failure)??;
        Ok(CaptureImportPolicy {
            manager: self.clone(),
            intent,
            journal,
            selected,
            _permit: permit,
        })
    }
}
impl CaptureImportPolicy {
    pub fn selected(&self) -> Option<&CaptureImportReceipt> {
        self.selected.as_ref()
    }
    pub(super) fn pin_current(
        &self,
    ) -> Result<crate::capture_control_state::CaptureControlPinned, Error> {
        if !self.manager.0.authority.admit(self.intent.principal)? {
            return Err(Error::Disabled);
        }
        if self.journal.resolve(&self.intent).map_err(import_failure)? != self.selected {
            return Err(Error::Unknown);
        }
        let pinned = self
            .manager
            .binding()?
            .journal
            .pin_selected()
            .map_err(report::failure)?;
        self.check_pinned(&pinned)?;
        Ok(pinned)
    }
    pub(super) fn check_pinned(
        &self,
        pinned: &crate::capture_control_state::CaptureControlPinned,
    ) -> Result<(), Error> {
        let binding = self.manager.binding()?;
        let policy = pinned.selected();
        if policy.generation != binding.generation || policy.generation != self.intent.generation {
            return Err(Error::Unavailable);
        }
        if policy.paused {
            return Err(Error::Unavailable);
        }
        if self.selected.is_none() && policy.revision != self.intent.expected_revision {
            return Err(Error::Stale);
        }
        if !self.manager.0.authority.admit(self.intent.principal)? {
            return Err(Error::Disabled);
        }
        pinned.check().map_err(report::failure)?;
        let runtime = binding.runtime.get().ok_or(Error::Unavailable)?;
        runtime.admit_import().map_err(|_| Error::Unavailable)?;
        if self.selected.is_none() {
            let sources = runtime.source_snapshot().map_err(|_| Error::Unavailable)?;
            let (generation, ordinal) =
                self.intent.selector.split_once(':').ok_or(Error::Invalid)?;
            let ordinal = u64::from_str_radix(ordinal, 16).map_err(|_| Error::Invalid)?;
            if generation != hex::encode(sources.generation)
                || !sources.sources.iter().any(|s| {
                    s.selector == ordinal
                        && s.phase != crate::capture_worker::CaptureWorkerPhase::Stopped
                })
            {
                return Err(Error::SourceStale);
            }
        }
        Ok(())
    }
    /// Holds the exact private control gate throughout a bounded synchronous settlement.
    /// Independent source/Write admission and durable intent precede any enrollment.
    pub fn with_current<T>(
        &self,
        apply: impl FnOnce(&dyn Fn() -> Result<(), Error>) -> T,
    ) -> Result<T, Error> {
        let pinned = self.pin_current()?;
        let check = || self.check_pinned(&pinned);
        check()?;
        #[cfg(test)]
        if let Some(hook) = self.manager.0.after_selection.lock().unwrap().take() {
            hook();
        }
        check()?;
        let applied = apply(&check);
        // Selection or enrollment may have happened; loss cannot be described as rejection.
        check().map_err(|_| Error::Unknown)?;
        Ok(applied)
    }
}
pub(super) fn import_failure(error: crate::capture_import_state::Error) -> Error {
    use crate::capture_import_state::Error as E;
    match error {
        E::Invalid => Error::Invalid,
        E::Unavailable => Error::Unavailable,
        E::Conflict => Error::Conflict,
        E::ForeignGeneration => Error::Stale,
        E::Capacity => Error::Capacity,
        E::OutcomeUnknown => Error::Unknown,
    }
}
