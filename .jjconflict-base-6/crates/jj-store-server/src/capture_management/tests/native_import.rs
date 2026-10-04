use super::import_policy::ImportFixture;
use super::*;
use crate::{
    capture_source::{
        CaptureImportStep, CaptureSourceStep, CodexCaptureImport, ImportedCodexCapturePipeline,
    },
    capture_test_fixture::{A, B, SECRET},
};
use jj_store_service::{auth::SystemClock, collaboration::AgentSessionPager, pagination};
use serde_json::json;
mod ownership;
mod prefix_progress;
mod reader_binding;

fn prepare(f: &ImportFixture) -> CodexCaptureImport {
    CodexCaptureImport::prepare(
        f.manager
            .begin_import(f.intent.clone(), f.imports.clone())
            .unwrap(),
        f.f.sources.clone(),
        f.f.privacy(),
        &f.f.spool,
    )
    .unwrap()
}
async fn enroll(f: &mut ImportFixture) -> ImportedCodexCapturePipeline {
    let mut import = prepare(f);
    for _ in 0..10 {
        match import.step(&mut f.f.context()).await.unwrap() {
            CaptureImportStep::Ready(pipeline) => return *pipeline,
            CaptureImportStep::Verifying | CaptureImportStep::ChangedDuringRead => {}
            CaptureImportStep::Withheld => panic!("unexpected withheld source"),
        }
    }
    panic!("bounded fixture did not settle")
}
async fn enqueue(f: &mut ImportFixture, pipeline: &mut ImportedCodexCapturePipeline) {
    for _ in 0..10 {
        match pipeline.ingest_step(&mut f.f.context()).await.unwrap() {
            CaptureSourceStep::Enqueued => return,
            CaptureSourceStep::Verifying | CaptureSourceStep::ChangedDuringRead => {}
            step => panic!("unexpected ingress: {step:?}"),
        }
    }
    panic!("bounded fixture did not enqueue")
}
async fn publish(f: &mut ImportFixture, pipeline: &mut ImportedCodexCapturePipeline) {
    for _ in 0..20 {
        let pending = f.f.spool.pending().unwrap();
        let Some(entry) = pending.first() else { return };
        let step = pipeline
            .publish_step(entry, &mut f.f.context())
            .await
            .unwrap();
        assert!(
            matches!(
                step,
                CaptureSourceStep::Verifying
                    | CaptureSourceStep::ChangedDuringRead
                    | CaptureSourceStep::Published(_)
            ),
            "{step:?}"
        );
    }
    panic!("bounded fixture did not publish")
}

#[tokio::test]
async fn selected_history_enrolls_after_native_write_and_publishes_sanitized_current_store() {
    let mut f = ImportFixture::new().await;
    f.f.message(
        "history",
        &format!(
            "historical note token={SECRET} {}/src/file",
            f.f.workspace(A).display()
        ),
    );
    let imports_before = f.imports.entries().unwrap();
    let sources_before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
    let mut import = prepare(&f);
    assert!(import.receipt().is_none());
    assert_eq!(
        fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
        sources_before
    );
    let CaptureImportStep::Ready(pipeline) = import.step(&mut f.f.context()).await.unwrap() else {
        panic!("small fixture")
    };
    let mut pipeline = *pipeline;
    assert!(imports_before.is_empty());
    assert_eq!(
        f.imports.entries().unwrap(),
        vec![pipeline.receipt().clone()]
    );
    assert!(
        f.f.sources
            .lookup_admitted(&f.f.qualified())
            .unwrap()
            .is_none()
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    enqueue(&mut f, &mut pipeline).await;
    let queued = serde_json::to_string(&f.f.spool.pending().unwrap()).unwrap();
    assert!(!queued.contains(SECRET) && !queued.contains(f.f.root.path().to_str().unwrap()));
    publish(&mut f, &mut pipeline).await;
    let session =
        f.f.host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(f.f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(
        body.contains("historical note")
            && body.contains("[REDACTED]")
            && body.contains("$WORKSPACE")
    );
    assert!(!body.contains(SECRET));
    drop(pipeline);
    assert!(f.manager.status(f.intent.principal).await.is_ok());
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn unavailable_native_write_withholds_intent_source_allocation_and_progress() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "retained private history");
    let before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
    let mut import = prepare(&f);
    f.f.without_native_authority();
    assert!(import.step(&mut f.f.context()).await.is_err());
    assert!(import.receipt().is_none());
    drop(import);
    assert!(f.imports.entries().unwrap().is_empty());
    assert_eq!(
        fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
        before
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn import_replay_uses_original_binding_and_existing_cursor_after_append() {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "original selected history");
    // A retained receipt from a previous runtime resolves before its obsolete selector.
    f.intent.selector = format!("{}:0000000000000001", "ef".repeat(16));
    let receipt = f
        .imports
        .record(&f.intent, &f.source_binding())
        .unwrap()
        .receipt;
    let mut pipeline = enroll(&mut f).await;
    enqueue(&mut f, &mut pipeline).await;
    publish(&mut f, &mut pipeline).await;
    let partition = pipeline.partition();
    let original =
        f.f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap();
    drop(pipeline);
    f.f.message("tail", "continuing appended history");
    f.select_control(7, 0, false);
    let mut replay = enroll(&mut f).await;
    assert_eq!(replay.receipt(), &receipt);
    assert_eq!(replay.partition(), partition);
    enqueue(&mut f, &mut replay).await;
    let advanced =
        f.f.spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap()
            .unwrap();
    assert_eq!(advanced.last_sequence, original.last_sequence + 1);
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
    publish(&mut f, &mut replay).await;
    drop(replay);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn imported_parking_renews_consent_and_preserves_progress_while_paused_or_unbound() {
    let mut f = ImportFixture::new().await;
    f.f.message("old", "imported history");
    let mut pipeline = enroll(&mut f).await;
    enqueue(&mut f, &mut pipeline).await;
    let receipt = pipeline.receipt().clone();
    let retained = f.f.spool.pending().unwrap();
    let mut parked = pipeline.park();
    f.select_control(7, 0, true);
    assert!(parked.resume_current(&mut f.f.context()).await.is_err());
    assert_eq!(parked.receipt(), &receipt);
    assert_eq!(f.f.spool.pending().unwrap(), retained);
    f.select_control(8, 1, false);
    f.f.catalog(&[]);
    assert!(matches!(
        parked.resume_current(&mut f.f.context()).await.unwrap(),
        CaptureImportStep::Withheld
    ));
    f.f.catalog(&[A, B]);
    let CaptureImportStep::Ready(pipeline) =
        parked.resume_current(&mut f.f.context()).await.unwrap()
    else {
        panic!("small fixture")
    };
    let mut pipeline = *pipeline;
    assert_eq!(pipeline.receipt(), &receipt);
    publish(&mut f, &mut pipeline).await;
    f.f.message("tail", "appended after parked import");
    enqueue(&mut f, &mut pipeline).await;
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn rewritten_selected_prefix_and_physical_replacement_cannot_inherit_consent() {
    for replacement in [false, true] {
        let mut f = ImportFixture::new().await;
        f.f.message("history", "original history");
        let pipeline = enroll(&mut f).await;
        let receipt = pipeline.receipt().clone();
        let mut parked = pipeline.park();
        let bytes = fs::read(&f.f.transcript).unwrap();
        if replacement {
            let replaced = f.f.transcript.with_extension("replacement");
            fs::write(&replaced, bytes).unwrap();
            fs::rename(replaced, &f.f.transcript).unwrap();
        } else {
            let changed = String::from_utf8(bytes)
                .unwrap()
                .replace("original history", "rewritten secret");
            fs::write(&f.f.transcript, changed).unwrap();
        }
        assert!(parked.resume_current(&mut f.f.context()).await.is_err());
        assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
        assert!(f.f.spool.pending().unwrap().is_empty());
        assert!(
            f.f.sources
                .lookup_admitted(&f.f.qualified())
                .unwrap()
                .is_none()
        );
        drop(parked);
        f.runtime.shutdown().await.unwrap();
    }
}

#[tokio::test]
async fn runtime_or_installation_revocation_withholds_ingestion_and_exact_queued_recovery() {
    for mode in ["disable", "authority", "pause", "stop"] {
        let mut f = ImportFixture::new().await;
        f.f.message("history", "selected private history");
        let mut pipeline = enroll(&mut f).await;
        enqueue(&mut f, &mut pipeline).await;
        let receipt = pipeline.receipt().clone();
        let retained = f.f.spool.pending().unwrap();
        f.f.message("tail", "withheld private tail");
        match mode {
            "disable" => {
                let mut config = f.config.clone();
                config.agent_capture.enabled = false;
                f.save(&config);
            }
            "authority" => {
                let mut config = f.config.clone();
                let Access::LocalTrusted { permission, .. } = &mut config.access else {
                    unreachable!()
                };
                *permission = Permission::Read;
                f.save(&config);
            }
            "pause" => {
                f.runtime.controls().pause().unwrap();
            }
            "stop" => {
                f.runtime.shutdown().await.unwrap();
            }
            _ => unreachable!(),
        }
        assert!(
            pipeline.ingest_step(&mut f.f.context()).await.is_err(),
            "{mode}"
        );
        assert!(
            pipeline
                .publish_step(&retained[0], &mut f.f.context())
                .await
                .is_err(),
            "{mode}"
        );
        assert_eq!(f.f.spool.pending().unwrap(), retained);
        assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
        drop(pipeline);
        if mode != "stop" {
            f.runtime.shutdown().await.unwrap();
        }
    }
}

#[tokio::test]
async fn native_import_preparation_remains_bounded_and_selects_last_complete_record() {
    let mut f = ImportFixture::new().await;
    for _ in 0..5 {
        f.f.append_bytes(
            format!("{{\"unknown\":\"{}\"}}\n", "x".repeat(1024 * 1024 - 32)).as_bytes(),
        );
    }
    f.f.append(json!({"type":"turn_context","payload":{"cwd":f.f.workspace(A)}}));
    f.f.message("last", "last complete imported record");
    let complete = fs::metadata(&f.f.transcript).unwrap().len();
    f.f.append_bytes(b"{\"unfinished\":");
    let sources_before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
    let mut import = prepare(&f);
    assert!(matches!(
        import.step(&mut f.f.context()).await.unwrap(),
        CaptureImportStep::Verifying
    ));
    assert!(import.receipt().is_none());
    assert_eq!(
        fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
        sources_before
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    let CaptureImportStep::Ready(pipeline) = import.step(&mut f.f.context()).await.unwrap() else {
        panic!("second bounded step")
    };
    assert_eq!(pipeline.receipt().source.complete_byte_offset, complete);
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}
