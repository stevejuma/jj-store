//! Transfer a verified disk generation into private ownership before JJ decoding.
use super::*;

impl Cache {
    pub(super) fn recover(
        &mut self,
        mut entry: Entry,
        mut receipt: Receipt,
        access: &dyn ReadAccess,
        budget: &Budget,
    ) -> Result<Entry> {
        let exclusive = match generation::exclusive(&entry.path) {
            Ok(guard) => guard,
            Err(Error::Busy) => {
                // Another readonly database owns this generation. Keep its
                // files in place, with an independently held shared pin, then
                // derive outside the namespace. No allocating decoder runs here.
                entry.pin = Some(Arc::new(generation::Pin::acquire(&entry.path)?));
                entry.sealed =
                    match resident::Verification::capture(&entry.path, receipt.files, budget) {
                        Ok(value) => Some(value),
                        Err(_) => {
                            budget.check()?;
                            None
                        }
                    };
                return Ok(entry);
            }
            Err(error) => return Err(error),
        };
        let data_bytes = receipt.files.values().try_fold(0u64, |total, file| {
            total.checked_add(file.bytes).ok_or(Error::Limit)
        })?;
        // Only the LRU clock can grow when installing these same admitted files.
        receipt.used = u64::MAX;
        let next_receipt = serde_json::to_vec(&receipt)
            .map_err(|_| Error::Unavailable)?
            .len() as u64;
        if next_receipt > RECEIPT_LIMIT {
            return Err(Error::Limit);
        }
        let old_receipt = fs::symlink_metadata(entry.path.join(RECEIPT))
            .map_err(|_| Error::Unavailable)?
            .len();
        let charged = data_bytes
            .checked_add(old_receipt.max(next_receipt))
            .and_then(|bytes| bytes.checked_add(reservation::OVERHEAD))
            .ok_or(Error::Limit)?;
        let staging = Reservation::sized(self, access.repository(), charged)?;
        let private = staging.path().join("generation");
        budget.check()?;
        fs::rename(&entry.path, &private).map_err(|_| Error::Unavailable)?;
        drop(exclusive);
        // Install ownership immediately so any following failure cleans up while
        // the lifetime lock still keeps its quota visible to other workers.
        entry.path = private;
        entry.slot = None;
        entry.staging = Some(staging);
        entry.admission_evicted = self.evicted;
        fs::remove_file(entry.path.join(RECEIPT)).map_err(|_| Error::Unavailable)?;
        entry.sealed = match resident::Verification::capture(&entry.path, receipt.files, budget) {
            Ok(verification) => Some(verification),
            Err(_) => {
                budget.check()?;
                None
            }
        };
        Ok(entry)
    }
}
