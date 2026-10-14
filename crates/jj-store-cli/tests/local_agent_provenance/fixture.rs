use jj_agent::*;
use jj_lib::{config::StackedConfig, settings::UserSettings};
use jj_store_jj::{local_evidence, local_metadata::*};
use jj_store_model::{self as m, portable::*};
use jj_store_service::local_repository::*;
use std::{collections::BTreeSet, path::Path, process::Command};

pub fn settings() -> UserSettings {
    UserSettings::from_config(StackedConfig::with_defaults()).unwrap()
}
pub fn stock(path: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_jj-store-jj"))
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
pub fn reopen(path: &Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
}
pub async fn fixture() -> (
    tempfile::TempDir,
    LocalRepository,
    LocalCodeEvidenceV1,
    AgentObjectRef,
    AgentSessionSnapshotV1,
) {
    let root = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    std::fs::write(root.path().join("code.txt"), "exact code\n").unwrap();
    stock(root.path(), &["describe", "-m", "Code before capture"]);
    let operation = stock(
        root.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let repo = reopen(root.path());
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([1; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(SanitizedObservationBatchV1 {
                schema_version: 1,
                batch_id: CaptureBatchId::new([1; 16]),
                integration: IntegrationId::Codex,
                source_instance_id: SourceInstanceId::new([1; 16]),
                sanitizer_version: "fixture-sanitized@1".into(),
                representation: SanitizedRepresentation::SanitizedNative,
                observations: values
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| SanitizedObservationV1 {
                        observation_id: ObservationId::new([i as u8 + 1; 16]),
                        source_sequence: i as u64 + 1,
                        occurred_at_millis: None,
                        source_kind: SourceKind::Hook,
                        source_trust: SourceTrust::LocalUserHook,
                        payload: SanitizedPayloadV1::new(
                            SanitizedMediaType::Json,
                            &serde_json::to_vec(&v).unwrap(),
                        )
                        .unwrap(),
                    })
                    .collect(),
            }),
        },
    })
    .await
    .unwrap();
    let captured = local_evidence::capture(
        &settings(),
        &root.path().join(".jj/repo"),
        m::RepositoryId::new([88; 16]),
        operation,
        repo.metadata_store(),
    )
    .await
    .unwrap();
    let bytes = repo
        .metadata_store()
        .read_staged_blob(captured.blob)
        .unwrap();
    let reference = CodeEvidenceRefV1 {
        repository: m::RepositoryId::new([88; 16]),
        profile: CodeProfileV1::GitSha1Jj045,
        operation,
        capsule: captured.capsule,
    };
    let store = repo.agent_store().await.unwrap();
    let selected = store
        .reads()
        .list_sessions(None, 20)
        .await
        .unwrap()
        .selections[0]
        .1;
    let AgentObjectV1::SessionSnapshot(session) = store.object(selected).await.unwrap() else {
        panic!()
    };
    (
        root,
        repo,
        LocalCodeEvidenceV1 { reference, bytes },
        selected,
        session,
    )
}
pub fn association(
    input: &LocalCodeEvidenceV1,
    session: &AgentSessionSnapshotV1,
    id: u8,
) -> CodeAssociationV2 {
    let capsule = input.validate().unwrap();
    let commit = *capsule
        .commits
        .keys()
        .find(|id| **id != m::GitSha1Id::new([0; 20]))
        .unwrap();
    CodeAssociationV2 {
        schema_version: 2,
        before: CodeAnchorV2::default(),
        after: CodeAnchorV2 {
            operations: BTreeSet::from([input.reference.clone()]),
            commits: BTreeSet::from([CodeCommitEvidenceV2 {
                evidence: input.reference.clone(),
                commit: CodeCommitIdV1::GitSha1Jj045(commit),
            }]),
        },
        facts: CodeAssociationFactsV1 {
            association_id: AssociationId::new([id; 16]),
            checkpoint_id: None,
            session_id: session.session_id,
            turn_id: None,
            tool_call_ids: session.tools.keys().copied().collect(),
            scope_id: None,
            relation: AssociationRelation::ManualAssertion,
            changes: BTreeSet::new(),
            paths: vec!["code.txt".into()],
            granularity: AssociationGranularity::File,
            evidence_method: EvidenceMethod::ManualAssertion,
            source_trust: SourceTrust::LocalUserHook,
            exclusive_write_scope: false,
            evidence: session.evidence.clone(),
            coverage: session.coverage.clone(),
            created_at_millis: 1,
            supersedes: None,
            correction_reason: None,
        },
    }
}
pub fn request(
    id: u8,
    expected: Option<AgentObjectRef>,
    object: AgentObjectV1,
    evidence: Vec<LocalCodeEvidenceV1>,
) -> LocalAgentRequest {
    LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command: LocalAgentCommand::Provenance {
            expected,
            object: Box::new(object),
            evidence,
        },
    }
}
pub async fn selected(repo: &LocalRepository, id: u8) -> AgentObjectRef {
    use jj_store_wal::publication::AgentSnapshotRead;
    repo.agent_store()
        .await
        .unwrap()
        .lookup(
            AgentCatalogKind::Associations,
            AgentCatalogKey::derive("association", AssociationId::new([id; 16]).as_bytes())
                .unwrap(),
        )
        .await
        .unwrap()
        .unwrap()
}
pub fn git_gc(path: &Path) {
    assert!(
        Command::new("git")
            .current_dir(path)
            .args(["gc", "--prune=now"])
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .status()
            .unwrap()
            .success()
    );
}
