use super::*;
use crate::{capture_admission::CaptureAdmission, registry::Registry};
use jj_store_capture::{agent_capture::AgentCaptureSpool, codex_import::CodexImportProbe};
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
};
use std::{
    path::PathBuf,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

#[tokio::test]
async fn parked_raw_import_retains_channel_and_recovers_exact_batch_after_native_refusal() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "raw import awaiting ingress capacity");
    let original_spool = f.f.spool.fork_handle();
    f.f.spool = AgentCaptureSpool::open(&f.f.root.path().join("spool"), 128).unwrap();
    let mut pipeline = enroll(&mut f).await;
    let receipt = pipeline.receipt().clone();
    assert!(
        pipeline
            .ingest_step(&mut f.f.context())
            .await
            .unwrap_err()
            .capacity_refusal()
            .is_some()
    );
    assert!(f.f.spool.pending().unwrap().is_empty());
    let mut parked = pipeline.park();
    let mut probe = CodexImportProbe::reconcile(f.f.qualified(), receipt.source.codex()).unwrap();
    assert!(matches!(
        probe.poll().unwrap(),
        jj_store_capture::codex_import::CodexImportPoll::Ready(_)
    ));
    assert!(
        f.f.sources
            .acquire_import(&probe, receipt.source.codex(), || Ok(()))
            .is_err(),
        "raw channel released while parked"
    );
    let host = f.f.host.clone();
    f.f.without_native_authority();
    assert!(parked.resume_current(&mut f.f.context()).await.is_err());
    assert!(
        f.f.sources
            .acquire_import(&probe, receipt.source.codex(), || Ok(()))
            .is_err(),
        "native refusal released parked channel"
    );
    f.f.host = host;
    // Restore the runtime's shared FIFO group, rather than reopening a live spool.
    f.f.spool = original_spool;
    let CaptureImportStep::Ready(pipeline) =
        parked.resume_current(&mut f.f.context()).await.unwrap()
    else {
        panic!("small fixture")
    };
    let mut pipeline = *pipeline;
    enqueue(&mut f, &mut pipeline).await;
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
    publish(&mut f, &mut pipeline).await;
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}

struct Gate {
    root: PathBuf,
    at: usize,
    calls: AtomicUsize,
    hold: Mutex<Option<Hold>>,
}
struct Hold {
    entered: tokio::sync::oneshot::Sender<()>,
    release: std::sync::mpsc::Receiver<()>,
}
impl WorkspaceVerifier for Gate {
    fn inspect(
        &self,
        _: &str,
    ) -> Result<VerifiedWorkspace, jj_store_service::local_management::Error> {
        Err(jj_store_service::local_management::Error::Unavailable)
    }
    fn inspect_capture_directory(
        &self,
        directory: &str,
    ) -> Result<VerifiedCaptureWorkspace, jj_store_service::local_management::Error> {
        if self.calls.fetch_add(1, Ordering::SeqCst) + 1 == self.at {
            let hold = self.hold.lock().unwrap().take().unwrap();
            hold.entered.send(()).unwrap();
            hold.release.recv_timeout(Duration::from_secs(10)).unwrap();
        }
        let workspace = self.root.join("workspace-a");
        Ok(VerifiedCaptureWorkspace {
            directory: directory.into(),
            workspace: VerifiedWorkspace {
                path: workspace.to_str().unwrap().into(),
                metadata: workspace.join(".jj/repo").to_str().unwrap().into(),
                repository: A,
                profile: WorkspaceProfile::Native,
            },
            binding_digest: [1; 32],
        })
    }
}

#[tokio::test(flavor = "current_thread")]
async fn cancelled_caller_cannot_release_owned_native_admission_gates_or_erase_selected_intent() {
    for at in [1, 4] {
        let mut f = ImportFixture::new().await;
        f.f.message("history", "history retained through admission await");
        let sources_before = fs::read(f.f.root.path().join("sources/sources.json")).unwrap();
        let import = prepare(&f);
        let (entered, observed) = tokio::sync::oneshot::channel();
        let (release, wait) = std::sync::mpsc::channel();
        let identity = Identity::load(&f.f.root.path().join("installation.json")).unwrap();
        let admission = CaptureAdmission::new(
            Arc::new(Registry::new(f.f.root.path().join("repositories.json"), identity).unwrap()),
            Arc::new(Gate {
                root: f.f.root.path().to_owned(),
                at,
                calls: AtomicUsize::new(0),
                hold: Mutex::new(Some(Hold {
                    entered,
                    release: wait,
                })),
            }),
        );
        let authority = crate::capture_authority::CaptureAuthority::prepare(&f.config)
            .unwrap()
            .unwrap();
        let host = f.f.host.clone();
        let mut spool = f.f.spool.fork_handle();
        let (done, finished) = tokio::sync::oneshot::channel();
        let caller = tokio::spawn(async move {
            tokio::spawn(async move {
                let mut import = import;
                let outcome = import
                    .step(&mut crate::capture_source::CaptureStepContext {
                        authority: &authority,
                        host: &host,
                        admission: &admission,
                        spool: &mut spool,
                    })
                    .await;
                done.send((import, outcome)).ok();
            })
            .await
            .unwrap()
        });
        tokio::time::timeout(Duration::from_secs(10), observed)
            .await
            .unwrap()
            .unwrap();
        caller.abort();
        assert!(caller.await.unwrap_err().is_cancelled());
        assert_eq!(f.manager.status(f.intent.principal).await, Err(Error::Busy));
        assert!(f.control.current().is_err());
        let mut revoked = f.config.clone();
        let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
            unreachable!()
        };
        *permission = Permission::Read;
        f.save(&revoked);
        release.send(()).unwrap();
        let (mut import, outcome) = tokio::time::timeout(Duration::from_secs(10), finished)
            .await
            .unwrap()
            .unwrap();
        assert!(outcome.is_err());
        let receipt = import.receipt().cloned();
        if at == 4 {
            // Retry the same staged owner after unknown enrollment and append.
            // Its original selected boundary must not become a new selection.
            f.save(&f.config);
            f.f.message("tail", "appended after unknown enrollment");
            let mut ready = None;
            for _ in 0..10 {
                match import.step(&mut f.f.context()).await.unwrap() {
                    CaptureImportStep::Ready(pipeline) => {
                        ready = Some(pipeline);
                        break;
                    }
                    CaptureImportStep::Verifying | CaptureImportStep::ChangedDuringRead => {}
                    CaptureImportStep::Withheld => panic!("unexpected withheld exact retry"),
                }
            }
            let pipeline = ready.expect("bounded original-intent reconciliation");
            assert_eq!(Some(pipeline.receipt()), receipt.as_ref());
            assert!(
                pipeline.receipt().source.complete_byte_offset
                    < fs::metadata(&f.f.transcript).unwrap().len()
            );
            drop(pipeline);
        }
        drop(import);
        if at == 1 {
            assert!(receipt.is_none());
            assert!(f.imports.entries().unwrap().is_empty());
            assert_eq!(
                fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
                sources_before
            );
        } else {
            assert_eq!(f.imports.entries().unwrap(), vec![receipt.unwrap()]);
            assert_ne!(
                fs::read(f.f.root.path().join("sources/sources.json")).unwrap(),
                sources_before
            );
        }
        assert!(f.f.spool.pending().unwrap().is_empty());
        assert!(
            f.f.sources
                .lookup_admitted(&f.f.qualified())
                .unwrap()
                .is_none()
        );
        f.save(&f.config);
        f.runtime.shutdown().await.unwrap();
    }
}

#[tokio::test(flavor = "current_thread")]
async fn revocation_during_publisher_native_await_preserves_exact_prepared_intent_without_commit() {
    let mut f = ImportFixture::new().await;
    f.f.message("history", "history guarded through publisher authorization");
    let mut pipeline = enroll(&mut f).await;
    enqueue(&mut f, &mut pipeline).await;
    let receipt = pipeline.receipt().clone();
    let original = f.f.spool.pending().unwrap().remove(0);
    let (entered, observed) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    let identity = Identity::load(&f.f.root.path().join("installation.json")).unwrap();
    let admission = CaptureAdmission::new(
        Arc::new(Registry::new(f.f.root.path().join("repositories.json"), identity).unwrap()),
        Arc::new(Gate {
            root: f.f.root.path().to_owned(),
            at: 11,
            calls: AtomicUsize::new(0),
            hold: Mutex::new(Some(Hold {
                entered,
                release: wait,
            })),
        }),
    );
    let authority = crate::capture_authority::CaptureAuthority::prepare(&f.config)
        .unwrap()
        .unwrap();
    let host = f.f.host.clone();
    let mut spool = f.f.spool.fork_handle();
    let entry = original.clone();
    let work = tokio::spawn(async move {
        let outcome = pipeline
            .publish_step(
                &entry,
                &mut crate::capture_source::CaptureStepContext {
                    authority: &authority,
                    host: &host,
                    admission: &admission,
                    spool: &mut spool,
                },
            )
            .await;
        (pipeline, outcome)
    });
    tokio::time::timeout(Duration::from_secs(10), observed)
        .await
        .unwrap()
        .unwrap();
    let mut revoked = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
        unreachable!()
    };
    *permission = Permission::Read;
    f.save(&revoked);
    release.send(()).unwrap();
    let (mut pipeline, outcome) = tokio::time::timeout(Duration::from_secs(10), work)
        .await
        .unwrap()
        .unwrap();
    assert!(outcome.is_err());
    let retained = f.f.spool.pending().unwrap();
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].batch, original.batch);
    assert_eq!(retained[0].cursor, original.cursor);
    assert!(matches!(
        retained[0].state,
        jj_store_capture::agent_capture::PendingCaptureStateV1::Prepared { .. }
    ));
    let session =
        f.f.host
            .endpoint(A)
            .unwrap()
            .bind_local_trusted(f.f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
    let pager =
        AgentSessionPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    assert!(pager.begin(session, 10).await.unwrap().sessions.is_empty());
    assert_eq!(f.imports.entries().unwrap(), vec![receipt]);
    f.save(&f.config);
    publish(&mut f, &mut pipeline).await;
    drop(pipeline);
    f.runtime.shutdown().await.unwrap();
}
