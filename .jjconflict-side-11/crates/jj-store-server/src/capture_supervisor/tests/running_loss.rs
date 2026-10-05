use super::*;
use crate::capture_worker::executor::faults::{FaultGate, Release};
use jj_store_capture::{
    CaptureResult,
    agent_capture::PendingCaptureStateV1,
    agent_publication::{AgentCapturePublisher, drain_capture_spool_entry},
};
use jj_store_model::{PublicationId, RequestId};
use jj_store_service::{
    auth::Permission,
    collaboration::{AgentReadSnapshot, AgentSessionPager},
    local_management::{VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier},
    transport::{
        AgentDeltaRequest, NormalizerProfileV1, PublishOutcome, Receipt,
        SanitizedObservationBatchV1,
    },
};
use std::{process::Command, sync::Mutex};

const MODE: &str = "JJ_STORE_RUNNING_CAPTURE_LOSS_TEST_MODE";

#[test]
fn cold_running_lane_loss_retains_storage_and_exact_unknown_receipt() {
    for mode in [
        "discovery-drain",
        "discovery-drop",
        "delivery-drain",
        "delivery-drop",
    ] {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "capture_supervisor::tests::running_loss::running_loss_helper",
                "--nocapture",
            ])
            .env(MODE, mode)
            .env("JJ_STORE_S3_ACCESS_KEY", "running-loss-fixture-access")
            .env("JJ_STORE_S3_SECRET_KEY", "running-loss-fixture-secret")
            .env_remove("JJ_STORE_S3_SESSION_TOKEN")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("running-loss-passed"));
    }
}

struct LoseAck<P> {
    native: P,
    receipt: Arc<Mutex<Option<Receipt>>>,
}
#[async_trait::async_trait]
impl<P: AgentCapturePublisher> AgentCapturePublisher for LoseAck<P> {
    async fn prepare_agent_capture(
        &self,
        request: RequestId,
        publication: PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> CaptureResult<AgentDeltaRequest> {
        self.native
            .prepare_agent_capture(request, publication, profile, batch, result)
            .await
    }
    async fn publish_agent_delta(
        &self,
        request: &AgentDeltaRequest,
    ) -> CaptureResult<PublishOutcome> {
        let PublishOutcome::Committed(receipt) = self.native.publish_agent_delta(request).await?
        else {
            panic!("native receipt missing");
        };
        *self.receipt.lock().unwrap() = Some(receipt);
        Ok(PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        })
    }
}
struct FailRecovery {
    proof: VerifiedCaptureWorkspace,
    gate: Mutex<Option<FaultGate>>,
}
impl WorkspaceVerifier for FailRecovery {
    fn inspect(
        &self,
        _: &str,
    ) -> std::result::Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Err(jj_store_service::local_management::Error::Unavailable)
    }
    fn inspect_capture_directory(
        &self,
        directory: &str,
    ) -> std::result::Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error>
    {
        if directory != self.proof.directory {
            return Err(jj_store_service::local_management::Error::Invalid);
        }
        let gate = self.gate.lock().unwrap().take();
        if let Some(gate) = gate {
            gate.fail();
        }
        Ok(self.proof.clone())
    }
}
async fn read(f: &Fixture) -> (AgentReadSnapshot, String) {
    let session = f
        .host
        .endpoint(fixture::A)
        .unwrap()
        .bind_local_trusted(f.authority.principal(), Permission::Read)
        .unwrap()
        .into_session();
    let pager = AgentSessionPager::new(
        Default::default(),
        Arc::new(jj_store_service::auth::SystemClock),
    )
    .unwrap();
    let page = pager.begin(session.clone(), 10).await.unwrap();
    assert_eq!(page.sessions.len(), 1);
    let detail = pager
        .detail(session, page.sessions[0].session_id)
        .await
        .unwrap();
    (page.snapshot, serde_json::to_string(&detail).unwrap())
}

#[test]
fn running_loss_helper() {
    let Ok(mode) = std::env::var(MODE) else {
        return;
    };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let discovery = mode.starts_with("discovery-");
            let drop_controller = mode.ends_with("-drop");
            let mut f = Fixture::new(4 * 1024 * 1024).await;
            f.message(
                "retained-running",
                &format!("exact running-loss receipt message {SECRET}"),
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
            let captured = f.spool.pending().unwrap().remove(0);
            let proof = f
                .admission
                .inspect(f.workspace(fixture::A).to_str().unwrap())
                .await
                .unwrap();
            let receipt = Arc::new(Mutex::new(None));
            let lost = LoseAck {
                native: f.authority.publisher(&f.host, &f.admission, &proof),
                receipt: receipt.clone(),
            };
            let report = drain_capture_spool_entry(&mut f.spool, &captured, &lost)
                .await
                .unwrap();
            assert!(report.outcome_unknown && report.committed == 0 && report.remaining == 1);
            drop(lost);
            let original = receipt.lock().unwrap().clone().unwrap();
            let queue = f.spool.pending().unwrap();
            let PendingCaptureStateV1::Prepared { request } = &queue[0].state else {
                panic!("prepared intent missing");
            };
            assert_eq!(
                (
                    request.request_id,
                    request.publication_id,
                    request.principal
                ),
                (
                    original.request_id,
                    original.publication_id,
                    original.principal
                )
            );
            let path = f
                .root
                .path()
                .join("spool/pending")
                .join(format!("{}.json", queue[0].batch.batch_id));
            let bytes = fs::read(&path).unwrap();
            let cursor = f
                .spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap();
            let before = read(&f).await;
            assert!(
                before.1.contains("exact running-loss receipt message")
                    && !before.1.contains(SECRET)
            );
            let (gate, entered, release): (FaultGate, _, Release) = FaultGate::new();
            let config = crate::config::Config::load(&f.root.path().join("server.toml")).unwrap();
            let storage = crate::storage::AuthoritativeStorage::start(&config)
                .await
                .unwrap()
                .with_state_lock(Some(
                    crate::policy::exclusive_lock(&config.state_dir.join("server.lock")).unwrap(),
                ))
                .unwrap();
            let weak = Arc::downgrade(&storage.store());
            let locked =
                || crate::policy::exclusive_lock(&config.state_dir.join("server.lock")).is_err();
            let mut worker = supervisor(&f);
            worker.retain_storage(storage.capture_lifetime()).unwrap();
            if discovery {
                worker.discovery.idle_mut().unwrap().fault = Some(gate);
            } else {
                worker.start_step().unwrap();
                settle(&mut worker).await;
                assert_eq!(worker.slots.len(), 1);
                worker.slots[0].owner.pause(); // Only delivery can invoke this recovery verifier.
                let identity =
                    crate::installation::Identity::load(&f.root.path().join("installation.json"))
                        .unwrap();
                let admission = CaptureAdmission::new(
                    Arc::new(
                        crate::registry::Registry::new(
                            f.root.path().join("repositories.json"),
                            identity,
                        )
                        .unwrap(),
                    ),
                    Arc::new(FailRecovery {
                        proof,
                        gate: Mutex::new(Some(gate)),
                    }),
                );
                worker
                    .delivery
                    .idle_mut()
                    .unwrap()
                    .admission_for_test(admission);
            }
            worker.start_step().unwrap();
            tokio::time::timeout(Duration::from_secs(10), entered)
                .await
                .unwrap()
                .unwrap();
            drop(storage);
            assert!(weak.upgrade().is_some() && locked());
            assert_eq!(fs::read(&path).unwrap(), bytes);
            worker.pause();
            assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
            if drop_controller {
                drop(worker);
                assert!(
                    weak.upgrade().is_some() && locked(),
                    "detached running lane lost storage"
                );
                drop(release); // Real blocking task panics after dropping its capture state.
            } else {
                assert!(
                    tokio::time::timeout(Duration::from_millis(100), worker.shutdown())
                        .await
                        .is_err()
                );
                assert!(weak.upgrade().is_some() && locked());
                drop(release);
                settle(&mut worker).await;
                assert_eq!(worker.status().unavailable, 1);
                worker.shutdown().await.unwrap();
                drop(worker);
            }
            tokio::time::timeout(Duration::from_secs(10), async {
                while weak.upgrade().is_some() {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            })
            .await
            .unwrap();
            assert!(!locked());
            assert_eq!(f.spool.pending().unwrap(), queue);
            assert_eq!(fs::read(&path).unwrap(), bytes);
            assert_eq!(
                f.spool
                    .latest_reader_progress(partition.integration, partition.source)
                    .unwrap(),
                cursor
            );
            assert_eq!(read(&f).await, before);
            f.catalog(&[]);
            let mut replacement = supervisor(&f);
            for _ in 0..4 {
                replacement.start_step().unwrap();
                settle(&mut replacement).await;
                assert_eq!(fs::read(&path).unwrap(), bytes);
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
            assert_eq!(
                read(&f).await,
                before,
                "receipt recovery rewrote native evidence"
            );
            assert_eq!(
                f.spool
                    .latest_reader_progress(partition.integration, partition.source)
                    .unwrap(),
                cursor
            );
            replacement.shutdown().await.unwrap();
        });
    println!("running-loss-passed");
}
