use super::*;

async fn complex(h: &mut History) -> (Arc<CommittedRead>, Vec<b::TreeValue>) {
    let mut files = vec![];
    for bytes in [b"left\r\n".as_slice(), b"base\n", b"right without newline"] {
        files.push(h.file(bytes).await);
    }
    let mut binaries = vec![];
    for byte in [1, 2, 3] {
        binaries.push(h.file(&[0, byte, 255]).await);
    }
    let mut copies = files.clone();
    for (i, file) in copies.iter_mut().enumerate() {
        let history = b::CopyHistory {
            current_path: RepoPathBuf::from_internal_string("copy").unwrap(),
            parents: vec![b::CopyId::placeholder()],
            salt: vec![i as u8],
        };
        let key = h.f.objects.put(&history).await.unwrap().0;
        let b::TreeValue::File { copy_id, .. } = file else {
            unreachable!()
        };
        *copy_id = b::CopyId::from_bytes(key.as_bytes());
    }
    let symkey =
        h.f.objects
            .put(&NativeSymlink("<script>".into()))
            .await
            .unwrap()
            .0;
    let symlink = b::TreeValue::Symlink(b::SymlinkId::from_bytes(symkey.as_bytes()));
    let mut trees = vec![];
    for i in 0..3 {
        let nested = h.tree(vec![("x", files[i].clone())]).await;
        let mut mode = files[i].clone();
        if i == 0 {
            let b::TreeValue::File { executable, .. } = &mut mode else {
                unreachable!()
            };
            *executable = true;
        }
        let mut entries = vec![
            ("binary", binaries[i].clone()),
            ("copy", copies[i].clone()),
            ("dir", b::TreeValue::Tree(nested.clone())),
            ("mode", mode),
            ("text", files[i].clone()),
            ("trivial", files[0].clone()),
            (
                "structural",
                if i == 0 {
                    b::TreeValue::Tree(nested)
                } else if i == 1 {
                    files[i].clone()
                } else {
                    symlink.clone()
                },
            ),
            (
                "submodule",
                if i == 0 {
                    b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[8; 20]))
                } else if i == 1 {
                    files[i].clone()
                } else {
                    symlink.clone()
                },
            ),
        ];
        if i > 0 {
            entries.push(("delete-modify", files[i].clone()));
        }
        trees.push(h.tree(entries).await);
    }
    (h.publish(trees, vec!["left", "base", "right"]).await, files)
}
fn reason(descriptor: &cf::Descriptor, reason: cf::Unsupported) {
    assert_eq!(descriptor.editor, cf::Editor::Unsupported { reason });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_descriptor_preserves_native_terms_labels_paths_and_editor_capabilities() {
    let mut h = History::new().await;
    let (read, files) = complex(&mut h).await;
    let before = h.f.session.snapshot().await.unwrap().manifest().clone();
    let response = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: String::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(response.conflicts.len(), 8);
    let find = |path: &str| response.conflicts.iter().find(|d| d.path == path).unwrap();
    for (path, why) in [
        ("binary", cf::Unsupported::Binary),
        ("copy", cf::Unsupported::CopyHistory),
        ("mode", cf::Unsupported::ExecutableMode),
        ("delete-modify", cf::Unsupported::DeleteModify),
        ("structural", cf::Unsupported::Structural),
        ("submodule", cf::Unsupported::Structural),
    ] {
        reason(find(path), why);
    }
    assert_eq!(
        find("text").editor,
        cf::Editor::Text {
            contents: [
                "left\r\n".into(),
                "base\n".into(),
                "right without newline".into()
            ],
            executable: false
        }
    );
    for (i, term) in find("text").terms.iter().enumerate() {
        assert_eq!(term.id, i as u32);
        assert_eq!(
            term.role,
            if i % 2 == 0 {
                cf::Role::Add
            } else {
                cf::Role::Remove
            }
        );
        assert_eq!(term.label.as_deref(), Some(["left", "base", "right"][i]));
        assert_eq!(
            term.value,
            Some(dto::TreeValue::try_from(&files[i]).unwrap())
        );
    }
    assert_eq!(find("delete-modify").terms[0].value, None);
    let store = h.native_store();
    let native = store
        .get_commit_async(&b::CommitId::from_bytes(h.commit.as_bytes()))
        .await
        .unwrap()
        .tree();
    let expected = native.conflicts().collect::<Vec<_>>();
    assert_eq!(expected.len(), response.conflicts.len());
    for ((path, value), descriptor) in expected.into_iter().zip(&response.conflicts) {
        assert_eq!(path.as_internal_file_string(), descriptor.path);
        assert_eq!(
            value
                .unwrap()
                .iter()
                .map(|v| v.as_ref().map(|v| dto::TreeValue::try_from(v).unwrap()))
                .collect::<Vec<_>>(),
            descriptor
                .terms
                .iter()
                .map(|t| t.value.clone())
                .collect::<Vec<_>>()
        );
    }
    let filtered = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: "dir".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(filtered.conflicts, vec![find("dir/x").clone()]);
    let diff = read
        .diff(
            &reader(),
            d::Request {
                from: roots::ROOT_COMMIT,
                to: h.commit,
                path: "text".into(),
                content: true,
            },
        )
        .await
        .unwrap();
    unavailable(&diff.changes[0], d::Unavailable::Conflict);
    assert_eq!(*h.f.session.snapshot().await.unwrap().manifest(), before);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_descriptor_multisided_and_fingerprints_bind_every_input() {
    let mut h = History::new().await;
    let mut trees = vec![];
    for bytes in [b"left".as_slice(), b"base", b"right", b"base", b"third"] {
        let file = h.file(bytes).await;
        trees.push(h.tree(vec![("file", file)]).await);
    }
    let read = h
        .publish(trees, vec!["left", "base", "right", "base", "third"])
        .await;
    let response = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: String::new(),
            },
        )
        .await
        .unwrap();
    let descriptor = &response.conflicts[0];
    assert_eq!(descriptor.terms.len(), 5);
    reason(descriptor, cf::Unsupported::MultipleSides);
    let epoch = read.snapshot().await.unwrap().authority.epoch;
    let fingerprint = cf::fingerprint(
        REPO,
        epoch,
        h.commit,
        response.change,
        &descriptor.path,
        &descriptor.terms,
    )
    .unwrap();
    assert_eq!(fingerprint, descriptor.fingerprint);
    assert_eq!(
        serde_json::from_str::<m::ConflictFingerprint>(
            &serde_json::to_string(&fingerprint).unwrap()
        )
        .unwrap(),
        fingerprint
    );
    for (repo, epoch, commit, change, path) in [
        (
            m::RepositoryId::new([11; 16]),
            epoch,
            h.commit,
            response.change,
            "file",
        ),
        (
            REPO,
            m::EpochId::new([12; 16]),
            h.commit,
            response.change,
            "file",
        ),
        (REPO, epoch, roots::ROOT_COMMIT, response.change, "file"),
        (REPO, epoch, h.commit, m::ChangeId::new([13; 16]), "file"),
        (REPO, epoch, h.commit, response.change, "another"),
    ] {
        assert_ne!(
            cf::fingerprint(repo, epoch, commit, change, path, &descriptor.terms).unwrap(),
            fingerprint
        );
    }
    for field in 0..5 {
        let mut terms = descriptor.terms.clone();
        match field {
            0 => terms.swap(0, 2),
            1 => terms[0].label = Some("changed".into()),
            2 => terms[0].id += 1,
            3 => terms[0].role = cf::Role::Remove,
            _ => terms[0].value = None,
        };
        assert_ne!(
            cf::fingerprint(
                REPO,
                epoch,
                h.commit,
                response.change,
                &descriptor.path,
                &terms
            )
            .unwrap(),
            fingerprint
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_descriptor_limits_revocation_and_verified_content_fail_closed() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let mut h = History::new().await;
    let (read, files) = complex(&mut h).await;
    let request = cf::Request {
        commit: h.commit,
        path: String::new(),
    };
    for limits in [
        c::Limits {
            max_changes: 1,
            ..c::Limits::default()
        },
        c::Limits {
            max_output_bytes: 1,
            ..c::Limits::default()
        },
        c::Limits {
            max_read_bytes: 1,
            ..c::Limits::default()
        },
    ] {
        assert_eq!(
            read.conflicts(&ContentReader::new(limits, 1).unwrap(), request.clone())
                .await
                .unwrap_err(),
            q::Error::Limit
        );
    }
    let response = read
        .conflicts(
            &ContentReader::new(
                c::Limits {
                    max_text_bytes: 1,
                    ..c::Limits::default()
                },
                1,
            )
            .unwrap(),
            cf::Request {
                path: "text".into(),
                ..request.clone()
            },
        )
        .await
        .unwrap();
    reason(&response.conflicts[0], cf::Unsupported::TooLarge);
    let session = h.f.session(h.f.reader.clone(), Protection::default());
    let OpenRead::Pinned(readonly) = session
        .open_read(None, ReadLimits::default())
        .await
        .unwrap()
    else {
        unreachable!()
    };
    let readonly = Arc::new(readonly);
    h.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([214; 16]),
            expected: h.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke {
                token: h.f.reader.id(),
            },
        })
        .await
        .unwrap();
    assert_eq!(
        readonly
            .conflicts(&reader(), request.clone())
            .await
            .unwrap_err(),
        q::Error::Denied
    );
    let b::TreeValue::File { id, .. } = &files[0] else {
        unreachable!()
    };
    let key = Key::new(
        m::ObjectKey::File(m::FileId::from_slice(id.as_bytes()).unwrap()).storage_path(REPO),
    )
    .unwrap();
    let filtered = cf::Request {
        path: "text".into(),
        ..request
    };
    assert_eq!(
        reader()
            .conflicts(
                Arc::new(MissingNative {
                    inner: h.f.store.clone(),
                    missing: key.clone()
                }),
                read.clone(),
                filtered.clone()
            )
            .await
            .unwrap_err(),
        q::Error::Incomplete
    );
    let old = h.f.store.get(&key, None).await.unwrap();
    let version = old.metadata.version.clone();
    let mut bytes = old.read_bounded(1_000_000).await.unwrap();
    *bytes.last_mut().unwrap() ^= 1;
    let upload = StagedUpload::from_bytes(&bytes, 1_000_000).await.unwrap();
    h.f.store
        .put(&key, Condition::Matches(version), &upload)
        .await
        .unwrap();
    assert_eq!(
        read.conflicts(&reader(), filtered).await.unwrap_err(),
        q::Error::Corrupt
    );
}

#[path = "resolve.rs"]
mod resolve_acceptance;
