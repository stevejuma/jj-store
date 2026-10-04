use super::*;

async fn select(f: &Fixture, commits: &[b::Commit]) -> Arc<CommittedRead> {
    let mut view = roots::view();
    view.head_ids.clear();
    for commit in commits {
        let id = f.objects.put(commit).await.unwrap().0;
        view.head_ids.insert(b::CommitId::from_bytes(id.as_bytes()));
    }
    if commits.is_empty() {
        view.head_ids
            .insert(b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes()));
    }
    let view = f.objects.put(&view).await.unwrap().0;
    let mut operation = o::Operation::make_root(o::ViewId::from_bytes(view.as_bytes()));
    operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
    let id = f.objects.put(&operation).await.unwrap().0;
    f.session
        .publish(
            BTreeSet::from([roots::ROOT_OPERATION]),
            m::OperationId::from_slice(id.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    let OpenRead::Pinned(read) = f
        .session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!()
    };
    Arc::new(read)
}
fn commit(description: &str, millis: i64) -> b::Commit {
    let mut commit = roots::commit();
    commit.change_id = b::ChangeId::from_bytes(&[71; 16]);
    commit.parents = vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())];
    commit.description = description.into();
    commit.committer.timestamp.timestamp = b::MillisSinceEpoch(millis);
    commit
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_code_summary_hydrates_only_selected_heads_without_ancestry_or_trees() {
    let f = Fixture::new().await;
    let mut parent = commit("Ancestor", 9999);
    for n in 0..12 {
        let key = f.objects.put(&parent).await.unwrap().0;
        parent.parents = vec![b::CommitId::from_bytes(key.as_bytes())];
        parent.description = format!("Ancestor {n}");
    }
    let commits = (0..12)
        .map(|n| {
            let mut c = parent.clone();
            c.description = format!("Head {n}");
            c.committer.timestamp.timestamp = b::MillisSinceEpoch(n);
            c
        })
        .collect::<Vec<_>>();
    let read = select(&f, &commits).await;
    let before = f.session.snapshot().await.unwrap().manifest().clone();
    let summary = read.code_summary(&reader(), 1).await.unwrap();
    let first = commits.iter().min_by_key(|c| c.key()).unwrap();
    assert_eq!(summary.commits, 1);
    assert!(summary.commits_partial);
    assert_eq!(
        summary.latest.as_ref().unwrap().id.as_bytes(),
        first.key().as_bytes()
    );
    assert_eq!(summary.stats.object_reads, 3);
    // Even an entire head window does not read the older (newer-timestamped) ancestry.
    let all = read.code_summary(&reader(), 12).await.unwrap();
    assert_eq!(all.commits, 12);
    assert!(all.commits_partial);
    assert_eq!(all.latest.unwrap().description, "Head 11");
    assert_eq!(all.stats.object_reads, 14);
    assert_eq!(f.session.snapshot().await.unwrap().manifest(), &before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_code_summary_orders_times_ties_and_complete_root_child_counts() {
    let f = Fixture::new().await;
    let commits = vec![
        commit("old", -4),
        commit("new one", 80),
        commit("new two", 80),
    ];
    let read = select(&f, &commits).await;
    let summary = read.code_summary(&reader(), 256).await.unwrap();
    let expected = commits
        .iter()
        .filter(|c| c.committer.timestamp.timestamp.0 == 80)
        .min_by_key(|c| c.key())
        .unwrap();
    assert_eq!(summary.commits, 3);
    assert!(!summary.commits_partial);
    assert_eq!(
        summary.latest.unwrap().id.as_bytes(),
        expected.key().as_bytes()
    );
    assert!(
        read.code_summary(&reader(), 2)
            .await
            .unwrap()
            .commits_partial
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_code_summary_excludes_the_virtual_root_and_enforces_limits() {
    let f = Fixture::new().await;
    let read = select(&f, &[]).await;
    let summary = read.code_summary(&reader(), 1).await.unwrap();
    assert_eq!(summary.commits, 0);
    assert!(!summary.commits_partial);
    assert!(summary.latest.is_none());
    assert_eq!(summary.stats.object_reads, 2);
    for limit in [0, 257] {
        assert_eq!(
            read.code_summary(&reader(), limit).await.unwrap_err(),
            q::Error::Limit
        );
    }
    let limited = ContentReader::new(
        c::Limits {
            max_output_bytes: 1,
            ..Default::default()
        },
        1,
    )
    .unwrap();
    assert_eq!(
        read.code_summary(&limited, 1).await.unwrap_err(),
        q::Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_code_summary_retains_historical_selection_and_fresh_authorization() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let f = ContentFixture::new().await;
    let pin = f.read.snapshot().await.unwrap().operation_id;
    let session = f.f.session(f.f.reader.clone(), Protection::default());
    let OpenRead::Pinned(read) = session
        .open_read(Some(pin), ReadLimits::default())
        .await
        .unwrap()
    else {
        panic!()
    };
    let read = Arc::new(read);
    let summary = read.code_summary(&reader(), 1).await.unwrap();
    assert_eq!(summary.latest.unwrap().description, f.commit.description);
    let mut later = f.operation.clone();
    later.parents = vec![o::OperationId::from_bytes(pin.as_bytes())];
    later.metadata.description = "Later operation".into();
    let key = f.f.objects.put(&later).await.unwrap().0;
    f.f.session
        .publish(
            BTreeSet::from([pin]),
            m::OperationId::from_slice(key.as_bytes()).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(read.snapshot().await.unwrap().operation_id, pin);
    f.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([97; 16]),
            expected: f.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: f.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        read.code_summary(&reader(), 1).await.unwrap_err(),
        q::Error::Denied
    );
}
