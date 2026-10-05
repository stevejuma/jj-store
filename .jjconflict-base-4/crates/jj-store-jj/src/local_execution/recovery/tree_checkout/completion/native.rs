use super::*;
use jj_lib::{protos::local_working_copy as p, repo_path::RepoPath};
use prost::Message as _;
/// Match upstream SystemTime duration conversion, including truncation towards
/// zero before the epoch. Native stat seconds are signed with positive nanos.
fn millis(seconds: u64, nanos: u64) -> Result<i64> {
    if nanos >= 1_000_000_000 {
        return Err(Error::Corrupt);
    }
    let total = i128::from(seconds as i64) * 1_000_000_000 + i128::from(nanos);
    (total / 1_000_000).try_into().map_err(|_| Error::Limit)
}
pub(super) fn bytes(plan: &TreeCheckoutPlanV1, installed: &Progress) -> Result<[Vec<u8>; 2]> {
    plan.encode()?;
    let mut checkout = p::Checkout::decode(plan.checkout.as_slice()).map_err(|_| Error::Invalid)?;
    checkout.operation_id = plan.candidate().0.as_bytes().to_vec();
    let mut tree = p::TreeState::decode(plan.tree_state.as_slice()).map_err(|_| Error::Invalid)?;
    let mut states: BTreeMap<_, _> = tree
        .file_states
        .into_iter()
        .map(|entry| (entry.path.clone(), entry))
        .collect();
    for (i, change) in plan.changes.iter().enumerate() {
        if let Some(value) = &change.after {
            let file = installed
                .candidates
                .get(i)
                .ok_or(Error::Corrupt)?
                .as_ref()
                .ok_or(Error::Corrupt)?;
            file.value(value)?;
            states.insert(
                change.path.clone(),
                p::FileStateEntry {
                    path: change.path.clone(),
                    state: Some(p::FileState {
                        mtime_millis_since_epoch: millis(file.stamp[5], file.stamp[6])?,
                        size: file.stamp[4],
                        file_type: if value.executable {
                            p::FileType::Executable
                        } else {
                            p::FileType::Normal
                        } as i32,
                        materialized_conflict_data: None,
                    }),
                },
            );
        } else {
            states.remove(&change.path);
        }
    }
    tree.file_states = states.into_values().collect();
    // RepoPath orders components, so "a/x" sorts before "a.b".
    tree.file_states.sort_by(|a, b| {
        RepoPath::from_internal_string(&a.path)
            .unwrap()
            .cmp(RepoPath::from_internal_string(&b.path).unwrap())
    });
    tree.is_file_states_sorted = true;
    #[expect(deprecated)]
    tree.legacy_tree_id.clear();
    tree.tree_ids = vec![plan.trees().1.as_bytes().to_vec()];
    tree.conflict_labels.clear();
    tree.watchman_clock = None;
    let tree = tree.encode_to_vec();
    let checkout = checkout.encode_to_vec();
    decode::checkout(&checkout, &tree)?;
    if tree.len() > MAX_CHECKOUT_BYTES || checkout.len() > 65536 {
        return Err(Error::Limit);
    }
    Ok([tree, checkout])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_native_milliseconds_match_upstream_duration_truncation() {
        assert_eq!(millis(0, 1_999_999).unwrap(), 1);
        assert_eq!(millis((-1i64) as u64, 999_999_999).unwrap(), 0);
        assert_eq!(millis((-1i64) as u64, 998_000_001).unwrap(), -1);
        assert_eq!(millis((-2i64) as u64, 1).unwrap(), -1999);
        assert!(millis(0, 1_000_000_000).is_err());
        assert!(millis(i64::MAX as u64, 0).is_err());
    }
}
