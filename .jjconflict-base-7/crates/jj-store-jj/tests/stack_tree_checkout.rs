#[path = "stack_foreground/fixture.rs"]
mod fixture;
use fixture::Fixture;
use jj_lib::{
    backend as b, config::StackedConfig, default_backend_factories::default_backend_factories,
    object_id::ObjectId as _, ref_name::WorkspaceNameBuf, repo::Repo as _, repo_path::RepoPathBuf,
    settings::UserSettings,
};
use jj_store_jj::{
    local_browser::{self, working_tree::content_digest},
    local_evidence::{OperationArchiveV1, capture_operation_archive},
    local_execution::{Coordinator, PhysicalGuard, RecoveredHistory, RecoveryBinding},
    local_metadata::{Limits, MetadataStore, StackPreparationKey},
};
use jj_store_model as m;
use std::{fs, path::PathBuf, sync::Arc};
#[path = "stack_tree_checkout/adversarial.rs"]
mod adversarial;
#[path = "stack_tree_checkout/lifecycle.rs"]
mod lifecycle;
#[path = "stack_tree_checkout/process.rs"]
mod process;
type FileChanges<'a> = Vec<(&'a str, Option<(&'a [u8], bool)>)>;

struct Files {
    root: PathBuf,
    original: Vec<u8>,
}
#[async_trait::async_trait]
impl PhysicalGuard for Files {
    async fn validate(
        &self,
        _: &RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        if expected != content_digest(&self.original)
            || fs::read(self.root.join("file.txt")).unwrap() != self.original
        {
            return Err(local_browser::Error::EpochChanged);
        }
        Ok(())
    }
}
fn guard(f: &Fixture) -> (Arc<dyn PhysicalGuard>, m::RecordDigest) {
    let original = fs::read(f.root().join("file.txt")).unwrap();
    let digest = content_digest(&original);
    (
        Arc::new(Files {
            root: f.root(),
            original,
        }),
        digest,
    )
}
async fn candidate(f: &Fixture, changes: FileChanges<'_>) -> OperationArchiveV1 {
    candidate_with_symlink(f, changes, None).await
}
async fn candidate_with_symlink(
    f: &Fixture,
    changes: FileChanges<'_>,
    link: Option<(&str, &str)>,
) -> OperationArchiveV1 {
    let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
    let loader = jj_lib::repo::RepoLoader::init_from_file_system(
        &settings,
        &f.root().join(".jj/repo"),
        &default_backend_factories(),
    )
    .unwrap();
    let state = f.witness().await;
    let op = loader
        .load_operation(&jj_lib::op_store::OperationId::from_bytes(
            state.baseline.operation.as_bytes(),
        ))
        .await
        .unwrap();
    let repo = loader.load_at(&op).await.unwrap();
    let source = repo
        .store()
        .get_commit(&b::CommitId::from_bytes(state.baseline.commit.as_bytes()))
        .unwrap();
    let mut tree = jj_lib::tree_builder::TreeBuilder::new(
        repo.store().clone(),
        source.tree_ids().as_resolved().unwrap().clone(),
    );
    for (path, value) in changes {
        let path = RepoPathBuf::from_internal_string(path).unwrap();
        if let Some((bytes, executable)) = value {
            let id = repo
                .store()
                .write_file(&path, &mut futures::io::Cursor::new(bytes))
                .await
                .unwrap();
            tree.set(
                path,
                b::TreeValue::File {
                    id,
                    executable,
                    copy_id: b::CopyId::placeholder(),
                },
            );
        } else {
            tree.remove(path);
        }
    }
    if let Some((path, target)) = link {
        let path = RepoPathBuf::from_internal_string(path).unwrap();
        let id = repo.store().write_symlink(&path, target).await.unwrap();
        tree.set(path, b::TreeValue::Symlink(id));
    }
    let tree = tree.write_tree().await.unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .rewrite_commit(&source)
        .set_tree(jj_lib::merged_tree::MergedTree::resolved(
            repo.store().clone(),
            tree,
        ))
        .write()
        .await
        .unwrap();
    tx.repo_mut().rebase_descendants().await.unwrap();
    tx.repo_mut()
        .set_wc_commit(WorkspaceNameBuf::from("default"), commit.id().clone())
        .unwrap();
    let repo = tx
        .write("Unpublished tree checkout fixture")
        .await
        .unwrap()
        .leave_unpublished();
    capture_operation_archive(
        &settings,
        &f.root().join(".jj/repo"),
        state.baseline.repository,
        m::OperationId::from_slice(repo.operation().id().as_bytes()).unwrap(),
        Default::default(),
    )
    .await
    .unwrap()
}
async fn retained(f: &Fixture, changes: FileChanges<'_>) -> RecoveredHistory {
    retained_with_inputs(f, changes).await.0
}
async fn retained_with_inputs(
    f: &Fixture,
    changes: FileChanges<'_>,
) -> (
    RecoveredHistory,
    jj_store_jj::local_execution::RecoveryWitnessV1,
    StackPreparationKey,
) {
    let history = candidate(f, changes).await;
    retained_archive(f, history).await
}
async fn retained_archive(
    f: &Fixture,
    history: OperationArchiveV1,
) -> (
    RecoveredHistory,
    jj_store_jj::local_execution::RecoveryWitnessV1,
    StackPreparationKey,
) {
    let state = f.witness().await;
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), state.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let key = StackPreparationKey {
        repository: state.baseline.repository,
        physical_workspace: state.physical_workspace,
        principal: m::PrincipalId::new([85; 16]),
        request: m::RequestId::new([94; 16]),
        intent: m::IntentDigest::new([95; 32]),
        prepared: m::RecordDigest::new([96; 32]),
    };
    MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .retain_stack_execution(&key, &history)
        .unwrap();
    drop(lease);
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    (recovery, witness, key)
}
#[tokio::test]
async fn retained_changed_tree_can_be_planned_without_checkout_or_publication() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let recovery = retained(
        &f,
        vec![
            ("file.txt", Some((b"changed\n", false))),
            ("src/binary", Some((&[0, 255, 3], true))),
        ],
    )
    .await;
    let before = f.state();
    let (physical, digest) = guard(&f);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(physical, digest)
        .await
        .unwrap();
    assert_eq!(plan.paths().collect::<Vec<_>>(), ["file.txt", "src/binary"]);
    assert_eq!(f.state(), before);
    assert!(!f.root().join("src").exists());
    recovery.validate_binding().await.unwrap();
}
