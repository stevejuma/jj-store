use super::*;

/// Noncloneable exact selection; owned work retains this gate through admission awaits.
pub(crate) struct CaptureImportSelected {
    pub(super) journal: CaptureImportJournal,
    pub(super) gate: files::Gate,
    pub(super) stamp: files::Stamp,
    selection: CaptureImportSelection,
}
impl CaptureImportJournal {
    pub(crate) fn select_owned(
        &self,
        intent: &CaptureImportIntent,
        source: &CaptureImportSourceBinding,
    ) -> Result<CaptureImportSelected> {
        self.validate(intent)?;
        let gate = self.root.gate()?;
        let (bytes, stamp) = self.root.read(&gate)?;
        let mut catalog = self.decode(&bytes)?;
        let selection = if let Some(receipt) = catalog.find(intent)? {
            self.root.sync_selected(&gate, stamp)?;
            CaptureImportSelection {
                receipt: receipt.clone(),
                replayed: true,
            }
        } else {
            source.validate()?;
            if catalog.receipts.len() == MAX_RECEIPTS {
                return Err(Error::Capacity);
            }
            let receipt = CaptureImportReceipt {
                ordinal: catalog.receipts.len() as u64 + 1,
                intent: intent.clone(),
                source: source.clone(),
            };
            catalog.receipts.push(receipt.clone());
            self.root.persist(&gate, &catalog.encode()?, Some(stamp))?;
            CaptureImportSelection {
                receipt,
                replayed: false,
            }
        };
        let (_, stamp) = self.root.read(&gate)?;
        Ok(CaptureImportSelected {
            journal: self.clone(),
            gate,
            stamp,
            selection,
        })
    }
}
impl CaptureImportSelected {
    pub(crate) fn selection(&self) -> &CaptureImportSelection {
        &self.selection
    }
    pub(crate) fn check(&self) -> Result<()> {
        let (bytes, stamp) = self.journal.root.read(&self.gate)?;
        if stamp != self.stamp
            || self
                .journal
                .decode(&bytes)?
                .find(&self.selection.receipt.intent)?
                != Some(&self.selection.receipt)
        {
            return Err(Error::Unavailable);
        }
        Ok(())
    }
}
