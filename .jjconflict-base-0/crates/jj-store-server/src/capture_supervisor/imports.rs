//! One body-free original command per existing retained source owner.
use super::*;
use crate::capture_management::CaptureImportDispatch;
impl CodexCaptureSupervisor {
    pub(crate) fn schedule_import(&mut self, dispatch: CaptureImportDispatch) -> Result<()> {
        if self.closed || self.paused {
            return Err(Error::Service("capture import supervisor unavailable"));
        }
        let mut matches = self
            .slots
            .iter_mut()
            .filter(|slot| slot.qualified.fingerprint() == dispatch.qualified.fingerprint());
        let slot = matches
            .next()
            .ok_or(Error::Service("capture import target retired"))?;
        if matches.next().is_some() || slot.owner.phase() == CaptureWorkerPhase::Stopped {
            return Err(Error::Service("capture import target unavailable"));
        }
        if let Some(original) = &slot.import {
            return if original.same_request(&dispatch) {
                Ok(())
            } else {
                Err(Error::Service(
                    "capture source already retains another import request",
                ))
            };
        }
        slot.import = Some(dispatch);
        Ok(())
    }
}
impl SourceSlot {
    pub(super) fn install_import(&mut self) {
        if self.import_installed {
            return;
        }
        let Some(dispatch) = &self.import else { return };
        let Some(work) = self.owner.idle_mut() else {
            return;
        };
        if work.install_import(dispatch.clone()) {
            // Held generic delivery commands share this revocable body-free flag.
            self.ordinary
                .store(false, std::sync::atomic::Ordering::SeqCst);
            self.import_installed = true;
            self.snapshot.needs_frame = true;
        }
    }
}
