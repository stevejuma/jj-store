use super::*;
use crate::capture_test_fixture as fixture;
use crate::{
    capture_config::HarnessConfig,
    capture_source::{CaptureSourceStep, CodexCaptureSource},
};
use fixture::{Fixture, SECRET};
use jj_store_capture::profile::CaptureProfile;
use std::{fs, sync::mpsc};
mod observations;
mod rebinding;
mod running_loss;

fn supervisor(f: &Fixture) -> CodexCaptureSupervisor {
    let config = crate::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    CodexCaptureSupervisor::prepare(
        AgentCaptureConfig {
            enabled: true,
            max_readers: 1,
            max_sources: 4,
            rescan_seconds: 1,
            max_pending_bytes: f.spool.pending_capacity(),
            harnesses: vec![HarnessConfig {
                profile: CaptureProfile::CodexCliJsonlV1,
                product_version: "0.156.1".into(),
                roots: vec![f.transcript.parent().unwrap().to_owned()],
                endpoint: None,
            }],
        },
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(CaptureAuthority::prepare(&config).unwrap().unwrap()),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(2),
    )
    .unwrap()
    .unwrap()
}
async fn settle(worker: &mut CodexCaptureSupervisor) {
    tokio::time::timeout(Duration::from_secs(10), async {
        while matches!(
            worker.phase(),
            CaptureWorkerPhase::Running
                | CaptureWorkerPhase::Overdue
                | CaptureWorkerPhase::Draining
        ) {
            worker.poll().await;
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        worker.poll().await;
    })
    .await
    .unwrap();
}

#[test]
fn queued_discovery_and_delivery_loss_preserve_exact_intent_until_explicit_replacement() {
    for discovery in [true, false] {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .max_blocking_threads(4)
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let mut f = Fixture::new(4 * 1024 * 1024).await;
            f.message(
                "retained",
                &format!("exact capture intent before lane loss {SECRET}"),
            );
            let mut pipeline = CodexCaptureSource::acquire_active(
                f.sources.clone(),
                f.start(),
                f.privacy(),
                &mut f.context(),
            )
            .await
            .unwrap()
            .into_pipeline();
            assert_eq!(
                pipeline.ingest_step(&mut f.context()).await.unwrap(),
                CaptureSourceStep::Enqueued
            );
            let partition = pipeline.partition();
            drop(pipeline);
            let queue = f.spool.pending().unwrap();
            let path = f
                .root
                .path()
                .join("spool/pending")
                .join(format!("{}.json", queue[0].batch.batch_id));
            let bytes = fs::read(&path).unwrap();
            assert!(!String::from_utf8_lossy(&bytes).contains(SECRET));
            let cursor = f
                .spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap();
            let mut worker = supervisor(&f);
            let mut holders = Vec::new();
            let mut releases = Vec::new();
            for _ in 0..4 {
                let (entered, received) = tokio::sync::oneshot::channel();
                let (release, wait) = mpsc::channel();
                holders.push(tokio::task::spawn_blocking(move || {
                    entered.send(()).unwrap();
                    wait.recv_timeout(Duration::from_secs(10)).unwrap();
                }));
                received.await.unwrap();
                releases.push(release);
            }
            worker.start_step().unwrap();
            if discovery {
                worker.discovery.abort_queued_task_for_test();
            } else {
                worker.delivery.abort_queued_task_for_test();
            }
            for release in releases {
                release.send(()).unwrap();
            }
            for holder in holders {
                holder.await.unwrap();
            }
            settle(&mut worker).await;
            assert_eq!(
                worker.status().unavailable,
                1,
                "lost lane omitted from unavailable diagnostics; discovery={discovery}"
            );
            let phase = if discovery {
                worker.discovery.phase()
            } else {
                worker.delivery.phase()
            };
            assert_eq!(phase, CaptureWorkerPhase::Stopped);
            assert_eq!(
                worker.status().discovery.phase == CaptureWorkerPhase::Stopped,
                discovery
            );
            assert_eq!(
                worker.rescan().is_err(),
                discovery,
                "rescan must not replace lost discovery"
            );
            assert_eq!(worker.status().caught_up, 0);
            worker.pause();
            assert!(!worker.status().discovery.rescan_pending);
            assert!(
                worker.resume().is_err(),
                "a lost global lane resumed without replacement"
            );
            for _ in 0..4 {
                assert!(worker.start_step().is_err());
                settle(&mut worker).await;
                assert_eq!(f.spool.pending().unwrap(), queue);
                assert_eq!(fs::read(&path).unwrap(), bytes);
                assert_eq!(
                    f.spool
                        .latest_reader_progress(partition.integration, partition.source)
                        .unwrap(),
                    cursor
                );
                assert_eq!(worker.status().caught_up, 0);
                assert_eq!(worker.status().unavailable, 1);
            }
            worker.shutdown().await.unwrap();
            drop(worker);
            f.catalog(&[]);
            let mut replacement = supervisor(&f);
            for _ in 0..4 {
                replacement.start_step().unwrap();
                settle(&mut replacement).await;
                assert_eq!(f.spool.pending().unwrap(), queue);
                assert_eq!(fs::read(&path).unwrap(), bytes);
                assert_eq!(
                    f.spool
                        .latest_reader_progress(partition.integration, partition.source)
                        .unwrap(),
                    cursor
                );
                assert_eq!(replacement.status().caught_up, 0);
            }
            f.catalog(&[fixture::A, fixture::B]);
            tokio::time::timeout(Duration::from_secs(30), async {
                while replacement.status().caught_up != 1 {
                    replacement.start_step().unwrap();
                    settle(&mut replacement).await;
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
            assert!(f.spool.pending().unwrap().is_empty());
            assert!(!path.exists());
            assert_eq!(
                f.spool
                    .latest_reader_progress(partition.integration, partition.source)
                    .unwrap(),
                cursor
            );
            replacement.shutdown().await.unwrap();
        });
    }
}
