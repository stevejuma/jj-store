use jj_review::*;
use jj_store_jj::{local_evidence, local_metadata::*};
use jj_store_model::{self as m, portable::*};
use jj_store_service::local_repository::*;
use std::{path::Path, process::Command};

fn jj(path: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(path)
        .args([
            "--no-pager",
            "--config",
            "user.name='Fixture'",
            "--config",
            "user.email='fixture@invalid'",
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn open(path: &Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
}
fn body() -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: "Keep the greeting".into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}
fn request(id: u8, command: LocalReviewCommand) -> LocalReviewRequest {
    LocalReviewRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    }
}
fn receipt(outcome: LocalReviewOutcome) -> LocalReviewReceipt {
    let LocalReviewOutcome::Committed { receipt } = outcome else {
        panic!("not committed")
    };
    *receipt
}
async fn source(
    path: &Path,
    repo: &LocalRepository,
    repository: m::RepositoryId,
) -> (LocalCodeEvidenceV1, PortableCodeArtifactV2) {
    let operation = jj(path, &["op", "log", "--no-graph", "-n", "1", "-T", "id"])
        .parse()
        .unwrap();
    let commit = jj(
        path,
        &[
            "log",
            "--no-graph",
            "--ignore-working-copy",
            "-r",
            "@",
            "-T",
            "commit_id",
        ],
    )
    .parse()
    .unwrap();
    let settings =
        jj_lib::settings::UserSettings::from_config(jj_lib::config::StackedConfig::with_defaults())
            .unwrap();
    let capsule = local_evidence::inspect(&settings, &path.join(".jj/repo"), repository, operation)
        .await
        .unwrap();
    let (id, _) = repo
        .metadata_store()
        .read_capsule_file(&capsule, commit, "code.txt")
        .unwrap();
    let reference = CodeEvidenceRefV1 {
        repository,
        profile: CodeProfileV1::GitSha1Jj045,
        operation,
        capsule: capsule.id().unwrap(),
    };
    let artifact = PortableCodeArtifactV2 {
        revision: PortableRevisionV2 {
            evidence: reference.clone(),
            commit: CodeCommitIdV1::GitSha1Jj045(commit),
            change: Some(capsule.commits[&commit].change_id),
        },
        path: "code.txt".into(),
        object: CodeObjectIdV1::GitSha1Jj045 {
            kind: GitObjectKindV1::Blob,
            id,
        },
        value_kind: CodeValueKind::File,
    };
    (
        LocalCodeEvidenceV1 {
            reference,
            bytes: capsule.encode().unwrap(),
        },
        artifact,
    )
}
fn anchor(repository: m::RepositoryId, artifact: PortableCodeArtifactV2) -> ArtifactAnchorV1 {
    ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: repository,
        target: ArtifactTargetV1::PortableCodeText(Box::new(PortableCodeTextV2 {
            artifact,
            text: TextSelectionV1 {
                encoding: TextEncoding::Utf8,
                start_byte: ByteOffset::new(0),
                end_byte: ByteOffset::new("café 🦀".len() as u64),
                selected_sha256: ReviewDigest::sha256("café 🦀".as_bytes()),
            },
            diff_context: None,
        })),
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    }
}

#[tokio::test]
async fn portable_file_reviews_replay_unknown_publication_and_retain_original_after_cleanup_and_gc()
{
    let root = tempfile::tempdir().unwrap();
    let path = root.path();
    jj(path, &["git", "init", "--colocate"]);
    std::fs::write(path.join("code.txt"), "café 🦀\nunchanged source\n").unwrap();
    jj(path, &["describe", "-m", "Source"]);
    let repo = open(path);
    let initialized = receipt(
        repo.apply_review_command(&request(1, LocalReviewCommand::Initialize))
            .await
            .unwrap(),
    );
    let (evidence, artifact) = source(path, &repo, initialized.repository).await;
    let original = anchor(initialized.repository, artifact.clone());
    let command = request(
        2,
        LocalReviewCommand::WithEvidence {
            intent: Box::new(ReviewCommandV1::CreateThread {
                anchor: Box::new(original.clone()),
                body: body(),
            }),
            evidence: vec![evidence.clone()],
        },
    );
    let pin = repo.metadata_store().read().unwrap().unwrap();
    let old = repo.review_store().await.unwrap();
    let lock = path.join(".git/refs/lapis/state/v1.lock");
    std::fs::write(&lock, b"test unknown outcome").unwrap();
    assert!(matches!(
        repo.apply_review_command(&command).await.unwrap(),
        LocalReviewOutcome::OutcomeUnknown { .. }
    ));
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        pin.commit()
    );
    assert!(
        repo.review_threads_at(None, 20, None)
            .await
            .unwrap()
            .threads
            .is_empty()
    );
    std::fs::remove_file(lock).unwrap();
    let committed = receipt(open(path).apply_review_command(&command).await.unwrap());
    let result = committed.command.clone().unwrap();
    let store = open(path).review_store().await.unwrap();
    let ReviewObjectV1::ThreadSnapshot(thread) =
        store.object(result.thread_snapshot).await.unwrap()
    else {
        panic!()
    };
    assert!(matches!(
        old.file_source(thread.original_anchor).await,
        Err(LocalError::Stale)
    ));
    assert_eq!(
        store.file_source(thread.original_anchor).await.unwrap(),
        "café 🦀\nunchanged source\n".as_bytes()
    );
    let mut bad = original.clone();
    if let ArtifactTargetV1::PortableCodeText(v) = &mut bad.target {
        v.text.start_byte = ByteOffset::new(4);
    }
    let refused = request(
        3,
        LocalReviewCommand::WithEvidence {
            intent: Box::new(ReviewCommandV1::CreateThread {
                anchor: Box::new(bad),
                body: body(),
            }),
            evidence: vec![evidence.clone()],
        },
    );
    let selected = repo.metadata_store().read().unwrap().unwrap().commit();
    assert!(repo.apply_review_command(&refused).await.is_err());
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        selected
    );
    let mut bad = original.clone();
    if let ArtifactTargetV1::PortableCodeText(v) = &mut bad.target {
        v.artifact.path = "missing.txt".into();
    }
    assert!(
        repo.apply_review_command(&request(
            4,
            LocalReviewCommand::WithEvidence {
                intent: Box::new(ReviewCommandV1::CreateThread {
                    anchor: Box::new(bad),
                    body: body()
                }),
                evidence: vec![evidence]
            }
        ))
        .await
        .is_err()
    );
    std::fs::rename(path.join("code.txt"), path.join("renamed.txt")).unwrap();
    std::fs::write(path.join("renamed.txt"), "later edits").unwrap();
    jj(path, &["describe", "-m", "Later source"]);
    std::fs::remove_file(path.join("renamed.txt")).unwrap();
    jj(path, &["describe", "-m", "Deleted source"]);
    // Historical source reads need neither the operation store nor the checkout.
    std::fs::rename(path.join(".jj/repo/op_store"), path.join("saved-op-store")).unwrap();
    let gc = Command::new("git")
        .current_dir(path)
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        gc.status.success(),
        "{}",
        String::from_utf8_lossy(&gc.stderr)
    );
    let reopened = open(path);
    assert_eq!(
        receipt(reopened.apply_review_command(&command).await.unwrap()),
        committed
    );
    let target = jj_collab::ReactionTargetV1::RepositoryReviewComment {
        repository: initialized.repository,
        thread_id: result.thread_id,
        comment_id: result.comment_id.unwrap(),
        revision: result.comment_revision.unwrap(),
    };
    verify_qualified_submission(&reopened, target.clone()).await;
    let qualified = reopened.qualified_file_comment(&target).await.unwrap();
    assert_eq!(qualified.body, body().text);
    assert_eq!(qualified.file, "café 🦀\nunchanged source\n".as_bytes());
    let (source, surrounding) =
        jj_store_service::agent_execution::context_submission::frozen_file_source(&qualified)
            .unwrap();
    assert_eq!(source.anchor, original);
    assert_eq!(source.quote, "café 🦀");
    assert_eq!(surrounding, "café 🦀\nunchanged source\n");
    let mut foreign = target.clone();
    if let jj_collab::ReactionTargetV1::RepositoryReviewComment { repository, .. } = &mut foreign {
        *repository = m::RepositoryId::new([99; 16]);
    }
    assert!(reopened.qualified_file_comment(&foreign).await.is_err());
    let restarted = reopened.review_store().await.unwrap();
    assert_eq!(
        restarted.file_source(thread.original_anchor).await.unwrap(),
        "café 🦀\nunchanged source\n".as_bytes()
    );
    assert_eq!(
        reopened
            .review_threads_at(None, 20, None)
            .await
            .unwrap()
            .threads
            .len(),
        1
    );
}

#[tokio::test]
async fn portable_review_diff_sides_and_whole_files_require_exact_capsules_paths_and_blobs() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path();
    jj(path, &["git", "init", "--colocate"]);
    std::fs::write(path.join("code.txt"), "café 🦀 before\n").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink("/etc/passwd", path.join("outside")).unwrap();
    jj(path, &["describe", "-m", "Before"]);
    let repo = open(path);
    let repository = receipt(
        repo.apply_review_command(&request(1, LocalReviewCommand::Initialize))
            .await
            .unwrap(),
    )
    .repository;
    let (before_evidence, before) = source(path, &repo, repository).await;
    std::fs::write(path.join("code.txt"), "café 🦀 after\n").unwrap();
    jj(path, &["describe", "-m", "After"]);
    let (after_evidence, after) = source(path, &repo, repository).await;
    let evidence = vec![before_evidence.clone(), after_evidence.clone()];
    for (id, side, artifact, expected) in [
        (2, DiffSide::Before, before.clone(), "café 🦀 before\n"),
        (3, DiffSide::After, after.clone(), "café 🦀 after\n"),
    ] {
        let mut selected = anchor(repository, artifact);
        if let ArtifactTargetV1::PortableCodeText(text) = &mut selected.target {
            text.diff_context = Some(PortableDiffContextV2 {
                before: before.revision.clone(),
                before_path: Some("code.txt".into()),
                after: after.revision.clone(),
                after_path: Some("code.txt".into()),
                side,
            });
        }
        let intent = ReviewCommandV1::CreateThread {
            anchor: Box::new(selected),
            body: body(),
        };
        let result = receipt(
            repo.apply_review_command(&request(
                id,
                LocalReviewCommand::WithEvidence {
                    intent: Box::new(intent),
                    evidence: evidence.clone(),
                },
            ))
            .await
            .unwrap(),
        )
        .command
        .unwrap();
        let store = repo.review_store().await.unwrap();
        let ReviewObjectV1::ThreadSnapshot(thread) =
            store.object(result.thread_snapshot).await.unwrap()
        else {
            panic!()
        };
        assert_eq!(
            store.file_source(thread.original_anchor).await.unwrap(),
            expected.as_bytes()
        );
    }
    let mut selected = anchor(repository, before.clone());
    selected.target = ArtifactTargetV1::PortableCodeArtifact(Box::new(before.clone()));
    let whole = request(
        4,
        LocalReviewCommand::WithEvidence {
            intent: Box::new(ReviewCommandV1::CreateThread {
                anchor: Box::new(selected.clone()),
                body: body(),
            }),
            evidence: vec![before_evidence.clone()],
        },
    );
    repo.apply_review_command(&whole).await.unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    // Supplying a physically present but wrong blob cannot authorize another path.
    if let ArtifactTargetV1::PortableCodeArtifact(v) = &mut selected.target {
        v.object = after.object.clone();
    }
    assert!(
        repo.apply_review_command(&request(
            5,
            LocalReviewCommand::WithEvidence {
                intent: Box::new(ReviewCommandV1::CreateThread {
                    anchor: Box::new(selected),
                    body: body()
                }),
                evidence: vec![before_evidence.clone()]
            }
        ))
        .await
        .is_err()
    );
    assert_eq!(repo.metadata_store().read().unwrap().unwrap().commit(), pin);
    let capsule = before_evidence.validate().unwrap();
    let CodeCommitIdV1::GitSha1Jj045(commit) = before.revision.commit else {
        panic!()
    };
    for path in [
        "outside",
        "../code.txt",
        ".git/config",
        ".jj/repo/store/type",
    ] {
        assert!(
            repo.metadata_store()
                .read_capsule_file(&capsule, commit, path)
                .is_err()
        );
    }
    // A fabricated capsule tree must not override the immutable Git commit.
    let mut forged = capsule;
    let CodeCommitIdV1::GitSha1Jj045(after_commit) = after.revision.commit else {
        panic!()
    };
    forged.commits.get_mut(&commit).unwrap().root_tree = after_evidence.validate().unwrap().commits
        [&after_commit]
        .root_tree
        .clone();
    assert!(
        repo.metadata_store()
            .read_capsule_file(&forged, commit, "code.txt")
            .is_err()
    );
}

struct ReviewAuthority<'a>(&'a LocalRepository);
#[async_trait::async_trait]
impl jj_collab::QualifiedReviewRead for ReviewAuthority<'_> {
    async fn comment(
        &self,
        target: &jj_collab::ReactionTargetV1,
    ) -> jj_collab::Result<jj_collab::QualifiedReviewComment> {
        self.0.qualified_file_comment(target).await
    }
}
async fn verify_qualified_submission(code: &LocalRepository, target: jj_collab::ReactionTargetV1) {
    use jj_collab::*;
    let root = tempfile::tempdir().unwrap();
    jj(root.path(), &["git", "init", "--colocate"]);
    let home = open(root.path());
    let thread_id = ThreadId::new([91; 16]);
    let create = LocalCollabRequest {
        version: 1,
        request_id: m::RequestId::new([91; 16]),
        command: CollabCommandV1::Thread {
            intent: ThreadCommandV1::Create {
                thread_id,
                title: "Code discussion".into(),
                baseline: None,
            },
        },
    };
    let LocalCollabOutcome::Committed { receipt } =
        home.apply_collab_command(&create).await.unwrap()
    else {
        panic!()
    };
    let CollabCommandResultV1::Thread { result } = receipt.result else {
        panic!()
    };
    let submit = LocalCollabRequest {
        version: 1,
        request_id: m::RequestId::new([92; 16]),
        command: CollabCommandV1::Thread {
            intent: ThreadCommandV1::Submit {
                submission: Box::new(ConversationSubmissionV1 {
                    emojis: Vec::new(),
                    workspace_comments: vec![],
                    thread_id,
                    expected_thread: result.version,
                    message_id: None,
                    text: None,
                    reply_to: None,
                    comments: vec![],
                    references: vec![target],
                }),
            },
        },
    };
    let before = home.metadata_store().read().unwrap().unwrap().commit();
    assert!(home.apply_collab_command(&submit).await.is_err());
    assert_eq!(
        home.metadata_store().read().unwrap().unwrap().commit(),
        before
    );
    let authority = ReviewAuthority(code);
    let code_before = code.metadata_store().read().unwrap().unwrap().commit();
    let outcome = home
        .apply_collab_command_with_reviews(&submit, Some(&authority))
        .await
        .unwrap();
    let LocalCollabOutcome::Committed { receipt } = &outcome else {
        panic!()
    };
    let CollabCommandResultV1::Thread { result } = &receipt.result else {
        panic!()
    };
    assert!(
        home.conversation_submission_context(thread_id, result.link.unwrap())
            .await
            .is_err()
    );
    let context = home
        .conversation_submission_context_with_reviews(
            thread_id,
            result.link.unwrap(),
            Some(&authority),
        )
        .await
        .unwrap();
    assert_eq!(context.comments.len(), 1);
    assert_eq!(context.comments[0].file.as_ref().unwrap().quote, "café 🦀");
    assert_eq!(context.comments[0].body, body().text);
    assert_eq!(
        home.apply_collab_command_with_reviews(&submit, Some(&authority))
            .await
            .unwrap(),
        outcome
    );
    assert_eq!(
        code.metadata_store().read().unwrap().unwrap().commit(),
        code_before
    );
    assert!(
        home.review_threads_at(None, 20, None)
            .await
            .unwrap()
            .threads
            .is_empty()
    );
}
