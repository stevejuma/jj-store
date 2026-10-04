#![cfg(unix)]
#[path = "capture_supervisor/backlog.rs"]
mod backlog;
#[path = "capture_supervisor/blocked.rs"]
mod blocked;
#[path = "capture_supervisor/capacity.rs"]
mod capacity;
#[path = "capture_source/fixture.rs"]
mod fixture;
#[path = "capture_supervisor/generations.rs"]
mod generations;
#[path = "capture_supervisor/headroom.rs"]
mod headroom;
#[path = "capture_supervisor/raw_channels.rs"]
mod raw_channels;
#[path = "capture_supervisor/raw_handoff.rs"]
mod raw_handoff;
#[path = "capture_supervisor/rearming.rs"]
mod rearming;
#[path = "capture_supervisor/rotation.rs"]
mod rotation;
#[path = "capture_supervisor/saturation.rs"]
mod saturation;
#[path = "capture_supervisor/task_loss.rs"]
mod task_loss;
use fixture::*;
use jj_store_capture::{agent_capture::AgentCaptureSpool, profile::CaptureProfile};
use jj_store_server::{
    capture_config::{AgentCaptureConfig, HarnessConfig},
    capture_supervisor::{
        CodexCaptureSupervisor, CodexSupervisorLocations, CodexSupervisorPoll,
        CodexSupervisorResources,
    },
    capture_worker::CaptureWorkerPhase,
};
use serde_json::json;
use std::{fs, sync::Arc, time::Duration};

async fn quantum(worker: &mut CodexCaptureSupervisor) {
    worker.start_step().unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            match worker.poll().await {
                CodexSupervisorPoll::Running => tokio::time::sleep(Duration::from_millis(1)).await,
                CodexSupervisorPoll::Completed(_) => break,
                other => panic!("unexpected supervisor completion: {other:?}"),
            }
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn enabled_supervisor_backfills_new_start_without_ingesting_inactive_history() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    f.message("history", &format!("eligible backfill {SECRET}"));
    let inactive = f.transcript.with_file_name("inactive.jsonl");
    let old = fs::read_to_string(&f.transcript).unwrap().replace(
        "00000000-0000-0000-0000-000000000001",
        "00000000-0000-0000-0000-000000000002",
    );
    fs::write(&inactive, format!("{old}{}\n{}\n", json!({"type":"event_msg","payload":{"type":"task_started"}}),json!({"type":"response_item","payload":{"type":"message","id":"inactive","role":"assistant","content":[{"type":"output_text","text":"inactive history must remain untouched"}]}}))).unwrap();
    let config = AgentCaptureConfig {
        enabled: true,
        max_readers: 1,
        max_sources: 4,
        max_pending_bytes: 4 * 1024 * 1024,
        harnesses: vec![HarnessConfig {
            profile: CaptureProfile::CodexCliJsonlV1,
            product_version: "0.156.1".into(),
            roots: vec![f.transcript.parent().unwrap().to_owned()],
            endpoint: None,
        }],
        ..Default::default()
    };
    let privacy = f.privacy();
    let transcript = f.transcript.clone();
    let sources = f.sources.clone();
    let spool_path = f.root.path().join("spool");
    let native = f
        .host
        .endpoint(A)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap();
    let heads = native.heads().await.unwrap();
    let host = f.host.clone();
    let principal = f.authority.principal();
    let mut worker = CodexCaptureSupervisor::prepare(
        config,
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(f.authority),
            host: f.host,
            admission: f.admission,
            sources: sources.clone(),
            spool: f.spool,
            privacy,
        },
        Duration::from_secs(30),
    )
    .unwrap()
    .unwrap();
    settle(&mut worker, |s| {
        s.tracked == 2
            && s.observed_sources == 2
            && s.awaiting_activity == 2
            && s.resident_source_frames == 0
    })
    .await;
    assert_eq!(worker.status().tracked, 2);
    assert_eq!(worker.status().awaiting_activity, 2);
    let spool = AgentCaptureSpool::open(&spool_path, 4 * 1024 * 1024).unwrap();
    assert!(spool.pending().unwrap().is_empty());
    let registry = fs::read_to_string(f.root.path().join("sources/sources.json")).unwrap();
    assert!(
        serde_json::from_str::<serde_json::Value>(&registry).unwrap()["sources"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    use std::io::Write as _;
    writeln!(
        fs::OpenOptions::new()
            .append(true)
            .open(&transcript)
            .unwrap(),
        "{}",
        json!({"type":"event_msg","payload":{"type":"task_started","turn_id":"new-start"}})
    )
    .unwrap();
    settle(&mut worker, |s| {
        s.awaiting_activity == 1 && s.caught_up == 1
    })
    .await;
    assert_eq!(worker.status().awaiting_activity, 1);
    assert_eq!(worker.status().caught_up, 1);
    assert!(spool.pending().unwrap().is_empty());
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let session = native.into_session();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session.clone(), page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(body.contains("eligible backfill"));
    assert!(!body.contains(SECRET));
    assert!(!body.contains("inactive history must remain untouched"));
    assert_eq!(
        host.endpoint(A)
            .unwrap()
            .bind_local_trusted(principal, jj_store_service::auth::Permission::Read)
            .unwrap()
            .heads()
            .await
            .unwrap(),
        heads
    );
    worker.shutdown().await.unwrap();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
}

async fn settle(
    worker: &mut CodexCaptureSupervisor,
    predicate: impl Fn(jj_store_server::capture_supervisor::CodexSupervisorStatus) -> bool,
) {
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(worker).await;
            while matches!(
                worker.phase(),
                CaptureWorkerPhase::Running | CaptureWorkerPhase::Overdue
            ) {
                worker.poll().await;
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            if predicate(worker.status()) {
                break;
            }
        }
    })
    .await
    .unwrap_or_else(|_| panic!("supervisor did not settle: {:?}", worker.status()));
}

fn resources(f: &Fixture) -> CodexSupervisorResources {
    CodexSupervisorResources {
        authority: Arc::new(
            jj_store_server::capture_authority::CaptureAuthority::prepare(
                &jj_store_server::config::Config::load(&f.root.path().join("server.toml")).unwrap(),
            )
            .unwrap()
            .unwrap(),
        ),
        host: f.host.clone(),
        admission: f.admission.clone(),
        sources: f.sources.clone(),
        spool: f.spool.fork_handle(),
        privacy: f.privacy(),
    }
}
fn config(f: &Fixture) -> AgentCaptureConfig {
    AgentCaptureConfig {
        enabled: true,
        max_readers: 1,
        max_sources: 4,
        rescan_seconds: 1,
        max_pending_bytes: 4 * 1024 * 1024,
        harnesses: vec![HarnessConfig {
            profile: CaptureProfile::CodexCliJsonlV1,
            product_version: "0.156.1".into(),
            roots: vec![f.transcript.parent().unwrap().to_owned()],
            endpoint: None,
        }],
    }
}
fn supervisor(f: &Fixture, config: AgentCaptureConfig) -> CodexCaptureSupervisor {
    CodexCaptureSupervisor::prepare(
        config,
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        resources(f),
        Duration::from_secs(30),
    )
    .unwrap()
    .unwrap()
}
async fn quiesce(worker: &mut CodexCaptureSupervisor) {
    tokio::time::timeout(Duration::from_secs(30), async {
        while matches!(
            worker.phase(),
            CaptureWorkerPhase::Running
                | CaptureWorkerPhase::Overdue
                | CaptureWorkerPhase::Draining
        ) {
            worker.poll().await;
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn disabled_selection_does_not_open_roots_or_dispatch() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let missing = f.root.path().join("missing");
    let mut settings = config(&f);
    settings.enabled = false;
    settings.harnesses[0].roots = vec![missing.clone()];
    assert!(
        CodexCaptureSupervisor::prepare(
            settings,
            CodexSupervisorLocations {
                home: None,
                codex_home: None
            },
            resources(&f),
            Duration::ZERO
        )
        .unwrap()
        .is_none()
    );
    assert!(!missing.exists());
    assert!(f.spool.pending().unwrap().is_empty());
    let mut settings = config(&f);
    settings.max_pending_bytes = 1;
    assert!(
        CodexCaptureSupervisor::prepare(
            settings,
            CodexSupervisorLocations {
                home: None,
                codex_home: None
            },
            resources(&f),
            Duration::from_secs(30)
        )
        .is_err()
    );
}

#[tokio::test(flavor = "current_thread")]
async fn overlap_and_hardlinks_are_deduplicated_and_tracking_is_bounded() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let root = f.transcript.parent().unwrap();
    fs::hard_link(&f.transcript, root.join("hardlink.jsonl")).unwrap();
    let original = fs::read_to_string(&f.transcript).unwrap();
    for id in 2..=4 {
        fs::write(
            root.join(format!("{id}.jsonl")),
            original.replace("000000000001", &format!("{id:012}")),
        )
        .unwrap();
    }
    let mut settings = config(&f);
    settings.max_sources = 2;
    settings.harnesses[0].roots.push(f.root.path().to_owned());
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 2 && s.source_limit_reached && s.resident_source_frames == 0
    })
    .await;
    assert_eq!(worker.status().awaiting_activity, 2);
    assert!(f.spool.pending().unwrap().is_empty());
    worker.shutdown().await.unwrap();
    // With enough tracking capacity, the two paths for the first file still
    // produce only one probe, including across periodic rescans.
    let mut settings = config(&f);
    settings.harnesses[0].roots.push(f.root.path().to_owned());
    let mut worker = supervisor(&f, settings);
    settle(&mut worker, |s| {
        s.tracked == 4 && s.resident_source_frames == 0
    })
    .await;
    tokio::time::sleep(Duration::from_millis(1050)).await;
    for _ in 0..24 {
        quantum(&mut worker).await;
        quiesce(&mut worker).await;
    }
    assert_eq!(worker.status().tracked, 4);
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn pause_preserves_armed_activity_and_shutdown_releases_resources() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    worker.pause();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Paused);
    assert!(worker.start_step().is_err());
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("paused", "activity retained while paused");
    assert!(f.spool.pending().unwrap().is_empty());
    worker.resume().unwrap();
    settle(&mut worker, |s| s.caught_up == 1).await;
    worker.shutdown().await.unwrap();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
    assert!(worker.start_step().is_err());
    assert!(worker.resume().is_err());
    assert!(f.sources.acquire_admitted(f.qualified()).unwrap().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn activity_without_current_native_write_cannot_enroll_or_acknowledge() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    f.without_native_authority();
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("denied", SECRET);
    settle(&mut worker, |s| s.unavailable > 0).await;
    assert_eq!(worker.status().awaiting_activity, 1);
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    assert!(worker.status().resident_source_frames <= 2);
    assert_eq!(worker.status().ingress_capacity_refusals, 0);
    assert_eq!(worker.status().ingress_required_capacity_bytes, None);
    assert_eq!(worker.status().delivery_required_capacity_bytes, None);
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn admitted_offline_tail_delivers_b_while_long_a_proof_is_withheld() {
    use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let active = f.start();
    let source = CodexCaptureSource::acquire_active(
        f.sources.clone(),
        active,
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap();
    let mut pipeline = source.into_pipeline();
    // An oversized row exercises several bounded proof/reader turns without
    // persisting its body. B's exact range is beyond the same long prefix.
    f.append_bytes(&vec![b'x'; 9 * 1024 * 1024]);
    f.append_bytes(b"\n");
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", &format!("independent offline b {SECRET}"));
    for _ in 0..40 {
        if pipeline.ingest_step(&mut f.context()).await.unwrap() == CaptureSourceStep::Waiting
            && f.spool.pending().unwrap().iter().any(|e| e.repository == B)
        {
            break;
        }
    }
    let retained: Vec<_> = f
        .spool
        .pending()
        .unwrap()
        .into_iter()
        .filter(|e| e.repository == A)
        .collect();
    assert!(!retained.is_empty());
    assert!(f.spool.pending().unwrap().iter().any(|e| e.repository == B));
    drop(pipeline);
    f.catalog(&[B]);
    f.message("offline", "final offline tail");
    let mut worker = supervisor(&f, config(&f));
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(&mut worker).await;
            quiesce(&mut worker).await;
            let s = worker.status();
            assert!(s.running_sources <= 1);
            assert!(s.resident_source_frames <= 2);
            assert!(s.resident_delivery_frames <= 2);
            if !f.spool.pending().unwrap().iter().any(|e| e.repository == B)
                && s.awaiting_activity == 0
            {
                break;
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(f.spool.pending().unwrap(), retained);
    assert_eq!(worker.status().caught_up, 0);
    let request = f
        .host
        .endpoint(B)
        .unwrap()
        .bind_local_trusted(
            f.authority.principal(),
            jj_store_service::auth::Permission::Read,
        )
        .unwrap();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let session = request.into_session();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    let detail = pager
        .detail(session, page.sessions[0].session_id)
        .await
        .unwrap();
    let body = serde_json::to_string(&detail).unwrap();
    assert!(body.contains("independent offline b"));
    assert!(!body.contains(SECRET));
    f.catalog(&[A, B]);
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert!(f.spool.pending().unwrap().is_empty());
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn live_parked_session_hands_off_to_b_while_exact_a_intent_remains_withheld() {
    use jj_store_server::capture_source::{CaptureSourceStep, CodexCaptureSource};
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let mut worker = supervisor(&f, config(&f));
    settle(&mut worker, |s| {
        s.tracked == 1 && s.observed_sources == 1 && s.resident_source_frames == 0
    })
    .await;
    f.append(json!({"type":"event_msg","payload":{"type":"task_started"}}));
    f.message("a", "first live repository A body");
    settle(&mut worker, |s| s.caught_up == 1).await;
    worker.pause();
    quiesce(&mut worker).await;
    // A separate admitted owner leaves exact durable A intent while the
    // supervisor retains its older parked frame at the preceding cursor.
    f.message("pending-a", "withheld repository A intent");
    let mut other = CodexCaptureSource::acquire_admitted(
        f.sources.clone(),
        f.qualified(),
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap()
    .into_pipeline();
    assert_eq!(
        other.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = f.spool.pending().unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].repository, A);
    drop(other);
    f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
    f.message("b", &format!("live current B body {SECRET}"));
    f.catalog(&[B]);
    worker.resume().unwrap();
    let pager = jj_store_service::collaboration::AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(&mut worker).await;
            quiesce(&mut worker).await;
            let s = worker.status();
            assert!(s.running_sources <= 1);
            assert!(s.resident_source_frames <= 2);
            assert!(s.resident_delivery_frames <= 2);
            let read = f
                .host
                .endpoint(B)
                .unwrap()
                .bind_local_trusted(
                    f.authority.principal(),
                    jj_store_service::auth::Permission::Read,
                )
                .unwrap()
                .into_session();
            let page = pager.begin(read.clone(), 10).await.unwrap();
            if let Some(session) = page.sessions.first() {
                let detail = pager.detail(read, session.session_id).await.unwrap();
                let body = serde_json::to_string(&detail).unwrap();
                if body.contains("live current B body") {
                    assert!(!body.contains(SECRET));
                    assert!(!body.contains("withheld repository A intent"));
                    break;
                }
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(f.spool.pending().unwrap(), retained);
    assert_eq!(worker.status().caught_up, 0);
    f.catalog(&[A, B]);
    settle(&mut worker, |s| s.caught_up == 1).await;
    assert!(f.spool.pending().unwrap().is_empty());
    worker.shutdown().await.unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn multi_step_inactive_probes_share_bounded_frames_and_all_reach_compact_eof() {
    let f = Fixture::new(4 * 1024 * 1024).await;
    let header = fs::read_to_string(&f.transcript).unwrap();
    for id in 1..=4 {
        let path = if id == 1 {
            f.transcript.clone()
        } else {
            f.transcript.with_file_name(format!("{id}.jsonl"))
        };
        fs::write(
            path,
            [
                header
                    .replace("000000000001", &format!("{id:012}"))
                    .as_bytes(),
                &vec![b'x'; 3 * 1024 * 1024],
            ]
            .concat(),
        )
        .unwrap();
    }
    let mut worker = supervisor(&f, config(&f));
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            quantum(&mut worker).await;
            quiesce(&mut worker).await;
            let s = worker.status();
            assert!(s.running_sources <= 1);
            assert!(s.resident_source_frames <= 2);
            if s.observed_sources == 4 && s.resident_source_frames == 0 {
                break;
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(worker.status().awaiting_activity, 4);
    assert!(f.spool.pending().unwrap().is_empty());
    assert!(f.sources.lookup_admitted(&f.qualified()).unwrap().is_none());
    worker.shutdown().await.unwrap();
}
