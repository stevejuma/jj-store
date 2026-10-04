//! Inventory wire validation shared by bounded page consumers.
use crate::{administration as admin, wire};
pub(crate) use jj_store_service::native_inventory::{MAX_SCAN, Page};
use tonic::Status;

pub(crate) fn error(error: jj_store_service::native_inventory::Error) -> Status {
    use jj_store_service::native_inventory::Error;
    match error {
        Error::Denied => Status::permission_denied("repository access denied"),
        Error::Limit | Error::Busy => Status::resource_exhausted("inventory resource limit"),
        Error::InvalidCursor => {
            Status::invalid_argument("invalid inventory cursor or changed query")
        }
        Error::CursorReset | Error::EpochChanged => {
            Status::failed_precondition("inventory selection reset; reselect")
        }
        Error::Unavailable => Status::unavailable("inventory unavailable"),
    }
}
pub(crate) fn decode(reply: wire::InventoryPageReply, limit: usize) -> Result<Page, Status> {
    let invalid = || Status::data_loss("invalid inventory page");
    if reply.scanned as usize > limit
        || reply.ids.len() > reply.scanned as usize
        || reply.next_cursor.len() > 1024
        || (!reply.next_cursor.is_empty() && reply.scanned == 0)
    {
        return Err(invalid());
    }
    let snapshot = admin::snapshot(reply.snapshot).map_err(admin::bad_response)?;
    let ids = reply
        .ids
        .into_iter()
        .map(|id| id.try_into().map_err(|_| invalid()))
        .collect::<Result<Vec<[u8; 64]>, _>>()?;
    if ids.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid());
    }
    Ok(Page {
        snapshot,
        ids,
        scanned: reply.scanned as usize,
        next_cursor: (!reply.next_cursor.is_empty()).then_some(reply.next_cursor),
    })
}
pub(crate) fn complete(reply: wire::Ids, limit: usize) -> Result<Vec<[u8; 64]>, Status> {
    let invalid = || Status::data_loss("invalid complete inventory");
    if reply.ids.len() > limit {
        return Err(invalid());
    }
    let ids = reply
        .ids
        .into_iter()
        .map(|id| id.try_into().map_err(|_| invalid()))
        .collect::<Result<Vec<[u8; 64]>, _>>()?;
    if ids.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid());
    }
    Ok(ids)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn reply() -> wire::InventoryPageReply {
        wire::InventoryPageReply {
            snapshot: Some(wire::Snapshot {
                epoch: vec![1; 16],
                position: 1,
                code_state_version: 0,
                heads: vec![vec![0; 64]],
            }),
            ids: vec![vec![2; 64]],
            scanned: 2,
            next_cursor: "cursor".into(),
        }
    }
    #[test]
    fn validates_scan_progress_and_bounded_canonical_ids() {
        assert!(decode(reply(), 2).is_ok());
        assert!(decode(reply(), 1).is_err());
        let mut value = reply();
        value.ids.clear();
        assert!(decode(value.clone(), 2).unwrap().next_cursor.is_some());
        value.scanned = 0;
        assert!(decode(value, 2).is_err());
        let mut value = reply();
        value.ids[0].pop();
        assert!(decode(value, 2).is_err());
        let mut value = reply();
        value.ids.push(vec![2; 64]);
        assert!(decode(value, 2).is_err());
        let mut value = reply();
        value.ids.push(vec![1; 64]);
        assert!(decode(value, 2).is_err());
        let mut value = reply();
        value.next_cursor = "x".repeat(1025);
        assert!(decode(value, 2).is_err());
        let mut value = reply();
        value.snapshot = None;
        assert!(decode(value, 2).is_err());
    }
    #[test]
    fn complete_inventory_rejects_oversized_malformed_duplicate_and_unordered_results() {
        let ids = || wire::Ids {
            ids: vec![vec![1; 64], vec![2; 64]],
        };
        assert_eq!(complete(ids(), 2).unwrap(), vec![[1; 64], [2; 64]]);
        assert!(complete(ids(), 1).is_err());
        let mut malformed = ids();
        malformed.ids[0].pop();
        assert!(complete(malformed, 2).is_err());
        let mut repeated = ids();
        repeated.ids[1] = repeated.ids[0].clone();
        assert!(complete(repeated, 2).is_err());
        let mut unordered = ids();
        unordered.ids.reverse();
        assert!(complete(unordered, 2).is_err());
        assert!(complete(wire::Ids { ids: vec![] }, 1).unwrap().is_empty());
    }
    #[test]
    fn legacy_inventory_wire_fields_remain_byte_identical() {
        use prost::Message as _;
        let request = wire::InventoryRequest {
            repository: vec![1; 16],
            copies: true,
            limit: 16,
        };
        let mut expected = vec![10, 16];
        expected.extend([1; 16]);
        expected.extend([16, 1, 24, 16]);
        assert_eq!(request.encode_to_vec(), expected);
        assert_eq!(
            wire::InventoryRequest::decode(expected.as_slice()).unwrap(),
            request
        );
    }
}
