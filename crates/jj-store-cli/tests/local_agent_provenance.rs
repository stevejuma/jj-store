//! Direct provider acceptance at the CLI-owned upstream JJ boundary.
#[path = "local_agent_provenance/fixture.rs"]
mod fixture;
use fixture::*;
use jj_agent::*;
use jj_store_model::{self as m, portable::*};
use jj_store_service::local_repository::*;
use jj_store_wal::publication::AgentSnapshotRead;

#[tokio::test]
async fn provenance_unknown_outcome_restarts_with_exact_original_request() {
    let (root, repo, input, _, session) = fixture().await;
    let value = association(&input, &session, 2);
    let command = request(
        2,
        None,
        AgentObjectV1::PortableAssociation(value),
        vec![input],
    );
    let journal = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(journal.path(), serde_json::to_vec(&command).unwrap()).unwrap();
    let lock = root.path().join(".git/refs/lapis/state/v1.lock");
    std::fs::write(&lock, b"fixture ref lock").unwrap();
    assert!(matches!(
        repo.apply_agent_command(&command).await.unwrap(),
        LocalAgentOutcome::OutcomeUnknown { .. }
    ));
    assert!(
        repo.agent_store()
            .await
            .unwrap()
            .reads()
            .list_catalog(AgentCatalogKind::Associations, None, 20)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    std::fs::remove_file(lock).unwrap();
    let retry: LocalAgentRequest =
        serde_json::from_slice(&std::fs::read(journal.path()).unwrap()).unwrap();
    assert_eq!(retry, command);
    let restart = reopen(root.path());
    let receipt = restart.apply_agent_command(&retry).await.unwrap();
    assert!(matches!(receipt, LocalAgentOutcome::Committed { .. }));
    assert_eq!(restart.apply_agent_command(&retry).await.unwrap(), receipt);
    assert_eq!(
        restart
            .agent_store()
            .await
            .unwrap()
            .reads()
            .list_catalog(AgentCatalogKind::Associations, None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
}

#[tokio::test]
async fn original_local_agent_descriptor_remains_readable_before_v2_publication() {
    use jj_store_jj::local_metadata::{Domain, Preparation, PublishOutcome, Update};
    use jj_store_wal::records::RecordRef;
    #[derive(serde::Serialize, serde::Deserialize)]
    struct V1 {
        version: u32,
        repository: m::RepositoryId,
        root: RecordRef,
        records: std::collections::BTreeMap<String, jj_store_jj::local_metadata::GitOid>,
    }
    let (_root, repo, input, _, session) = fixture().await;
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let bytes = repo
        .metadata_store()
        .read_domain(&selected, Domain::Agent)
        .unwrap()
        .unwrap();
    let mut legacy: V1 = serde_json::from_slice(&bytes).unwrap();
    legacy.version = 1;
    let update = Update {
        repository: selected.repository(),
        actor: repo.actor().unwrap().unwrap(),
        request: m::RequestId::new([90; 16]),
        intent: m::IntentDigest::new([90; 32]),
        domain: Domain::Agent,
        expected: selected.selection(Domain::Agent).cloned(),
        bytes: serde_json::to_vec(&legacy).unwrap(),
        result: vec![],
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = repo
        .metadata_store()
        .prepare(Some(&selected), &update)
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(
        repo.metadata_store().publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert_eq!(
        repo.agent_store()
            .await
            .unwrap()
            .reads()
            .list_sessions(None, 20)
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    repo.apply_agent_command(&request(
        2,
        None,
        AgentObjectV1::PortableAssociation(association(&input, &session, 2)),
        vec![input],
    ))
    .await
    .unwrap();
    let selected = repo.metadata_store().read().unwrap().unwrap();
    let bytes = repo
        .metadata_store()
        .read_domain(&selected, Domain::Agent)
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["version"],
        2
    );
    repo.agent_store().await.unwrap();
}

#[tokio::test]
async fn portable_association_retains_exact_code_after_restart_op_loss_and_git_gc() {
    let (root, repo, input, _, session) = fixture().await;
    let old = repo.agent_store().await.unwrap();
    let operation = stock(
        root.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    );
    let value = association(&input, &session, 2);
    let command = request(
        2,
        None,
        AgentObjectV1::PortableAssociation(value.clone()),
        vec![input.clone()],
    );
    let receipt = repo.apply_agent_command(&command).await.unwrap();
    let reference = selected(&repo, 2).await;
    let pinned = repo.agent_store().await.unwrap();
    assert_eq!(
        pinned.object(reference).await.unwrap(),
        AgentObjectV1::PortableAssociation(value)
    );
    assert_eq!(old.object(reference).await.unwrap_err(), LocalError::Stale);
    assert_eq!(
        old.code_evidence(&input.reference).unwrap_err(),
        LocalError::Stale
    );
    let capsule = pinned.code_evidence(&input.reference).unwrap();
    assert_ne!(capsule.repository, pinned.snapshot.repo_id.unwrap());
    assert_eq!(
        stock(
            root.path(),
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"]
        ),
        operation
    );
    let relation = AgentRelationSubjectV1::PortableOperation {
        evidence: input.reference.clone(),
    };
    assert_eq!(
        pinned
            .reads()
            .relations(&relation, None, 20)
            .await
            .unwrap()
            .selections,
        vec![reference]
    );
    let unrelated = CodeEvidenceRefV1 {
        capsule: m::CodeCapsuleId::new([9; 32]),
        ..input.reference.clone()
    };
    assert_eq!(
        pinned.code_evidence(&unrelated).unwrap_err(),
        LocalError::Stale
    );
    std::fs::rename(
        root.path().join(".jj/repo/op_store"),
        root.path().join("archived-op-store"),
    )
    .unwrap();
    git_gc(root.path());
    let restart = reopen(root.path());
    assert_eq!(
        restart.apply_agent_command(&command).await.unwrap(),
        receipt
    );
    assert_eq!(
        restart
            .agent_store()
            .await
            .unwrap()
            .code_evidence(&input.reference)
            .unwrap(),
        capsule
    );
    assert_eq!(pinned.code_evidence(&input.reference).unwrap(), capsule);
}

#[tokio::test]
async fn provenance_corrections_are_stale_safe_and_replay_original_receipts() {
    let (_root, repo, input, _, session) = fixture().await;
    let value = association(&input, &session, 2);
    let command = request(
        2,
        None,
        AgentObjectV1::PortableAssociation(value.clone()),
        vec![input.clone()],
    );
    let receipt = repo.apply_agent_command(&command).await.unwrap();
    let reference = selected(&repo, 2).await;
    let old = repo.agent_store().await.unwrap();
    let mut corrected = value.clone();
    corrected.facts.supersedes = Some(reference);
    corrected.facts.correction_reason = Some("Corrected file observation".into());
    corrected.facts.paths = vec!["other.txt".into()];
    let correction = request(
        3,
        Some(reference),
        AgentObjectV1::PortableAssociation(corrected),
        vec![],
    );
    repo.apply_agent_command(&correction).await.unwrap();
    let next = selected(&repo, 2).await;
    assert_ne!(next, reference);
    assert_eq!(repo.apply_agent_command(&command).await.unwrap(), receipt);
    let mut stale = correction.clone();
    stale.request_id = m::RequestId::new([4; 16]);
    assert_eq!(
        repo.apply_agent_command(&stale).await.unwrap_err(),
        LocalError::Stale
    );
    let mut collision = command.clone();
    collision.command = correction.command;
    assert_eq!(
        repo.apply_agent_command(&collision).await.unwrap_err(),
        LocalError::IntentConflict
    );
    assert_eq!(
        old.object(reference).await.unwrap(),
        AgentObjectV1::PortableAssociation(value.clone())
    );
    assert_eq!(
        repo.agent_store()
            .await
            .unwrap()
            .object(reference)
            .await
            .unwrap(),
        AgentObjectV1::PortableAssociation(value)
    );
    let relation = AgentRelationSubjectV1::Session {
        session_id: session.session_id,
    };
    let store = repo.agent_store().await.unwrap();
    let first = store.reads().relations(&relation, None, 1).await.unwrap();
    assert_eq!(first.selections, vec![next]);
    let second = store
        .reads()
        .relations(&relation, first.next, 1)
        .await
        .unwrap();
    assert_eq!(second.selections, vec![reference]);
}

#[tokio::test]
async fn invalid_capsules_commits_sessions_and_dependencies_never_publish() {
    let (_root, repo, input, _, session) = fixture().await;
    let original = repo.metadata_store().read().unwrap().unwrap().commit();
    let value = association(&input, &session, 2);
    let mut wrong = value.clone();
    let mut commit = wrong.after.commits.pop_first().unwrap();
    commit.commit = CodeCommitIdV1::GitSha1Jj045(m::GitSha1Id::new([9; 20]));
    wrong.after.commits.insert(commit);
    assert!(
        repo.apply_agent_command(&request(
            2,
            None,
            AgentObjectV1::PortableAssociation(wrong),
            vec![input.clone()]
        ))
        .await
        .is_err()
    );
    let mut wrong = value.clone();
    wrong.facts.session_id = AgentSessionId::new([9; 16]);
    assert_eq!(
        repo.apply_agent_command(&request(
            3,
            None,
            AgentObjectV1::PortableAssociation(wrong),
            vec![input.clone()]
        ))
        .await
        .unwrap_err(),
        LocalError::Stale
    );
    let mut wrong = value.clone();
    wrong.facts.evidence[0].id = AgentObjectId::new([9; 32]);
    assert_eq!(
        repo.apply_agent_command(&request(
            4,
            None,
            AgentObjectV1::PortableAssociation(wrong),
            vec![input.clone()]
        ))
        .await
        .unwrap_err(),
        LocalError::Stale
    );
    assert!(
        repo.apply_agent_command(&request(
            5,
            None,
            AgentObjectV1::PortableAssociation(value.clone()),
            vec![]
        ))
        .await
        .is_err()
    );
    let mut wrong = input.clone();
    wrong.reference.operation = m::OperationId::new([9; 64]);
    assert!(wrong.validate().is_err());
    let mut wrong = input.clone();
    wrong.bytes.push(b'\n');
    assert!(wrong.validate().is_err());
    let mut wrong = input.clone();
    wrong.reference.repository = m::RepositoryId::new([9; 16]);
    assert!(wrong.validate().is_err());
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        original
    );
    assert!(
        repo.agent_store()
            .await
            .unwrap()
            .reads()
            .list_catalog(AgentCatalogKind::Associations, None, 20)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
}

#[tokio::test]
async fn portable_checkpoint_pins_agent_versions_and_independent_outcomes() {
    let (_root, repo, input, session_ref, session) = fixture().await;
    let association = association(&input, &session, 2);
    repo.apply_agent_command(&request(
        2,
        None,
        AgentObjectV1::PortableAssociation(association.clone()),
        vec![input.clone()],
    ))
    .await
    .unwrap();
    let reference = selected(&repo, 2).await;
    let value = AgentCheckpointVersionV2 {
        schema_version: 2,
        before: association.after.clone(),
        after: association.after.clone(),
        code_relation: CodeRelation::NoChange,
        facts: AgentCheckpointFactsV1 {
            checkpoint_id: AgentCheckpointId::new([3; 16]),
            session_id: session.session_id,
            turn_id: None,
            scope_id: None,
            session_snapshot: session_ref,
            transcript_manifest: session.transcripts[0],
            event_batches: session.event_batches.clone(),
            tools: session.tools.clone(),
            associations: vec![reference],
            evidence_method: EvidenceMethod::ManualAssertion,
            source_trust: SourceTrust::LocalUserHook,
            exclusive_write_scope: false,
            coverage: session.coverage.clone(),
            capture_status: CaptureStatusV1::Partial,
            execution_outcome: ExecutionOutcomeV1::Failed,
            supersedes: None,
        },
    };
    let command = request(
        3,
        None,
        AgentObjectV1::PortableCheckpointVersion(value.clone()),
        vec![],
    );
    repo.apply_agent_command(&command).await.unwrap();
    let store = repo.agent_store().await.unwrap();
    let selected = store
        .lookup(
            AgentCatalogKind::Checkpoints,
            AgentCatalogKey::derive("checkpoint", value.facts.checkpoint_id.as_bytes()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        store.object(selected).await.unwrap(),
        AgentObjectV1::PortableCheckpointVersion(value.clone())
    );
    let mut mismatched = value.clone();
    let tool = *mismatched.facts.tools.keys().next().unwrap();
    let wrong_version = *mismatched.facts.tools.values().nth(1).unwrap();
    mismatched.facts.tools.insert(tool, wrong_version);
    mismatched.facts.supersedes = Some(selected);
    assert_eq!(
        repo.apply_agent_command(&request(
            5,
            Some(selected),
            AgentObjectV1::PortableCheckpointVersion(mismatched),
            vec![]
        ))
        .await
        .unwrap_err(),
        LocalError::Corrupt
    );
    let mut bad = value;
    bad.facts.tools.clear();
    bad.facts.session_id = AgentSessionId::new([9; 16]);
    bad.facts.supersedes = Some(selected);
    assert!(
        repo.apply_agent_command(&request(
            4,
            Some(selected),
            AgentObjectV1::PortableCheckpointVersion(bad),
            vec![]
        ))
        .await
        .is_err()
    );
}

#[tokio::test]
async fn concurrent_disjoint_provenance_preserves_shared_relation_history() {
    let (_root, repo, input, _, session) = fixture().await;
    let a = request(
        2,
        None,
        AgentObjectV1::PortableAssociation(association(&input, &session, 2)),
        vec![input.clone()],
    );
    let b = request(
        3,
        None,
        AgentObjectV1::PortableAssociation(association(&input, &session, 3)),
        vec![input],
    );
    let other = repo.clone();
    let first = tokio::spawn(async move { other.apply_agent_command(&a).await });
    repo.apply_agent_command(&b).await.unwrap();
    first.await.unwrap().unwrap();
    let store = repo.agent_store().await.unwrap();
    let relation = AgentRelationSubjectV1::Session {
        session_id: session.session_id,
    };
    let result = store.reads().relations(&relation, None, 20).await.unwrap();
    assert_eq!(result.selections.len(), 2);
    assert!(result.selections.contains(&selected(&repo, 2).await));
    assert!(result.selections.contains(&selected(&repo, 3).await));
}
