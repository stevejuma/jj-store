use super::*;

/// Owned exact file/policy binding. No runtime, source or publication authority.
pub(crate) struct CaptureControlPinned {
    journal: CaptureControlJournal,
    gate: files::Gate,
    stamp: files::Stamp,
    selected: CaptureControlSnapshot,
}
impl CaptureControlJournal {
    pub(crate) fn pin_selected(&self) -> Result<CaptureControlPinned> {
        let gate = self.root.gate()?;
        let (bytes, stamp) = self.root.read(&gate)?;
        let selected = Catalog::decode(&bytes, self.installation)?.snapshot()?;
        Ok(CaptureControlPinned {
            journal: self.clone(),
            gate,
            stamp,
            selected,
        })
    }
}
impl CaptureControlPinned {
    pub(crate) fn selected(&self) -> CaptureControlSnapshot {
        self.selected
    }
    pub(crate) fn check(&self) -> Result<()> {
        let (bytes, stamp) = self.journal.root.read(&self.gate)?;
        if stamp != self.stamp
            || Catalog::decode(&bytes, self.journal.installation)?.snapshot()? != self.selected
        {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
}
