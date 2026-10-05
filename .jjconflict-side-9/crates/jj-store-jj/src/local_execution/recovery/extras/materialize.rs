use super::*;
use jj_lib::{
    config::StackedConfig, git_backend::GitBackend, settings::UserSettings,
    stacked_table::TableStore,
};
use prost::Message as _;
use std::collections::BTreeMap;
pub(super) fn backend(recovery: &RecoveredHistory) -> Result<GitBackend> {
    let settings = UserSettings::from_config(StackedConfig::with_defaults())
        .map_err(|_| Error::Unavailable)?;
    let backend = GitBackend::load(&settings, &recovery.access.metadata_path().join("store"))
        .map_err(|_| Error::Unavailable)?;
    if backend.git_repo_path() != recovery.witness.git_path() {
        return Err(Error::EpochChanged);
    }
    Ok(backend)
}
pub(super) struct Output {
    pub segments: BTreeMap<String, Vec<u8>>,
    pub heads: Vec<String>,
}
pub(super) fn missing(
    recovery: &RecoveredHistory,
    source: &destination::Snapshot,
    budget: &Budget,
) -> Result<Output> {
    let private = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
    source.materialize(private.path())?;
    let backend = backend(recovery)?;
    let reader = crate::local_browser::git_read::Reader::open(&backend, private.path(), budget)?;
    let extras = crate::local_browser::git_read::extras::Extras::open(private.path(), budget)?;
    let mut entries = vec![];
    for (id, expected) in &recovery.archive.commits {
        budget.work(1)?;
        if id.as_bytes() == &[0; 20] {
            continue;
        }
        let native = jj_lib::backend::CommitId::from_bytes(id.as_bytes());
        if extras.find_record(id.as_bytes(), budget)?.is_some() {
            let actual = reader.exact_commit(&backend, &native, budget)?;
            if crate::local_evidence::GitCommitV1::try_from(&actual).map_err(evidence_error)?
                != *expected
            {
                return Err(Error::Corrupt);
            }
            continue;
        }
        let proto = jj_lib::protos::git_store::Commit {
            change_id: expected.change_id.as_bytes().to_vec(),
            predecessors: expected
                .predecessors
                .iter()
                .map(|v| v.as_bytes().to_vec())
                .collect(),
            root_tree: if expected.root_tree.len() > 1 {
                expected
                    .root_tree
                    .iter()
                    .map(|v| v.as_bytes().to_vec())
                    .collect()
            } else {
                vec![]
            },
            uses_tree_conflict_format: true,
            ..Default::default()
        };
        entries.push((id.as_bytes().to_vec(), proto.encode_to_vec()));
    }
    let mut output = Output {
        segments: Default::default(),
        heads: vec![],
    };
    let mut position = 0;
    while position < entries.len() {
        budget.work(1)?;
        let batch = tempfile::tempdir().map_err(|_| Error::Unavailable)?;
        let table = TableStore::init(batch.path().to_path_buf(), 20);
        let head = table.get_head().map_err(|_| Error::Unavailable)?;
        let mut mutable = head.start_mutation();
        let mut size = 8;
        while position < entries.len() {
            let (key, value) = &entries[position];
            let n = 24 + value.len();
            if n > MAX_SEGMENT - 8 {
                return Err(Error::Limit);
            }
            // One supported large record must advance the batch on its own.
            if size > 8 && size + n > 3 * 1024 * 1024 {
                break;
            }
            mutable.add_entry(key.clone(), value.clone());
            size += n;
            position += 1;
        }
        let head = table.save_table(mutable).map_err(|_| Error::Unavailable)?;
        let name = head.name().to_owned();
        let bytes = std::fs::read(batch.path().join(&name)).map_err(|_| Error::Unavailable)?;
        if bytes.len() > MAX_SEGMENT
            || output.segments.values().map(Vec::len).sum::<usize>() + bytes.len() > MAX_EXTRA_BYTES
        {
            return Err(Error::Limit);
        }
        output.segments.insert(name.clone(), bytes);
        output.heads.push(name);
    }
    let added_segments = output
        .segments
        .iter()
        .filter(|(name, _)| !source.segments.contains_key(*name));
    let added_heads = output
        .heads
        .iter()
        .filter(|name| !source.heads.contains_key(*name))
        .count();
    if source.heads.len() + added_heads > 64
        || source.segments.len() + added_segments.clone().count() > 128
        || source
            .segments
            .values()
            .map(|v| v.bytes.len())
            .sum::<usize>()
            + added_segments.map(|(_, bytes)| bytes.len()).sum::<usize>()
            > MAX_EXTRA_BYTES
    {
        return Err(Error::Limit);
    }
    // Prove the complete candidate before creating a source directory or segment.
    for (name, bytes) in &output.segments {
        if let Some(old) = source.segments.get(name)
            && old.bytes != *bytes
        {
            return Err(Error::Corrupt);
        }
        std::fs::write(private.path().join("store/extra").join(name), bytes)
            .map_err(|_| Error::Unavailable)?;
    }
    for name in &output.heads {
        std::fs::write(private.path().join("store/extra/heads").join(name), [])
            .map_err(|_| Error::Unavailable)?;
    }
    let reader = crate::local_browser::git_read::Reader::open(&backend, private.path(), budget)?;
    for (id, expected) in &recovery.archive.commits {
        let actual = reader.exact_commit(
            &backend,
            &jj_lib::backend::CommitId::from_bytes(id.as_bytes()),
            budget,
        )?;
        if crate::local_evidence::GitCommitV1::try_from(&actual).map_err(evidence_error)?
            != *expected
        {
            return Err(Error::Corrupt);
        }
    }
    Ok(output)
}
