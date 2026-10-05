use super::*;

#[test]
fn native_projection_admits_two_million_identities_with_index_headroom() {
    // Format-6 native rows: 84 graph/commit bytes and 20 change bytes.
    // Framing admission retains twice the segment size for decoding.
    let commits = 2_000_000;
    let raw = u64::from(commits) * 104 * 2 + 1024 * 1024;
    let projection = commit_weight(commits) + 1024 * 1024;
    assert!(
        raw + projection + 128 * 1024 * 1024 < memory::LIMIT,
        "raw={raw}, projection={projection}; preserve 128MiB for other admitted state"
    );
}
