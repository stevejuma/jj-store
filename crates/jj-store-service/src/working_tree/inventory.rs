use super::*;
use jj_store_jj::local_execution::{PhysicalFileV1, PhysicalInventoryV1};
impl WorkingTree {
    /// Complete private capture input. Wire summaries do not expose these rows.
    pub fn inventory(&self) -> Result<PhysicalInventoryV1> {
        let inventory = PhysicalInventoryV1::new(
            self.repository,
            self.generation,
            self.workspace.0,
            self.directories.clone(),
            self.entries
                .iter()
                .map(|file| PhysicalFileV1 {
                    path: file.path.clone(),
                    fingerprint: file.fingerprint,
                })
                .collect(),
        )
        .map_err(observer::query_error)?;
        if inventory.fingerprint() != self.fingerprint {
            return Err(Error::Stale);
        }
        Ok(inventory)
    }
}
