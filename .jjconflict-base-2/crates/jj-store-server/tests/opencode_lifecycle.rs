#![cfg(unix)]
#[path = "opencode_source/fixture.rs"]
mod api;
#[path = "capture_source/fixture.rs"]
#[allow(dead_code)]
mod native;
use api::{Api, message};
use jj_store_capture::opencode_ingress::IntegrationId;
use jj_store_server::{capture_source::CaptureSourceStep, opencode_source::OpenCodeCaptureSource};
use native::{A, Fixture};
async fn source(f: &mut Fixture, api: &Api) -> OpenCodeCaptureSource {
    OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
}
async fn catch_up(s: &mut OpenCodeCaptureSource, f: &mut Fixture) {
    for _ in 0..100 {
        if s.lifecycle_step(&mut f.context()).await.unwrap() == CaptureSourceStep::Enqueued {
            continue;
        }
        if s.ingest_step(&mut f.context()).await.unwrap() == CaptureSourceStep::Waiting
            && s.compact_idle()
        {
            return;
        }
    }
    panic!("API lifecycle never settled");
}
async fn publish(s: &mut OpenCodeCaptureSource, f: &mut Fixture) {
    for _ in 0..100 {
        if f.spool.pending().unwrap().is_empty() {
            return;
        }
        assert!(
            matches!(s.publish_step(&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
        );
    }
    panic!("status did not publish");
}
async fn state(f: &Fixture) -> String {
    let native = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap()
        .into_session();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        std::sync::Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(native.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    serde_json::to_string(
        &pager
            .detail(native, page.sessions[0].session_id)
            .await
            .unwrap(),
    )
    .unwrap()
}
#[tokio::test]
async fn empty_active_session_publishes_idle_after_restart_without_inventing_messages() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(directory, vec![], true).await;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    assert_eq!(f.spool.pending().unwrap().len(), 1);
    publish(&mut s, &mut f).await;
    let body = state(&f).await;
    assert!(body.contains("\"state\":\"active\""));
    assert!(!body.contains("\"message_id\""));
    drop(s);
    api.data.lock().unwrap().busy = false;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    assert_eq!(f.spool.pending().unwrap().len(), 1);
    publish(&mut s, &mut f).await;
    assert!(state(&f).await.contains("\"state\":\"idle\""));
    s.refresh(&mut f.context()).unwrap();
    catch_up(&mut s, &mut f).await;
    assert!(f.spool.pending().unwrap().is_empty());
}
#[tokio::test]
async fn queued_status_and_message_proofs_survive_current_idle_without_recap_or_identity_change() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_original",
            Some(&directory),
            "one original message",
        )],
        true,
    )
    .await;
    let mut s = source(&mut f, &api).await;
    let id = s.source_instance_id();
    catch_up(&mut s, &mut f).await;
    let old = f.spool.pending().unwrap();
    assert_eq!(old.len(), 2);
    drop(s);
    api.data.lock().unwrap().busy = false;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    let queue = f.spool.pending().unwrap();
    assert_eq!(queue.len(), 3);
    assert!(old.iter().all(|e| queue.contains(e)));
    let progress = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, id)
        .unwrap()
        .unwrap();
    assert_eq!(
        progress.metadata.opencode_snapshot.unwrap().message_records,
        1
    );
    publish(&mut s, &mut f).await;
    let body = state(&f).await;
    assert!(body.contains("one original message"));
    assert!(body.contains("\"state\":\"idle\""));
    drop(s);
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    assert!(f.spool.pending().unwrap().is_empty());
    api.data.lock().unwrap().busy = true;
    api.data.lock().unwrap().messages.push(message(
        "msg_tail",
        Some(&directory),
        "new active tail",
    ));
    s.refresh(&mut f.context()).unwrap();
    catch_up(&mut s, &mut f).await;
    assert_eq!(f.spool.pending().unwrap().len(), 2);
    publish(&mut s, &mut f).await;
    let body = state(&f).await;
    assert!(body.contains("new active tail"));
    assert!(body.contains("\"state\":\"active\""));
}

#[tokio::test]
async fn message_only_history_upgrades_without_changing_retained_intent() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_old",
            Some(&directory),
            "old message-only checkpoint",
        )],
        true,
    )
    .await;
    let mut s = source(&mut f, &api).await;
    assert_eq!(
        s.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let old = f.spool.pending().unwrap().remove(0);
    assert!(
        old.reader_metadata
            .as_ref()
            .unwrap()
            .opencode_snapshot
            .is_none()
    );
    drop(s);
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    assert_eq!(f.spool.pending().unwrap().len(), 2);
    assert!(f.spool.pending().unwrap().contains(&old));
    publish(&mut s, &mut f).await;
    assert!(state(&f).await.contains("old message-only checkpoint"));
    assert!(state(&f).await.contains("\"state\":\"active\""));
}

#[tokio::test]
async fn retry_and_repeated_status_keep_zero_message_progress_and_current_authority() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(directory, vec![], true).await;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    let busy = f.spool.pending().unwrap();
    api.data.lock().unwrap().retry = true;
    f.catalog(&[]);
    assert!(s.lifecycle_step(&mut f.context()).await.is_err());
    assert_eq!(f.spool.pending().unwrap(), busy);
    f.catalog(&[A]);
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retry = f.spool.pending().unwrap();
    assert_eq!(retry.len(), 2);
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Waiting
    );
    assert_eq!(f.spool.pending().unwrap(), retry);
    let progress = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, s.source_instance_id())
        .unwrap()
        .unwrap();
    let checkpoint = progress.metadata.opencode_snapshot.unwrap();
    assert_eq!(checkpoint.message_records, 0);
    assert_eq!(
        checkpoint.lifecycle,
        jj_store_capture::opencode_checkpoint::OpenCodeLifecycleV1::Retry
    );
    publish(&mut s, &mut f).await;
    assert!(state(&f).await.contains("\"state\":\"active\""));
}

#[tokio::test]
async fn lifecycle_spool_pressure_never_acknowledges_state() {
    let mut f = Fixture::new(128).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(directory, vec![], true).await;
    let mut s = source(&mut f, &api).await;
    assert!(s.lifecycle_step(&mut f.context()).await.is_err());
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(
        f.spool
            .latest_reader_progress(IntegrationId::OpenCode, s.source_instance_id())
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn rewritten_message_prefix_retains_status_but_withholds_original_message() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_rewrite",
            Some(&directory),
            "original native row",
        )],
        true,
    )
    .await;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    let original = f.spool.pending().unwrap();
    drop(s);
    api.data.lock().unwrap().messages[0] =
        message("msg_rewrite", Some(&directory), "rewritten native row");
    let mut s = source(&mut f, &api).await;
    assert_eq!(
        s.lifecycle_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Verifying
    );
    assert_eq!(
        s.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::ChangedDuringRead
    );
    assert_eq!(f.spool.pending().unwrap(), original);
    catch_up(&mut s, &mut f).await;
    let queue = f.spool.pending().unwrap();
    assert_eq!(queue.len(), 3);
    assert!(original.iter().all(|e| queue.contains(e)));
    let progress = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, s.source_instance_id())
        .unwrap()
        .unwrap();
    assert_eq!(progress.cursor.generation, 2);
    assert!(
        matches!(s.publish_step(&mut f.context()).await.unwrap(), CaptureSourceStep::Published(r) if r.committed == 1)
    );
    assert_eq!(
        s.publish_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Withheld
    );
}

#[tokio::test]
async fn returning_witness_recovers_original_generation_above_newer_allocation_floors() {
    let mut f = Fixture::new(16 * 1024 * 1024).await;
    let a = f.workspace(A).to_str().unwrap().to_owned();
    let b = f.workspace(native::B).to_str().unwrap().to_owned();
    let original_message = message("msg_original", Some(&a), "returning witness original");
    let api = Api::new(a.clone(), vec![original_message.clone()], true).await;
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    let original = f.spool.pending().unwrap();
    drop(s);
    {
        let mut data = api.data.lock().unwrap();
        data.directory = b.clone();
        data.messages = vec![message("msg_other", Some(&b), "different witness")];
    }
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    drop(s);
    {
        let mut data = api.data.lock().unwrap();
        data.directory = a.clone();
        data.busy = false;
        data.messages = vec![
            original_message,
            message("msg_tail", Some(&a), "returning witness tail"),
        ];
    }
    let mut s = source(&mut f, &api).await;
    catch_up(&mut s, &mut f).await;
    let progress = f
        .spool
        .latest_reader_progress(IntegrationId::OpenCode, s.source_instance_id())
        .unwrap()
        .unwrap();
    assert_eq!(progress.cursor.generation, 3);
    assert_eq!(
        progress.metadata.opencode_snapshot.unwrap().message_records,
        2
    );
    assert!(
        original
            .iter()
            .all(|e| f.spool.pending().unwrap().contains(e))
    );
    for _ in 0..12 {
        let _ = s.publish_step(&mut f.context()).await;
        if f.spool.pending().unwrap().iter().all(|e| e.repository != A) {
            break;
        }
    }
    assert!(
        f.spool
            .pending()
            .unwrap()
            .iter()
            .all(|e| e.repository == native::B)
    );
    let body = state(&f).await;
    assert!(body.contains("returning witness original"));
    assert!(body.contains("returning witness tail"));
    assert!(body.contains("\"state\":\"idle\""));
    assert!(!body.contains("different witness"));
}
