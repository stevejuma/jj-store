//! Strict bounded framing for the additive selected-family capability.
use crate::{MAX_IDS, REQUEST_BYTES, RESPONSE_BYTES, inventory, wire};
use jj_store_model as m;
use jj_store_service::transport::{CopyFamily, CopyFamilyError, CopyFamilyLimits};
use prost::Message as _;
use std::collections::BTreeSet;
use tonic::Status;

pub(crate) fn admit(count: usize, limits: CopyFamilyLimits) -> Result<(), Status> {
    if limits.max_nodes == 0
        || limits.max_work == 0
        || limits.max_bytes == 0
        || count > limits.max_nodes.min(MAX_IDS)
        || count
            .checked_mul(512)
            .is_none_or(|bytes| bytes > limits.max_bytes)
    {
        return Err(Status::resource_exhausted("copy family request limit"));
    }
    Ok(())
}
pub(crate) fn request(
    body: &wire::CopyFamilyRequest,
    ceiling: CopyFamilyLimits,
) -> Result<(BTreeSet<m::CopyId>, BTreeSet<m::CopyId>, CopyFamilyLimits), Status> {
    if body.encoded_len() > REQUEST_BYTES
        || body.timeout_millis == 0
        || body.timeout_millis > 3_600_000
    {
        return Err(Status::resource_exhausted("copy family request limit"));
    }
    let limits = CopyFamilyLimits {
        max_nodes: (body.max_nodes as usize).min(ceiling.max_nodes),
        max_work: usize::try_from(body.max_work)
            .map_err(|_| Status::resource_exhausted("copy family work limit"))?
            .min(ceiling.max_work),
        max_bytes: usize::try_from(body.max_bytes)
            .map_err(|_| Status::resource_exhausted("copy family byte limit"))?
            .min(ceiling.max_bytes),
    };
    admit(body.seeds.len(), limits)?;
    let mut seeds = BTreeSet::new();
    let mut required = BTreeSet::new();
    let mut previous = None;
    for seed in &body.seeds {
        let id = m::CopyId::from_slice(&seed.id)
            .map_err(|_| Status::invalid_argument("copy seed identity"))?;
        if previous.is_some_and(|old| old >= id) {
            return Err(Status::invalid_argument(
                "copy seeds must be sorted and unique",
            ));
        }
        previous = Some(id);
        seeds.insert(id);
        if seed.required {
            required.insert(id);
        }
    }
    Ok((seeds, required, limits))
}
pub(crate) fn decode(
    body: wire::CopyFamilyReply,
    seeds: &BTreeSet<m::CopyId>,
    required: &BTreeSet<m::CopyId>,
    limits: CopyFamilyLimits,
) -> Result<CopyFamily, Status> {
    let invalid = || Status::data_loss("invalid copy family response");
    if body.encoded_len() > RESPONSE_BYTES
        || body.ids.len() > limits.max_nodes.min(MAX_IDS)
        || body
            .ids
            .len()
            .checked_mul(512)
            .is_none_or(|bytes| bytes > limits.max_bytes)
        || body.work > limits.max_work as u64
        || body.work < body.ids.len().max(seeds.len()) as u64
        || (seeds.is_empty() && (!body.ids.is_empty() || body.work != 0))
    {
        return Err(invalid());
    }
    // Complete validates lengths and lexical order before BTreeSet growth.
    let ids: BTreeSet<_> = inventory::complete(wire::Ids { ids: body.ids }, limits.max_nodes)?
        .into_iter()
        .map(m::CopyId::new)
        .collect();
    if !required.is_subset(&ids) {
        return Err(invalid());
    }
    Ok(CopyFamily {
        ids,
        work: usize::try_from(body.work).map_err(|_| invalid())?,
    })
}
pub(crate) fn error(error: CopyFamilyError) -> Status {
    match error {
        CopyFamilyError::Denied => Status::permission_denied("copy family access denied"),
        CopyFamilyError::Limit => Status::resource_exhausted("copy family resource limit"),
        CopyFamilyError::NotCommitted => Status::failed_precondition("copy seed is not selected"),
        CopyFamilyError::EpochChanged => {
            Status::failed_precondition("copy family selection changed")
        }
        CopyFamilyError::Corrupt => Status::data_loss("copy family data is corrupt"),
        CopyFamilyError::Unavailable => Status::unavailable("copy family unavailable"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn request(
        body: &wire::CopyFamilyRequest,
    ) -> Result<(BTreeSet<m::CopyId>, BTreeSet<m::CopyId>, CopyFamilyLimits), Status> {
        super::request(body, CopyFamilyLimits::default())
    }
    fn body() -> wire::CopyFamilyRequest {
        wire::CopyFamilyRequest {
            repository: vec![1; 16],
            seeds: vec![wire::CopyFamilySeed {
                id: vec![2; 64],
                required: true,
            }],
            max_nodes: 2,
            max_work: 10,
            max_bytes: 1024,
            timeout_millis: 1000,
        }
    }
    #[test]
    fn selected_family_request_bounds_and_canonical_seed_flags() {
        let original = body();
        assert_eq!(
            wire::CopyFamilyRequest::decode(original.encode_to_vec().as_slice()).unwrap(),
            original
        );
        let (seeds, required, limits) = request(&body()).unwrap();
        assert_eq!(seeds, required);
        assert_eq!(limits.max_nodes, 2);
        for value in [0, 3_600_001] {
            let mut b = body();
            b.timeout_millis = value;
            assert!(request(&b).is_err());
        }
        let mut b = body();
        b.seeds[0].id.pop();
        assert_eq!(
            request(&b).unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
        let mut b = body();
        b.seeds.push(b.seeds[0].clone());
        assert!(request(&b).is_err());
        let mut b = body();
        b.seeds.push(wire::CopyFamilySeed {
            id: vec![1; 64],
            required: false,
        });
        assert!(request(&b).is_err());
        let mut b = body();
        b.max_nodes = 0;
        assert!(request(&b).is_err());
        let mut b = body();
        b.max_work = 0;
        assert!(request(&b).is_err());
        let mut b = body();
        b.max_bytes = 511;
        assert!(request(&b).is_err());
        let mut tightened = body();
        tightened.seeds.push(wire::CopyFamilySeed {
            id: vec![3; 64],
            required: false,
        });
        assert!(
            super::request(
                &tightened,
                CopyFamilyLimits {
                    max_nodes: 1,
                    ..CopyFamilyLimits::default()
                }
            )
            .is_err()
        );
        let mut b = body();
        b.max_nodes = 1;
        b.seeds.push(wire::CopyFamilySeed {
            id: vec![3; 64],
            required: false,
        });
        assert!(request(&b).is_err());
        let mut b = body();
        b.seeds[0].required = false;
        assert!(request(&b).unwrap().1.is_empty());
        let mut b = body();
        b.max_nodes = u32::MAX;
        b.max_work = u64::MAX;
        b.max_bytes = u64::MAX;
        let bounded = request(&b).unwrap().2;
        let ceiling = CopyFamilyLimits::default();
        assert_eq!(
            (bounded.max_nodes, bounded.max_work, bounded.max_bytes),
            (ceiling.max_nodes, ceiling.max_work, ceiling.max_bytes)
        );
    }
    #[test]
    fn selected_family_response_rejects_truncation_work_and_invalid_identities() {
        let (seeds, required, limits) = request(&body()).unwrap();
        let reply = || wire::CopyFamilyReply {
            ids: vec![vec![2; 64]],
            work: 3,
        };
        assert_eq!(decode(reply(), &seeds, &required, limits).unwrap().work, 3);
        let mut b = reply();
        b.ids.clear();
        assert!(decode(b, &seeds, &required, limits).is_err());
        let mut b = reply();
        b.ids[0].pop();
        assert!(decode(b, &seeds, &required, limits).is_err());
        let mut b = reply();
        b.ids.push(vec![2; 64]);
        assert!(decode(b, &seeds, &required, limits).is_err());
        let mut b = reply();
        b.ids.push(vec![1; 64]);
        assert!(decode(b, &seeds, &required, limits).is_err());
        for work in [0, 11, u64::MAX] {
            let mut b = reply();
            b.work = work;
            assert!(decode(b, &seeds, &required, limits).is_err());
        }
        let mut b = reply();
        b.ids.extend([vec![3; 64], vec![4; 64]]);
        assert!(decode(b, &seeds, &required, limits).is_err());
        assert!(
            decode(
                reply(),
                &seeds,
                &required,
                CopyFamilyLimits {
                    max_bytes: 511,
                    ..limits
                }
            )
            .is_err()
        );
        assert!(decode(reply(), &BTreeSet::new(), &BTreeSet::new(), limits).is_err());
        assert!(
            decode(
                wire::CopyFamilyReply {
                    ids: vec![],
                    work: 1
                },
                &seeds,
                &BTreeSet::new(),
                limits
            )
            .is_ok()
        );
    }
}
