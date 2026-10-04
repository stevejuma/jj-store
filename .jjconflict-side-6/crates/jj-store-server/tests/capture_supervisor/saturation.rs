use super::*;
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    source_registry::QualifiedCodexSource,
};
use serde_json::Value;
use std::{io::Write as _, path::Path};

fn append(path: &Path, value: Value) {
    writeln!(
        fs::OpenOptions::new().append(true).open(path).unwrap(),
        "{value}"
    )
    .unwrap();
}
fn message(path: &Path, id: usize, text: &str) {
    append(
        path,
        json!({"type":"response_item","payload":{"type":"message","id":format!("message-{id}-{text}"),"role":"assistant","content":[{"type":"output_text","text":text}]}}),
    );
}
fn qualified(path: &Path) -> QualifiedCodexSource {
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let canonical = fs::canonicalize(path).unwrap();
    let candidate = scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .find(|c| c.path() == canonical)
        .unwrap();
    QualifiedCodexSource::inspect(CaptureProfile::CodexCliJsonlV1, candidate)
        .unwrap()
        .unwrap()
}
async fn body(f: &Fixture) -> (usize, String) {
    let session = f
        .host
        .endpoint(B)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let mut bodies = String::new();
    for item in &page.sessions {
        let detail = pager
            .detail(session.clone(), item.session_id)
            .await
            .unwrap();
        bodies.push_str(&serde_json::to_string(&detail).unwrap());
    }
    (page.sessions.len(), bodies)
}

#[tokio::test(flavor = "current_thread")]
async fn saturated_frames_keep_unacknowledged_intent_and_restart_only_admitted_sessions() {
    let mut f = Fixture::new(1).await;
    let header = fs::read_to_string(&f.transcript).unwrap();
    let mut paths = vec![f.transcript.clone()];
    for id in 2..=4 {
        let path = f.transcript.with_file_name(format!("source-{id}.jsonl"));
        fs::write(&path, header.replace("000000000001", &format!("{id:012}"))).unwrap();
        paths.push(path);
    }
    let mut settings = config(&f);
    settings.max_pending_bytes = 1;
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 4
            && s.observed_sources == 4
            && s.awaiting_activity == 4
            && s.resident_source_frames == 0
    })
    .await;
    for (id, path) in paths.iter().enumerate() {
        append(
            path,
            json!({"type":"event_msg","payload":{"type":"task_started","turn_id":format!("original-{id}")}}),
        );
        message(path, id, &format!("old A body {id} {SECRET}"));
    }
    settle(&mut worker, |s| {
        s.resident_source_frames == 2 && s.awaiting_activity == 2 && s.unavailable > 0
    })
    .await;
    let admitted: Vec<_> = paths
        .iter()
        .filter_map(|p| f.sources.lookup_admitted(&qualified(p)).unwrap())
        .collect();
    assert_eq!(admitted.len(), 2);
    let integration = serde_json::from_value(json!("codex")).unwrap();
    assert!(f.spool.pending().unwrap().is_empty());
    for source in &admitted {
        assert!(
            f.spool
                .latest_reader_progress(integration, *source)
                .unwrap()
                .is_none()
        );
    }
    for (id, path) in paths.iter().enumerate() {
        append(
            path,
            json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}),
        );
        message(path, id, &format!("recoverable B body {id} {SECRET}"));
    }
    f.catalog(&[B]);
    // No unacknowledged raw frame may be evicted to admit another source.
    for _ in 0..24 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        let s = worker.status();
        assert!(s.running_sources <= 1);
        assert_eq!(s.resident_source_frames, 2);
        assert!(s.resident_delivery_frames <= 2);
        assert_eq!(s.caught_up, 0);
        assert_eq!(s.awaiting_activity, 2);
        assert!(f.spool.pending().unwrap().is_empty());
    }
    worker.pause();
    assert!(worker.start_step().is_err());
    worker.shutdown().await.unwrap();
    assert_eq!(worker.status().resident_source_frames, 0);
    drop(worker);
    // Only sanitized journals and exact native admission survive worker restart.
    // Increasing the configured capacity is explicit; raw bytes are reconstructed.
    f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| {
        s.tracked == 4 && s.awaiting_activity == 2 && s.caught_up == 2
    })
    .await;
    let (count, captured) = body(&f).await;
    assert_eq!(count, 2);
    assert!(captured.contains("recoverable B body"));
    assert!(!captured.contains("old A body"));
    assert!(!captured.contains(SECRET));
    assert!(f.spool.pending().unwrap().is_empty());
    for source in &admitted {
        assert!(
            f.spool
                .latest_reader_progress(integration, *source)
                .unwrap()
                .is_some()
        );
    }
    // Unadmitted historical starts are withheld until genuinely new activity.
    for (id, path) in paths.iter().enumerate() {
        if f.sources
            .lookup_admitted(&qualified(path))
            .unwrap()
            .is_none()
        {
            append(
                path,
                json!({"type":"event_msg","payload":{"type":"task_started","turn_id":format!("fresh-{id}")}}),
            );
            message(path, id, &format!("new B activity {id}"));
        }
    }
    settle(&mut worker, |s| {
        s.awaiting_activity == 0 && s.caught_up == 4
    })
    .await;
    let (count, captured) = body(&f).await;
    assert_eq!(count, 4);
    assert!(captured.contains("new B activity"));
    assert!(!captured.contains("old A body"));
    assert!(!captured.contains(SECRET));
    assert!(f.spool.pending().unwrap().is_empty());
    worker.shutdown().await.unwrap();
}
