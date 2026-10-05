#[path = "stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_unchanged_checkout/lifecycle.rs"]
mod lifecycle;
use fixture::*;
use jj_store_jj::{
    local_browser::{self, working_tree::CurrentState},
    local_execution::{Coordinator, PhysicalGuard, RecoveryWitnessV1, Reword},
    local_metadata::{Limits, MetadataStore, StackPreparationKey},
};
use jj_store_model as m;
use std::{fs, sync::Arc};
struct Files {
    root: std::path::PathBuf,
    expected: Vec<Vec<u8>>,
}
#[async_trait::async_trait]
impl PhysicalGuard for Files {
    async fn validate(
        &self,
        _binding: &jj_store_jj::local_execution::RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        if expected != m::RecordDigest::new([82; 32]) {
            return Err(local_browser::Error::Invalid);
        }
        let actual: Vec<_> = ["file.txt", "ignored.txt", ".gitignore"]
            .into_iter()
            .map(|p| fs::read(self.root.join(p)).unwrap())
            .collect();
        if actual != self.expected {
            return Err(local_browser::Error::EpochChanged);
        }
        Ok(())
    }
}
fn files(f: &Fixture) -> Arc<dyn PhysicalGuard> {
    Arc::new(Files {
        root: f.root(),
        expected: ["file.txt", "ignored.txt", ".gitignore"]
            .into_iter()
            .map(|p| fs::read(f.root().join(p)).unwrap())
            .collect(),
    })
}
async fn prepare(f: &Fixture) -> (RecoveryWitnessV1, StackPreparationKey, CurrentState) {
    let state = f.witness().await;
    let lease = Coordinator::new(1)
        .unwrap()
        .acquire(f.access(), f.root(), state.clone())
        .await
        .unwrap();
    let (lease, witness) = lease.recovery_witness().await.unwrap();
    let (lease, candidate) = lease
        .prepare_reword(Reword {
            principal: m::PrincipalId::new([85; 16]),
            immutable_heads: vec![],
            commit: state.baseline.commit,
            message: "Complete exact unchanged checkout".into(),
        })
        .await
        .unwrap();
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
        .retain_stack_execution(&key, &candidate.history)
        .unwrap();
    drop(lease);
    (witness, key, state)
}
#[tokio::test]
async fn exact_unchanged_checkout_updates_only_operation_pointer_and_retries_after_reopen() {
    let f = Fixture::new().await;
    let (witness, key, state) = prepare(&f).await;
    let before = f.state();
    let guard = files(&f);
    let physical = m::RecordDigest::new([82; 32]);
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (recovery, published) = recovery.publish_head().await.unwrap();
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&published, guard.clone(), physical)
        .await
        .unwrap();
    assert_eq!(&f.state()[..4], &before[..4]);
    let encoded = checkout.encode().unwrap();
    let checkout =
        jj_store_jj::local_execution::CheckoutWitnessV1::decode(checkout.id().unwrap(), &encoded)
            .unwrap();
    let (recovery, proof) = recovery
        .complete_unchanged_checkout(checkout.clone(), guard.clone())
        .await
        .unwrap();
    assert_eq!(proof.operation, published.operation);
    assert_eq!(proof.view, published.view);
    assert_eq!(proof.preparation(), &key);
    assert_eq!(proof.physical(), physical);
    assert_eq!(f.state()[0], before[0]);
    assert_eq!(f.state()[1], before[1]);
    assert_eq!(f.state()[3], before[3]);
    let current = local_browser::Executor::new(1)
        .unwrap()
        .working_tree_current(f.access(), f.root(), published.operation)
        .await
        .unwrap();
    assert_eq!(current.baseline.trees, state.baseline.trees);
    assert_eq!(current.checkout.operation, published.operation);
    recovery.validate_binding().await.unwrap();
    drop(recovery);
    let selected = f.state();
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout.clone())
        .await
        .unwrap();
    let (_recovery, same) = recovery
        .complete_unchanged_checkout(checkout, guard)
        .await
        .unwrap();
    assert_eq!(same, proof);
    assert_eq!(f.state(), selected);
}

async fn published(
    f: &Fixture,
) -> (
    jj_store_jj::local_execution::RecoveredHistory,
    jj_store_jj::local_execution::PublishedHead,
    RecoveryWitnessV1,
    StackPreparationKey,
) {
    let (witness, key, _) = prepare(f).await;
    let recovery = Coordinator::new(1)
        .unwrap()
        .resume_publication(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap()
        .restore_extras()
        .await
        .unwrap()
        .restore_indexes()
        .await
        .unwrap();
    let (recovery, proof) = recovery.publish_head().await.unwrap();
    (recovery, proof, witness, key)
}

#[tokio::test]
async fn changed_dirty_ignored_and_ignore_files_fail_before_pointer_update_and_preserve_new_bytes()
{
    for path in ["file.txt", "ignored.txt", ".gitignore"] {
        let f = Fixture::new().await;
        let guard = files(&f);
        let (recovery, proof, _, _) = published(&f).await;
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&proof, guard.clone(), m::RecordDigest::new([82; 32]))
            .await
            .unwrap();
        fs::write(f.root().join(path), "new external bytes\n").unwrap();
        let before = f.state();
        assert!(matches!(
            recovery.complete_unchanged_checkout(checkout, guard).await,
            Err(local_browser::Error::EpochChanged)
        ));
        assert_eq!(f.state(), before);
        assert_eq!(
            fs::read(f.root().join(path)).unwrap(),
            b"new external bytes\n"
        );
    }
}

#[tokio::test]
async fn later_cli_checkout_cannot_be_replaced_with_saved_candidate_pointer() {
    let f = Fixture::new().await;
    let (recovery, proof, witness, key) = published(&f).await;
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&proof, files(&f), m::RecordDigest::new([82; 32]))
        .await
        .unwrap();
    drop(recovery);
    stock(
        &f.root(),
        &["describe", "-r", "@", "-m", "Later foreground CLI checkout"],
    );
    let before = f.state();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout)
            .await
            .is_err()
    );
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn scoped_witness_canonical_encoding_and_reuse_reject_tampering() {
    let f = Fixture::new().await;
    let (recovery, proof, witness, key) = published(&f).await;
    let (recovery, checkout) = recovery
        .prepare_unchanged_checkout(&proof, files(&f), m::RecordDigest::new([82; 32]))
        .await
        .unwrap();
    let bytes = checkout.encode().unwrap();
    assert!(
        jj_store_jj::local_execution::CheckoutWitnessV1::decode(
            m::RecordDigest::new([0; 32]),
            &bytes
        )
        .is_err()
    );
    let mut spaced = bytes.clone();
    spaced.push(b' ');
    assert!(
        jj_store_jj::local_execution::CheckoutWitnessV1::decode(checkout.id().unwrap(), &spaced)
            .is_err()
    );
    assert!(
        jj_store_jj::local_execution::CheckoutWitnessV1::decode(
            checkout.id().unwrap(),
            &vec![b' '; 128 * 1024 + 1]
        )
        .is_err()
    );
    drop(recovery);
    let before = f.state();
    let mut other = key.clone();
    other.request = m::RequestId::new([97; 16]);
    assert!(
        Coordinator::new(1)
            .unwrap()
            .resume_unchanged_checkout(
                f.access(),
                f.root(),
                witness.clone(),
                other,
                checkout.clone()
            )
            .await
            .is_err()
    );
    // Private records cannot change their native before/after bytes while
    // retaining the original admitted digest.
    let mut changed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    changed["after"] = serde_json::json!([1, 2, 3]);
    assert!(
        jj_store_jj::local_execution::CheckoutWitnessV1::decode(
            checkout.id().unwrap(),
            &serde_json::to_vec(&changed).unwrap()
        )
        .is_err()
    );
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn replaced_or_unsafe_checkout_entries_fail_without_changing_targets() {
    use std::os::unix::fs::symlink;
    for mode in ["same-bytes-replacement", "symlink", "hardlink", "oversized"] {
        let f = Fixture::new().await;
        let (recovery, proof, witness, key) = published(&f).await;
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&proof, files(&f), m::RecordDigest::new([82; 32]))
            .await
            .unwrap();
        drop(recovery);
        let path = f.root().join(".jj/working_copy/checkout");
        let bytes = fs::read(&path).unwrap();
        let side = tempfile::tempdir().unwrap();
        let outside = side.path().join("outside");
        fs::write(&outside, &bytes).unwrap();
        let old = path.with_file_name("original-checkout");
        fs::rename(&path, &old).unwrap();
        match mode {
            "same-bytes-replacement" => fs::write(&path, &bytes).unwrap(),
            "symlink" => symlink(&outside, &path).unwrap(),
            "hardlink" => fs::hard_link(&outside, &path).unwrap(),
            _ => fs::File::create(&path).unwrap().set_len(65537).unwrap(),
        }
        let before = f.state();
        assert!(
            Coordinator::new(1)
                .unwrap()
                .resume_unchanged_checkout(f.access(), f.root(), witness, key, checkout)
                .await
                .is_err(),
            "{mode}"
        );
        assert_eq!(f.state(), before);
        assert_eq!(fs::read(outside).unwrap(), bytes);
        assert_eq!(fs::read(old).unwrap(), bytes);
    }
}

#[tokio::test]
async fn deterministic_staging_reuses_only_exact_prefix_and_rejects_unsafe_entries() {
    use std::os::unix::fs::symlink;
    for mode in [
        "empty", "partial", "complete", "corrupt", "symlink", "hardlink",
    ] {
        let f = Fixture::new().await;
        let (recovery, proof, _, _) = published(&f).await;
        let guard = files(&f);
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&proof, guard.clone(), m::RecordDigest::new([82; 32]))
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&checkout.encode().unwrap()).unwrap();
        let output: Vec<u8> = json["after"]
            .as_array()
            .unwrap()
            .iter()
            .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
            .collect();
        let staged = f
            .root()
            .join(".jj/working_copy")
            .join(format!(".lapis-stack-checkout-{}", checkout.id().unwrap()));
        let side = tempfile::tempdir().unwrap();
        let outside = side.path().join("outside");
        fs::write(&outside, &output).unwrap();
        match mode {
            "empty" => fs::write(&staged, []).unwrap(),
            "partial" => fs::write(&staged, &output[..output.len() / 2]).unwrap(),
            "complete" => fs::write(&staged, &output).unwrap(),
            "corrupt" => fs::write(&staged, b"not the exact candidate").unwrap(),
            "symlink" => symlink(&outside, &staged).unwrap(),
            _ => fs::hard_link(&outside, &staged).unwrap(),
        }
        let before = f.state();
        let result = recovery.complete_unchanged_checkout(checkout, guard).await;
        if matches!(mode, "empty" | "partial" | "complete") {
            result.unwrap();
            assert!(!staged.exists(), "{mode}");
            assert_eq!(f.state()[2], output, "{mode}");
            assert_eq!(f.state()[3], before[3], "{mode}");
        } else {
            assert!(result.is_err(), "{mode}");
            assert_eq!(f.state(), before, "{mode}");
        }
        assert_eq!(fs::read(outside).unwrap(), output, "{mode}");
    }
}
