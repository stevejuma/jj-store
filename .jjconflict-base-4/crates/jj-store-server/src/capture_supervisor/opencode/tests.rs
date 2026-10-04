use super::*;
use crate::{
    capture_source::CaptureSourceStep,
    capture_test_fixture::{A, Fixture},
    opencode_source::OpenCodeCaptureSource,
};
#[path = "../../../tests/opencode_source/fixture.rs"]
mod api_fixture;
use api_fixture::{Api, message};
use jj_store_capture::profile::CaptureProfile;
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier,
};
use std::{
    sync::{Mutex, mpsc},
    time::Instant,
};

struct Gate {
    proof: VerifiedCaptureWorkspace,
    entered: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    release: Mutex<mpsc::Receiver<()>>,
}
impl WorkspaceVerifier for Gate {
    fn inspect(
        &self,
        _: &str,
    ) -> std::result::Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Err(jj_store_service::local_management::Error::Unavailable)
    }
    fn inspect_capture_directory(
        &self,
        path: &str,
    ) -> std::result::Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error>
    {
        if path != self.proof.directory {
            return Err(jj_store_service::local_management::Error::Invalid);
        }
        if let Some(sender) = self.entered.lock().unwrap().take() {
            let _ = sender.send(());
            self.release
                .lock()
                .unwrap()
                .recv_timeout(Duration::from_secs(20))
                .unwrap();
        }
        Ok(self.proof.clone())
    }
}
struct Release(mpsc::Sender<()>);
impl Drop for Release {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}
fn supervisor(f: &Fixture, endpoint: &str) -> CodexCaptureSupervisor {
    let config = crate::config::Config::load(&f.root.path().join("server.toml")).unwrap();
    CodexCaptureSupervisor::prepare(
        AgentCaptureConfig {
            enabled: true,
            max_readers: 1,
            max_sources: 4,
            rescan_seconds: 1,
            max_pending_bytes: f.spool.pending_capacity(),
            harnesses: vec![crate::capture_config::HarnessConfig {
                profile: CaptureProfile::OpencodeReadOnlyApiV1,
                product_version: "1.18.30".into(),
                roots: vec![],
                endpoint: Some(endpoint.into()),
            }],
        },
        CodexSupervisorLocations {
            home: None,
            codex_home: None,
        },
        CodexSupervisorResources {
            authority: Arc::new(
                crate::capture_authority::CaptureAuthority::prepare(&config)
                    .unwrap()
                    .unwrap(),
            ),
            host: f.host.clone(),
            admission: f.admission.clone(),
            sources: f.sources.clone(),
            spool: f.spool.fork_handle(),
            privacy: f.privacy(),
        },
        Duration::from_secs(5),
    )
    .unwrap()
    .unwrap()
}
async fn tick(worker: &mut CodexCaptureSupervisor) {
    worker.poll().await;
    if !worker.paused && !worker.closed {
        worker.start_step().unwrap();
    }
    tokio::time::sleep(Duration::from_millis(5)).await;
}
#[tokio::test(flavor = "current_thread")]
async fn blocked_api_publication_retains_drain_ownership_while_ingress_continues() {
    // A status plus two message intents each reserve their full preparation budget.
    let f = Fixture::new(8 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_first",
            Some(&directory),
            "blocked publication",
        )],
        true,
    )
    .await;
    let (entered, mut received) = tokio::sync::oneshot::channel();
    let (release, wait) = mpsc::channel();
    let release = Release(release);
    let proof = f.admission.inspect(&directory).await.unwrap();
    let identity =
        crate::installation::Identity::load(&f.root.path().join("installation.json")).unwrap();
    let admission = CaptureAdmission::new(
        Arc::new(
            crate::registry::Registry::new(f.root.path().join("repositories.json"), identity)
                .unwrap(),
        ),
        Arc::new(Gate {
            proof,
            entered: Mutex::new(Some(entered)),
            release: Mutex::new(wait),
        }),
    );
    let mut worker = supervisor(&f, &api.endpoint);
    worker
        .api
        .as_mut()
        .unwrap()
        .delivery
        .idle_mut()
        .unwrap()
        .admission_for_test(admission);
    let expires = Instant::now() + Duration::from_secs(15);
    loop {
        tick(&mut worker).await;
        if received.try_recv().is_ok() {
            break;
        }
        assert!(
            Instant::now() < expires,
            "API publication did not enter gate"
        );
    }
    api.data.lock().unwrap().messages.push(message(
        "msg_tail",
        Some(&directory),
        "ingress during blocked publication",
    ));
    loop {
        tick(&mut worker).await;
        let status = worker.status();
        assert!(status.running_sources <= 1);
        assert!(status.resident_source_frames <= 2);
        let pending = f.spool.pending().unwrap();
        if pending.iter().any(|entry| {
            entry.batch.observations.iter().any(|observation| {
                String::from_utf8_lossy(observation.payload.bytes())
                    .contains("ingress during blocked publication")
            })
        }) {
            assert_eq!(pending.len(), 3);
            break;
        }
        assert!(
            Instant::now() < expires,
            "blocked publication stopped ingress: {status:?}"
        );
    }
    let lane = worker.api.as_ref().unwrap();
    assert_eq!(lane.slots.len(), 1);
    assert_eq!(lane.slots[0].selector, 1);
    let catalog = worker.source_catalog();
    assert_eq!(catalog.sources.len(), 1);
    assert_eq!(
        catalog.sources[0].profile,
        CaptureProfile::OpencodeReadOnlyApiV1
    );
    worker.pause();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
    tokio::time::sleep(Duration::from_millis(50)).await;
    worker.poll().await;
    assert_eq!(worker.phase(), CaptureWorkerPhase::Draining);
    drop(release);
    worker.shutdown().await.unwrap();
    assert_eq!(worker.phase(), CaptureWorkerPhase::Stopped);
    assert_eq!(worker.status().resident_source_frames, 0);
}
#[tokio::test]
async fn independent_api_proof_retains_lease_and_refuses_changed_sanitized_entry() {
    let mut f = Fixture::new(4 * 1024 * 1024).await;
    let directory = f.workspace(A).to_str().unwrap().to_owned();
    let api = Api::new(
        directory.clone(),
        vec![message(
            "msg_first",
            Some(&directory),
            "exact publication proof",
        )],
        true,
    )
    .await;
    let mut source = OpenCodeCaptureSource::acquire(
        api.api.clone(),
        f.sources.clone(),
        "ses_first",
        f.privacy(),
        &mut f.context(),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(
        source.ingest_step(&mut f.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let proof = source.publication_proofs(&f.context()).unwrap().remove(0);
    let pending = f.spool.pending().unwrap().remove(0);
    drop(source);
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .is_err()
    );
    let mut changed = pending.clone();
    changed.reader_metadata = None;
    assert!(proof.publish(&changed, &mut f.context()).await.is_err());
    assert_eq!(f.spool.pending().unwrap(), vec![pending.clone()]);
    assert!(
        matches!(proof.publish(&pending,&mut f.context()).await.unwrap(),CaptureSourceStep::Published(report) if report.committed==1)
    );
    drop(proof);
    assert!(
        OpenCodeCaptureSource::acquire(
            api.api.clone(),
            f.sources.clone(),
            "ses_first",
            f.privacy(),
            &mut f.context()
        )
        .await
        .unwrap()
        .is_some()
    );
}
