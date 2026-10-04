use super::*;
use jj_store_service::{
    auth::Permission,
    collaboration::AgentSessionPager,
    local_management::{VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceVerifier},
};
use std::{collections::BTreeMap, path::Path, sync::Mutex};

struct Hold {
    entered: tokio::sync::oneshot::Sender<()>,
    release: mpsc::Receiver<()>,
}
struct Verifier {
    proof: VerifiedCaptureWorkspace,
    hold: Mutex<Option<Hold>>,
}
impl WorkspaceVerifier for Verifier {
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
        let hold = self.hold.lock().unwrap().take();
        if let Some(hold) = hold {
            hold.entered.send(()).unwrap();
            hold.release.recv_timeout(Duration::from_secs(20)).unwrap();
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
fn tree(root: &Path) -> BTreeMap<std::path::PathBuf, Vec<u8>> {
    fn visit(root: &Path, at: &Path, files: &mut BTreeMap<std::path::PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(at).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), files);
            } else {
                assert!(entry.file_type().unwrap().is_file());
                files.insert(
                    entry.path().strip_prefix(root).unwrap().to_owned(),
                    fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[tokio::test(flavor = "current_thread")]
async fn delivery_rechecks_rebound_source_root_registry_and_spool_after_exact_recovery() {
    use std::os::unix::fs::symlink;
    for mode in [
        "transcript-root",
        "source-state",
        "spool-state",
        "catalog-loss",
    ] {
        let mut f = Fixture::new(4 * 1024 * 1024).await;
        f.message("rebind", &format!("exact rebound recovery body {SECRET}"));
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
        assert_eq!(queue.len(), 1);
        let cursor = f
            .spool
            .latest_reader_progress(partition.integration, partition.source)
            .unwrap();
        let source_root = f.root.path().join("sources");
        let spool_root = f.root.path().join("spool");
        let source_bytes = tree(&source_root);
        let spool_bytes = tree(&spool_root);
        let transcript_bytes = fs::read(&f.transcript).unwrap();
        let pager = AgentSessionPager::new(
            Default::default(),
            Arc::new(jj_store_service::auth::SystemClock),
        )
        .unwrap();
        let native = f
            .host
            .endpoint(fixture::A)
            .unwrap()
            .bind_local_trusted(f.authority.principal(), Permission::Read)
            .unwrap()
            .into_session();
        let before = pager.begin(native.clone(), 10).await.unwrap();
        assert!(before.sessions.is_empty());

        let mut worker = supervisor(&f);
        worker.start_step().unwrap();
        settle(&mut worker).await;
        assert_eq!(worker.slots.len(), 1);
        worker.slots[0].owner.pause(); // Isolate exact delivery from ingress.
        let proof = f
            .admission
            .inspect(f.workspace(fixture::A).to_str().unwrap())
            .await
            .unwrap();
        let identity =
            crate::installation::Identity::load(&f.root.path().join("installation.json")).unwrap();
        let registry =
            crate::registry::Registry::new(f.root.path().join("repositories.json"), identity)
                .unwrap();
        let (entered, observed) = tokio::sync::oneshot::channel();
        let (release, released) = mpsc::channel();
        let release = Release(release);
        worker
            .delivery
            .idle_mut()
            .unwrap()
            .admission_for_test(CaptureAdmission::new(
                Arc::new(registry),
                Arc::new(Verifier {
                    proof,
                    hold: Mutex::new(Some(Hold {
                        entered,
                        release: released,
                    })),
                }),
            ));
        worker.start_step().unwrap();
        tokio::time::timeout(Duration::from_secs(10), observed)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(worker.phase(), CaptureWorkerPhase::Running);
        let logical = match mode {
            "transcript-root" => f.transcript.parent().unwrap().to_owned(),
            "source-state" => source_root.clone(),
            "spool-state" => spool_root.clone(),
            "catalog-loss" => source_root.join("sources.json"),
            _ => unreachable!(),
        };
        let retained = f.root.path().join(format!("retained-{mode}"));
        fs::rename(&logical, &retained).unwrap();
        if mode == "transcript-root" {
            fs::create_dir(&logical).unwrap();
            // Identical bytes cannot substitute a different physical qualification.
            fs::write(&f.transcript, &transcript_bytes).unwrap();
        } else if mode != "catalog-loss" {
            // The original bytes remain reachable, but the private binding changed.
            symlink(&retained, &logical).unwrap();
        }
        drop(release);
        settle(&mut worker).await;
        assert_eq!(worker.status().caught_up, 0, "{mode}");
        assert!(worker.status().resident_delivery_frames <= 2);
        let old_sources = if mode == "source-state" {
            &retained
        } else {
            &source_root
        };
        let old_spool = if mode == "spool-state" {
            &retained
        } else {
            &spool_root
        };
        let mut expected_sources = source_bytes.clone();
        if mode == "catalog-loss" {
            assert_eq!(
                fs::read(&retained).unwrap(),
                expected_sources.remove(Path::new("sources.json")).unwrap()
            );
        }
        assert_eq!(
            tree(old_sources),
            expected_sources,
            "{mode}: source state changed"
        );
        assert_eq!(
            tree(old_spool),
            spool_bytes,
            "{mode}: durable intent changed"
        );
        let withheld = pager.begin(native.clone(), 10).await.unwrap();
        assert_eq!(
            withheld.snapshot, before.snapshot,
            "{mode}: native roots changed"
        );
        assert!(withheld.sessions.is_empty());
        worker.shutdown().await.unwrap();
        drop(worker);
        if mode == "transcript-root" {
            fs::remove_dir_all(&logical).unwrap();
        } else if mode != "catalog-loss" {
            fs::remove_file(&logical).unwrap();
        }
        fs::rename(retained, logical).unwrap();
        assert_eq!(
            tree(&source_root),
            source_bytes,
            "{mode}: source allocation changed"
        );
        assert_eq!(f.spool.pending().unwrap(), queue);
        assert_eq!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            cursor
        );
        let mut replacement = supervisor(&f);
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
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap(),
            cursor
        );
        let recovered = pager.begin(native.clone(), 10).await.unwrap();
        assert_eq!(recovered.sessions.len(), 1);
        let detail = pager
            .detail(native, recovered.sessions[0].session_id)
            .await
            .unwrap();
        assert_eq!(
            detail.event_batches.len(),
            1,
            "{mode}: duplicate native batch"
        );
        let body = serde_json::to_string(&detail).unwrap();
        assert!(body.contains("exact rebound recovery body") && !body.contains(SECRET));
        replacement.shutdown().await.unwrap();
    }
}
