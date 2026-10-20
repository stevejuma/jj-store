#![cfg(feature = "vault-backup")]
use jj_store_jj::vault_workspace::{
    Binding, PushOutcome, RemoteTransfer, Session, VaultRestoreRequest,
};
use std::{fs, path::Path, process::Command};
fn vault(base: &Path, id: &str) -> Binding {
    let root = base.join(id);
    fs::create_dir_all(root.join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3"))
        .unwrap();
    Binding {
        vault_id: id.into(),
        root,
        state: base.join("private").join(id),
    }
}
async fn sync(binding: &Binding, origin: &Path, request: &str) {
    let transfer = RemoteTransfer::connect_local(binding.clone(), origin, "main")
        .await
        .unwrap();
    let fetched = transfer.fetch().await.unwrap();
    let before = fetched.as_ref().map(|h| h.commit().to_owned());
    let mut session = Session::open(binding.clone()).await.unwrap();
    session.capture(&[], "Local save").await.unwrap();
    if let Some(fetched) = fetched {
        let incoming = session.import_fetched(&fetched).await.unwrap();
        let proposal = session.prepare_sync(&incoming.commit, &[]).await.unwrap();
        assert!(proposal.conflicts.is_empty());
        if let Some(candidate) = proposal.candidate {
            let preview = session.preview_sync(&candidate.commit, &[]).await.unwrap();
            session
                .apply_sync(
                    &VaultRestoreRequest {
                        request_id: request.into(),
                        checkpoint: candidate.commit,
                        expected: preview.expected,
                    },
                    &[],
                )
                .await
                .unwrap();
        }
    }
    let intent = session
        .prepare_push(&transfer, request, before)
        .await
        .unwrap();
    drop(session);
    assert_eq!(
        transfer.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
}
#[tokio::test]
async fn transcript_sync_conflicts_require_explicit_resolution() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let origin = base.join("origin.git");
    assert!(
        Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&origin)
            .status()
            .unwrap()
            .success()
    );
    let a = vault(&base, "a");
    let b = vault(&base, "b");
    fs::write(
        a.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "base\n",
    )
    .unwrap();
    sync(&a, &origin, "seed").await;
    sync(&b, &origin, "restore").await;
    fs::write(
        a.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "device a\n",
    )
    .unwrap();
    fs::write(
        b.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "device b\n",
    )
    .unwrap();
    sync(&a, &origin, "a-edit").await;
    let remote = RemoteTransfer::connect_local(b.clone(), &origin, "main")
        .await
        .unwrap();
    let fetched = remote.fetch().await.unwrap().unwrap();
    let mut session = Session::open(b.clone()).await.unwrap();
    let incoming = session.import_fetched(&fetched).await.unwrap();
    let proposal = session.prepare_sync(&incoming.commit, &[]).await.unwrap();
    assert_eq!(
        proposal.conflicts,
        vec![".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"]
    );
    let candidate = proposal.candidate.unwrap();
    assert!(session.preview_sync(&candidate.commit, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(
            b.root.join(
                ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"
            )
        )
        .unwrap(),
        "device b\n"
    );
    let details = session
        .sync_conflict(
            &candidate.commit,
            ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl",
        )
        .await
        .unwrap();
    assert_eq!(details.terms.len(), 3);
    let resolved = session
        .resolve_sync(
            &candidate.commit,
            ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl",
            jj_store_jj::vault_workspace::SyncResolution::Text {
                text: "merged a and b\n".into(),
                executable: false,
            },
            &[],
        )
        .await
        .unwrap();
    let preview = session.preview_sync(&resolved.commit, &[]).await.unwrap();
    let request = VaultRestoreRequest {
        request_id: "resolve".into(),
        checkpoint: resolved.commit.clone(),
        expected: preview.expected,
    };
    fs::write(
        b.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "later local edit\n",
    )
    .unwrap();
    assert!(session.apply_sync(&request, &[]).await.is_err());
    assert_eq!(
        fs::read_to_string(
            b.root.join(
                ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"
            )
        )
        .unwrap(),
        "later local edit\n"
    );
    fs::write(
        b.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "device b\n",
    )
    .unwrap();
    session.apply_sync(&request, &[]).await.unwrap();
    fs::write(
        b.root
            .join(".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"),
        "after known result\n",
    )
    .unwrap();
    session.apply_sync(&request, &[]).await.unwrap();
    assert_eq!(
        fs::read_to_string(
            b.root.join(
                ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl"
            )
        )
        .unwrap(),
        "after known result\n"
    );
    let intent = session
        .prepare_push(&remote, "merged-push", Some(incoming.commit))
        .await
        .unwrap();
    drop(session);
    remote.reserve(&intent).unwrap();
    assert_eq!(remote.pending_push().unwrap(), Some(intent.clone()));
    assert_eq!(
        remote.publish(&intent).await.unwrap(),
        PushOutcome::Complete
    );
}
