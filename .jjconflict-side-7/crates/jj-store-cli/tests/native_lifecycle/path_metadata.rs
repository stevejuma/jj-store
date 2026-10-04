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
    let mut trees = vec![];
    for (a, b) in [(old, old), (new, old), (old, new), (new, new)] {
        let tree = b::Tree::from_sorted_entries(
            [("a.txt", file(a)), ("b.txt", file(b)), ("c.txt", file(old))]
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
    for (entry, expected) in page
        .entries
        .iter()
        .zip(["Left edit", "Merge feature", "Base"])
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
}
