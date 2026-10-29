use super::*;
use crate::query::blame_probe::commit;
use jj_lib::{
    config::StackedConfig, settings::UserSettings, simple_backend::SimpleBackend,
    workspace::Workspace,
};
fn key(id: &b::CommitId) -> Selection {
    Selection {
        scope: "fixture".into(),
        repository: m::RepositoryId::new([71; 16]),
        operation: m::OperationId::new([72; 64]),
        commit: id.hex(),
        path: "file".into(),
    }
}
#[tokio::test]
async fn annotation_matches_jj_merges_repeated_lines_empty_and_missing_newline() {
    for git in [false, true] {
        let dir = tempfile::tempdir().unwrap();
        let settings =
            UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
        let (_, repo) = if git {
            Workspace::init_internal_git(&settings, dir.path(), gix::hash::Kind::Sha1)
                .await
                .unwrap()
        } else {
            Workspace::init_with_backend(
                &settings,
                dir.path(),
                &|_, p| Ok(Box::new(SimpleBackend::init(p))),
                jj_lib::signing_factory::signer_from_settings(&settings).unwrap(),
            )
            .await
            .unwrap()
        };
        let (repo, base) = commit(
            &repo,
            vec![repo.store().root_commit_id().clone()],
            b"base\nrepeat\nrepeat\nlast",
        )
        .await;
        let (repo, left) = commit(
            &repo,
            vec![base.id().clone()],
            b"left\nrepeat\nrepeat\nlast",
        )
        .await;
        let (repo, right) = commit(
            &repo,
            vec![base.id().clone()],
            b"base\nrepeat\nrepeat\nright",
        )
        .await;
        let (repo, merge) = commit(
            &repo,
            vec![left.id().clone(), right.id().clone()],
            b"left\nrepeat\nrepeat\nright",
        )
        .await;
        let (repo, empty) = commit(&repo, vec![merge.id().clone()], b"").await;
        let cache = Cache::default();
        for selected in [base, left, right, merge, empty] {
            let key = key(selected.id());
            let mut page = read(
                repo.as_ref(),
                &cache,
                key.clone(),
                None,
                || Ok(()),
                Stats::default,
                async || Ok(()),
            )
            .await
            .unwrap();
            while !page.complete {
                page = read(
                    repo.as_ref(),
                    &cache,
                    key.clone(),
                    page.continuation.as_deref(),
                    || Ok(()),
                    Stats::default,
                    async || Ok(()),
                )
                .await
                .unwrap();
            }
            let mut full = FileAnnotator::from_commit(
                &selected,
                &RepoPathBuf::from_internal_string("file").unwrap(),
            )
            .await
            .unwrap();
            full.compute(repo.as_ref(), &RevsetExpression::all())
                .await
                .unwrap();
            let mut actual = vec![None; page.line_count];
            for range in &page.ranges {
                for line in &mut actual[range.start - 1..range.end - 1] {
                    *line = Some(range.commit_id.clone());
                }
            }
            let expected: Vec<_> = full
                .to_annotation()
                .line_origins()
                .map(|(origin, _)| origin.ok().map(|o| o.commit_id.hex()))
                .collect();
            assert_eq!(actual, expected);
            let cached = read(
                repo.as_ref(),
                &cache,
                key,
                None,
                || Ok(()),
                Stats::default,
                async || Ok(()),
            )
            .await
            .unwrap();
            assert_eq!(cached.history_commits, page.history_commits);
        }
    }
}
#[tokio::test]
async fn retries_cancellation_reset_and_selection_binding() {
    let dir = tempfile::tempdir().unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let (_, mut repo) = Workspace::init_internal_git(&settings, dir.path(), gix::hash::Kind::Sha1)
        .await
        .unwrap();
    let mut parent = repo.store().root_commit_id().clone();
    for _ in 0..90 {
        let (r, c) = commit(&repo, vec![parent], b"unchanged\n").await;
        repo = r;
        parent = c.id().clone();
    }
    let cache = Cache::default();
    let key = key(&parent);
    let checks = std::cell::Cell::new(0);
    let first = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        None,
        || {
            checks.set(checks.get() + 1);
            if checks.get() == 3 {
                // Let the cooperative slice expire during its first window.
                std::thread::sleep(Duration::from_millis(1001));
            }
            Ok(())
        },
        Stats::default,
        async || Ok(()),
    )
    .await
    .unwrap();
    // A deliberately short slice below keeps the 90-commit fixture progressive.
    assert!(!first.complete);
    let cursor = first.continuation.clone().unwrap();
    let fail = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        Some(&cursor),
        || Err(Error::Canceled),
        Stats::default,
        async || Ok(()),
    )
    .await;
    assert!(matches!(fail, Err(Error::Canceled)));
    let same = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        None,
        || Ok(()),
        Stats::default,
        async || Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(same.history_commits, first.history_commits);
    // A failed final authorization cannot publish trial progress or bypass a hit.
    let finishes = std::cell::Cell::new(0);
    let denied = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        Some(&cursor),
        || Ok(()),
        Stats::default,
        async || {
            finishes.set(finishes.get() + 1);
            if finishes.get() > 1 {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        },
    )
    .await;
    assert!(matches!(denied, Err(Error::Canceled)));
    let retained = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        None,
        || Ok(()),
        Stats::default,
        async || Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(retained.history_commits, first.history_commits);
    assert!(matches!(
        read(
            repo.as_ref(),
            &cache,
            key.clone(),
            None,
            || Ok(()),
            Stats::default,
            async || Err(Error::NotCommitted)
        )
        .await,
        Err(Error::NotCommitted)
    ));
    let next = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        Some(&cursor),
        || Ok(()),
        Stats::default,
        async || Ok(()),
    )
    .await
    .unwrap();
    let retried = read(
        repo.as_ref(),
        &cache,
        key.clone(),
        Some(&cursor),
        || Ok(()),
        Stats::default,
        async || Ok(()),
    )
    .await
    .unwrap();
    assert_eq!(next.history_commits, retried.history_commits);
    let other = Selection {
        scope: "different registration".into(),
        ..key.clone()
    };
    assert!(matches!(
        read(
            repo.as_ref(),
            &cache,
            other,
            Some(&cursor),
            || Ok(()),
            Stats::default,
            async || Ok(())
        )
        .await,
        Err(Error::BlameReset)
    ));
    cache.entries.lock().unwrap().clear();
    assert!(matches!(
        read(
            repo.as_ref(),
            &cache,
            key,
            Some(&cursor),
            || Ok(()),
            Stats::default,
            async || Ok(())
        )
        .await,
        Err(Error::BlameReset)
    ));
}

#[test]
fn cache_expiry_and_capacity_preserve_active_progress() {
    let cache = Cache::default();
    let key = key(&b::CommitId::from_bytes(&[31; 20]));
    let active = cache.entry(&key, None).unwrap();
    *active.expires.lock().unwrap() = Instant::now() - Duration::from_secs(1);
    assert!(Arc::ptr_eq(&active, &cache.entry(&key, None).unwrap()));
    drop(active);
    assert!(matches!(
        cache.entry(&key, Some("expired")),
        Err(Error::BlameReset)
    ));
    for index in 0..12 {
        let another = Selection {
            path: format!("file{index}"),
            ..key.clone()
        };
        drop(cache.entry(&another, None).unwrap());
    }
    assert_eq!(cache.entries.lock().unwrap().len(), 8);
}
