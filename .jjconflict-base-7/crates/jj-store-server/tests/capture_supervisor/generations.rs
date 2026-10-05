use super::*;
use jj_store_capture::reader_progress::ReaderProgressV1;
use std::io::Write as _;

#[tokio::test(flavor = "current_thread")]
async fn retained_physical_generations_do_not_replay_unchanged_files() {
    let f = Fixture::new(8 * 1024 * 1024).await;
    let mut settings = config(&f);
    settings.max_pending_bytes = 8 * 1024 * 1024;
    let header = fs::read_to_string(&f.transcript).unwrap();
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message(
        "original",
        "original accepted evidence before physical rotation",
    );
    settle(&mut worker, |s| s.caught_up == 1).await;
    let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    let integration = serde_json::from_value(json!("codex")).unwrap();
    let original = f
        .spool
        .latest_reader_progress(integration, source)
        .unwrap()
        .unwrap();
    worker.pause();
    quiesce(&mut worker).await;
    let archive = f.transcript.with_file_name("archive.jsonl");
    fs::rename(&f.transcript, &archive).unwrap();
    fs::write(&f.transcript, &header).unwrap();
    f.message("rotated", &format!("new physical file body {SECRET}"));
    assert_ne!(
        fs::metadata(&archive).unwrap().len(),
        fs::metadata(&f.transcript).unwrap().len()
    );
    worker.resume().unwrap();
    let length = fs::metadata(&f.transcript).unwrap().len();
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(&mut worker).await;
            quiesce(&mut worker).await;
            if worker.status().tracked == 2
                && f.spool.latest_reader_progress(integration, source).unwrap().is_some_and(|p| {
                    p.cursor.generation > original.cursor.generation
                        && serde_json::to_value(&p.cursor.position).unwrap()["complete_byte_offset"] == length
                }) {
                break;
            }
        }
    }).await.unwrap();
    let accepted = f
        .spool
        .latest_reader_progress(integration, source)
        .unwrap()
        .unwrap();
    for _ in 0..16 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(
            f.spool.latest_reader_progress(integration, source).unwrap(),
            Some(accepted.clone()),
            "unchanged archive/current files cannot advance acknowledged generations"
        );
    }
    worker.shutdown().await.unwrap();
}

async fn advanced(
    f: &Fixture,
    worker: &mut CodexCaptureSupervisor,
    previous: &ReaderProgressV1,
) -> ReaderProgressV1 {
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(worker).await;
            quiesce(worker).await;
            let progress = f
                .spool
                .latest_reader_progress(
                    previous.cursor.integration,
                    previous.cursor.source_instance_id,
                )
                .unwrap()
                .unwrap();
            if progress.cursor.generation > previous.cursor.generation
                && f.spool.pending().unwrap().is_empty()
            {
                return progress;
            }
        }
    })
    .await
    .unwrap()
}
fn row(id: &str, text: &str) -> String {
    format!(
        "{}\n",
        json!({"type":"response_item","payload":{"type":"message","id":id,"role":"assistant","content":[{"type":"output_text","text":text}]}})
    )
}
fn offset(progress: &ReaderProgressV1) -> u64 {
    serde_json::to_value(&progress.cursor.position).unwrap()["complete_byte_offset"]
        .as_u64()
        .unwrap()
}

#[tokio::test(flavor = "current_thread")]
async fn delayed_archived_writes_rewrites_and_restart_preserve_prefixes_and_channel_order() {
    let f = Fixture::new(8 * 1024 * 1024).await;
    let mut settings = config(&f);
    settings.max_pending_bytes = 8 * 1024 * 1024;
    let header = fs::read_to_string(&f.transcript).unwrap();
    let mut worker = supervisor(&f, settings.clone());
    settle(&mut worker, |s| {
        s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("original", "one original row");
    settle(&mut worker, |s| s.caught_up == 1).await;
    let source = f.sources.lookup_admitted(&f.qualified()).unwrap().unwrap();
    let integration = serde_json::from_value(json!("codex")).unwrap();
    let original = f
        .spool
        .latest_reader_progress(integration, source)
        .unwrap()
        .unwrap();
    worker.pause();
    quiesce(&mut worker).await;
    let archive = f.transcript.with_file_name("archive.jsonl");
    fs::rename(&f.transcript, &archive).unwrap();
    fs::write(&f.transcript, &header).unwrap();
    f.message("new-file", &format!("current physical file {SECRET}"));
    worker.resume().unwrap();
    let rotated = advanced(&f, &mut worker, &original).await;
    assert_eq!(rotated.last_sequence, original.last_sequence + 2); // header and message
    assert_eq!(offset(&rotated), fs::metadata(&f.transcript).unwrap().len());

    fs::OpenOptions::new()
        .append(true)
        .open(&archive)
        .unwrap()
        .write_all(row("late-archive", "one delayed archive row").as_bytes())
        .unwrap();
    let delayed = advanced(&f, &mut worker, &rotated).await;
    assert_eq!(delayed.last_sequence, rotated.last_sequence + 1);
    assert_eq!(offset(&delayed), fs::metadata(&archive).unwrap().len());
    assert_eq!(delayed.metadata, original.metadata);

    f.message("late-current", "one delayed current row");
    let current = advanced(&f, &mut worker, &delayed).await;
    assert_eq!(current.last_sequence, delayed.last_sequence + 1);
    assert_eq!(current.metadata, rotated.metadata);
    assert_eq!(offset(&current), fs::metadata(&f.transcript).unwrap().len());
    // Same physical/header witness, shorter file: requalify above every channel generation.
    fs::write(
        &archive,
        format!("{header}{}", row("truncated", "after truncate")),
    )
    .unwrap();
    assert!(fs::metadata(&archive).unwrap().len() < offset(&delayed));
    let truncated = advanced(&f, &mut worker, &current).await;
    assert_eq!(truncated.last_sequence, current.last_sequence + 2);
    assert_eq!(truncated.metadata, original.metadata);
    // Equal-length acknowledged prefix rewrite must also create a fresh generation.
    let rewritten = fs::read_to_string(&archive)
        .unwrap()
        .replace("after truncate", "after rewrites");
    assert_eq!(rewritten.len() as u64, offset(&truncated));
    fs::write(&archive, rewritten).unwrap();
    let rewritten = advanced(&f, &mut worker, &truncated).await;
    assert_eq!(rewritten.last_sequence, truncated.last_sequence + 2);
    worker.shutdown().await.unwrap();
    drop(worker);

    let progress = f
        .spool
        .source_reader_progress(integration, source, rotated.metadata.generation_digest)
        .unwrap();
    assert_eq!(progress.progress, Some(current));
    assert_eq!(progress.generation_floor, rewritten.cursor.generation);
    assert_eq!(progress.sequence_floor, rewritten.last_sequence);
    let recovered = AgentCaptureSpool::open(&f.root.path().join("spool"), 8 * 1024 * 1024).unwrap();
    assert_eq!(
        recovered
            .source_reader_progress(integration, source, rotated.metadata.generation_digest)
            .unwrap(),
        progress
    );
    drop(recovered);
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| s.tracked == 2 && s.caught_up == 2).await;
    for _ in 0..12 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
        assert_eq!(
            f.spool.latest_reader_progress(integration, source).unwrap(),
            Some(rewritten.clone())
        );
        assert!(f.spool.pending().unwrap().is_empty());
    }
    worker.shutdown().await.unwrap();
}
