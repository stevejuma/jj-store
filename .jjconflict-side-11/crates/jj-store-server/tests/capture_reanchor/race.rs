use super::*;
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
};
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

struct AppendDuringHandoff {
    root: PathBuf,
    transcript: PathBuf,
    calls: AtomicUsize,
    append_at: usize,
}
impl WorkspaceVerifier for AppendDuringHandoff {
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
        for (name, repository) in [("workspace-a", A), ("workspace-b", B)] {
            let path = self.root.join(name);
            if Path::new(directory).starts_with(&path) {
                if repository == B
                    && self.calls.fetch_add(1, Ordering::SeqCst) + 1 == self.append_at
                {
                    use std::io::Write as _;
                    writeln!(
                        fs::OpenOptions::new()
                            .append(true)
                            .open(&self.transcript)
                            .unwrap(),
                        "{}",
                        json!({"type":"turn_context","payload":{"cwd":path}})
                    )
                    .unwrap();
                }
                return Ok(VerifiedCaptureWorkspace {
                    directory: directory.into(),
                    workspace: VerifiedWorkspace {
                        path: path.to_str().unwrap().into(),
                        metadata: path.join(".jj/repo").to_str().unwrap().into(),
                        repository,
                        profile: WorkspaceProfile::Native,
                    },
                    binding_digest: [repository.as_bytes()[0]; 32],
                });
            }
        }
        Err(jj_store_service::local_management::Error::Invalid)
    }
}

#[tokio::test]
async fn append_during_each_admission_boundary_releases_acquisition_and_retains_the_frame() {
    for append_at in 1..=4 {
        let mut f = Fixture::new(1).await;
        f.message("a", "original pending batch");
        let active = f.start();
        let mut pipeline = CodexCaptureSource::acquire_active(
            f.sources.clone(),
            active,
            f.privacy(),
            &mut f.context(),
        )
        .await
        .unwrap()
        .into_pipeline();
        let original_end = fs::metadata(&f.transcript).unwrap().len();
        let partition = pipeline.partition();
        assert!(pipeline.ingest_step(&mut f.context()).await.is_err());
        let mut parked = pipeline.park();
        f.append(json!({"type":"turn_context","payload":{"cwd":f.workspace(B)}}));
        f.catalog(&[B]);
        let identity =
            jj_store_server::installation::Identity::load(&f.root.path().join("installation.json"))
                .unwrap();
        f.admission = jj_store_server::capture_admission::CaptureAdmission::new(
            Arc::new(
                jj_store_server::registry::Registry::new(
                    f.root.path().join("repositories.json"),
                    identity,
                )
                .unwrap(),
            ),
            Arc::new(AppendDuringHandoff {
                root: f.root.path().to_owned(),
                transcript: f.transcript.clone(),
                calls: AtomicUsize::new(0),
                append_at,
            }),
        );
        assert!(matches!(
            current(&mut parked, &mut f).await,
            CaptureParkedResumeStep::ChangedDuringRead | CaptureParkedResumeStep::Withheld
        ));
        assert!(f.spool.pending().unwrap().is_empty());
        assert!(
            f.spool
                .latest_reader_progress(partition.integration, partition.source)
                .unwrap()
                .is_none()
        );
        drop(f.sources.acquire_admitted(f.qualified()).unwrap().unwrap());
        let CaptureParkedResumeStep::Ready(mut pipeline) = current(&mut parked, &mut f).await
        else {
            panic!("renewed snapshot withheld")
        };
        f.spool = AgentCaptureSpool::open(&f.root.path().join("spool"), 4 * 1024 * 1024).unwrap();
        assert_eq!(
            pipeline.ingest_step(&mut f.context()).await.unwrap(),
            CaptureSourceStep::Enqueued
        );
        let batch = f.spool.pending().unwrap().remove(0);
        assert_eq!(
            serde_json::to_value(&batch.cursor.position).unwrap()["complete_byte_offset"].as_u64(),
            Some(original_end)
        );
        assert!(!bodies(&batch).contains("original pending batch"));
    }
}
