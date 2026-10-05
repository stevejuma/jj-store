use super::*;
use crate::inventory::{self, MAX_SCAN, Page};

/// Full enumeration remains an explicit bounded caller operation. Individual
/// pages are independent of the selected repository's total object count.
pub const MAX_INVENTORY_SCANNED: usize = 2_000_000;
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
        tokio::time::timeout(self.options.timeout, async {
            let mut result = BTreeSet::new();
            let mut snapshot = None;
            let mut cursor: Option<String> = None;
            let mut scanned = 0usize;
            loop {
                let remaining = MAX_INVENTORY_SCANNED
                    .checked_sub(scanned)
                    .filter(|remaining| *remaining != 0)
                    .ok_or_else(|| Status::resource_exhausted("inventory total scan limit"))?;
                // Fixed request size keeps every continuation bound to one query.
                // Reserve a full next window; never exceed the whole-call bound.
                if remaining < MAX_SCAN {
                    return Err(Status::resource_exhausted("inventory total scan limit"));
                }
                let page = self
                    .inventory_page(copies, MAX_SCAN, cursor.as_deref())
                    .await?;
                if snapshot
                    .as_ref()
                    .is_some_and(|selected| selected != &page.snapshot)
                {
                    return Err(Status::data_loss("inventory snapshot changed"));
                }
                snapshot = Some(page.snapshot);
                scanned = scanned
                    .checked_add(page.scanned)
                    .ok_or_else(|| Status::resource_exhausted("inventory total scan limit"))?;
                for id in page.ids {
                    if result.contains(&id) {
                        return Err(Status::data_loss("duplicate inventory ID"));
                    }
                    if result.len() == limit {
                        return Err(Status::resource_exhausted("inventory result limit"));
                    }
                    result.insert(id);
                }
                if page.next_cursor.is_some() && page.next_cursor == cursor {
                    return Err(Status::data_loss("inventory continuation did not advance"));
                }
                cursor = page.next_cursor;
                if cursor.is_none() {
                    return Ok(result.into_iter().collect());
                }
            }
        })
        .await
        .map_err(|_| Status::deadline_exceeded("inventory total deadline"))?
    }
}
