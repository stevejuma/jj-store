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
    let mut tip = ids[3].clone();
    for index in 0..70 {
        let mut commit = b::make_root_commit(
            b::ChangeId::from_bytes(&[index + 50; 16]),
            b::TreeId::from_bytes(trees[3].as_bytes()),
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
    assert!(page.entries.iter().all(|entry| matches!(
        entry.last_change,
        c::paths::LastChange::Unavailable {
            reason: c::paths::LastChangeUnavailable::Budget
        }
    )));
    // The managed path reader follows JJ's original merged-branch history and
    // resumes old metadata without repeating its first64 candidates.
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
    let indexed = reader.with_code_index(prepared);
    let first = read
        .paths(
            &indexed,
            request(m::CommitId::from_slice(tip.as_bytes()).unwrap()),
        )
        .await
        .unwrap();
    assert!(first.entries.iter().all(|e| matches!(
        e.last_change,
        c::paths::LastChange::Unavailable {
            reason: c::paths::LastChangeUnavailable::Pending
        }
    )));
    let completed = read
        .paths(
            &indexed,
            request(m::CommitId::from_slice(tip.as_bytes()).unwrap()),
        )
        .await
        .unwrap();
    for (entry, expected) in
        completed
            .entries
            .iter()
            .zip(["Left edit", "Right edit", "Base", "Left edit"])
    {
        assert!(
            matches!(&entry.last_change, c::paths::LastChange::Available { description, .. } if description == expected),
            "{}: {:?}",
            entry.path,
            entry.last_change
        );
    }
    let cached = read
        .paths(
            &indexed,
            request(m::CommitId::from_slice(tip.as_bytes()).unwrap()),
        )
        .await
        .unwrap();
    assert!(cached.stats.object_reads < 12, "{:?}", cached.stats);
    // Equal-time parallel edits follow the native JJ index's topological order;
    // merged.txt must identify a branch edit, never the clean merge itself.
    assert_eq!(
        f.objects
            .put_file(&b"write fault probe\n"[..])
            .await
            .unwrap()
            .1,
        jj_store_jj::objects::UploadStatus::Unknown
    );
}
