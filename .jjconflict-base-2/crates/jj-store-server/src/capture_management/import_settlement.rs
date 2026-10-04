use super::*;
use crate::{
    capture_control_state::CaptureControlPinned,
    capture_import_state::{
        CaptureImportReceipt, CaptureImportSelected, CaptureImportSelection,
        CaptureImportSourceBinding,
    },
};

/// Owned policy/import gates, not a frame, task, native Write or storage lease.
/// The caller must move this guard into retained work before any admission await.
pub struct CaptureImportSettlement {
    // Release filesystem descriptors before the installation coordination permit.
    selected: Option<CaptureImportSelected>,
    control: CaptureControlPinned,
    pub(super) policy: CaptureImportPolicy,
}
impl CaptureImportPolicy {
    /// Retains exact control selection and manager coordination across owned awaits.
    pub fn into_settlement(self) -> Result<CaptureImportSettlement, Error> {
        let control = self.pin_current()?;
        Ok(CaptureImportSettlement {
            policy: self,
            control,
            selected: None,
        })
    }
}
impl CaptureImportSettlement {
    /// Call only after fresh original-prefix/native admission opens the leased reader.
    pub(crate) fn bind_reader(
        &mut self,
        reader: &jj_store_capture::source_registry::LeasedCodexTranscriptReader,
    ) -> Result<(), Error> {
        self.check()?;
        let receipt = self.receipt().ok_or(Error::Unknown)?;
        if !receipt.source.codex().matches_qualified(reader.qualified()) {
            return Err(Error::Unknown);
        }
        let binding = crate::capture_import_state::CaptureImportReaderBinding {
            ordinal: receipt.ordinal,
            source: reader.source_instance_id(),
            generation: reader.generation(),
            generation_digest: reader.qualified().generation_digest(),
            prefix_verified: true,
        };
        self.selected
            .as_mut()
            .ok_or(Error::Unknown)?
            .bind_reader(binding)
            .map_err(|_| Error::Unknown)?;
        self.check()
    }
    pub fn receipt(&self) -> Option<&CaptureImportReceipt> {
        self.policy.selected.as_ref()
    }
    /// Current checks never recursively reacquire this guard's held import lock.
    pub fn check(&self) -> Result<(), Error> {
        let checked = (|| {
            self.policy.check_pinned(&self.control)?;
            if let Some(selected) = &self.selected {
                selected.check().map_err(import_policy::import_failure)?;
            }
            Ok(())
        })();
        if self.selected.is_some() {
            checked.map_err(|_| Error::Unknown)
        } else {
            checked
        }
    }
    /// Intent selection only. Native Write and opaque source preparation are separate.
    /// Replay resolves original metadata before interpreting offered new binding.
    pub fn select(
        &mut self,
        source: &CaptureImportSourceBinding,
    ) -> Result<CaptureImportSelection, Error> {
        self.check()?;
        if let Some(selected) = &self.selected {
            let mut result = selected.selection().clone();
            result.replayed = true;
            return Ok(result);
        }
        let selected = self
            .policy
            .journal
            .select_owned(&self.policy.intent, source)
            .map_err(import_policy::import_failure)?;
        let receipt = &selected.selection().receipt;
        if self
            .policy
            .selected
            .as_ref()
            .is_some_and(|original| original != receipt)
        {
            return Err(Error::Unknown);
        }
        self.policy.selected = Some(receipt.clone());
        self.selected = Some(selected);
        self.check()?;
        Ok(self
            .selected
            .as_ref()
            .expect("selected import")
            .selection()
            .clone())
    }
}
