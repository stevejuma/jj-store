//! Exact source context survives without its operation store or derived state.
use jj_lib::{
    config::StackedConfig,
    default_backend_factories::default_backend_factories,
    object_id::ObjectId as _,
    repo::{Repo as _, RepoLoader},
    settings::UserSettings,
};
use jj_store_jj::{local_evidence::*, local_metadata::*};
use jj_store_model as m;
use sha2::Digest as _;
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
fn settings() -> UserSettings {
    UserSettings::from_config(StackedConfig::with_defaults()).unwrap()
}
fn stock(path: &Path, args: &[&str]) -> String {
    let output = Command::new("jj")
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
fn fixture() -> (tempfile::TempDir, MetadataStore, m::OperationId) {
    let root = tempfile::tempdir().unwrap();
    stock(root.path(), &["git", "init", "--colocate"]);
    fs::write(root.path().join("file.txt"), "one\n").unwrap();
    stock(root.path(), &["describe", "-m", "First"]);
    stock(root.path(), &["new", "-m", "Second"]);
    fs::write(root.path().join("file.txt"), "two\n").unwrap();
    stock(root.path(), &["describe", "-m", "Rewritten second"]);
    let operation = stock(
        root.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let store = MetadataStore::open(&root.path().join(".git"), Limits::default()).unwrap();
    (root, store, operation)
}
fn select(store: &MetadataStore, evidence: &CapturedEvidence) -> Snapshot {
    let update = Update {
        repository: REPO,
        actor: m::PrincipalId::new([72; 16]),
        request: m::RequestId::new([73; 16]),
        intent: m::IntentDigest::new([74; 32]),
        domain: Domain::Review,
        expected: None,
        bytes: serde_json::to_vec(&(evidence.capsule, evidence.blob)).unwrap(),
        result: vec![],
        retained: evidence.retained.clone(),
    };
    let Preparation::Ready(candidate) = store.prepare(None, &update).unwrap() else {
        panic!()
    };
    assert!(matches!(
        store.publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    store.read().unwrap().unwrap()
}

#[tokio::test]
async fn retained_capsule_blob_without_its_code_closure_is_rejected() {
    let (root, store, op) = fixture();
    let mut evidence = capture(&settings(), &root.path().join(".jj/repo"), REPO, op, &store)
        .await
        .unwrap();
    evidence.retained = BTreeMap::from([(evidence.blob, RetainedKind::Blob)]);
    let selected = select(&store, &evidence);
    assert!(
        store
            .read_evidence(&selected, REPO, evidence.capsule, evidence.blob)
            .is_err()
    );
}

#[tokio::test]
async fn capsule_retains_full_source_context_after_op_store_loss_and_git_gc() {
    let (root, store, op) = fixture();
    let repo_path = root.path().join(".jj/repo");
    let dirty = root.path().join("dirty.txt");
    fs::write(&dirty, "unsnapshotted").unwrap();
    let evidence = capture(&settings(), &repo_path, REPO, op, &store)
        .await
        .unwrap();
    assert!(store.read().unwrap().is_none());
    let unrelated = store.stage_blob(b"not selected").unwrap();
    let selected = select(&store, &evidence);
    assert!(matches!(
        store.read_retained_blob(&selected, unrelated),
        Err(Error::Stale)
    ));
    let capsule = store
        .read_evidence(&selected, REPO, evidence.capsule, evidence.blob)
        .unwrap();
    assert_eq!(capsule.operation_id, op);
    assert_eq!(capsule.profile, m::portable::CodeProfileV1::GitSha1Jj045);
    assert!(
        capsule
            .operation
            .commit_predecessors
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    );
    assert!(capsule.commits.len() >= 4);
    let loader =
        RepoLoader::init_from_file_system(&settings(), &repo_path, &default_backend_factories())
            .unwrap();
    let pinned = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(op.as_bytes()))
        .await
        .unwrap();
    let repo = loader.load_at(&pinned).await.unwrap();
    let original = pinned.op_store().read_operation(pinned.id()).await.unwrap();
    assert_eq!(
        jj_lib::op_store::Operation::try_from(capsule.operation.clone()).unwrap(),
        original
    );
    assert_eq!(
        jj_lib::op_store::View::try_from(capsule.view.clone()).unwrap(),
        *repo.view().store_view()
    );
    for (id, commit) in &capsule.commits {
        assert_eq!(
            jj_lib::backend::Commit::try_from(commit.clone()).unwrap(),
            repo.store()
                .backend()
                .read_commit(&jj_lib::backend::CommitId::from_bytes(id.as_bytes()))
                .await
                .unwrap()
        );
    }
    drop(repo);
    drop(loader);
    fs::remove_dir_all(repo_path.join("op_store")).unwrap();
    fs::remove_dir_all(repo_path.join("index")).unwrap();
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(root.path().join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(output.status.success());
    let restarted = MetadataStore::open(&root.path().join(".git"), Limits::default()).unwrap();
    assert_eq!(
        restarted
            .read_evidence(
                &restarted.read().unwrap().unwrap(),
                REPO,
                evidence.capsule,
                evidence.blob
            )
            .unwrap(),
        capsule
    );
    assert_eq!(fs::read(&dirty).unwrap(), b"unsnapshotted");
}

#[tokio::test]
async fn capsule_codec_rejects_wrong_profiles_tampering_and_missing_extra_records() {
    let (root, store, op) = fixture();
    let evidence = capture(&settings(), &root.path().join(".jj/repo"), REPO, op, &store)
        .await
        .unwrap();
    let selected = select(&store, &evidence);
    let capsule = store
        .read_evidence(&selected, REPO, evidence.capsule, evidence.blob)
        .unwrap();
    let bytes = capsule.encode().unwrap();
    assert!(GitEvidenceCapsuleV1::decode(m::CodeCapsuleId::new([1; 32]), &bytes).is_err());
    let mut changed = capsule.clone();
    changed.operation.metadata.description.push_str("changed");
    assert!(changed.encode().is_err());
    let mut changed = capsule.clone();
    changed.profile = m::portable::CodeProfileV1::NativeV1;
    assert!(changed.encode().is_err());
    let mut changed = capsule.clone();
    changed.commits.pop_first();
    assert!(changed.encode().is_err());
    let mut changed = capsule.clone();
    changed
        .view
        .local_tags
        .insert("bad".into(), GitRefTargetV1(vec![None, None]));
    assert!(changed.encode().is_err());
    assert!(
        store
            .read_evidence(
                &selected,
                m::RepositoryId::new([99; 16]),
                evidence.capsule,
                evidence.blob
            )
            .is_err()
    );
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["version"] = serde_json::json!(2);
    let unknown = serde_json::to_vec(&value).unwrap();
    let id = m::CodeCapsuleId::new(sha2::Sha256::digest(&unknown).into());
    assert!(GitEvidenceCapsuleV1::decode(id, &unknown).is_err());
}

#[tokio::test]
async fn root_capsule_preserves_upstream_sentinels_without_git_null_edges() {
    let (root, store, _) = fixture();
    let evidence = capture(
        &settings(),
        &root.path().join(".jj/repo"),
        REPO,
        m::OperationId::new([0; 64]),
        &store,
    )
    .await
    .unwrap();
    assert!(
        !evidence
            .retained
            .keys()
            .any(|id| id.to_string() == "0".repeat(40))
    );
    let selected = select(&store, &evidence);
    let capsule = store
        .read_evidence(&selected, REPO, evidence.capsule, evidence.blob)
        .unwrap();
    assert_eq!(
        capsule.commits.keys().copied().collect::<Vec<_>>(),
        vec![m::GitSha1Id::new([0; 20])]
    );
}

#[tokio::test]
async fn source_codec_keeps_conflicted_refs_remote_tags_and_all_operation_fields() {
    use jj_lib::{backend as b, merge::Merge, op_store as o};
    let (root, store, id) = fixture();
    let repo_path = root.path().join(".jj/repo");
    let loader =
        RepoLoader::init_from_file_system(&settings(), &repo_path, &default_backend_factories())
            .unwrap();
    let op = loader
        .load_operation(&o::OperationId::from_bytes(id.as_bytes()))
        .await
        .unwrap();
    let repo = loader.load_at(&op).await.unwrap();
    let commits = repo
        .view()
        .store_view()
        .head_ids
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let first = commits[0].clone();
    let zero = b::CommitId::from_bytes(&[0; 20]);
    let conflict = o::RefTarget::from_merge(Merge::from_vec(vec![
        Some(first.clone()),
        Some(zero.clone()),
        None,
    ]));
    let remote = o::RemoteRef {
        target: conflict.clone(),
        state: o::RemoteRefState::Tracked,
    };
    let mut view = repo.view().store_view().clone();
    view.local_bookmarks
        .insert("conflicted".into(), conflict.clone());
    view.local_tags.insert("tag".into(), conflict.clone());
    view.git_refs
        .insert("refs/custom/a".into(), conflict.clone());
    view.git_heads
        .insert("git-worktree".into(), conflict.clone());
    view.wc_commit_ids
        .insert("other-workspace".into(), first.clone());
    view.remote_views.insert(
        "origin".into(),
        o::RemoteView {
            bookmarks: BTreeMap::from([("remote-bookmark".into(), remote.clone())]),
            tags: BTreeMap::from([("remote-tag".into(), remote)]),
        },
    );
    let view_id = loader.op_store().write_view(&view).await.unwrap();
    let mut operation = loader.op_store().read_operation(op.id()).await.unwrap();
    operation.view_id = view_id.clone();
    operation.metadata.time.start = b::Timestamp {
        timestamp: b::MillisSinceEpoch(-1234),
        tz_offset: -45,
    };
    operation.metadata.time.end = b::Timestamp {
        timestamp: b::MillisSinceEpoch(4567),
        tz_offset: 90,
    };
    operation.metadata.hostname = "source-host".into();
    operation.metadata.username = "source-user".into();
    operation.metadata.is_snapshot = true;
    operation.metadata.workspace_name = Some("source-workspace".into());
    operation.metadata.attributes = BTreeMap::from([("attribute".into(), "exact value".into())]);
    operation.commit_predecessors = Some(BTreeMap::from([(first, vec![zero])]));
    let op_id = loader.op_store().write_operation(&operation).await.unwrap();
    let before = fs::read_dir(repo_path.join("op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<std::collections::BTreeSet<_>>();
    let evidence = capture(
        &settings(),
        &repo_path,
        REPO,
        m::OperationId::from_slice(op_id.as_bytes()).unwrap(),
        &store,
    )
    .await
    .unwrap();
    let selected = select(&store, &evidence);
    let capsule = store
        .read_evidence(&selected, REPO, evidence.capsule, evidence.blob)
        .unwrap();
    assert_eq!(o::View::try_from(capsule.view).unwrap(), view);
    assert_eq!(
        o::Operation::try_from(capsule.operation).unwrap(),
        operation
    );
    let after = fs::read_dir(repo_path.join("op_heads/heads"))
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(before, after);
}

#[tokio::test]
async fn missing_evidence_dependencies_and_blob_limits_never_select_partial_state() {
    let (root, store, id) = fixture();
    let evidence = capture(&settings(), &root.path().join(".jj/repo"), REPO, id, &store)
        .await
        .unwrap();
    let tiny = MetadataStore::open(
        &root.path().join(".git"),
        Limits {
            object_bytes: 4,
            ..Limits::default()
        },
    )
    .unwrap();
    assert!(matches!(tiny.stage_blob(b"too large"), Err(Error::Limit)));
    let missing = evidence
        .retained
        .iter()
        .find(|(_, kind)| **kind == RetainedKind::Tree)
        .unwrap()
        .0
        .to_string();
    fs::remove_file(
        root.path()
            .join(".git/objects")
            .join(&missing[..2])
            .join(&missing[2..]),
    )
    .unwrap();
    let update = Update {
        repository: REPO,
        actor: m::PrincipalId::new([72; 16]),
        request: m::RequestId::new([73; 16]),
        intent: m::IntentDigest::new([74; 32]),
        domain: Domain::Review,
        expected: None,
        bytes: b"{}".to_vec(),
        result: vec![],
        retained: evidence.retained,
    };
    assert!(store.prepare(None, &update).is_err());
    assert!(store.read().unwrap().is_none());
}
