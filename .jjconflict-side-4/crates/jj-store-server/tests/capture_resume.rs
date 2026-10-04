#![cfg(unix)]
#[path = "capture_source/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_server::capture_source::{
    CaptureResumeStep, CaptureSourceStep, CodexCaptureResume, CodexCaptureSource,
};
use jj_store_service::local_management::{
    VerifiedCaptureWorkspace, VerifiedWorkspace, WorkspaceProfile, WorkspaceVerifier,
};
use serde_json::json;
use std::fs;
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

struct AppendDuringAdmission {
    root: PathBuf,
    transcript: PathBuf,
    first: AtomicBool,
}
impl WorkspaceVerifier for AppendDuringAdmission {
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
        if self.first.swap(false, Ordering::SeqCst) {
            use std::io::Write as _;
            writeln!(
                fs::OpenOptions::new()
                    .append(true)
                    .open(&self.transcript)
                    .unwrap(),
                "{}",
                json!({"type":"turn_context","payload":{"cwd":self.root.join("workspace-a")}})
            )
            .unwrap();
        }
        for (name, repository) in [("workspace-a", A), ("workspace-b", B)] {
            let path = self.root.join(name);
            if Path::new(directory).starts_with(&path) {
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
async fn admitted_resume_uses_current_repository_when_original_header_is_revoked() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let mut source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    source.step(&mut fixture.context()).await.unwrap();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.message("b", &format!("pending current repository {SECRET}"));
    assert_eq!(
        source.step(&mut fixture.context()).await.unwrap(),
        CaptureSourceStep::Enqueued
    );
    let retained = fixture.spool.pending().unwrap();
    drop(source);
    fixture.catalog(&[B]);
    let mut preparation = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    let CaptureResumeStep::Ready(mut resumed) =
        preparation.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("small current prefix")
    };
    assert_eq!(resumed.partition().repository, B);
    assert_eq!(fixture.spool.pending().unwrap(), retained);
    assert!(
        matches!(resumed.step(&mut fixture.context()).await.unwrap(), CaptureSourceStep::Published(report) if report.committed == 1)
    );
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert!(preparation.step(&mut fixture.context()).await.is_err());
}

#[tokio::test]
async fn resume_preparation_is_bounded_read_only_and_partial_context_cannot_rebind() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    let source_id = source.partition().source;
    drop(source); // Admission can survive a crash before the first sanitized enqueue.
    for _ in 0..7 {
        fixture.append(json!({"padding":"x".repeat(800 * 1024)}));
    }
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.append_bytes(br#"{"type":"turn_context","payload":{"cwd":"/unbound/partial"#);
    fixture.catalog(&[B]);
    let registry = fixture.root.path().join("sources");
    let before = fs::read(registry.join("sources.json")).unwrap();
    let locks = fs::read_dir(&registry).unwrap().count();
    let mut preparation = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    assert!(matches!(
        preparation.step(&mut fixture.context()).await.unwrap(),
        CaptureResumeStep::Verifying
    ));
    assert_eq!(fs::read(registry.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&registry).unwrap().count(), locks);
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert!(!fixture.root.path().join("rules").exists());
    let CaptureResumeStep::Ready(resumed) = preparation.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("second bounded prefix step")
    };
    assert_eq!(resumed.partition().source, source_id);
    assert_eq!(resumed.partition().repository, B);
    assert!(
        fixture
            .spool
            .latest_reader_progress(resumed.partition().integration, source_id)
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn resume_cannot_enroll_rotation_without_write_or_import_unadmitted_history() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    assert!(
        CodexCaptureResume::prepare(
            fixture.sources.clone(),
            fixture.qualified(),
            fixture.privacy(),
            &fixture.spool
        )
        .unwrap()
        .is_none()
    );
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    drop(source);
    let original = fs::read(&fixture.transcript).unwrap();
    fs::rename(
        &fixture.transcript,
        fixture.transcript.with_extension("old"),
    )
    .unwrap();
    fs::write(&fixture.transcript, original).unwrap();
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.catalog(&[B]);
    let registry = fixture.root.path().join("sources");
    let before = fs::read(registry.join("sources.json")).unwrap();
    let locks = fs::read_dir(&registry).unwrap().count();
    let mut preparation = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    let host = fixture.host.clone();
    fixture.without_native_authority();
    assert!(preparation.step(&mut fixture.context()).await.is_err());
    assert_eq!(fs::read(registry.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&registry).unwrap().count(), locks);
    assert!(fixture.spool.pending().unwrap().is_empty());
    assert!(!fixture.root.path().join("rules").exists());
    fixture.host = host;
    let CaptureResumeStep::Ready(resumed) = preparation.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("restored native authority")
    };
    assert_eq!(resumed.partition().repository, B);
    assert_ne!(fs::read(registry.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&registry).unwrap().count(), locks + 1);
}

#[tokio::test]
async fn file_changes_during_native_admission_withhold_before_source_acquisition() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    drop(source);
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    let registry = fixture.root.path().join("sources");
    let before = fs::read(registry.join("sources.json")).unwrap();
    let locks = fs::read_dir(&registry).unwrap().count();
    let mut preparation = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    let catalog = Arc::new(
        jj_store_server::registry::Registry::new(
            fixture.root.path().join("repositories.json"),
            jj_store_server::installation::Identity {
                version: 1,
                installation: "ab".repeat(16),
            },
        )
        .unwrap(),
    );
    fixture.admission = jj_store_server::capture_admission::CaptureAdmission::new(
        catalog,
        Arc::new(AppendDuringAdmission {
            root: fixture.root.path().to_owned(),
            transcript: fixture.transcript.clone(),
            first: AtomicBool::new(true),
        }),
    );
    assert!(matches!(
        preparation.step(&mut fixture.context()).await.unwrap(),
        CaptureResumeStep::Withheld
    ));
    assert_eq!(fs::read(registry.join("sources.json")).unwrap(), before);
    assert_eq!(fs::read_dir(&registry).unwrap().count(), locks);
    assert!(fixture.spool.pending().unwrap().is_empty());
    let CaptureResumeStep::Ready(resumed) = preparation.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("new snapshot resumes")
    };
    assert_eq!(resumed.partition().repository, A);
}

#[tokio::test]
async fn current_binding_and_private_spool_are_revalidated_at_each_preparation_step() {
    let mut fixture = Fixture::new(4 * 1024 * 1024).await;
    let active = fixture.start();
    let sources = fixture.sources.clone();
    let privacy = fixture.privacy();
    let source =
        CodexCaptureSource::acquire_active(sources, active, privacy, &mut fixture.context())
            .await
            .unwrap();
    drop(source);
    fixture.append(json!({"type":"turn_context","payload":{"cwd":fixture.workspace(B)}}));
    fixture.catalog(&[B]);
    let mut preparation = CodexCaptureResume::prepare(
        fixture.sources.clone(),
        fixture.qualified(),
        fixture.privacy(),
        &fixture.spool,
    )
    .unwrap()
    .unwrap();
    fixture.catalog(&[]);
    assert!(matches!(
        preparation.step(&mut fixture.context()).await.unwrap(),
        CaptureResumeStep::Withheld
    ));
    fixture.catalog(&[B]);
    fixture.spool = jj_store_capture::agent_capture::AgentCaptureSpool::open(
        &fixture.root.path().join("other-spool"),
        4 * 1024 * 1024,
    )
    .unwrap();
    assert!(preparation.step(&mut fixture.context()).await.is_err());
    fixture.spool = jj_store_capture::agent_capture::AgentCaptureSpool::open(
        &fixture.root.path().join("spool"),
        4 * 1024 * 1024,
    )
    .unwrap();
    let CaptureResumeStep::Ready(resumed) = preparation.step(&mut fixture.context()).await.unwrap()
    else {
        panic!("restored binding")
    };
    assert_eq!(resumed.partition().repository, B);
}
