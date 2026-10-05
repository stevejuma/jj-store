use super::*;
use jj_store_jj::local_execution::TreeCheckoutPlanV1;
use std::os::unix::fs::{PermissionsExt as _, symlink};
#[tokio::test]
async fn dirty_missing_and_wrong_mode_source_files_are_preserved_and_rejected() {
    for kind in ["dirty", "missing", "executable"] {
        let f = Fixture::new().await;
        fs::write(f.root().join("file.txt"), "original\n").unwrap();
        let recovery = retained(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
        match kind {
            "dirty" => fs::write(f.root().join("file.txt"), "later dirty\n").unwrap(),
            "missing" => fs::remove_file(f.root().join("file.txt")).unwrap(),
            _ => fs::set_permissions(f.root().join("file.txt"), fs::Permissions::from_mode(0o755))
                .unwrap(),
        }
        // Deliberately permit a fresh guard for the changed fixture: provider
        // preflight itself must compare affected bytes/mode to the retained tree.
        let physical: Arc<dyn PhysicalGuard> = Arc::new(Allow);
        let before = fs::read(f.root().join(".jj/working_copy/tree_state")).unwrap();
        let observed = fs::read(f.root().join("file.txt")).ok();
        assert!(
            matches!(
                recovery
                    .prepare_tree_checkout(physical, m::RecordDigest::new([1; 32]))
                    .await,
                Err(local_browser::Error::EpochChanged)
            ),
            "{kind}"
        );
        assert_eq!(fs::read(f.root().join("file.txt")).ok(), observed);
        assert_eq!(
            fs::read(f.root().join(".jj/working_copy/tree_state")).unwrap(),
            before
        );
    }
}
struct Allow;
#[async_trait::async_trait]
impl PhysicalGuard for Allow {
    async fn validate(&self, _: &RecoveryBinding, _: m::RecordDigest) -> local_browser::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn untracked_and_ignored_addition_targets_are_not_adopted_even_with_matching_bytes() {
    for path in ["ignored.txt", "untracked.txt"] {
        let f = Fixture::new().await;
        let recovery = retained(&f, vec![(path, Some((b"obstacle\n", false)))]).await;
        fs::write(f.root().join(path), "obstacle\n").unwrap();
        let before = f.state();
        let (physical, digest) = guard(&f);
        assert!(matches!(
            recovery.prepare_tree_checkout(physical, digest).await,
            Err(local_browser::Error::EpochChanged)
        ));
        assert_eq!(f.state(), before);
        assert_eq!(fs::read(f.root().join(path)).unwrap(), b"obstacle\n");
    }
}
#[tokio::test]
async fn safe_deletion_keeps_unrelated_dirty_and_ignored_files_untouched() {
    let f = Fixture::new().await;
    // file.txt is intentionally dirty and absent from this command's effects.
    let recovery = retained(&f, vec![(".gitignore", None)]).await;
    let before = f.state();
    let (physical, digest) = guard(&f);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(physical.clone(), digest)
        .await
        .unwrap();
    assert_eq!(plan.paths().collect::<Vec<_>>(), [".gitignore"]);
    let bytes = plan.encode().unwrap();
    let decoded = TreeCheckoutPlanV1::decode(plan.id().unwrap(), &bytes).unwrap();
    let (_recovery, same) = recovery
        .revalidate_tree_checkout(decoded, physical)
        .await
        .unwrap();
    assert_eq!(same.encode().unwrap(), bytes);
    assert_eq!(f.state(), before);
    assert!(f.root().join(".gitignore").exists());
}
#[tokio::test]
async fn unsafe_files_and_parent_links_cannot_redirect_checkout_preflight() {
    for kind in [
        "file-symlink",
        "file-hardlink",
        "parent-symlink",
        "parent-file",
    ] {
        let f = Fixture::new().await;
        fs::write(f.root().join("file.txt"), "original\n").unwrap();
        let path = if kind.starts_with("parent") {
            "src/file"
        } else {
            "file.txt"
        };
        let recovery = retained(&f, vec![(path, Some((b"candidate", false)))]).await;
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("file"), "original\n").unwrap();
        match kind {
            "file-symlink" => {
                fs::remove_file(f.root().join("file.txt")).unwrap();
                symlink(outside.path().join("file"), f.root().join("file.txt")).unwrap();
            }
            "file-hardlink" => {
                fs::remove_file(f.root().join("file.txt")).unwrap();
                fs::hard_link(outside.path().join("file"), f.root().join("file.txt")).unwrap();
            }
            "parent-symlink" => symlink(outside.path(), f.root().join("src")).unwrap(),
            _ => fs::write(f.root().join("src"), "obstacle").unwrap(),
        }
        let before = f.state();
        assert!(
            recovery
                .prepare_tree_checkout(Arc::new(Allow), m::RecordDigest::new([1; 32]))
                .await
                .is_err(),
            "{kind}"
        );
        assert_eq!(f.state(), before);
        assert_eq!(
            fs::read(outside.path().join("file")).unwrap(),
            b"original\n"
        );
    }
}
#[tokio::test]
async fn replaced_files_and_directories_invalidate_saved_plan_even_when_bytes_match() {
    for kind in ["file", "directory"] {
        let f = Fixture::new().await;
        fs::write(f.root().join("file.txt"), "original\n").unwrap();
        fs::create_dir(f.root().join("src")).unwrap();
        let recovery = retained(
            &f,
            vec![
                ("file.txt", Some((b"candidate", false))),
                ("src/new", Some((b"added", false))),
            ],
        )
        .await;
        let (physical, digest) = guard(&f);
        let (recovery, plan) = recovery
            .prepare_tree_checkout(physical.clone(), digest)
            .await
            .unwrap();
        if kind == "file" {
            fs::rename(f.root().join("file.txt"), f.root().join("saved-file")).unwrap();
            fs::write(f.root().join("file.txt"), "original\n").unwrap();
        } else {
            fs::rename(f.root().join("src"), f.root().join("saved-dir")).unwrap();
            fs::create_dir(f.root().join("src")).unwrap();
        }
        let before = f.state();
        assert!(
            matches!(
                recovery.revalidate_tree_checkout(plan, physical).await,
                Err(local_browser::Error::EpochChanged)
            ),
            "{kind}"
        );
        assert_eq!(f.state(), before);
        assert!(!f.root().join("src/new").exists());
    }
}
#[tokio::test]
async fn aliases_and_file_directory_transitions_are_rejected_without_changes() {
    for changes in [
        vec![("FILE.txt", Some((b"alias".as_slice(), false)))],
        vec![
            ("file.txt", None),
            ("file.txt/child", Some((b"child".as_slice(), false))),
        ],
        vec![("caf\u{e9}", Some((b"unicode".as_slice(), false)))],
        vec![
            ("Src/a", Some((b"one".as_slice(), false))),
            ("src/b", Some((b"two".as_slice(), false))),
        ],
    ] {
        let f = Fixture::new().await;
        let recovery = retained(&f, changes).await;
        let before = f.state();
        let (physical, digest) = guard(&f);
        assert!(matches!(
            recovery.prepare_tree_checkout(physical, digest).await,
            Err(local_browser::Error::Invalid)
        ));
        assert_eq!(f.state(), before);
    }
}
#[tokio::test]
async fn oversized_blobs_are_rejected_before_physical_capture() {
    let f = Fixture::new().await;
    let data = vec![0u8; 2 * 1024 * 1024 + 1];
    let recovery = retained(&f, vec![("large", Some((&data, false)))]).await;
    let before = f.state();
    let (physical, digest) = guard(&f);
    assert!(matches!(
        recovery.prepare_tree_checkout(physical, digest).await,
        Err(local_browser::Error::Limit)
    ));
    assert_eq!(f.state(), before);
    assert!(!f.root().join("large").exists());
}
#[tokio::test]
async fn plan_decode_rejects_wrong_digest_noncanonical_and_unsupported_fields() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let recovery = retained(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
    let (physical, digest) = guard(&f);
    let (_recovery, plan) = recovery
        .prepare_tree_checkout(physical, digest)
        .await
        .unwrap();
    let mut bytes = plan.encode().unwrap();
    assert!(TreeCheckoutPlanV1::decode(m::RecordDigest::new([0; 32]), &bytes).is_err());
    bytes.push(b' ');
    assert!(TreeCheckoutPlanV1::decode(plan.id().unwrap(), &bytes).is_err());
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["unsupported"] = serde_json::json!(true);
    assert!(
        TreeCheckoutPlanV1::decode(plan.id().unwrap(), &serde_json::to_vec(&value).unwrap())
            .is_err()
    );
}

#[tokio::test]
async fn symlink_candidate_is_readable_but_cannot_gain_checkout_plan_support() {
    let f = Fixture::new().await;
    let archive = candidate_with_symlink(&f, vec![], Some(("link", "../outside"))).await;
    let (recovery, _, _) = retained_archive(&f, archive).await;
    let before = f.state();
    let (physical, digest) = guard(&f);
    assert!(matches!(
        recovery.prepare_tree_checkout(physical, digest).await,
        Err(local_browser::Error::Invalid)
    ));
    assert_eq!(f.state(), before);
    assert!(!f.root().join("link").exists());
}

#[tokio::test]
async fn recomputed_plan_digest_cannot_reauthorize_a_tampered_retained_request() {
    use sha2::{Digest as _, Sha256};
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    let recovery = retained(&f, vec![("file.txt", Some((b"candidate", false)))]).await;
    let (physical, digest) = guard(&f);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(physical.clone(), digest)
        .await
        .unwrap();
    let before = f.state();
    let mut value: serde_json::Value = serde_json::from_slice(&plan.encode().unwrap()).unwrap();
    value["key"]["request"] = serde_json::to_value(m::RequestId::new([37; 16])).unwrap();
    let bytes = serde_json::to_vec(&value).unwrap();
    let mut hash = Sha256::new_with_prefix(b"jj-store/tree-checkout-plan/v1\0");
    hash.update(&bytes);
    let plan =
        TreeCheckoutPlanV1::decode(m::RecordDigest::new(hash.finalize().into()), &bytes).unwrap();
    assert!(matches!(
        recovery.revalidate_tree_checkout(plan, physical).await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn creating_a_previously_missing_parent_invalidates_saved_absence() {
    let f = Fixture::new().await;
    let recovery = retained(&f, vec![("src/new", Some((b"candidate", false)))]).await;
    let (physical, digest) = guard(&f);
    let (recovery, plan) = recovery
        .prepare_tree_checkout(physical.clone(), digest)
        .await
        .unwrap();
    fs::create_dir(f.root().join("src")).unwrap();
    let before = f.state();
    assert!(matches!(
        recovery.revalidate_tree_checkout(plan, physical).await,
        Err(local_browser::Error::EpochChanged)
    ));
    assert_eq!(f.state(), before);
    assert!(!f.root().join("src/new").exists());
}

#[tokio::test]
async fn native_sparse_checkout_is_unsupported_without_expanding_its_disk_scope() {
    let f = Fixture::new().await;
    fs::write(f.root().join("file.txt"), "original\n").unwrap();
    fixture::stock(
        &f.root(),
        &["sparse", "set", "--clear", "--add", "file.txt"],
    );
    let recovery = retained(&f, vec![("new", Some((b"candidate", false)))]).await;
    let before = f.state();
    let (physical, digest) = guard(&f);
    assert!(matches!(
        recovery.prepare_tree_checkout(physical, digest).await,
        Err(local_browser::Error::Invalid)
    ));
    assert_eq!(f.state(), before);
    assert!(!f.root().join("new").exists());
}
