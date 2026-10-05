use super::*;
use crate::query::prepared;
use std::collections::BTreeMap;
fn hex_name(bytes: &[u8]) -> String {
    bytes.iter().map(|v| format!("{v:02x}")).collect()
}
pub(super) async fn missing(
    recovery: &RecoveredHistory,
    saved: &destination::Snapshot,
    budget: &Budget,
) -> Result<BTreeMap<(String, String), Vec<u8>>> {
    let source = archive::Archive { recovery, budget };
    let operations = std::collections::BTreeSet::from([
        recovery.witness.binding().operation,
        recovery.archive.operation,
    ]);
    let mut out = BTreeMap::new();
    let store = verify::store(recovery)?;
    let mut expected = BTreeMap::new();
    for id in &operations {
        let directory = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
        let native = jj_lib::op_store::OperationId::from_bytes(id.as_bytes());
        budget
            .until(prepared::build_from(
                directory.path(),
                &native,
                &source,
                128 * 1024 * 1024,
            ))
            .await??;
        expected.insert(
            *id,
            verify::signature(directory.path(), *id, &source, &store).await?,
        );
        if saved
            .files
            .contains_key(&("op_links".into(), id.to_string()))
        {
            continue;
        }
        let private_root = directory
            .path()
            .canonicalize()
            .map_err(|_| Error::Unavailable)?;
        let root = physical::directory(&private_root)?;
        for group in ["segments", "op_links"] {
            let dir = physical::child(&root, group)?;
            for name in destination::names(&dir, budget)? {
                let bytes = physical::read(&dir, &name, MAX_SEGMENT, false, budget)?
                    .ok_or(Error::Unavailable)?
                    .bytes;
                let key = (group.to_owned(), name);
                if let Some(existing) = saved.files.get(&key)
                    && existing.bytes != bytes
                {
                    return Err(Error::Corrupt);
                }
                if let Some(existing) = out.insert(key, bytes.clone())
                    && existing != bytes
                {
                    return Err(Error::Corrupt);
                }
            }
        }
    }
    let mut complete = saved.clone();
    for (key, bytes) in &out {
        if !complete.files.contains_key(key) {
            // A private snapshot only; actual file stamps are captured on source selection.
            let placeholder = physical::Bytes {
                bytes: bytes.clone(),
                fingerprint: m::RecordDigest::new([0; 32]),
            };
            complete.files.insert(key.clone(), placeholder);
        }
    }
    if complete
        .files
        .values()
        .map(|v| v.bytes.len())
        .sum::<usize>()
        > MAX_BYTES
        || complete
            .files
            .keys()
            .filter(|(dir, _)| dir == "segments")
            .count()
            > 128
        || complete
            .files
            .keys()
            .filter(|(dir, _)| dir == "op_links")
            .count()
            > 4096
    {
        return Err(Error::Limit);
    }
    let private = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
    complete.materialize(private.path())?;
    // Existing segments as well as generated output must have supported bounded framing.
    let weight =
        crate::query::memory::framing::weight(private.path(), 20, 16, 100_000, &mut || {
            budget.work(1)
        })?;
    if weight > 2 * MAX_BYTES as u64 + 2 * 1024 * 1024 {
        return Err(Error::Limit);
    }
    use blake2::{Blake2b512, Digest as _};
    use prost::Message as _;
    for ((group, name), data) in &complete.files {
        if group == "op_links" {
            let control =
                jj_lib::protos::default_index::SegmentControl::decode(data.bytes.as_slice())
                    .map_err(|_| Error::Corrupt)?;
            if control.encode_to_vec() != data.bytes
                || control.commit_segment_id.len() != 64
                || !complete
                    .files
                    .contains_key(&("segments".into(), hex_name(&control.commit_segment_id)))
                || control.changed_path_segment_ids.len() > 128
            {
                return Err(Error::Corrupt);
            }
            for segment in &control.changed_path_segment_ids {
                if segment.len() != 64
                    || !complete
                        .files
                        .contains_key(&("changed_paths".into(), hex_name(segment)))
                {
                    return Err(Error::Corrupt);
                }
            }
        } else if format!("{:x}", Blake2b512::digest(&data.bytes)) != *name {
            return Err(Error::Corrupt);
        }
    }
    for id in &operations {
        if verify::signature(private.path(), *id, &source, &store).await? != expected[id] {
            return Err(Error::Corrupt);
        }
    }
    recovery.check(budget).await?;
    Ok(out)
}
