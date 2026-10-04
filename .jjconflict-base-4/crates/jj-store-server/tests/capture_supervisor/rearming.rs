use super::*;

#[tokio::test(flavor = "current_thread")]
async fn unadmitted_prefix_rewrite_rearms_without_admitting_rewritten_history() {
    rearm_after_changed_prefix(false).await;
}

#[tokio::test(flavor = "current_thread")]
async fn unadmitted_truncation_rearms_without_admitting_rewritten_history() {
    rearm_after_changed_prefix(true).await;
}

async fn rearm_after_changed_prefix(truncate: bool) {
    let f = Fixture::new(4 * 1024 * 1024).await;
    f.message("old", &"old inactive text ".repeat(128));
    let initial = fs::read_to_string(&f.transcript).unwrap();
    let original = f.qualified();
    let catalog = fs::read(f.root.path().join("sources/sources.json")).unwrap();
    let read = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap();
    let heads = read.heads().await.unwrap();
    let mut settings = config(&f);
    settings.max_sources = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    let unavailable = worker.status().unavailable;
    // Keep the physical file and qualified header: discovery deduplication must
    // not conceal a terminally invalidated activity probe at this same slot.
    let changed = if truncate {
        format!("{}\n", initial.lines().next().unwrap())
    } else {
        initial.replace("old inactive text", "new inactive text")
    };
    fs::write(&f.transcript, &changed).unwrap();
    f.append(
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"rewritten-history"}}),
    );
    f.message("history", "start already present when rearmed");
    assert!(original.is_current().unwrap());
    if truncate {
        assert!(fs::metadata(&f.transcript).unwrap().len() < initial.len() as u64);
    }
    settle(&mut worker, |s| {
        s.unavailable > unavailable
            && s.tracked == 1
            && s.awaiting_activity == 1
            && s.resident_source_frames == 0
    })
    .await;
    worker.pause();
    quiesce(&mut worker).await;
    worker.resume().unwrap();
    for _ in 0..8 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(worker.status().tracked, 1);
        assert_eq!(worker.status().awaiting_activity, 1);
        assert_eq!(worker.status().resident_source_frames, 0);
        assert_eq!(worker.status().caught_up, 0);
        assert!(!worker.status().source_limit_reached);
        assert!(f.spool.pending().unwrap().is_empty());
        assert!(f.sources.lookup_admitted(&original).unwrap().is_none());
        assert_eq!(
            fs::read(f.root.path().join("sources/sources.json")).unwrap(),
            catalog
        );
    }
    // Growth alone cannot recover eligibility. Only a genuinely new start
    // beyond the newly armed boundary admits sanitized backfill and this tail.
    f.message("growth", "growth does not establish activity");
    settle(&mut worker, |s| s.resident_source_frames == 0).await;
    assert!(f.sources.lookup_admitted(&original).unwrap().is_none());
    f.append(json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"fresh-start"}}));
    f.message(
        "fresh",
        &format!("fresh activity after probe rearming {SECRET}"),
    );
    settle(&mut worker, |s| s.caught_up == 1).await;
    let source = f.sources.lookup_admitted(&original).unwrap().unwrap();
    let integration = serde_json::from_value(json!("codex")).unwrap();
    assert!(
        f.spool
            .latest_reader_progress(integration, source)
            .unwrap()
            .is_some()
    );
    assert!(f.spool.pending().unwrap().is_empty());
    assert_eq!(read.heads().await.unwrap(), heads);
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let session = read.into_session();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(body.contains("fresh activity after probe rearming"));
    assert!(body.contains("start already present when rearmed"));
    assert!(!body.contains("old inactive text"));
    assert!(!body.contains(SECRET));
    worker.shutdown().await.unwrap();
}
