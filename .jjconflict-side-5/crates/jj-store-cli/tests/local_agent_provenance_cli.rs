//! Cold executable provenance publication and private journal recovery.
#[allow(dead_code)]
#[path = "local_agent_provenance/fixture.rs"]
mod fixture;
use fixture::*;
use jj_agent::*;
use jj_store_jj::local_evidence::GitEvidenceCapsuleV1;
use jj_store_model as m;
use jj_store_service::local_repository::LocalCodeEvidenceV1;
use serde_json::Value;
use std::{
    path::Path,
    process::{Command, Output},
};
fn cli(path: &Path, state: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jj"))
        .current_dir(path)
        .args([
            "--no-pager",
            "agent",
            "--format",
            "json",
            "--state-dir",
            state.to_str().unwrap(),
        ])
        .args(args)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap()
}
fn json(path: &Path, state: &Path, args: &[&str]) -> Value {
    let output = cli(path, state, args);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
fn document(path: &Path, value: &impl serde::Serialize) {
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn id(n: u8) -> String {
    m::RequestId::new([n; 16]).to_string()
}
async fn context() -> (tempfile::TempDir, tempfile::TempDir, CodeAssociationV2) {
    let (root, repo, old, _, session) = fixture().await;
    let device = tempfile::tempdir().unwrap();
    let before = repo.metadata_store().read().unwrap().unwrap().commit();
    let inspected = json(
        root.path(),
        device.path(),
        &[
            "code-evidence",
            "--operation",
            &old.reference.operation.to_string(),
        ],
    );
    assert_eq!(inspected["selected"], false);
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before
    );
    let capsule: GitEvidenceCapsuleV1 =
        serde_json::from_value(inspected["capsule"].clone()).unwrap();
    let input = LocalCodeEvidenceV1 {
        reference: serde_json::from_value(inspected["reference"].clone()).unwrap(),
        bytes: capsule.encode().unwrap(),
    };
    (root, device, association(&input, &session, 2))
}
#[tokio::test]
async fn executable_provenance_pages_bind_snapshots_and_corrections() {
    let (root, device, mut value) = context().await;
    let record = device.path().join("association.json");
    document(&record, &value);
    let args = [
        "associate",
        "--request-id",
        &id(2),
        "--record",
        record.to_str().unwrap(),
    ];
    let first = json(root.path(), device.path(), &args);
    assert_eq!(first["status"], "committed");
    assert_eq!(json(root.path(), device.path(), &args), first);
    let old = json(root.path(), device.path(), &["associations"])["objects"][0]["object"]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    value.facts.association_id = AssociationId::new([3; 16]);
    document(&record, &value);
    json(
        root.path(),
        device.path(),
        &[
            "associate",
            "--request-id",
            &id(3),
            "--record",
            record.to_str().unwrap(),
        ],
    );
    let page = json(
        root.path(),
        device.path(),
        &["associations", "--limit", "1"],
    );
    let cursor = page["next_cursor"].as_str().unwrap();
    value.facts.association_id = AssociationId::new([4; 16]);
    document(&record, &value);
    json(
        root.path(),
        device.path(),
        &[
            "associate",
            "--request-id",
            &id(4),
            "--record",
            record.to_str().unwrap(),
        ],
    );
    let next = json(
        root.path(),
        device.path(),
        &["associations", "--limit", "1", "--cursor", cursor],
    );
    assert_eq!(page["snapshot"], next["snapshot"]);
    assert!(next["next_cursor"].is_null());
    assert_ne!(page["objects"][0]["object"], next["objects"][0]["object"]);
    assert!(
        !cli(
            root.path(),
            device.path(),
            &["checkpoints", "--limit", "1", "--cursor", cursor]
        )
        .status
        .success()
    );
    assert!(
        !cli(
            root.path(),
            device.path(),
            &["associations", "--limit", "2", "--cursor", cursor]
        )
        .status
        .success()
    );
    value.facts.association_id = AssociationId::new([2; 16]);
    value.facts.supersedes = Some(AgentObjectRef::new(
        AgentObjectKind::Association,
        old.parse().unwrap(),
    ));
    value.facts.correction_reason = Some("Explicit correction".into());
    document(&record, &value);
    json(
        root.path(),
        device.path(),
        &[
            "associate",
            "--request-id",
            &id(5),
            "--record",
            record.to_str().unwrap(),
            "--expected-version",
            &old,
        ],
    );
    assert!(
        !cli(
            root.path(),
            device.path(),
            &[
                "associate",
                "--request-id",
                &id(6),
                "--record",
                record.to_str().unwrap(),
                "--expected-version",
                &old
            ]
        )
        .status
        .success()
    );
    assert_eq!(
        json(root.path(), device.path(), &["receipt", &id(2)]),
        first
    );
}
#[tokio::test]
async fn executable_unknown_provenance_retries_without_source_operations_or_document() {
    let (root, device, value) = context().await;
    let record = device.path().join("association.json");
    document(&record, &value);
    let lock = root.path().join(".git/refs/lapis/state/v1.lock");
    std::fs::write(&lock, b"fixture ref lock").unwrap();
    let result = json(
        root.path(),
        device.path(),
        &[
            "associate",
            "--request-id",
            &id(2),
            "--record",
            record.to_str().unwrap(),
        ],
    );
    assert_eq!(result["status"], "outcome_unknown");
    let pending = device
        .path()
        .join("provenance")
        .join(format!("{}.json", id(2)));
    assert!(pending.exists());
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Journal {
        version: u32,
        repository: m::RepositoryId,
        actor: m::PrincipalId,
        intent: m::IntentDigest,
        request: jj_store_service::local_repository::LocalAgentRequest,
    }
    let original = std::fs::read(&pending).unwrap();
    let mut altered: Journal = serde_json::from_slice(&original).unwrap();
    let jj_store_service::local_repository::LocalAgentCommand::Provenance { object, .. } =
        &mut altered.request.command
    else {
        panic!()
    };
    let AgentObjectV1::PortableAssociation(association) = object.as_mut() else {
        panic!()
    };
    association.facts.paths = vec!["valid-but-corrupt.txt".into()];
    std::fs::write(&pending, serde_json::to_vec(&altered).unwrap()).unwrap();
    let rejected = cli(root.path(), device.path(), &["retry-provenance", &id(2)]);
    assert!(!rejected.status.success());
    assert!(
        String::from_utf8_lossy(&rejected.stderr).contains("provenance journal intent mismatch")
    );
    std::fs::write(&pending, &original).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(
            std::fs::metadata(&pending).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    let mut changed = value;
    changed.facts.paths = vec!["different.txt".into()];
    document(&record, &changed);
    assert!(
        !cli(
            root.path(),
            device.path(),
            &[
                "associate",
                "--request-id",
                &id(2),
                "--record",
                record.to_str().unwrap()
            ]
        )
        .status
        .success()
    );
    std::fs::remove_file(&lock).unwrap();
    std::fs::remove_file(&record).unwrap();
    std::fs::rename(
        root.path().join(".jj/repo/op_store"),
        root.path().join("archived-op-store"),
    )
    .unwrap();
    std::fs::write(
        device.path().join("provenance/.tmp-interrupted"),
        b"incomplete staging",
    )
    .unwrap();
    let committed = json(root.path(), device.path(), &["retry-provenance", &id(2)]);
    assert_eq!(committed["status"], "committed");
    assert!(!pending.exists());
    assert!(!device.path().join("provenance/.tmp-interrupted").exists());
    assert_eq!(
        json(root.path(), device.path(), &["receipt", &id(2)]),
        committed
    );
    git_gc(root.path());
    assert_eq!(
        json(root.path(), device.path(), &["associations"])["objects"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}
#[test]
fn executable_empty_provenance_pages_leave_metadata_and_actor_absent() {
    let root = tempfile::tempdir().unwrap();
    let device = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    for command in ["associations", "checkpoints"] {
        let page = json(root.path(), device.path(), &[command]);
        assert!(page["objects"].as_array().unwrap().is_empty());
        assert!(page["snapshot"]["repo_id"].is_null());
    }
    assert!(!root.path().join(".jj/repo/lapis").exists());
    assert!(!device.path().join("provenance").exists());
}

#[tokio::test]
async fn executable_checkpoint_retains_exact_session_and_association_versions() {
    let (root, device, association) = context().await;
    let record = device.path().join("association.json");
    document(&record, &association);
    json(
        root.path(),
        device.path(),
        &[
            "associate",
            "--request-id",
            &id(2),
            "--record",
            record.to_str().unwrap(),
        ],
    );
    let row = json(root.path(), device.path(), &["sessions"])["sessions"][0].clone();
    let session: AgentSessionSnapshotV1 = serde_json::from_value(row["session"].clone()).unwrap();
    let session_ref = serde_json::from_value(row["object"].clone()).unwrap();
    let association_ref = serde_json::from_value(
        json(root.path(), device.path(), &["associations"])["objects"][0]["object"].clone(),
    )
    .unwrap();
    let checkpoint = AgentCheckpointVersionV2 {
        schema_version: 2,
        before: association.after.clone(),
        after: association.after,
        code_relation: CodeRelation::NoChange,
        facts: AgentCheckpointFactsV1 {
            checkpoint_id: AgentCheckpointId::new([3; 16]),
            session_id: session.session_id,
            turn_id: None,
            scope_id: None,
            session_snapshot: session_ref,
            transcript_manifest: session.transcripts[0],
            event_batches: session.event_batches,
            tools: session.tools,
            associations: vec![association_ref],
            evidence_method: EvidenceMethod::ManualAssertion,
            source_trust: SourceTrust::LocalUserHook,
            exclusive_write_scope: false,
            coverage: session.coverage,
            capture_status: CaptureStatusV1::Partial,
            execution_outcome: ExecutionOutcomeV1::Failed,
            supersedes: None,
        },
    };
    document(&record, &checkpoint);
    let result = json(
        root.path(),
        device.path(),
        &[
            "checkpoint",
            "--request-id",
            &id(3),
            "--record",
            record.to_str().unwrap(),
        ],
    );
    assert_eq!(result["status"], "committed");
    let page = json(root.path(), device.path(), &["checkpoints"]);
    assert_eq!(page["objects"].as_array().unwrap().len(), 1);
    assert_eq!(
        page["objects"][0]["value"]["value"],
        serde_json::to_value(checkpoint).unwrap()
    );
}
