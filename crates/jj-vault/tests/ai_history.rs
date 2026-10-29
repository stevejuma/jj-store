use jj_vault::{
    AiHistoryRequest as Request, AiHistoryResponse as Response, AiTurnContext, AiTurnOutcome,
    AiTurnPhase, AiUndoRequest, Backup, Binding, NativeRepository, Timing,
};
use std::fs;

fn context(turn: &str) -> AiTurnContext {
    AiTurnContext {
        conversation: "conversation-1".into(),
        turn: turn.into(),
        message: None,
        binding: None,
        session: None,
        agent: "writer".into(),
        harness: "lapis".into(),
        requested_model: Some("small-local".into()),
        resolved_model: None,
    }
}
fn fixture() -> (tempfile::TempDir, Binding, Backup) {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "ai-history".into(),
        root: base.join("notes"),
        state: base.join("history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(binding.root.join("Note.md"), "before\n").unwrap();
    let backup = Backup::start_idle(
        NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        },
        Timing::default(),
    )
    .unwrap();
    (temp, binding, backup)
}

#[tokio::test]
async fn checkpoints_keep_context_and_guarded_undo_preserves_later_and_unrelated_edits() {
    let (_temp, binding, backup) = fixture();
    let begin = backup
        .ai_history(Request::Begin(context("turn-1")))
        .await
        .unwrap();
    let Response::Turn(begin) = begin else {
        panic!()
    };
    fs::write(binding.root.join("Note.md"), "after\n").unwrap();
    fs::write(binding.root.join("Added.md"), "added\n").unwrap();
    let checkpoint = backup.capture().await.unwrap().checkpoint.unwrap();
    assert!(
        checkpoint
            .description
            .contains("Lapis-AI-Observation-Version: 1")
    );
    assert!(checkpoint.description.contains("small-local"));
    let Response::Turn(end) = backup
        .ai_history(Request::End {
            conversation: "conversation-1".into(),
            turn: "turn-1".into(),
            outcome: AiTurnOutcome::Completed,
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(end.phase, AiTurnPhase::Complete);
    assert_ne!(begin.before.commit, end.after.unwrap().commit);
    fs::write(binding.root.join("Unrelated.md"), "keep me").unwrap();
    let Response::Preview(preview) = backup
        .ai_history(Request::PreviewUndo {
            conversation: "conversation-1".into(),
            turn: "turn-1".into(),
            paths: vec![],
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(preview.paths, ["Added.md", "Note.md"]);
    let request = AiUndoRequest {
        request_id: "undo-1".into(),
        conversation: "conversation-1".into(),
        turn: "turn-1".into(),
        paths: preview.paths,
        expected: preview.expected,
    };
    fs::write(binding.root.join("Note.md"), "later human edit").unwrap();
    assert!(
        backup
            .ai_history(Request::Undo(request.clone()))
            .await
            .is_err()
    );
    assert!(
        binding.root.join("Added.md").exists(),
        "preflight refuses before any writes"
    );
    fs::write(binding.root.join("Note.md"), "after\n").unwrap();
    backup
        .ai_history(Request::Undo(request.clone()))
        .await
        .unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "before\n"
    );
    assert!(!binding.root.join("Added.md").exists());
    assert_eq!(
        fs::read_to_string(binding.root.join("Unrelated.md")).unwrap(),
        "keep me"
    );
    fs::write(binding.root.join("Note.md"), "after undo user edit").unwrap();
    backup.ai_history(Request::Undo(request)).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("Note.md")).unwrap(),
        "after undo user edit"
    );
    backup.close().await.unwrap();
}

#[tokio::test]
async fn no_change_reuses_checkpoint_and_restart_marks_open_turn_incomplete() {
    let (_temp, binding, backup) = fixture();
    let Response::Turn(begin) = backup
        .ai_history(Request::Begin(context("unchanged")))
        .await
        .unwrap()
    else {
        panic!()
    };
    let Response::Turn(end) = backup
        .ai_history(Request::End {
            conversation: "conversation-1".into(),
            turn: "unchanged".into(),
            outcome: AiTurnOutcome::Completed,
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(begin.before.commit, end.after.unwrap().commit);
    backup
        .ai_history(Request::Begin(context("interrupted")))
        .await
        .unwrap();
    assert!(
        backup
            .ai_history(Request::PreviewUndo {
                conversation: "conversation-1".into(),
                turn: "unchanged".into(),
                paths: vec![]
            })
            .await
            .is_err()
    );
    backup.close().await.unwrap();
    let backup = Backup::start_idle(
        NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        },
        Timing::default(),
    )
    .unwrap();
    let Response::Turn(record) = backup
        .ai_history(Request::Read {
            conversation: "conversation-1".into(),
            turn: "interrupted".into(),
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(record.phase, AiTurnPhase::Incomplete);
    assert!(record.after.is_none());
    fs::write(binding.root.join("Note.md"), "after restart").unwrap();
    let checkpoint = backup.capture().await.unwrap().checkpoint.unwrap();
    assert!(!checkpoint.description.contains("interrupted"));
    backup.close().await.unwrap();
}

#[tokio::test]
async fn overlapping_turns_keep_both_contexts_and_transcripts_are_never_undo_targets() {
    let (_temp, binding, backup) = fixture();
    let transcript = binding
        .root
        .join(".lapis/agents/sessions/11111111-1111-4111-8111-111111111111/transcript.jsonl");
    fs::create_dir_all(transcript.parent().unwrap()).unwrap();
    fs::write(&transcript, "before transcript").unwrap();
    backup
        .ai_history(Request::Begin(context("first")))
        .await
        .unwrap();
    let mut second = context("second");
    second.agent = "reviewer".into();
    backup
        .ai_history(Request::Begin(second.clone()))
        .await
        .unwrap();
    second.binding = Some("binding-two".into());
    second.session = Some("native-session-two".into());
    backup.ai_history(Request::Bind(second)).await.unwrap();
    fs::write(
        binding.root.join("Note.md"),
        "changed by either participant",
    )
    .unwrap();
    fs::write(&transcript, "after transcript").unwrap();
    let captured = backup.capture().await.unwrap().checkpoint.unwrap();
    assert!(captured.description.contains("writer"));
    assert!(captured.description.contains("reviewer"));
    assert!(captured.description.contains("native-session-two"));
    for turn in ["first", "second"] {
        backup
            .ai_history(Request::End {
                conversation: "conversation-1".into(),
                turn: turn.into(),
                outcome: AiTurnOutcome::Completed,
            })
            .await
            .unwrap();
    }
    let Response::Preview(preview) = backup
        .ai_history(Request::PreviewUndo {
            conversation: "conversation-1".into(),
            turn: "first".into(),
            paths: vec![],
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(preview.paths, ["Note.md"]);
    backup
        .ai_history(Request::Undo(AiUndoRequest {
            request_id: "undo-first".into(),
            conversation: "conversation-1".into(),
            turn: "first".into(),
            paths: preview.paths,
            expected: preview.expected,
        }))
        .await
        .unwrap();
    assert_eq!(fs::read_to_string(transcript).unwrap(), "after transcript");
    backup.close().await.unwrap();
}

#[cfg(unix)]
#[tokio::test]
async fn executable_changes_and_symlink_ancestors_refuse_undo() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    let (_temp, binding, backup) = fixture();
    fs::create_dir(binding.root.join("Folder")).unwrap();
    fs::write(binding.root.join("Folder/note.md"), "before").unwrap();
    backup
        .ai_history(Request::Begin(context("modes")))
        .await
        .unwrap();
    fs::write(binding.root.join("Folder/note.md"), "after").unwrap();
    backup
        .ai_history(Request::End {
            conversation: "conversation-1".into(),
            turn: "modes".into(),
            outcome: AiTurnOutcome::Completed,
        })
        .await
        .unwrap();
    let file = binding.root.join("Folder/note.md");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(
        backup
            .ai_history(Request::PreviewUndo {
                conversation: "conversation-1".into(),
                turn: "modes".into(),
                paths: vec![]
            })
            .await
            .is_err()
    );
    fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
    fs::rename(binding.root.join("Folder"), binding.root.join("Actual")).unwrap();
    symlink(binding.root.join("Actual"), binding.root.join("Folder")).unwrap();
    assert!(
        backup
            .ai_history(Request::PreviewUndo {
                conversation: "conversation-1".into(),
                turn: "modes".into(),
                paths: vec![]
            })
            .await
            .is_err()
    );
    fs::remove_file(binding.root.join("Folder")).unwrap();
    fs::rename(binding.root.join("Actual"), binding.root.join("Folder")).unwrap();
    backup.close().await.unwrap();
}

#[test]
fn cold_native_history_survives_two_processes_and_undo() {
    let directory = tempfile::tempdir().unwrap();
    for phase in ["capture", "undo"] {
        let result = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "cold_native_history_child", "--nocapture"])
            .env("LAPIS_AI_HISTORY_TEST_ROOT", directory.path())
            .env("LAPIS_AI_HISTORY_TEST_PHASE", phase)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn cold_native_history_child() {
    let Some(root) = std::env::var_os("LAPIS_AI_HISTORY_TEST_ROOT") else {
        return;
    };
    let root = fs::canonicalize(root).unwrap();
    let binding = Binding {
        vault_id: "cold-ai".into(),
        root: root.join("notes"),
        state: root.join("history"),
    };
    let phase = std::env::var("LAPIS_AI_HISTORY_TEST_PHASE").unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        if phase == "capture" {
            fs::create_dir(&binding.root).unwrap();
            fs::write(binding.root.join("Note.md"), "original").unwrap();
        }
        let backup = Backup::start_idle(
            NativeRepository {
                binding: binding.clone(),
                exclusions: vec![],
            },
            Timing::default(),
        )
        .unwrap();
        if phase == "capture" {
            backup
                .ai_history(Request::Begin(context("cold-turn")))
                .await
                .unwrap();
            fs::write(binding.root.join("Note.md"), "agent update").unwrap();
            backup
                .ai_history(Request::End {
                    conversation: "conversation-1".into(),
                    turn: "cold-turn".into(),
                    outcome: AiTurnOutcome::Completed,
                })
                .await
                .unwrap();
        } else {
            let Response::Turn(saved) = backup
                .ai_history(Request::Read {
                    conversation: "conversation-1".into(),
                    turn: "cold-turn".into(),
                })
                .await
                .unwrap()
            else {
                panic!()
            };
            assert!(
                saved
                    .after
                    .as_ref()
                    .unwrap()
                    .description
                    .contains("small-local")
            );
            let Response::Preview(preview) = backup
                .ai_history(Request::PreviewUndo {
                    conversation: "conversation-1".into(),
                    turn: "cold-turn".into(),
                    paths: vec![],
                })
                .await
                .unwrap()
            else {
                panic!()
            };
            backup
                .ai_history(Request::Undo(AiUndoRequest {
                    request_id: "cold-undo".into(),
                    conversation: "conversation-1".into(),
                    turn: "cold-turn".into(),
                    paths: preview.paths,
                    expected: preview.expected,
                }))
                .await
                .unwrap();
            assert_eq!(
                fs::read_to_string(binding.root.join("Note.md")).unwrap(),
                "original"
            );
        }
        backup.close().await.unwrap();
    });
}
