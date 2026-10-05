use super::*;
use jj_review::{
    AgentToolContentPartV2, ArtifactAnchorV2, ArtifactTargetV2, ByteOffset, CreatedContextV1,
    PortableAgentToolTextV2, PortableAgentToolV2, ReviewDigest, SensitivityV1, TextEncoding,
    TextSelectionV1,
};

async fn review_anchor(repo: &LocalRepository) -> ArtifactAnchorV2 {
    let store = repo.agent_store().await.unwrap();
    let (session_snapshot, session) = session(&store).await;
    for (tool_call_id, tool_snapshot) in &session.tools {
        let AgentObjectV1::ToolSnapshot(tool) = store.object(*tool_snapshot).await.unwrap() else {
            panic!("tool");
        };
        assert!(tool.attempts.is_empty());
        let Some(content) = tool.output else { continue };
        let AgentObjectV1::Content(value) = store.object(content).await.unwrap() else {
            panic!("content");
        };
        let bytes = value.payload.bytes();
        assert!(!bytes.is_empty());
        return ArtifactAnchorV2 {
            schema_version: 2,
            repository_id: store.snapshot.repo_id.unwrap(),
            target: ArtifactTargetV2::AgentToolText(Box::new(PortableAgentToolTextV2 {
                tool: PortableAgentToolV2 {
                    session_id: session.session_id,
                    session_snapshot,
                    tool_call_id: *tool_call_id,
                    tool_snapshot: *tool_snapshot,
                },
                part: AgentToolContentPartV2::Output,
                content,
                text: TextSelectionV1 {
                    encoding: TextEncoding::Utf8,
                    start_byte: ByteOffset::new(0),
                    end_byte: ByteOffset::new(bytes.len() as u64),
                    selected_sha256: ReviewDigest::sha256(bytes),
                },
            })),
            created_context: CreatedContextV1 {
                publication_position: None,
                code_state_version: None,
                agent_root: store.snapshot.agent_root,
            },
            sensitivity: SensitivityV1::Repository,
        };
    }
    panic!("missing recorded output")
}

#[tokio::test]
async fn portable_agent_reviews_read_exact_committed_records_after_restart_and_gc() {
    let (root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let before = repo.metadata_store().read().unwrap().unwrap();
    let anchor = review_anchor(&repo).await;
    repo.validate_portable_agent_review(before.commit(), &anchor)
        .await
        .unwrap();
    let path = root.path().join("repo");
    let op_files = std::fs::read_dir(path.join(".jj/repo/op_heads/heads"))
        .unwrap()
        .map(|p| p.unwrap().file_name())
        .collect::<std::collections::BTreeSet<_>>();
    let gc = Command::new("git")
        .arg("--git-dir")
        .arg(path.join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        gc.status.success(),
        "{}",
        String::from_utf8_lossy(&gc.stderr)
    );
    let reopened = reopen(&path).bound_to(anchor.repository_id);
    reopened
        .validate_portable_agent_review(before.commit(), &anchor)
        .await
        .unwrap();
    assert_eq!(
        reopened.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
    assert_eq!(
        std::fs::read_dir(path.join(".jj/repo/op_heads/heads"))
            .unwrap()
            .map(|p| p.unwrap().file_name())
            .collect::<std::collections::BTreeSet<_>>(),
        op_files
    );
}

#[tokio::test]
async fn portable_agent_review_rejects_staged_records_wrong_pins_and_identity() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let mut anchor = review_anchor(&repo).await;
    let store = repo.agent_store().await.unwrap();
    let (_, mut session) = session(&store).await;
    session.state = SessionState::Idle;
    let objects = Arc::new(MemoryStore::default());
    let records = Records::new(objects.clone(), anchor.repository_id);
    let reference = records
        .put(
            RecordKind::AgentObject,
            None,
            &AgentObjectV1::SessionSnapshot(session),
        )
        .await
        .unwrap();
    let bytes = objects
        .get(&reference.key(anchor.repository_id).unwrap(), None)
        .await
        .unwrap()
        .read_bounded(16 * 1024 * 1024)
        .await
        .unwrap();
    repo.metadata_store().stage_blob(&bytes).unwrap();
    let ArtifactTargetV2::AgentToolText(value) = &mut anchor.target else {
        panic!()
    };
    value.tool.session_snapshot.id = AgentObjectId::new(*reference.digest.as_bytes());
    assert_eq!(
        repo.validate_portable_agent_review(pin, &anchor)
            .await
            .unwrap_err(),
        LocalError::Stale
    );
    let original = review_anchor(&repo).await;
    assert_eq!(
        repo.validate_portable_agent_review(
            "0909090909090909090909090909090909090909".parse().unwrap(),
            &original
        )
        .await
        .unwrap_err(),
        LocalError::Stale
    );
    let mut wrong = original.clone();
    wrong.repository_id = m::RepositoryId::new([8; 16]);
    assert_eq!(
        repo.validate_portable_agent_review(pin, &wrong)
            .await
            .unwrap_err(),
        LocalError::Invalid
    );
    assert_eq!(
        repo.clone()
            .bound_to(m::RepositoryId::new([8; 16]))
            .validate_portable_agent_review(pin, &original)
            .await
            .unwrap_err(),
        LocalError::Stale
    );
    assert_eq!(repo.metadata_store().read().unwrap().unwrap().commit(), pin);
}

#[tokio::test]
async fn portable_agent_review_validates_original_payload_and_optional_root_context() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let original = review_anchor(&repo).await;
    for mode in 0..5 {
        let mut changed = original.clone();
        let ArtifactTargetV2::AgentToolText(value) = &mut changed.target else {
            panic!()
        };
        match mode {
            0 => value.text.selected_sha256 = ReviewDigest::sha256(b"displayed text"),
            1 => value.part = AgentToolContentPartV2::Input,
            2 => value.tool.tool_call_id = ToolCallId::new([8; 16]),
            3 => changed.created_context.agent_root = Some(m::RecordDigest::new([8; 32])),
            _ => changed.created_context.publication_position = Some(1),
        }
        assert!(
            repo.validate_portable_agent_review(pin, &changed)
                .await
                .is_err(),
            "mode {mode}"
        );
    }
    repo.validate_portable_agent_review(pin, &original)
        .await
        .unwrap();
    assert_eq!(repo.metadata_store().read().unwrap().unwrap().commit(), pin);
}

#[tokio::test]
async fn portable_agent_review_pin_cannot_read_future_selected_sessions() {
    let (_root, repo) = fixture();
    repo.apply_agent_command(&request(1, batch()))
        .await
        .unwrap();
    let old_pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let mut source = batch();
    source.batch_id = CaptureBatchId::new([72; 16]);
    source.source_instance_id = SourceInstanceId::new([72; 16]);
    repo.apply_agent_command(&request(2, source)).await.unwrap();
    let current_pin = repo.metadata_store().read().unwrap().unwrap().commit();
    let page = repo
        .browser_agent_sessions(current_pin, 20, None)
        .await
        .unwrap();
    let old = repo
        .browser_agent_sessions(old_pin, 20, None)
        .await
        .unwrap();
    let row = page
        .sessions
        .iter()
        .find(|row| row.session_id != old.sessions[0].session_id)
        .unwrap();
    let anchor = ArtifactAnchorV2 {
        schema_version: 2,
        repository_id: page.snapshot.repo_id.unwrap(),
        target: ArtifactTargetV2::AgentSession(jj_review::PortableAgentSessionV2 {
            session_id: row.session_id,
            session_snapshot: AgentObjectRef::new(
                AgentObjectKind::SessionSnapshot,
                row.snapshot_id,
            ),
        }),
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    };
    repo.validate_portable_agent_review(current_pin, &anchor)
        .await
        .unwrap();
    assert_eq!(
        repo.validate_portable_agent_review(old_pin, &anchor)
            .await
            .unwrap_err(),
        LocalError::Stale
    );
}

#[tokio::test]
async fn portable_agent_review_reads_never_initialize_metadata_or_actor() {
    let (root, repo) = fixture();
    let anchor = ArtifactAnchorV2 {
        schema_version: 2,
        repository_id: m::RepositoryId::new([8; 16]),
        target: ArtifactTargetV2::AgentSession(jj_review::PortableAgentSessionV2 {
            session_id: AgentSessionId::new([1; 16]),
            session_snapshot: AgentObjectRef::new(
                AgentObjectKind::SessionSnapshot,
                AgentObjectId::new([2; 32]),
            ),
        }),
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    };
    assert_eq!(
        repo.validate_portable_agent_review(
            "0909090909090909090909090909090909090909".parse().unwrap(),
            &anchor
        )
        .await
        .unwrap_err(),
        LocalError::Stale
    );
    assert!(repo.metadata_store().read().unwrap().is_none());
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
}
