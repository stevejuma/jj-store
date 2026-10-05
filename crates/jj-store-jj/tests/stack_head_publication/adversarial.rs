use super::*;
use jj_lib::{op_store::OpStore as _, simple_op_store::SimpleOpStore};
use jj_store_jj::local_browser::Error;
use std::{fs::File, os::unix::fs::symlink};

#[tokio::test]
async fn missing_extras_and_indexes_require_explicit_repair_before_head_selection() {
    for path in ["store/extra", "index"] {
        let f = Fixture::new().await;
        let (witness, key, candidate) = prepare(&f).await;
        let recovery = recover(&f, witness.clone(), key.clone()).await;
        let before = f.state();
        let missing = f.root().join(".jj/repo").join(path);
        fs::remove_dir_all(&missing).unwrap();
        assert!(matches!(
            recovery.publish_head().await,
            Err(Error::Incomplete)
        ));
        assert!(!missing.exists());
        assert_eq!(f.state(), before);
        let (_recovery, proof) = recover(&f, witness, key)
            .await
            .publish_head()
            .await
            .unwrap();
        assert_eq!(proof.operation, candidate);
    }
}

#[tokio::test]
async fn missing_or_corrupt_archived_operation_and_view_fail_preflight_without_selection() {
    for (group, corrupt) in [
        ("operations", false),
        ("operations", true),
        ("views", false),
        ("views", true),
    ] {
        let f = Fixture::new().await;
        let (witness, key, candidate) = prepare(&f).await;
        let recovery = recover(&f, witness, key.clone()).await;
        let retained = MetadataStore::open(&f.root().join(".git"), Limits::default())
            .unwrap()
            .read_stack_prepared(&key)
            .unwrap()
            .unwrap();
        let archive = retained.history.unwrap();
        let name = if group == "operations" {
            candidate.to_string()
        } else {
            archive.operations[&candidate].view_id.to_string()
        };
        let path = f.root().join(".jj/repo/op_store").join(group).join(name);
        if corrupt {
            fs::write(path, b"invalid native record").unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
        let before = f.state();
        assert!(recovery.publish_head().await.is_err());
        assert_eq!(f.state(), before);
    }
}

#[tokio::test]
async fn unrelated_missing_and_hash_substituted_current_heads_are_rejected() {
    for case in ["unrelated", "missing", "substituted"] {
        let f = Fixture::new().await;
        let (witness, key, candidate) = prepare(&f).await;
        let directory = f.root().join(".jj/repo/op_heads/heads");
        let foreign = if case == "unrelated" {
            let archive = MetadataStore::open(&f.root().join(".git"), Limits::default())
                .unwrap()
                .read_stack_prepared(&key)
                .unwrap()
                .unwrap()
                .history
                .unwrap();
            *archive
                .operations
                .keys()
                .find(|id| {
                    **id != witness.binding().operation
                        && **id != candidate
                        && id.as_bytes() != &[0; 64]
                })
                .unwrap()
        } else {
            m::OperationId::new([99; 64])
        };
        if case == "substituted" {
            fs::copy(
                f.root()
                    .join(".jj/repo/op_store/operations")
                    .join(candidate.to_string()),
                f.root()
                    .join(".jj/repo/op_store/operations")
                    .join(foreign.to_string()),
            )
            .unwrap();
        }
        fs::remove_file(directory.join(witness.binding().operation.to_string())).unwrap();
        fs::write(directory.join(foreign.to_string()), []).unwrap();
        let before = f.state();
        assert!(
            Coordinator::new(1)
                .unwrap()
                .resume_publication(f.access(), f.root(), witness, key)
                .await
                .is_err()
        );
        assert_eq!(&f.state()[..4], &before[..4]);
        assert_eq!(heads(&f), vec![foreign.to_string()]);
    }
}

#[tokio::test]
async fn native_head_lock_contention_is_nonblocking_and_retry_preserves_original() {
    let f = Fixture::new().await;
    let (witness, key, _) = prepare(&f).await;
    let recovery = recover(&f, witness.clone(), key.clone()).await;
    let file = File::create(f.root().join(".jj/repo/op_heads/heads/lock")).unwrap();
    rustix::fs::flock(&file, rustix::fs::FlockOperation::LockExclusive).unwrap();
    let before = f.state();
    assert!(matches!(
        tokio::time::timeout(std::time::Duration::from_secs(5), recovery.publish_head())
            .await
            .unwrap(),
        Err(Error::Busy)
    ));
    assert_eq!(f.state(), before);
    drop(file);
    recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
}

#[tokio::test]
async fn unsafe_head_markers_coordination_files_and_arbitrary_staging_names_fail_closed() {
    for case in [
        "symlink-head",
        "hardlink-head",
        "nonempty-head",
        "symlink-lock",
        "hardlink-lock",
        "nonempty-lock",
        "unknown",
        "staging-symlink",
        "staging-nonempty",
        "staging-malformed",
    ] {
        let f = Fixture::new().await;
        let (witness, key, _) = prepare(&f).await;
        let directory = f.root().join(".jj/repo/op_heads/heads");
        let side = tempfile::tempdir().unwrap();
        let outside = side.path().join("outside");
        fs::write(&outside, []).unwrap();
        let name = match case {
            "symlink-head" | "hardlink-head" | "nonempty-head" => {
                witness.binding().operation.to_string()
            }
            "symlink-lock" | "hardlink-lock" | "nonempty-lock" => "lock".into(),
            "staging-symlink" | "staging-nonempty" => ".lapis-stack-recovery-1-2-3".into(),
            "staging-malformed" => ".lapis-stack-recovery-whatever".into(),
            _ => "arbitrary".into(),
        };
        let path = directory.join(name);
        if path.exists() {
            fs::remove_file(&path).unwrap();
        }
        if case.contains("symlink") {
            symlink(&outside, &path).unwrap();
        } else if case.contains("hardlink") {
            fs::hard_link(&outside, &path).unwrap();
        } else {
            fs::write(
                path,
                if case.contains("nonempty") {
                    &b"unsafe"[..]
                } else {
                    &[]
                },
            )
            .unwrap();
        }
        let before = f.state();
        assert!(
            Coordinator::new(1)
                .unwrap()
                .resume_publication(f.access(), f.root(), witness, key)
                .await
                .is_err(),
            "{case}"
        );
        assert_eq!(f.state(), before);
        assert_eq!(fs::read(outside).unwrap(), b"");
    }
}

#[tokio::test]
async fn safe_interrupted_head_staging_is_ignored_and_never_selected_as_a_head() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let path = f
        .root()
        .join(".jj/repo/op_heads/heads/.lapis-stack-recovery-123-456-0");
    fs::write(&path, []).unwrap();
    let (_recovery, proof) = recover(&f, witness, key)
        .await
        .publish_head()
        .await
        .unwrap();
    assert_eq!(proof.operation, candidate);
    assert_eq!(heads(&f), vec![candidate.to_string()]);
    assert_eq!(fs::read(path).unwrap(), b"");
}

#[tokio::test]
async fn candidate_with_another_operation_parent_cannot_publish_under_saved_witness() {
    let f = Fixture::new().await;
    let (witness, key, candidate) = prepare(&f).await;
    let store = MetadataStore::open(&f.root().join(".git"), Limits::default()).unwrap();
    let mut archive = store
        .read_stack_prepared(&key)
        .unwrap()
        .unwrap()
        .history
        .unwrap();
    let mut op: jj_lib::op_store::Operation =
        archive.operations[&candidate].clone().try_into().unwrap();
    op.parents
        .push(jj_lib::op_store::OperationId::from_bytes(&[0; 64]));
    let native = SimpleOpStore::load(
        &f.root().join(".jj/repo/op_store"),
        jj_lib::op_store::RootOperationData {
            root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
        },
    );
    let id = native.write_operation(&op).await.unwrap();
    use jj_lib::object_id::ObjectId as _;
    let id = m::OperationId::new(id.as_bytes().try_into().unwrap());
    archive.operations.remove(&candidate);
    archive.operations.insert(
        id,
        jj_store_jj::local_evidence::GitOperationV1::try_from(&op).unwrap(),
    );
    archive.operation = id;
    // Its complete native archive still includes the original witness, but
    // multiple operation parents do not authorize this publication protocol.
    archive.validate().unwrap();
    let mut other = key;
    other.request = m::RequestId::new([97; 16]);
    store.retain_stack_execution(&other, &archive).unwrap();
    let before = f.state();
    assert!(
        Coordinator::new(1)
            .unwrap()
            .resume_publication(f.access(), f.root(), witness, other)
            .await
            .is_err()
    );
    assert_eq!(f.state(), before);
}

#[tokio::test]
async fn oversized_ancestry_and_excessive_head_enumeration_fail_before_selection() {
    for case in ["oversized-record", "excessive-heads", "excessive-staging"] {
        let f = Fixture::new().await;
        let (witness, key, _) = prepare(&f).await;
        let directory = f.root().join(".jj/repo/op_heads/heads");
        if case == "oversized-record" {
            let foreign = m::OperationId::new([98; 64]);
            fs::remove_file(directory.join(witness.binding().operation.to_string())).unwrap();
            fs::write(directory.join(foreign.to_string()), []).unwrap();
            File::create(
                f.root()
                    .join(".jj/repo/op_store/operations")
                    .join(foreign.to_string()),
            )
            .unwrap()
            .set_len(4 * 1024 * 1024 + 1)
            .unwrap();
        } else {
            for n in 0..129 {
                let name = if case == "excessive-heads" {
                    m::OperationId::new([n as u8; 64]).to_string()
                } else {
                    format!(".lapis-stack-recovery-1-2-{n}")
                };
                fs::write(directory.join(name), []).unwrap();
            }
        }
        let before = f.state();
        assert!(
            matches!(
                Coordinator::new(1)
                    .unwrap()
                    .resume_publication(f.access(), f.root(), witness, key)
                    .await,
                Err(Error::Limit)
            ),
            "{case}"
        );
        assert_eq!(f.state(), before);
    }
}

#[tokio::test]
async fn actual_native_ancestry_count_and_parent_fanout_are_bounded() {
    use jj_lib::object_id::ObjectId as _;
    for case in ["count", "parents"] {
        let f = Fixture::new().await;
        let (witness, key, _) = prepare(&f).await;
        let native = SimpleOpStore::load(
            &f.root().join(".jj/repo/op_store"),
            jj_lib::op_store::RootOperationData {
                root_commit_id: jj_lib::backend::CommitId::from_bytes(&[0; 20]),
            },
        );
        let mut parent =
            jj_lib::op_store::OperationId::from_bytes(witness.binding().operation.as_bytes());
        let mut op = native.read_operation(&parent).await.unwrap();
        let count = if case == "count" { 4096 } else { 1 };
        for n in 0..count {
            op.metadata.description = format!("Bounded native ancestry {n}");
            op.parents = vec![parent.clone(); if case == "parents" { 65 } else { 1 }];
            parent = native.write_operation(&op).await.unwrap();
        }
        let id = m::OperationId::new(parent.as_bytes().try_into().unwrap());
        let directory = f.root().join(".jj/repo/op_heads/heads");
        fs::remove_file(directory.join(witness.binding().operation.to_string())).unwrap();
        fs::write(directory.join(id.to_string()), []).unwrap();
        let before = f.state();
        let result = Coordinator::new(1)
            .unwrap()
            .resume_publication(f.access(), f.root(), witness, key)
            .await;
        if case == "count" {
            assert!(matches!(result, Err(Error::Limit)));
        } else {
            assert!(matches!(result, Err(Error::Invalid)));
        }
        assert_eq!(f.state(), before);
    }
}
