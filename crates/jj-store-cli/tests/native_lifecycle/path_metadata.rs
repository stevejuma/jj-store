use super::*;
use jj_lib::{backend as b, repo_path::RepoPathComponentBuf};
use jj_store_jj::query::content::{self as c, ContentReader};
use jj_store_service::reads::{OpenRead, ReadLimits};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn path_metadata_crosses_merges_and_retains_the_ancestry_bound() {
    let f = Fixture::new().await;
    let old = f.objects.put_file(&b"old\n"[..]).await.unwrap().0;
    let new = f.objects.put_file(&b"new\n"[..]).await.unwrap().0;
    let file = |id: m::FileId| b::TreeValue::File {
        id: b::FileId::from_bytes(id.as_bytes()),
        executable: false,
        copy_id: b::CopyId::placeholder(),
    };
    let mut merged_files = vec![];
    for bytes in [
        &b"a\nb\nc\n"[..],
        &b"A\nb\nc\n"[..],
        &b"a\nb\nC\n"[..],
        &b"A\nb\nC\n"[..],
    ] {
        merged_files.push(f.objects.put_file(bytes).await.unwrap().0);
    }
    let mut trees = vec![];
    for (index, (a, b)) in [(old, old), (new, old), (old, new), (new, new)]
        .into_iter()
        .enumerate()
    {
        let tree = b::Tree::from_sorted_entries(
            [
                ("a.txt", file(a)),
                ("b.txt", file(b)),
                ("c.txt", file(old)),
                ("merged.txt", file(merged_files[index])),
            ]
            .into_iter()
            .map(|(name, value)| (RepoPathComponentBuf::new(name).unwrap(), value))
            .collect(),
        );
        trees.push(f.objects.put(&tree).await.unwrap().0);
    }
    let mut ids: Vec<b::CommitId> = vec![];
    for (index, description) in ["Base", "Left edit", "Right edit", "Merge feature"]
        .into_iter()
        .enumerate()
    {
        let mut commit = b::make_root_commit(
            b::ChangeId::from_bytes(&[index as u8 + 31; 16]),
            b::TreeId::from_bytes(trees[index].as_bytes()),
        );
        commit.parents = match index {
            0 => vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())],
            1 | 2 => vec![ids[0].clone()],
            _ => vec![ids[1].clone(), ids[2].clone()],
        };
        commit.description = description.into();
        let id = f.objects.put(&commit).await.unwrap().0;
        ids.push(b::CommitId::from_bytes(id.as_bytes()));
    }
    let merge = m::CommitId::from_slice(ids[3].as_bytes()).unwrap();
    let recent_tree = b::Tree::from_sorted_entries(
        [
            ("a.txt", file(new)),
            ("b.txt", file(new)),
            ("c.txt", file(old)),
            ("merged.txt", file(merged_files[3])),
            ("recent.txt", file(new)),
        ]
        .into_iter()
        .map(|(name, value)| (RepoPathComponentBuf::new(name).unwrap(), value))
        .collect(),
    );
    let recent_tree = f.objects.put(&recent_tree).await.unwrap().0;
    let mut tip = ids[3].clone();
    for index in 0..70 {
        let mut commit = b::make_root_commit(
            b::ChangeId::from_bytes(&[index + 50; 16]),
            b::TreeId::from_bytes(if index == 69 {
                recent_tree.as_bytes()
            } else {
                trees[3].as_bytes()
            }),
        );
        commit.parents = vec![tip];
        commit.description = "Unrelated change".into();
        let id = f.objects.put(&commit).await.unwrap().0;
        tip = b::CommitId::from_bytes(id.as_bytes());
    }
    let view = f
        .objects
        .put(&o::View::make_root(tip.clone()))
        .await
        .unwrap()
        .0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    let operation = f.objects.put(&operation).await.unwrap().0;
    let operation = m::OperationId::from_slice(operation.as_bytes()).unwrap();
    f.session
        .publish(BTreeSet::from([roots::ROOT_OPERATION]), operation)
        .await
        .unwrap();
    let OpenRead::Pinned(read) = f
        .session
        .open_read(Some(operation), ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!()
    };
    let read = Arc::new(read);
    // A metadata read must not consume even an attempted authoritative write.
    f.store.inject(jj_store_object::memory::Fault::BeforeWrite);
    let reader = ContentReader::new(c::Limits::default(), 2).unwrap();
    let request = |commit| c::paths::Request {
        commit,
        path: String::new(),
        recursive: false,
        include_last_change: true,
        last_change_mode: Default::default(),
        limit: 20,
        anchor: None,
        direction: Default::default(),
    };
    let page = read.paths(&reader, request(merge)).await.unwrap();
    for (entry, expected) in
        page.entries
            .iter()
            .zip(["Left edit", "Merge feature", "Base", "Merge feature"])
    {
        assert!(
            matches!(&entry.last_change, c::paths::LastChange::Available { description, .. } if description == expected),
            "{}: {:?}",
            entry.path,
            entry.last_change
        );
    }
    let page = read
        .paths(
            &reader,
            request(m::CommitId::from_slice(tip.as_bytes()).unwrap()),
        )
        .await
        .unwrap();
    let selected = m::CommitId::from_slice(tip.as_bytes()).unwrap();
    assert!(page.entries.iter().take(4).all(|entry| matches!(
        &entry.last_change,
        c::paths::LastChange::Available { attribution: c::paths::Attribution::Snapshot, commit, description, .. }
            if *commit == selected && description == "Unrelated change"
    )));
    assert!(matches!(&page.entries[4].last_change,
        c::paths::LastChange::Available { attribution: c::paths::Attribution::PathChange, commit, .. }
            if *commit == selected));
    // Hitting a hard resource limit must never turn an invalid read into a snapshot.
    let limited = ContentReader::new(
        c::Limits {
            max_work: 1,
            ..Default::default()
        },
        1,
    )
    .unwrap();
    assert!(read.paths(&limited, request(selected)).await.is_err());
    // Ready index membership does not change the terminal best-effort semantics.
    let index_root = tempfile::tempdir().unwrap();
    let exec = jj_store_jj::query::GraphExecutor::new(
        index_root.path().to_owned(),
        jj_store_jj::query::Limits::default(),
        2,
    )
    .unwrap();
    let prepared = Arc::new(
        exec.prepare_code_index(
            f.store.clone(),
            read.clone(),
            index_root.path().join("standard"),
            jj_store_jj::query::Cancellation::default(),
        )
        .await
        .unwrap(),
    );
    let blame_cache = Arc::new(jj_store_jj::query::blame::Cache::default());
    let blame_selection = jj_store_jj::query::blame::Selection {
        scope: "native-test".into(),
        repository: jj_store_jj::query::ReadAccess::repository(read.as_ref()),
        operation,
        commit: merge.to_string(),
        path: "merged.txt".into(),
    };
    let blame = exec
        .blame_code_index(
            f.store.clone(),
            read.clone(),
            prepared.clone(),
            blame_cache.clone(),
            blame_selection.clone(),
            None,
        )
        .await
        .unwrap();
    assert!(blame.complete);
    assert_eq!(blame.line_count, 3);
    let descriptions: Vec<_> = blame
        .ranges
        .iter()
        .map(|range| blame.commits[&range.commit_id].description.as_str())
        .collect();
    assert_eq!(descriptions, ["Left edit", "Base", "Right edit"]);
    let cached = exec
        .blame_code_index(
            f.store.clone(),
            read.clone(),
            prepared.clone(),
            blame_cache.clone(),
            blame_selection.clone(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(cached.history_commits, blame.history_commits);
    let wrong_operation = jj_store_jj::query::blame::Selection {
        operation: m::OperationId::new([111; 64]),
        ..blame_selection
    };
    assert!(
        exec.blame_code_index(
            f.store.clone(),
            read.clone(),
            prepared.clone(),
            blame_cache,
            wrong_operation,
            None
        )
        .await
        .is_err()
    );
    let indexed = reader.with_code_index(prepared);
    let indexed_page = read.paths(&indexed, request(selected)).await.unwrap();
    assert_eq!(indexed_page.entries, page.entries);
    let indexed_merge = read.paths(&indexed, request(merge)).await.unwrap();
    for (entry, expected) in
        indexed_merge
            .entries
            .iter()
            .zip(["Left edit", "Merge feature", "Base", "Merge feature"])
    {
        assert!(matches!(&entry.last_change,
            c::paths::LastChange::Available { attribution: c::paths::Attribution::PathChange, description, .. }
                if description == expected));
    }
    let mut history_request = request(selected);
    history_request.last_change_mode = c::paths::LastChangeMode::History;
    let mut detailed = read.paths(&indexed, history_request.clone()).await.unwrap();
    for _ in 0..3 {
        if detailed
            .entries
            .iter()
            .all(|entry| matches!(entry.last_change, c::paths::LastChange::Available { .. }))
        {
            break;
        }
        detailed = read.paths(&indexed, history_request.clone()).await.unwrap();
    }
    for (entry, expected) in detailed.entries.iter().zip([
        "Left edit",
        "Right edit",
        "Base",
        "Left edit",
        "Unrelated change",
    ]) {
        assert!(
            matches!(&entry.last_change, c::paths::LastChange::Available { attribution: c::paths::Attribution::PathChange, description, .. } if description == expected),
            "{entry:?}"
        );
    }
    // Tree comparison must never consume a source write, including clean merges.
    assert_eq!(
        f.objects
            .put_file(&b"write fault probe\n"[..])
            .await
            .unwrap()
            .1,
        jj_store_jj::objects::UploadStatus::Unknown
    );
}
