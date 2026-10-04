//! Lossless identity, structural rejection and pinned reference conformance.
use jj_lib::backend::Backend as _;
use jj_lib::content_hash::blake2b_hash;
use jj_lib::merge::Merge;
use jj_lib::object_id::ObjectId as _;
use jj_lib::op_store::OpStore as _;
use jj_lib::repo_path::{RepoPath, RepoPathBuf, RepoPathComponentBuf};
use jj_lib::simple_backend::SimpleBackend;
use jj_lib::simple_op_store::SimpleOpStore;
use jj_lib::{backend as b, op_store as o};
use jj_store_jj::codec::{self, MAX_METADATA_BYTES, NativeCodec, NativeFile, NativeSymlink};
use jj_store_model::{self as m, envelope};
use std::collections::BTreeMap;

fn c(n: u8) -> b::CommitId {
    b::CommitId::from_bytes(&[n; 64])
}
fn t(n: u8) -> b::TreeId {
    b::TreeId::from_bytes(&[n; 64])
}
fn sample_commit() -> b::Commit {
    let mut commit = b::make_root_commit(b::ChangeId::from_bytes(&[1; 16]), t(7));
    commit.parents = vec![c(2), c(1)];
    commit.predecessors = vec![c(8), c(6)];
    commit.root_tree = Merge::from_vec(vec![t(7), t(6), t(5)]);
    commit.conflict_labels = Merge::from_vec(vec!["left".into(), "base".into(), "right".into()]);
    commit.description = "Merge binary data — 日本語\0\n".into();
    commit.author = b::Signature {
        name: "Alice\n".into(),
        email: "a@example.invalid".into(),
        timestamp: b::Timestamp {
            timestamp: b::MillisSinceEpoch(-123456789),
            tz_offset: -210,
        },
    };
    commit.committer = b::Signature {
        name: "Bob".into(),
        email: "b@example.invalid".into(),
        timestamp: b::Timestamp {
            timestamp: b::MillisSinceEpoch(1729000123456),
            tz_offset: 345,
        },
    };
    commit.secure_sig = Some(b::SecureSig {
        data: vec![0, 255, 2],
        sig: vec![254, 0, 3],
    });
    commit
}
fn sample_tree() -> b::Tree {
    let values = vec![
        (
            "binary",
            b::TreeValue::File {
                id: b::FileId::from_bytes(&[1; 64]),
                executable: false,
                copy_id: b::CopyId::placeholder(),
            },
        ),
        (
            "copy",
            b::TreeValue::File {
                id: b::FileId::from_bytes(&[1; 64]),
                executable: true,
                copy_id: b::CopyId::from_bytes(&[4; 64]),
            },
        ),
        ("dir", b::TreeValue::Tree(t(3))),
        (
            "link",
            b::TreeValue::Symlink(b::SymlinkId::from_bytes(&[2; 64])),
        ),
        (
            "module",
            b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[6; 20])),
        ),
    ];
    b::Tree::from_sorted_entries(
        values
            .into_iter()
            .map(|(n, v)| (RepoPathComponentBuf::new(n).unwrap(), v))
            .collect(),
    )
}
fn sample_copy() -> b::CopyHistory {
    b::CopyHistory {
        current_path: RepoPathBuf::from_internal_string("src/日本語.rs").unwrap(),
        parents: vec![
            b::CopyId::from_bytes(&[9; 64]),
            b::CopyId::placeholder(),
            b::CopyId::from_bytes(&[8; 64]),
        ],
        salt: vec![0, 255, 9],
    }
}
fn sample_view() -> o::View {
    let mut v = o::View::make_root(c(1));
    v.head_ids.insert(c(2));
    let conflict = o::RefTarget::from_merge(Merge::from_vec(vec![Some(c(3)), None, Some(c(4))]));
    v.local_bookmarks
        .insert("conflicted".into(), conflict.clone());
    v.local_bookmarks
        .insert("deleted".into(), o::RefTarget::absent());
    v.local_tags.insert("v1".into(), o::RefTarget::normal(c(7)));
    v.remote_views.insert(
        "origin".into(),
        o::RemoteView {
            bookmarks: BTreeMap::from([(
                "main".into(),
                o::RemoteRef {
                    target: conflict.clone(),
                    state: o::RemoteRefState::Tracked,
                },
            )]),
            tags: BTreeMap::from([(
                "tag".into(),
                o::RemoteRef {
                    target: o::RefTarget::normal(c(8)),
                    state: o::RemoteRefState::New,
                },
            )]),
        },
    );
    v.git_refs
        .insert("refs/heads/main".into(), conflict.clone());
    v.git_heads.insert("laptop".into(), conflict);
    v.git_heads.insert("desktop".into(), o::RefTarget::absent());
    v.wc_commit_ids.insert("laptop".into(), c(4));
    v.wc_commit_ids.insert("desktop".into(), c(5));
    v
}
fn sample_op() -> o::Operation {
    let mut op = o::Operation::make_root(o::ViewId::from_bytes(&[2; 64]));
    op.parents = vec![
        o::OperationId::from_bytes(&[3; 64]),
        o::OperationId::from_bytes(&[1; 64]),
    ];
    op.metadata.time.start = b::Timestamp {
        timestamp: b::MillisSinceEpoch(-42),
        tz_offset: -210,
    };
    op.metadata.time.end = b::Timestamp {
        timestamp: b::MillisSinceEpoch(87654321),
        tz_offset: 345,
    };
    op.metadata.description = "describe / 日本語".into();
    op.metadata.hostname = "host".into();
    op.metadata.username = "user".into();
    op.metadata.is_snapshot = true;
    op.metadata.workspace_name = Some("".into());
    op.metadata.attributes =
        BTreeMap::from([("z".into(), "\0".into()), ("a".into(), "value".into())]);
    op.commit_predecessors = Some(BTreeMap::from([
        (c(4), vec![c(3), c(2)]),
        (c(5), vec![]),
        (c(6), vec![c(4)]),
    ]));
    op
}
fn roundtrip<T: NativeCodec + std::fmt::Debug>(value: T) {
    let (key, bytes) = codec::encode(&value, MAX_METADATA_BYTES).unwrap();
    let restored = codec::decode::<T>(key, &bytes, MAX_METADATA_BYTES).unwrap();
    assert_eq!(restored, value);
    assert_eq!(
        codec::encode(&restored, MAX_METADATA_BYTES).unwrap(),
        (key, bytes)
    );
}
#[test]
fn all_native_fields_roundtrip_without_identity_loss() {
    roundtrip(sample_commit());
    roundtrip(sample_tree());
    roundtrip(sample_copy());
    roundtrip(sample_view());
    roundtrip(sample_op());
    roundtrip(NativeFile(vec![0, 1, 127, 128, 255]));
    roundtrip(NativeFile(vec![]));
    roundtrip(NativeSymlink("../dir/日本語".into()));
    roundtrip(b::Tree::default());
    let mut commit = sample_commit();
    commit.secure_sig = None;
    commit.root_tree = Merge::resolved(t(1));
    commit.conflict_labels = Merge::resolved("".into());
    roundtrip(commit);
    let mut op = sample_op();
    op.commit_predecessors = None;
    roundtrip(op.clone());
    let absent = op.key();
    op.commit_predecessors = Some(BTreeMap::new());
    roundtrip(op.clone());
    assert_ne!(absent, op.key());
    let empty_workspace = op.key();
    op.metadata.workspace_name = None;
    roundtrip(op.clone());
    assert_ne!(empty_workspace, op.key());
}
#[tokio::test]
async fn codec_ids_match_pinned_simple_stores() {
    let dir = tempfile::tempdir().unwrap();
    let backend = SimpleBackend::init(dir.path());
    let ops = tempfile::tempdir().unwrap();
    let store = SimpleOpStore::init(
        ops.path(),
        o::RootOperationData {
            root_commit_id: backend.root_commit_id().clone(),
        },
    )
    .unwrap();
    let file = NativeFile(vec![0, 255, 8, 2]);
    let mut input = futures::io::Cursor::new(file.0.clone());
    let id = backend
        .write_file(RepoPath::root(), &mut input)
        .await
        .unwrap();
    assert_eq!(id.as_bytes(), file.key().as_bytes());
    let link = NativeSymlink("../file".into());
    let id = backend
        .write_symlink(RepoPath::root(), &link.0)
        .await
        .unwrap();
    assert_eq!(id.as_bytes(), link.key().as_bytes());
    let mut unsigned = sample_commit();
    unsigned.secure_sig = None;
    let (id, stored) = backend.write_commit(unsigned, None).await.unwrap();
    assert_eq!(id.as_bytes(), stored.key().as_bytes());
    roundtrip(stored);
    // SimpleBackend cannot write copies/submodules, so test its supported tree subset.
    let tree = b::Tree::from_sorted_entries(vec![(
        RepoPathComponentBuf::new("f").unwrap(),
        b::TreeValue::File {
            id: b::FileId::from_bytes(file.key().as_bytes()),
            executable: true,
            copy_id: b::CopyId::placeholder(),
        },
    )]);
    assert_eq!(
        backend
            .write_tree(RepoPath::root(), &tree)
            .await
            .unwrap()
            .as_bytes(),
        tree.key().as_bytes()
    );
    let view = sample_view();
    assert_eq!(
        store.write_view(&view).await.unwrap().as_bytes(),
        view.key().as_bytes()
    );
    let op = sample_op();
    assert_eq!(
        store.write_operation(&op).await.unwrap().as_bytes(),
        op.key().as_bytes()
    );
    assert_eq!(
        sample_copy().key().as_bytes(),
        blake2b_hash(&sample_copy()).as_slice()
    );
    assert_eq!(
        sample_tree().key().as_bytes(),
        blake2b_hash(&sample_tree()).as_slice()
    );
}

#[test]
fn pinned_profile_vectors_include_every_stored_kind() {
    let observed = serde_json::json!({
        "commit":sample_commit().key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "tree":sample_tree().key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "copy":sample_copy().key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "view":sample_view().key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "operation":sample_op().key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "file":NativeFile(vec![0,1,127,128,255]).key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "symlink":NativeSymlink("../dir/日本語".into()).key().as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>(),
    });
    let expected: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/native-profile/m1-hashes.json"
    ))
    .unwrap();
    assert_eq!(observed, expected, "{observed}");
}

fn malformed<T: NativeCodec>(value: &T, edit: impl FnOnce(&mut serde_json::Value)) {
    let mut data: serde_json::Value =
        serde_json::from_slice(&value.encode_payload().unwrap()).unwrap();
    edit(&mut data);
    let payload = serde_json::to_vec(&data).unwrap();
    let forged = envelope::encode(value.key(), &payload, MAX_METADATA_BYTES).unwrap();
    let err = codec::decode::<T>(value.key(), &forged, MAX_METADATA_BYTES)
        .err()
        .expect("invalid payload accepted");
    assert!(
        !matches!(err, codec::CodecError::Noncanonical),
        "test must reach structural or identity validation"
    );
}
#[test]
fn corrupt_and_forged_payloads_fail_closed() {
    let commit = sample_commit();
    let (key, bytes) = codec::encode(&commit, MAX_METADATA_BYTES).unwrap();
    for i in 0..bytes.len() {
        let mut broken = bytes.clone();
        broken[i] ^= 1;
        assert!(
            codec::decode::<b::Commit>(key, &broken, MAX_METADATA_BYTES).is_err(),
            "offset {i}"
        );
    }
    malformed(&commit, |v| {
        v["description"] = "forged".into();
    });
    // Even a valid checksum cannot make a different native object match the key.
    let mut changed = commit.clone();
    changed.description = "other".into();
    let forged =
        envelope::encode(key, &changed.encode_payload().unwrap(), MAX_METADATA_BYTES).unwrap();
    assert!(matches!(
        codec::decode::<b::Commit>(key, &forged, MAX_METADATA_BYTES),
        Err(codec::CodecError::Identity)
    ));
    assert!(codec::decode::<b::Tree>(key, &bytes, MAX_METADATA_BYTES).is_err());
}
#[test]
fn malformed_native_structures_never_panic() {
    let c = sample_commit();
    malformed(&c, |v| {
        v["root_tree"] = serde_json::json!([]);
    });
    malformed(&c, |v| {
        v["root_tree"] = serde_json::json!(["00", "00"]);
    });
    malformed(&c, |v| {
        v["conflict_labels"] = serde_json::json!(["bad"]);
    });
    malformed(&c, |v| {
        v["change_id"] = "aa".into();
    });
    malformed(&c, |v| {
        v["future_field"] = true.into();
    });
    let tree = sample_tree();
    malformed(&tree, |v| {
        v["entries"][0][0] = "a/b".into();
    });
    malformed(&tree, |v| {
        v["entries"][1][0] = "binary".into();
    });
    malformed(&tree, |v| {
        v["entries"][0][1]["type"] = "unsupported".into();
    });
    malformed(&sample_copy(), |v| {
        v["current_path"] = "//bad/path".into();
    });
    malformed(&sample_view(), |v| {
        v["head_ids"] = serde_json::json!([]);
    });
    malformed(&sample_view(), |v| {
        v["head_ids"][1] = v["head_ids"][0].clone();
    });
    malformed(&sample_view(), |v| {
        v["local_bookmarks"]["conflicted"] = serde_json::json!([]);
    });
    malformed(&sample_view(), |v| {
        v["remote_views"]["origin"]["tags"]["tag"]["state"] = "unknown".into();
    });
    // Direct payload decoder structural checks, independent of field ordering.
    assert!(
        b::Tree::decode_payload(
            b"{\"entries\":[[\"bad/path\",{\"type\":\"submodule\",\"id\":[1]}]]}"
        )
        .is_err()
    );
    assert!(NativeSymlink::decode_payload(&[255]).is_err());
    let invalid_key = m::ObjectKey::File(m::FileId::new([0; 64]));
    let forged = envelope::encode(invalid_key, b"payload", 100).unwrap();
    assert!(codec::decode::<NativeFile>(invalid_key, &forged, 100).is_err());
}
#[test]
fn canonical_encoding_and_size_budgets_are_enforced() {
    let op = sample_op();
    let mut data = op.encode_payload().unwrap();
    data.push(b' ');
    assert!(matches!(
        o::Operation::decode_payload(&data),
        Err(codec::CodecError::Noncanonical)
    ));
    let tree = sample_tree();
    assert!(codec::encode(&tree, 1).is_err());
    // JSON duplicate fields cannot be hidden by map overwrite and reserialization.
    assert!(b::Tree::decode_payload(b"{\"entries\":[],\"entries\":[]}").is_err());
    let oversize = vec![b' '; MAX_METADATA_BYTES as usize + 1];
    assert!(matches!(
        b::Tree::decode_payload(&oversize),
        Err(codec::CodecError::Envelope(envelope::EnvelopeError::Limit))
    ));
}

#[tokio::test]
async fn virtual_roots_match_upstream_and_stay_distinct_from_content_ids() {
    use jj_store_jj::roots;
    let dir = tempfile::tempdir().unwrap();
    let backend = SimpleBackend::init(dir.path());
    let ops = tempfile::tempdir().unwrap();
    let store = SimpleOpStore::init(
        ops.path(),
        o::RootOperationData {
            root_commit_id: backend.root_commit_id().clone(),
        },
    )
    .unwrap();
    assert_eq!(
        roots::commit(),
        backend.read_commit(backend.root_commit_id()).await.unwrap()
    );
    assert_eq!(roots::empty_tree_id(), *backend.empty_tree_id());
    assert_eq!(
        roots::operation(),
        store
            .read_operation(store.root_operation_id())
            .await
            .unwrap()
    );
    assert_eq!(
        roots::view(),
        store.read_view(&roots::operation().view_id).await.unwrap()
    );
    assert_ne!(
        roots::commit().key().as_bytes(),
        roots::ROOT_COMMIT.as_bytes()
    );
    assert_ne!(roots::view().key().as_bytes(), roots::ROOT_VIEW.as_bytes());
    assert_ne!(
        roots::operation().key().as_bytes(),
        roots::ROOT_OPERATION.as_bytes()
    );
    roundtrip(roots::commit());
    roundtrip(roots::view());
    roundtrip(roots::operation());
}

#[test]
fn frozen_v1_envelopes_remain_readable() {
    fn record<T: NativeCodec>(value: T) -> String {
        codec::encode(&value, MAX_METADATA_BYTES)
            .unwrap()
            .1
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }
    let corpus = serde_json::json!({"commit":record(sample_commit()),"tree":record(sample_tree()),"copy":record(sample_copy()),"view":record(sample_view()),"operation":record(sample_op()),"file":record(NativeFile(vec![0,1,127,128,255])),"symlink":record(NativeSymlink("../dir/日本語".into()))});
    let historical: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/native-profile/v1-envelopes.json"
    ))
    .unwrap();
    assert_eq!(
        corpus, historical,
        "V1 byte representation changed; introduce a new version instead"
    );
    fn read<T: NativeCodec + std::fmt::Debug>(
        historical: &serde_json::Value,
        name: &str,
        expected: T,
    ) {
        let hex = historical[name].as_str().unwrap();
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        assert_eq!(
            codec::decode::<T>(expected.key(), &bytes, MAX_METADATA_BYTES).unwrap(),
            expected
        );
    }
    read(&historical, "commit", sample_commit());
    read(&historical, "tree", sample_tree());
    read(&historical, "copy", sample_copy());
    read(&historical, "view", sample_view());
    read(&historical, "operation", sample_op());
    read(&historical, "file", NativeFile(vec![0, 1, 127, 128, 255]));
    read(
        &historical,
        "symlink",
        NativeSymlink("../dir/日本語".into()),
    );
}
