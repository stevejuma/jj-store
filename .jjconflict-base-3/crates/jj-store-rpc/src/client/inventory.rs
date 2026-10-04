use super::*;
use crate::inventory::{self, MAX_SCAN, Page};

/// Full enumeration remains an explicit bounded caller operation. Individual
/// pages are independent of the selected repository's total object count.
pub use jj_store_service::native_inventory::MAX_SCANNED as MAX_INVENTORY_SCANNED;
impl NativeClient {
    pub async fn inventory_page(
        &self,
        copies: bool,
        scan_limit: usize,
        cursor: Option<&str>,
    ) -> Result<Page, Status> {
        if scan_limit == 0 || scan_limit > MAX_SCAN {
            return Err(Status::resource_exhausted("inventory scan limit"));
        }
        if cursor.is_some_and(|cursor| cursor.is_empty() || cursor.len() > 1024) {
            return Err(Status::invalid_argument("inventory cursor"));
        }
        let response = self
            .wire()
            .inventory_page(self.request(wire::InventoryPageRequest {
                repository: self.repository.as_bytes().to_vec(),
                copies,
                scan_limit: scan_limit as u32,
                cursor: cursor.unwrap_or_default().to_owned(),
            }))
            .await?
            .into_inner();
        inventory::decode(response, scan_limit)
    }
    /// Complete sorted kind inventory, never a successful truncated result.
    /// Call inventory_page when a caller needs progressive bounded enumeration.
    pub async fn inventory(&self, copies: bool, limit: usize) -> Result<Vec<[u8; 64]>, Status> {
        if limit == 0 || limit > MAX_IDS {
            return Err(Status::resource_exhausted("inventory limit"));
        }
        let deadline = tokio::time::Instant::now()
            .checked_add(self.options.timeout)
            .ok_or_else(|| Status::deadline_exceeded("inventory total deadline"))?;
        let ids = tokio::time::timeout_at(deadline, async {
            let response = self
                .wire()
                .inventory(self.request(wire::InventoryRequest {
                    repository: self.repository.as_bytes().to_vec(),
                    copies,
                    limit: limit as u32,
                }))
                .await?
                .into_inner();
            inventory::complete(response, limit)
        })
        .await
        .map_err(|_| Status::deadline_exceeded("inventory total deadline"))??;
        if tokio::time::Instant::now() >= deadline {
            return Err(Status::deadline_exceeded("inventory total deadline"));
        }
        Ok(ids)
    }
}
