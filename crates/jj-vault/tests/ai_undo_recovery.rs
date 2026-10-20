use jj_vault::{
    AiHistoryRequest as Request, AiHistoryResponse as Response, AiTurnContext, AiTurnOutcome,
    AiUndoRequest, Backup, Binding, NativeRepository, Timing,
};
use std::fs;

#[tokio::test]
async fn interrupted_batch_checks_completed_paths_before_touching_remaining_files() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "partial-ai-undo".into(),
        root: base.join("notes"),
        state: base.join("history"),
    };
    fs::create_dir(&binding.root).unwrap();
    for path in ["A.md", "B.md"] {
        fs::write(binding.root.join(path), "original").unwrap();
    }
    let backup = Backup::start_idle(
        NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        },
        Timing::default(),
    )
    .unwrap();
    backup
        .ai_history(Request::Begin(AiTurnContext {
            conversation: "conversation".into(),
            turn: "turn".into(),
            message: None,
            binding: None,
            session: None,
            agent: "agent".into(),
            harness: "local".into(),
            requested_model: None,
            resolved_model: None,
        }))
        .await
        .unwrap();
    for path in ["A.md", "B.md"] {
        fs::write(binding.root.join(path), "updated").unwrap();
    }
    let Response::Turn(record) = backup
        .ai_history(Request::End {
            conversation: "conversation".into(),
            turn: "turn".into(),
            outcome: AiTurnOutcome::Completed,
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    let Response::Preview(preview) = backup
        .ai_history(Request::PreviewUndo {
            conversation: "conversation".into(),
            turn: "turn".into(),
            paths: vec![],
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    let request = AiUndoRequest {
        request_id: "interrupted".into(),
        conversation: "conversation".into(),
        turn: "turn".into(),
        paths: preview.paths,
        expected: preview.expected,
    };
    backup.close().await.unwrap();
    // Fault fixture: one guarded replacement and its progress were durable when
    // the original process stopped. The native recovery API owns continuation.
    fs::create_dir_all(binding.state.join("ai-undo")).unwrap();
    fs::write(binding.state.join("ai-undo/interrupted.json"), serde_json::to_vec(&serde_json::json!({ "request": request, "safety": record.after.unwrap(), "next": 1, "result": null })).unwrap()).unwrap();
    fs::write(
        binding.state.join("ai-undo/.tmp-interrupted"),
        "partial atomic write",
    )
    .unwrap();
    fs::write(binding.root.join("A.md"), "later human edit").unwrap();
    let backup = Backup::start_idle(
        NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        },
        Timing::default(),
    )
    .unwrap();
    let Response::Pending(pending) = backup.ai_history(Request::PendingUndo {}).await.unwrap()
    else {
        panic!()
    };
    assert_eq!(pending.as_slice(), std::slice::from_ref(&request));
    assert!(
        backup
            .ai_history(Request::Undo(request.clone()))
            .await
            .is_err()
    );
    assert_eq!(
        fs::read_to_string(binding.root.join("B.md")).unwrap(),
        "updated"
    );
    fs::write(binding.root.join("A.md"), "original").unwrap();
    backup.ai_history(Request::Undo(request)).await.unwrap();
    assert_eq!(
        fs::read_to_string(binding.root.join("B.md")).unwrap(),
        "original"
    );
    let Response::Pending(pending) = backup.ai_history(Request::PendingUndo {}).await.unwrap()
    else {
        panic!()
    };
    assert!(pending.is_empty());
    backup.close().await.unwrap();
}
