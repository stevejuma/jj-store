use jj_review::*;
use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore};
use jj_store_model as m;
use jj_store_service::{local_repository::*, work::*};
use std::{path::Path, process::Command};

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("repo");
    let result = Command::new("/opt/homebrew/bin/jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(&path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    (root, reopen(&path))
}
fn reopen(path: &Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
}
fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}
fn request(byte: u8, command: LocalReviewCommand) -> LocalReviewRequest {
    LocalReviewRequest {
        version: 1,
        request_id: m::RequestId::new([byte; 16]),
        command,
    }
}
fn create(byte: u8) -> LocalReviewRequest {
    request(
        byte,
        LocalReviewCommand::RepositoryThread {
            body: body("Initial review"),
        },
    )
}
fn receipt(outcome: LocalReviewOutcome) -> LocalReviewReceipt {
    match outcome {
        LocalReviewOutcome::Committed { receipt } => *receipt,
        other => panic!("expected commit: {other:?}"),
    }
}

#[tokio::test]
async fn local_review_reads_leave_metadata_and_actor_absent() {
    let (root, repo) = fixture();
    let store = repo.review_store().await.unwrap();
    assert!(store.snapshot.repo_id.is_none());
    assert!(
        store
            .reads()
            .list_threads(None, 20)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    assert!(matches!(
        repo.review_command_status(m::RequestId::new([1; 16]))
            .unwrap(),
        LocalReviewOutcome::NotKnown { .. }
    ));
    assert!(repo.metadata_store().read().unwrap().is_none());
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
}

#[tokio::test]
async fn local_review_preserves_domains_replays_after_gc_and_pins_old_threads() {
    let (root, repo) = fixture();
    repo.apply_work_command(&Request {
        version: 1,
        request_id: m::RequestId::new([70; 16]),
        command: WorkCommandV1::CreateProject {
            id: ProjectId::new([70; 16]),
            title: "Project".into(),
            body: String::new(),
        },
    })
    .unwrap();
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([71; 16]),
        command: LocalAgentCommand::Initialize,
    })
    .await
    .unwrap();
    let before = repo.metadata_store().read().unwrap().unwrap();
    let created = receipt(repo.apply_review_command(&create(1)).await.unwrap());
    let first_result = created.command.clone().unwrap();
    let first = repo.review_store().await.unwrap();
    let reply = request(
        2,
        LocalReviewCommand::Apply {
            intent: Box::new(ReviewCommandV1::AppendReply {
                thread_id: first_result.thread_id,
                expected_workflow_version: 1,
                parent_comment_id: first_result.comment_id,
                body: body("Reply"),
            }),
        },
    );
    let replied = receipt(repo.apply_review_command(&reply).await.unwrap());
    let reply_result = replied.command.unwrap();
    assert!(matches!(
        first.object(reply_result.thread_snapshot).await,
        Err(LocalError::Stale)
    ));
    let ReviewObjectV1::ThreadSnapshot(thread) =
        first.object(first_result.thread_snapshot).await.unwrap()
    else {
        panic!("thread")
    };
    assert_eq!(thread.comments.len(), 1);
    let stale = request(
        3,
        LocalReviewCommand::Apply {
            intent: Box::new(ReviewCommandV1::ResolveThread {
                thread_id: first_result.thread_id,
                expected_thread_version: ThreadSnapshotId::new(
                    *first_result.thread_snapshot.id.as_bytes(),
                ),
                reason: None,
            }),
        },
    );
    assert_eq!(
        repo.apply_review_command(&stale).await.unwrap_err(),
        LocalError::Stale
    );
    let resolve = request(
        4,
        LocalReviewCommand::Apply {
            intent: Box::new(ReviewCommandV1::ResolveThread {
                thread_id: first_result.thread_id,
                expected_thread_version: ThreadSnapshotId::new(
                    *reply_result.thread_snapshot.id.as_bytes(),
                ),
                reason: None,
            }),
        },
    );
    repo.apply_review_command(&resolve).await.unwrap();
    assert!(matches!(
        repo.apply_review_command(&request(5, reply.command.clone()))
            .await,
        Err(LocalError::ThreadResolved)
    ));
    let after = repo.metadata_store().read().unwrap().unwrap();
    assert_eq!(
        after.selection(Domain::Work),
        before.selection(Domain::Work)
    );
    assert_eq!(
        after.selection(Domain::Agent),
        before.selection(Domain::Agent)
    );
    let path = root.path().join("repo");
    assert!(
        Command::new("git")
            .arg("--git-dir")
            .arg(path.join(".git"))
            .args(["gc", "--prune=now"])
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .status()
            .unwrap()
            .success()
    );
    let restarted = reopen(&path);
    assert_eq!(
        receipt(restarted.apply_review_command(&create(1)).await.unwrap()),
        created
    );
    assert_eq!(
        receipt(
            restarted
                .review_command_status(create(1).request_id)
                .unwrap()
        ),
        created
    );
    assert_eq!(
        restarted.metadata_store().read().unwrap().unwrap().commit(),
        after.commit()
    );
    let current = restarted.review_store().await.unwrap();
    assert_eq!(
        current
            .reads()
            .list_threads(None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    let mut altered = create(1);
    altered.command = LocalReviewCommand::RepositoryThread {
        body: body("Changed intent"),
    };
    assert_eq!(
        restarted.apply_review_command(&altered).await.unwrap_err(),
        LocalError::IntentConflict
    );
}

#[tokio::test]
async fn local_review_unknown_outcome_retries_exact_request_after_restart() {
    let (root, repo) = fixture();
    repo.apply_review_command(&create(1)).await.unwrap();
    let before = repo.metadata_store().read().unwrap().unwrap();
    let path = root.path().join("repo");
    let operation_heads = std::fs::read_dir(path.join(".jj/repo/op_heads"))
        .unwrap()
        .map(|v| v.unwrap().file_name())
        .collect::<Vec<_>>();
    let lock = path.join(".git/refs/lapis/state/v1.lock");
    std::fs::write(&lock, b"disposable fixture lock").unwrap();
    let command = create(2);
    assert!(
        matches!(repo.apply_review_command(&command).await.unwrap(), LocalReviewOutcome::OutcomeUnknown { request_id } if request_id == command.request_id)
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
    std::fs::remove_file(lock).unwrap();
    let restarted = reopen(&path);
    let committed = restarted.apply_review_command(&command).await.unwrap();
    assert_eq!(
        restarted.apply_review_command(&command).await.unwrap(),
        committed
    );
    assert_eq!(
        restarted
            .review_store()
            .await
            .unwrap()
            .reads()
            .list_threads(None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        2
    );
    assert_eq!(
        std::fs::read_dir(path.join(".jj/repo/op_heads"))
            .unwrap()
            .map(|v| v.unwrap().file_name())
            .collect::<Vec<_>>(),
        operation_heads
    );
}

#[tokio::test]
async fn local_review_rejects_foreign_and_unqualified_anchors_without_publication() {
    let (_root, repo) = fixture();
    let initialized = receipt(repo.apply_review_command(&create(1)).await.unwrap());
    let before = repo.metadata_store().read().unwrap().unwrap();
    let mut anchor = ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: m::RepositoryId::new([9; 16]),
        target: ArtifactTargetV1::Repository,
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    };
    let foreign = request(
        2,
        LocalReviewCommand::Apply {
            intent: Box::new(ReviewCommandV1::CreateThread {
                anchor: Box::new(anchor.clone()),
                body: body("Foreign"),
            }),
        },
    );
    assert_eq!(
        repo.apply_review_command(&foreign).await.unwrap_err(),
        LocalError::Invalid
    );
    anchor.repository_id = initialized.repository;
    anchor.target = ArtifactTargetV1::Operation {
        operation_id: m::OperationId::new([1; 64]),
    };
    let unsupported = request(
        3,
        LocalReviewCommand::Apply {
            intent: Box::new(ReviewCommandV1::CreateThread {
                anchor: Box::new(anchor),
                body: body("Unqualified"),
            }),
        },
    );
    assert_eq!(
        repo.apply_review_command(&unsupported).await.unwrap_err(),
        LocalError::Unsupported
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
    assert!(matches!(
        repo.review_command_status(foreign.request_id).unwrap(),
        LocalReviewOutcome::NotKnown { .. }
    ));
}

#[tokio::test]
async fn local_review_selected_inventory_must_contain_the_complete_root_closure() {
    use jj_store_jj::local_metadata::{GitOid, Preparation, PublishOutcome, RetainedKind, Update};
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Descriptor {
        version: u32,
        repository: m::RepositoryId,
        root: jj_store_wal::records::RecordRef,
        records: std::collections::BTreeMap<String, GitOid>,
    }
    let (_root, repo) = fixture();
    repo.apply_review_command(&create(1)).await.unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let bytes = repo
        .metadata_store()
        .read_domain(&selected, Domain::Review)
        .unwrap()
        .unwrap();
    let mut descriptor: Descriptor = serde_json::from_slice(&bytes).unwrap();
    let object = descriptor
        .records
        .keys()
        .find(|key| key.contains("/review-objects/"))
        .unwrap()
        .clone();
    descriptor.records.remove(&object);
    let update = Update {
        repository: selected.repository(),
        actor: repo.actor().unwrap().unwrap(),
        request: m::RequestId::new([90; 16]),
        intent: m::IntentDigest::new([90; 32]),
        domain: Domain::Review,
        expected: selected.selection(Domain::Review).cloned(),
        bytes: serde_json::to_vec(&descriptor).unwrap(),
        result: vec![],
        retained: descriptor
            .records
            .values()
            .map(|id| (*id, RetainedKind::Blob))
            .collect(),
    };
    let Preparation::Ready(candidate) = repo
        .metadata_store()
        .prepare(Some(&selected), &update)
        .unwrap()
    else {
        panic!("candidate")
    };
    assert!(matches!(
        repo.metadata_store().publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert!(matches!(
        repo.review_store().await,
        Err(LocalError::Corrupt)
    ));
}
