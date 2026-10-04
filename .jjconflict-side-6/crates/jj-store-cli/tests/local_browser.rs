#[path = "local_browser/bounded_reads.rs"]
mod bounded_reads;
#[path = "local_browser/code_summary.rs"]
mod code_summary;
#[path = "local_browser/index_tasks.rs"]
mod index_tasks_acceptance;
#[path = "local_browser/managed_code.rs"]
mod managed_code;
#[path = "local_browser/managed_http.rs"]
mod managed_http;
#[path = "local_browser/managed_lifecycle.rs"]
mod managed_lifecycle;
#[path = "local_browser/preparation.rs"]
mod preparation_acceptance;
#[path = "local_browser/prepared_comparisons.rs"]
mod prepared_comparisons;
#[path = "local_browser/prepared_details.rs"]
mod prepared_details;
#[path = "local_browser/prepared_evolution_pages.rs"]
mod prepared_evolution_pages;
#[path = "local_browser/prepared_log_pages.rs"]
mod prepared_log_pages;
#[path = "local_browser/prepared_operation_admission.rs"]
mod prepared_operation_admission;
#[path = "local_browser/prepared_operation_history.rs"]
mod prepared_operation_history;
#[path = "local_browser/prepared_operations.rs"]
mod prepared_operations;
#[path = "local_browser/prepared_points.rs"]
mod prepared_points;
#[path = "local_browser/prepared_points_http.rs"]
mod prepared_points_http;
#[path = "local_browser/prepared_references.rs"]
mod prepared_references;
#[path = "local_browser/reference_embedded.rs"]
mod reference_embedded;
#[path = "local_browser/reviews.rs"]
mod review_acceptance;
#[path = "local_browser/scoped_differences.rs"]
mod scoped_differences;
#[path = "local_browser/stacks.rs"]
mod stacks;
#[path = "local_browser/working_copy.rs"]
mod working_copy_acceptance;
use jj_store_cli::local_browser::LocalFolders;
use jj_store_service::local_browser::FolderProvider;
use std::{fs, process::Command};

fn jj(root: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_jj"))
        .args([
            "--no-pager",
            "--config",
            "user.name='Local browser test'",
            "--config",
            "user.email='browser@example.invalid'",
        ])
        .args(args)
        .current_dir(root)
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

#[test]
fn local_folder_inspection_preserves_uninitialized_metadata_and_dirty_files() {
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    fs::write(folder.join("dirty.txt"), "unsnapshotted").unwrap();
    let provider = LocalFolders;
    let inspected = provider.inspect(folder.to_str().unwrap()).unwrap();
    assert!(inspected.repository.is_none());
    assert!(!folder.join(".jj/repo/lapis").exists());
    assert_eq!(
        fs::read_to_string(folder.join("dirty.txt")).unwrap(),
        "unsnapshotted"
    );
    assert_eq!(
        provider.inspect("relative"),
        Err(jj_store_service::local_browser::Error::Invalid)
    );
}

#[tokio::test]
async fn registration_replays_across_restart_and_removal_preserves_metadata() {
    use jj_store_server::{
        installation::{Identity, initialize_registry},
        local_repositories::DeviceRepositories,
        registry::Registry,
    };
    use jj_store_service::local_browser::*;
    use std::sync::Arc;
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    fs::write(folder.join("dirty.txt"), "unsnapshotted").unwrap();
    let installed = initialize_registry(&temp.path().join("state")).unwrap();
    let identity = Identity::load(&installed.installation_file).unwrap();
    let native = Arc::new(Registry::new(installed.catalog.clone(), identity.clone()).unwrap());
    let make = || {
        DeviceRepositories::new(
            installed.state_dir.clone(),
            identity.clone(),
            Arc::new(LocalFolders),
            native.clone(),
        )
        .unwrap()
    };
    let manager = make();
    let request = RegisterFolder {
        version: 1,
        request_id: jj_store_model::RequestId::new([1; 16]),
        expected_revision: 0,
        path: folder.to_str().unwrap().into(),
        name: "Local demo".into(),
    };
    let metadata_lock = folder.join(".git/refs/lapis/state/v1.lock");
    fs::create_dir_all(metadata_lock.parent().unwrap()).unwrap();
    fs::write(&metadata_lock, b"fixture interruption").unwrap();
    assert!(matches!(
        manager.register(request.clone()).await.unwrap(),
        RegistrationOutcome::OutcomeUnknown { .. }
    ));
    assert!(manager.list().unwrap().bindings.is_empty());
    let journal: serde_json::Value = serde_json::from_slice(
        &fs::read(installed.state_dir.join("local-repositories.json")).unwrap(),
    )
    .unwrap();
    let proposed = journal["requests"][0]["proposed_repository"].clone();
    fs::remove_file(metadata_lock).unwrap();
    let first = manager.register(request.clone()).await.unwrap();
    let RegistrationOutcome::Committed {
        binding, revision, ..
    } = first
    else {
        panic!("registration must commit")
    };
    assert_eq!(binding.availability, Availability::Ready);
    assert_eq!(serde_json::json!(binding.repository), proposed);
    let selected = LocalFolders
        .open(&LocalFolders.inspect(&request.path).unwrap())
        .unwrap()
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    assert_eq!(make().list().unwrap().revision, revision);
    let replay = make().register(request.clone()).await.unwrap();
    assert!(
        matches!(replay, RegistrationOutcome::Committed { binding: ref b, .. } if b.repository == binding.repository)
    );
    assert!(matches!(
        make().status(request.request_id).await.unwrap(),
        RegistrationOutcome::Committed { .. }
    ));
    let mut wrong = request.clone();
    wrong.name = "Other intent".into();
    assert!(matches!(make().register(wrong).await, Err(Error::Conflict)));
    let copy = temp.path().join("conflicting-copy");
    assert!(
        Command::new("cp")
            .arg("-R")
            .arg(&folder)
            .arg(&copy)
            .status()
            .unwrap()
            .success()
    );
    assert!(matches!(
        make().reconnect(
            binding.repository,
            ReconnectFolder {
                version: 1,
                expected_revision: revision,
                path: copy.to_str().unwrap().into()
            }
        ),
        Err(Error::Conflict)
    ));
    assert!(matches!(
        make()
            .register(RegisterFolder {
                request_id: jj_store_model::RequestId::new([8; 16]),
                expected_revision: revision,
                path: copy.to_str().unwrap().into(),
                ..request.clone()
            })
            .await,
        Err(Error::Conflict)
    ));
    let renamed = temp.path().join("moved");
    fs::rename(&folder, &renamed).unwrap();
    assert_eq!(
        make().list().unwrap().bindings[0].availability,
        Availability::Unavailable
    );
    let reconnected = make()
        .reconnect(
            binding.repository,
            ReconnectFolder {
                version: 1,
                expected_revision: revision,
                path: renamed.to_str().unwrap().into(),
            },
        )
        .unwrap();
    assert_eq!(reconnected.bindings[0].repository, binding.repository);
    assert_eq!(reconnected.bindings[0].availability, Availability::Ready);
    assert!(reconnected.bindings[0].generation > binding.generation);
    assert!(matches!(
        make().reconnect(
            binding.repository,
            ReconnectFolder {
                version: 1,
                expected_revision: revision,
                path: renamed.to_str().unwrap().into()
            }
        ),
        Err(Error::Stale)
    ));
    fs::rename(&renamed, &folder).unwrap();
    let reconnected = make()
        .reconnect(
            binding.repository,
            ReconnectFolder {
                version: 1,
                expected_revision: reconnected.revision,
                path: folder.to_str().unwrap().into(),
            },
        )
        .unwrap();

    make()
        .remove(
            binding.repository,
            RemoveBinding {
                version: 1,
                expected_revision: reconnected.revision,
            },
        )
        .unwrap();
    assert!(make().list().unwrap().bindings.is_empty());
    assert_eq!(
        LocalFolders
            .open(&LocalFolders.inspect(&request.path).unwrap())
            .unwrap()
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected
    );
    assert_eq!(
        fs::read_to_string(folder.join("dirty.txt")).unwrap(),
        "unsnapshotted"
    );
    assert!(!folder.join(".jj/repo/store-connection.json").exists());
}

#[tokio::test]
async fn registration_recovers_after_initialization_and_rejects_identity_replacement() {
    use jj_store_server::{
        installation::{Identity, initialize_registry},
        local_repositories::DeviceRepositories,
        registry::Registry,
    };
    use jj_store_service::local_browser::*;
    use std::sync::Arc;
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    let installed = initialize_registry(&temp.path().join("state")).unwrap();
    let identity = Identity::load(&installed.installation_file).unwrap();
    let native = Arc::new(Registry::new(installed.catalog.clone(), identity.clone()).unwrap());
    let make = || {
        DeviceRepositories::new(
            installed.state_dir.clone(),
            identity.clone(),
            Arc::new(LocalFolders),
            native.clone(),
        )
        .unwrap()
    };
    let request = RegisterFolder {
        version: 1,
        request_id: jj_store_model::RequestId::new([2; 16]),
        expected_revision: 0,
        path: folder.to_str().unwrap().into(),
        name: "Recovery".into(),
    };
    let original = LocalFolders.inspect(&request.path).unwrap();
    let RegistrationOutcome::Committed { binding, .. } =
        make().register(request.clone()).await.unwrap()
    else {
        panic!()
    };
    // Emulate loss of the final registry write after a selected companion receipt.
    let registry = installed.state_dir.join("local-repositories.json");
    let mut state: serde_json::Value =
        serde_json::from_slice(&fs::read(&registry).unwrap()).unwrap();
    state["bindings"] = serde_json::json!([]);
    state["revision"] = serde_json::json!(0);
    state["requests"][0]["folder"] = serde_json::to_value(original).unwrap();
    state["requests"][0]["result"] = serde_json::Value::Null;
    fs::write(&registry, serde_json::to_vec(&state).unwrap()).unwrap();
    assert_eq!(
        state["requests"][0]["proposed_repository"],
        serde_json::json!(binding.repository)
    );
    let before = LocalFolders
        .open(&LocalFolders.inspect(&request.path).unwrap())
        .unwrap()
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    assert!(
        matches!(make().status(request.request_id).await.unwrap(), RegistrationOutcome::Committed {binding:ref b,..} if b.repository == binding.repository)
    );
    let after = LocalFolders
        .open(&LocalFolders.inspect(&request.path).unwrap())
        .unwrap()
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    assert_eq!(before, after);
    let valid_state = fs::read(&registry).unwrap();
    let mut corrupt: serde_json::Value = serde_json::from_slice(&valid_state).unwrap();
    corrupt["requests"][0]["result"]["request_id"] =
        serde_json::json!("99999999999999999999999999999999");
    fs::write(&registry, serde_json::to_vec(&corrupt).unwrap()).unwrap();
    assert!(matches!(make().list(), Err(Error::Unavailable)));
    fs::write(&registry, valid_state).unwrap();
    jj(
        &folder,
        &[
            "workspace",
            "add",
            temp.path().join("linked").to_str().unwrap(),
        ],
    );
    assert_eq!(
        LocalFolders
            .inspect(temp.path().join("linked").to_str().unwrap())
            .unwrap()
            .metadata,
        LocalFolders.inspect(&request.path).unwrap().metadata
    );
    fs::rename(&folder, temp.path().join("original")).unwrap();
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    assert_eq!(
        make().list().unwrap().bindings[0].availability,
        Availability::IdentityChanged
    );
    assert!(matches!(
        make().open(binding.repository),
        Err(Error::IdentityChanged)
    ));
    assert!(!folder.join(".jj/repo/lapis").exists());
}

#[tokio::test]
async fn local_code_reads_pin_operations_and_preserve_repository_state() {
    use jj_store_jj::local_browser::{Access, Executor, Request, Response};
    use jj_store_model as m;
    use std::sync::Arc;
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    fs::write(temp.path().join("hello.txt"), "one\n").unwrap();
    jj(temp.path(), &["describe", "-m", "First"]);
    jj(temp.path(), &["bookmark", "create", "main"]);
    let commit = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let op = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let heads_dir = temp.path().join(".jj/repo/op_heads/heads");
    let heads = fs::read_dir(&heads_dir)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect::<Vec<_>>();
    fs::write(temp.path().join("hello.txt"), "dirty unsnapshotted\n").unwrap();
    fs::write(temp.path().join("untracked.txt"), "untouched").unwrap();
    let access: Arc<dyn Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let reader = Executor::new(2).unwrap();
    let head = reader.heads(access.clone()).await.unwrap();
    assert_eq!(head.heads, vec![op]);
    let Response::Commit(detail) = reader
        .read(access.clone(), op, Request::Commit { commit })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(detail.commit.id, commit);
    assert_eq!(detail.commit.id.to_string().len(), 40);
    let Response::Paths(paths) = reader
        .read(
            access.clone(),
            op,
            Request::Paths(jj_store_jj::local_browser::paths::Request {
                include_last_change: true,
                commit,
                path: "".into(),
                recursive: true,
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let jj_store_jj::local_browser::dto::TreeValue::File { id, .. } =
        paths.entries[0].terms[0].as_ref().unwrap()
    else {
        panic!()
    };
    let Response::File(file) = reader
        .read(access.clone(), op, Request::File { file: *id })
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(file.text.as_deref(), Some("one\n"));
    assert!(
        reader
            .read(
                access.clone(),
                op,
                Request::File {
                    file: m::GitSha1Id::new([9; 20])
                }
            )
            .await
            .is_err()
    );
    let Response::References(refs) = reader
        .read(
            access.clone(),
            op,
            Request::References(jj_store_jj::local_browser::references::Request {
                exact: None,
                operation: op,
                query: "".into(),
                kind: Default::default(),
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(!refs.references.is_empty());
    assert_eq!(
        fs::read_to_string(temp.path().join("hello.txt")).unwrap(),
        "dirty unsnapshotted\n"
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("untracked.txt")).unwrap(),
        "untouched"
    );
    assert_eq!(
        fs::read_dir(heads_dir)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect::<Vec<_>>(),
        heads
    );
    assert!(!temp.path().join(".jj/repo/lapis").exists());
}

struct ReadBinding(std::path::PathBuf);
#[async_trait::async_trait]
impl jj_store_jj::local_browser::Access for ReadBinding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([20; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &std::path::Path {
        &self.0
    }
    async fn authorize(&self) -> jj_store_jj::local_browser::Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn local_code_reads_cover_graphs_diffs_roots_conflicts_and_multiple_heads() {
    use jj_store_jj::local_browser::{self as q, *};
    use jj_store_model as m;
    use std::sync::Arc;
    let temp = tempfile::tempdir().unwrap();
    jj(temp.path(), &["git", "init", "--colocate"]);
    fs::write(temp.path().join("file.txt"), "base\n").unwrap();
    fs::write(temp.path().join("binary.bin"), [1, 0, 2]).unwrap();
    fs::write(temp.path().join("large.txt"), vec![b'x'; 300_000]).unwrap();
    jj(temp.path(), &["describe", "-m", "Base"]);
    jj(temp.path(), &["bookmark", "create", "main"]);
    let base = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["new", "main", "-m", "Left"]);
    fs::write(temp.path().join("file.txt"), "left\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Left"]);
    jj(temp.path(), &["bookmark", "create", "left"]);
    let left = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    jj(temp.path(), &["new", "main", "-m", "Right"]);
    fs::write(temp.path().join("file.txt"), "right\n").unwrap();
    jj(temp.path(), &["describe", "-m", "Right"]);
    jj(temp.path(), &["bookmark", "create", "right"]);
    jj(temp.path(), &["new", "left", "right", "-m", "Merge"]);
    let merge = jj(
        temp.path(),
        &["log", "--no-graph", "-r", "@", "-T", "commit_id"],
    )
    .parse()
    .unwrap();
    let op = jj(
        temp.path(),
        &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
    )
    .parse()
    .unwrap();
    let reader = Executor::new(1).unwrap();
    let access: Arc<dyn Access> = Arc::new(ReadBinding(temp.path().join(".jj/repo")));
    let query = GraphRequest::Log {
        revset: "all()".into(),
        workspace: None,
        user_email: "".into(),
        at_time_millis: 1_700_000_000_000,
    };
    let Response::Graph(page) = reader
        .read(
            access.clone(),
            op,
            Request::Graph(
                query.clone(),
                GraphWindow {
                    limit: 1,
                    after: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(page.next.is_some());
    let Response::Graph(next) = reader
        .read(
            access.clone(),
            op,
            Request::Graph(
                query.clone(),
                GraphWindow {
                    limit: 1,
                    after: page.next,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_ne!(page.data, next.data);
    assert!(
        reader
            .read(
                access.clone(),
                op,
                Request::Graph(
                    query,
                    GraphWindow {
                        limit: 0,
                        after: None
                    }
                )
            )
            .await
            .is_err()
    );
    let Response::Commit(root) = reader
        .read(
            access.clone(),
            op,
            Request::Commit {
                commit: m::GitSha1Id::new([0; 20]),
            },
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(root.object.virtual_object);
    let Response::Diff(diff) = reader
        .read(
            access.clone(),
            op,
            Request::Diff(q::diff::Request {
                from: base,
                to: left,
                path: "file.txt".into(),
                content: true,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(
        matches!(&diff.changes[0].content, q::diff::Content::Text { hunks } if hunks.iter().any(|h| h.after.contains("left")))
    );
    let Response::Paths(paths) = reader
        .read(
            access.clone(),
            op,
            Request::Paths(q::paths::Request {
                include_last_change: true,
                commit: merge,
                path: "".into(),
                recursive: true,
                limit: 10,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    for row in paths.entries {
        if let Some(Some(q::dto::TreeValue::File { id, .. })) = row.terms.first()
            && (row.path == "binary.bin" || row.path == "large.txt")
        {
            let Response::File(file) = reader
                .read(access.clone(), op, Request::File { file: *id })
                .await
                .unwrap()
            else {
                panic!()
            };
            assert!(file.text.is_none());
            if row.path == "binary.bin" {
                assert!(file.binary);
                assert_eq!(file.bytes, 3);
            } else {
                assert!(file.too_large);
                assert_eq!(file.bytes, 300_000);
            }
        }
    }
    let Response::Conflicts(conflicts) = reader
        .read(
            access.clone(),
            op,
            Request::Conflicts(q::conflicts::Request {
                commit: merge,
                path: "".into(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(conflicts.conflicts.len(), 1);
    assert_eq!(conflicts.conflicts[0].terms.len(), 3);
    assert!(matches!(
        conflicts.conflicts[0].editor,
        q::conflicts::Editor::Text { .. }
    ));
    let Response::Graph(evolution) = reader
        .read(
            access.clone(),
            op,
            Request::Graph(
                GraphRequest::Evolution { commit: left },
                GraphWindow {
                    limit: 20,
                    after: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(evolution.data, Nodes::Evolution(nodes) if !nodes.is_empty()));
    let Response::Operation(detail) = reader
        .read(access.clone(), op, Request::Operation { operation: op })
        .await
        .unwrap()
    else {
        panic!()
    };
    let ObjectData::Operation(operation) = detail.operation.data else {
        panic!()
    };
    let Response::OperationDiff(diff) = reader
        .read(
            access.clone(),
            op,
            Request::OperationDiff(q::operations::Request {
                operation: op,
                parent: None,
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(matches!(diff.data, q::operations::Data::Difference(_)));
    let Response::OperationChanges(changes) = reader
        .read(
            access.clone(),
            op,
            Request::OperationChanges(q::operation_changes::Request {
                operation: op,
                parent: operation.parents[0],
                limit: 20,
                anchor: None,
                direction: Default::default(),
            }),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert!(!changes.groups.is_empty());
    // Concurrent explicit-operation commands leave siblings. GET never reconciles them.
    jj(
        temp.path(),
        &[
            "--at-op",
            &op.to_string(),
            "--ignore-working-copy",
            "new",
            "main",
            "-m",
            "Sibling A",
        ],
    );
    jj(
        temp.path(),
        &[
            "--at-op",
            &op.to_string(),
            "--ignore-working-copy",
            "new",
            "main",
            "-m",
            "Sibling B",
        ],
    );
    let before = reader.heads(access.clone()).await.unwrap();
    assert_eq!(before.heads.len(), 2);
    reader
        .read(access.clone(), op, Request::Commit { commit: merge })
        .await
        .unwrap();
    assert_eq!(reader.heads(access).await.unwrap().heads, before.heads);
}

#[tokio::test]
async fn filesystem_runtime_registers_and_reads_without_native_storage() {
    use jj_store_server::{config::Config, installation, process::PreparedServer};
    use serde_json::{Value, json};
    use std::sync::Arc;
    let temp = tempfile::tempdir().unwrap();
    let folder = temp.path().join("repo");
    jj(
        temp.path(),
        &["git", "init", "--colocate", folder.to_str().unwrap()],
    );
    fs::write(folder.join("file.txt"), "filesystem browser\n").unwrap();
    jj(&folder, &["describe", "-m", "Recorded code"]);
    jj(&folder, &["bookmark", "create", "main"]);
    let installed = installation::initialize_filesystem(&temp.path().join("state")).unwrap();
    let mut config = Config::load(&installed.config).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    config.bind = address;
    config.public_url = format!("http://{address}/");
    let prepared = PreparedServer::prepare_with_browser(
        &config,
        true,
        None,
        None,
        None,
        Some(Arc::new(jj_store_cli::local_browser::LocalFolders)),
    )
    .await
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        prepared
            .run(async {
                let _ = stopped.await;
            })
            .await
    });
    let client = reqwest::Client::new();
    let endpoint = format!("http://{address}/v1");
    let read = |path: &str| {
        client
            .get(format!("{endpoint}{path}"))
            .header("x-jj-store-protocol", "1")
    };
    let post = |path: &str, value: Value| {
        client
            .post(format!("{endpoint}{path}"))
            .header("x-jj-store-protocol", "1")
            .json(&value)
    };
    let bootstrap: Value = read("/bootstrap")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        bootstrap["capabilities"]["local_repository_management"],
        true
    );
    assert_eq!(bootstrap["capabilities"]["typed_mutations"], json!([]));
    assert_eq!(
        bootstrap["capabilities"]["conflict_resolution_actions"],
        json!([])
    );
    let inspected: Value = post("/local/repositories/inspect", json!({"path":folder}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(inspected["repository"].is_null());
    assert!(!folder.join(".jj/repo/lapis").exists());
    let request = json!({"version":1,"request_id":"12121212121212121212121212121212","expected_revision":0,"path":folder,"name":"Filesystem demo"});
    let result: Value = post("/local/repositories", request.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(result["status"], "committed");
    let id = result["binding"]["repository"].as_str().unwrap();
    let repos: Value = read("/repositories")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(repos["repositories"][0]["id"], id);
    assert_eq!(repos["repositories"][0]["profile"], "git_sha1_jj045");
    assert_eq!(repos["repositories"][0]["availability"], "ready");
    let snapshot: Value = read(&format!("/local/repos/{id}/snapshot"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    // Ordinary browser startup fans out discovery, WorkStore and code snapshot reads.
    // The installation's admitted request limit bounds waiting for local workers.
    let (first, second, capabilities) = tokio::join!(
        read(&format!("/local/repos/{id}/snapshot")).send(),
        read(&format!("/local/repos/{id}/snapshot")).send(),
        read(&format!("/local/repos/{id}/capabilities")).send()
    );
    assert_eq!(first.unwrap().status(), reqwest::StatusCode::OK);
    assert_eq!(second.unwrap().status(), reqwest::StatusCode::OK);
    assert_eq!(capabilities.unwrap().status(), reqwest::StatusCode::OK);
    let operation = snapshot["heads"][0].as_str().unwrap();
    assert!(!snapshot["metadata_commit"].is_null());
    let query = json!({"operation":operation,"request":{"kind":"graph","query":[{"graph":"log","revset":"all()","workspace":null,"user_email":"","at_time_millis":1700000000000_i64},{"limit":1,"after":null}]},"cursor":null});
    let cold = post(&format!("/local/repos/{id}/query"), query.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(cold.status(), reqwest::StatusCode::CONFLICT);
    let cold: Value = cold.json().await.unwrap();
    assert_eq!(cold["code"], "index_rebuild_required");
    assert_eq!(cold["local_snapshot"]["operation"], operation);
    assert_eq!(cold["local_snapshot"]["repository"], id);
    assert!(cold.get("snapshot").is_none());
    for request in [
        json!({"kind":"commit","query":{"commit":"0000000000000000000000000000000000000000"}}),
        json!({"kind":"graph","query":[{"graph":"evolution","commit":"0000000000000000000000000000000000000000"},{"limit":1,"after":null}]}),
    ] {
        let cold = post(
            &format!("/local/repos/{id}/query"),
            json!({"operation":operation,"request":request,"cursor":null}),
        )
        .send()
        .await
        .unwrap();
        assert_eq!(cold.status(), reqwest::StatusCode::CONFLICT);
        let cold: Value = cold.json().await.unwrap();
        assert_eq!(cold["code"], "index_rebuild_required");
        assert_eq!(cold["local_snapshot"]["operation"], operation);
    }
    assert_eq!(
        post(
            &format!("/local/repos/{id}/index/tasks"),
            json!({"operation":operation,"max_commits":4000000})
        )
        .send()
        .await
        .unwrap()
        .status(),
        reqwest::StatusCode::BAD_REQUEST
    );
    let task: Value = post(
        &format!("/local/repos/{id}/index/tasks"),
        json!({"operation":operation}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(task["snapshot"]["operation"], operation);
    let task_id = task["id"].as_str().unwrap();
    let ready_task = tokio::time::timeout(std::time::Duration::from_secs(15), async {
        loop {
            let value: Value = read(&format!("/local/repos/{id}/index/tasks/{task_id}"))
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .json()
                .await
                .unwrap();
            if value["status"] != "running" && value["status"] != "canceling" {
                break value;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(ready_task["status"], "ready", "{ready_task}");
    let cancel_task: Value = client
        .post(format!(
            "{endpoint}/local/repos/{id}/index/tasks/{task_id}/cancel"
        ))
        .header("x-jj-store-protocol", "1")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(cancel_task["status"], "ready");
    assert_eq!(
        read(&format!(
            "/local/repos/{id}/index/tasks/{}",
            "ff".repeat(16)
        ))
        .send()
        .await
        .unwrap()
        .status(),
        reqwest::StatusCode::GONE
    );
    let page: Value = post(&format!("/local/repos/{id}/query"), query.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(page["snapshot"]["profile"], "git_sha1_jj045");
    assert_eq!(
        page["data"]["data"]["data"]["nodes"][0]["commit"]["id"]
            .as_str()
            .unwrap()
            .len(),
        40
    );
    let selected = &page["data"]["data"]["data"]["nodes"][0]["commit"]["id"];
    let stack_request = json!({"kind":"stack_history","query":{"head":selected,"base":null,"limit":1,"after":null}});
    let stack_page: Value = post(
        &format!("/local/repos/{id}/query"),
        json!({"operation":operation,"request":stack_request,"cursor":null}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(stack_page["snapshot"]["operation"], operation);
    assert_eq!(stack_page["data"]["kind"], "stack_history");
    assert_eq!(stack_page["data"]["data"]["head"], *selected);
    assert_eq!(
        stack_page["data"]["data"]["commits"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let stack_cursor = stack_page["next_cursor"].as_str().unwrap();
    let next_stack_page: Value = post(
        &format!("/local/repos/{id}/query"),
        json!({"operation":operation,"request":stack_request,"cursor":stack_cursor}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_ne!(
        stack_page["data"]["data"]["commits"][0]["commit"]["id"],
        next_stack_page["data"]["data"]["commits"][0]["commit"]["id"]
    );
    let changed_stack_range = json!({"kind":"stack_history","query":{"head":selected,"base":selected,"limit":1,"after":null}});
    assert_eq!(
        post(
            &format!("/local/repos/{id}/query"),
            json!({"operation":operation,"request":changed_stack_range,"cursor":stack_cursor})
        )
        .send()
        .await
        .unwrap()
        .status(),
        reqwest::StatusCode::BAD_REQUEST
    );
    let forged_anchor = json!({"kind":"stack_history","query":{"head":selected,"base":null,"limit":1,"after":selected}});
    assert_eq!(
        post(
            &format!("/local/repos/{id}/query"),
            json!({"operation":operation,"request":forged_anchor,"cursor":null})
        )
        .send()
        .await
        .unwrap()
        .status(),
        reqwest::StatusCode::BAD_REQUEST
    );

    for request in [
        json!({"kind":"commit","query":{"commit":selected}}),
        json!({"kind":"graph","query":[{"graph":"evolution","commit":selected},{"limit":1,"after":null}]}),
    ] {
        let ready: Value = post(
            &format!("/local/repos/{id}/query"),
            json!({"operation":operation,"request":request,"cursor":null}),
        )
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
        assert_eq!(ready["snapshot"]["operation"], operation);
        assert_eq!(ready["data"]["kind"], request["kind"]);
    }
    let mut file_reads = vec![];
    for request in [
        json!({"kind":"commit_summary", "query":{"commit":selected}}),
        json!({"kind":"paths", "query":{"commit":selected,"path":"","recursive":false,
            "include_last_change":false,"limit":10,"anchor":null,"direction":"forward"}}),
    ] {
        let value: Value = post(
            &format!("/local/repos/{id}/query"),
            json!({"operation":operation,"request":request,"cursor":null}),
        )
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
        file_reads.push(value);
    }
    let entry = &file_reads[1]["data"]["data"]["entries"][0];
    assert_eq!(entry["last_change"]["reason"], "not_requested");
    let value: Value = post(&format!("/local/repos/{id}/query"),
        json!({"operation":operation,"request":{"kind":"file_at",
            "query":{"commit":selected,"path":entry["path"],"file":entry["terms"][0]["id"]}},"cursor":null}))
        .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(value["data"]["data"]["text"], "filesystem browser\n");
    file_reads.push(value);
    let cursor = page["next_cursor"].as_str().unwrap();
    let mut next = query.clone();
    next["cursor"] = json!(cursor);
    post(&format!("/local/repos/{id}/query"), next.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    next["request"]["query"][1]["limit"] = json!(2);
    assert_eq!(
        post(&format!("/local/repos/{id}/query"), next)
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let other_folder = temp.path().join("other");
    jj(
        temp.path(),
        &["git", "init", "--colocate", other_folder.to_str().unwrap()],
    );
    let other: Value = post("/local/repositories", json!({"version":1,"request_id":"18181818181818181818181818181818","expected_revision":result["revision"],"path":other_folder,"name":"Other local"})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    let mut cross = query.clone();
    cross["cursor"] = json!(cursor);
    assert_eq!(
        post(
            &format!(
                "/local/repos/{}/query",
                other["binding"]["repository"].as_str().unwrap()
            ),
            cross
        )
        .send()
        .await
        .unwrap()
        .status(),
        400
    );
    let reconnected: Value = post(
        &format!("/local/repositories/{id}/reconnect"),
        json!({"version":1,"expected_revision":other["revision"],"path":folder}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    let rejected = post(&format!("/local/repos/{id}/query"), query)
        .header("Origin", "https://untrusted.invalid")
        .send()
        .await
        .unwrap();
    assert_eq!(rejected.status(), 403);
    assert_eq!(
        read(&format!("/repos/{id}/snapshot"))
            .send()
            .await
            .unwrap()
            .status(),
        501
    );
    assert!(
        !installed
            .state_dir
            .join(installation::CREDENTIALS_FILE)
            .exists()
    );
    assert!(
        !installed
            .state_dir
            .join(installation::VOLUME_DIRECTORY)
            .exists()
    );
    assert!(
        !installed
            .state_dir
            .join(installation::RPC_SOCKET_FILE)
            .exists()
    );
    let replay: Value = post("/local/repositories", request)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(replay["binding"]["repository"], id);
    let work_command = json!({"version":1,"request_id":"13131313131313131313131313131313",
        "command":{"command":"create_project","id":"14141414141414141414141414141414","title":"Browser home","body":"Shared CLI actor"}});
    let work_url = format!("/local/repos/{id}/work/commands");
    let work_outcome: Value = post(&work_url, work_command.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(work_outcome["status"], "committed");
    assert_eq!(work_outcome["receipt"]["actor"], snapshot["actor"]);
    let cli_receipt: Value = serde_json::from_str(&jj(
        &folder,
        &[
            "project",
            "--format",
            "json",
            "status",
            "13131313131313131313131313131313",
        ],
    ))
    .unwrap();
    assert_eq!(cli_receipt, work_outcome);
    // A second CLI publication must not advance a previously selected browser root.
    let cli_project: Value = serde_json::from_str(&jj(
        &folder,
        &[
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            "15151515151515151515151515151515",
            "--title",
            "CLI home",
        ],
    ))
    .unwrap();
    let cli_project_id = cli_project["receipt"]["result"]["project"]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let mut activity_projects = vec!["14".repeat(16), cli_project_id.clone()];
    activity_projects.sort();
    for (request, project) in [("18", "14".repeat(16)), ("19", cli_project_id)] {
        let result: Value = post(&work_url, json!({"version":1,"request_id":request.repeat(16),
            "command":{"command":"create_task","id":request.repeat(16),
            "project":project,"expected_project_version":1,
            "title":"Task activity fixture","body":"private task body","repository":null,"parent":null}}))
            .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
        assert_eq!(result["status"], "committed");
    }
    let work_snapshot: Value = read(&format!("/local/repos/{id}/snapshot"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let metadata = &work_snapshot["metadata_commit"];
    let page_query = json!({"metadata_commit":metadata,"query":{"limit":1}});
    let work_page: Value = post(&format!("/local/repos/{id}/work/page"), page_query.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(work_page["projects"].as_array().unwrap().len(), 1);
    assert!(
        work_page["next_cursor"]
            .as_str()
            .unwrap()
            .starts_with("jlw1_")
    );
    let browse_query =
        json!({"metadata_commit":metadata,"query":{"kind":"projects","limit":1,"q":"home"}});
    let work_browse: Value = post(
        &format!("/local/repos/{id}/work/browse"),
        browse_query.clone(),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(work_browse["total_matches"], 2);
    let activity_query = json!({"metadata_commit":metadata,"query":{"limit":1}});
    let task_activity: Value = post(
        &format!("/local/repos/{id}/work/activity"),
        activity_query.clone(),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(task_activity["total_groups"], 2);
    assert_eq!(task_activity["groups"][0]["tasks"], 1);
    assert!(!task_activity.to_string().contains("private task body"));
    assert!(!task_activity.to_string().contains("Task activity fixture"));
    assert!(
        task_activity["next_cursor"]
            .as_str()
            .unwrap()
            .starts_with("jlw1_")
    );
    let _: Value = serde_json::from_str(&jj(
        &folder,
        &[
            "project",
            "--format",
            "json",
            "create",
            "--request-id",
            "16161616161616161616161616161616",
            "--title",
            "Later home",
        ],
    ))
    .unwrap();
    let mut resume_query = browse_query.clone();
    resume_query["query"]["cursor"] = work_browse["next_cursor"].clone();
    let resumed: Value = post(
        &format!("/local/repos/{id}/work/browse"),
        resume_query.clone(),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(resumed["snapshot"], work_browse["snapshot"]);
    assert_eq!(resumed["total_matches"], 2);
    assert_eq!(resumed["projects"][0]["title"], "CLI home");
    let mut activity_resume = activity_query.clone();
    activity_resume["query"]["cursor"] = task_activity["next_cursor"].clone();
    let task_activity_resumed: Value = post(
        &format!("/local/repos/{id}/work/activity"),
        activity_resume.clone(),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(task_activity_resumed["snapshot"], task_activity["snapshot"]);
    assert_eq!(task_activity_resumed["total_groups"], 2);
    assert_eq!(
        task_activity_resumed["groups"][0]["project"],
        activity_projects[1]
    );
    assert!(task_activity_resumed["next_cursor"].is_null());
    activity_resume["query"]["limit"] = json!(2);
    assert_eq!(
        post(&format!("/local/repos/{id}/work/activity"), activity_resume)
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    assert_eq!(
        post(
            &format!("/local/repos/{id}/work/activity"),
            json!({"metadata_commit":metadata,"query":{"limit":0}})
        )
        .send()
        .await
        .unwrap()
        .status(),
        400
    );
    assert_eq!(
        post(&format!("/local/repos/{id}/work/activity"), activity_query)
            .header("Origin", "https://untrusted.invalid")
            .send()
            .await
            .unwrap()
            .status(),
        403
    );
    resume_query["query"]["q"] = json!("different");
    assert_eq!(
        post(&format!("/local/repos/{id}/work/browse"), resume_query)
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let empty_pin: Value = post(
        &format!("/local/repos/{id}/work/page"),
        json!({"metadata_commit":snapshot["metadata_commit"],"query":{}}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert!(empty_pin["projects"].as_array().unwrap().is_empty());
    let replay_work: Value = post(&work_url, work_command)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(replay_work, work_outcome);
    let work_status: Value = read(&format!("{work_url}/13131313131313131313131313131313"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(work_status, work_outcome);
    let stale = json!({"version":1,"request_id":"17171717171717171717171717171717",
        "command":{"command":"update_project","id":"14141414141414141414141414141414","expected_version":99,"title":"Stale","body":""}});
    assert_eq!(post(&work_url, stale).send().await.unwrap().status(), 409);
    let denied = post(&work_url, json!({}))
        .header("Origin", "https://untrusted.invalid")
        .send()
        .await
        .unwrap();
    assert_eq!(denied.status(), 403);
    assert_eq!(
        std::fs::read_to_string(folder.join("file.txt")).unwrap(),
        "filesystem browser\n"
    );
    // External CLI capture is visible only after choosing its companion selection.
    use jj_store_service::local_repository::*;
    use jj_store_wal::publication::*;
    let local = LocalFolders
        .open(&LocalFolders.inspect(folder.to_str().unwrap()).unwrap())
        .unwrap();
    let values: Vec<Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    local
        .apply_agent_command(&LocalAgentRequest {
            version: 1,
            request_id: jj_store_model::RequestId::new([50; 16]),
            command: LocalAgentCommand::Capture {
                profile: NormalizerProfileV1::codex_0_156_1(),
                batch: Box::new(SanitizedObservationBatchV1 {
                    schema_version: 1,
                    batch_id: CaptureBatchId::new([50; 16]),
                    integration: IntegrationId::Codex,
                    source_instance_id: SourceInstanceId::new([50; 16]),
                    sanitizer_version: "fixture-sanitized@1".into(),
                    representation: SanitizedRepresentation::SanitizedNative,
                    observations: values
                        .into_iter()
                        .enumerate()
                        .map(|(i, value)| SanitizedObservationV1 {
                            observation_id: ObservationId::new([i as u8 + 1; 16]),
                            source_sequence: i as u64 + 1,
                            occurred_at_millis: None,
                            source_kind: SourceKind::Hook,
                            source_trust: SourceTrust::LocalUserHook,
                            payload: SanitizedPayloadV1::new(
                                SanitizedMediaType::Json,
                                &serde_json::to_vec(&value).unwrap(),
                            )
                            .unwrap(),
                        })
                        .collect(),
                }),
            },
        })
        .await
        .unwrap();
    let agent_pin = local.metadata_store().read().unwrap().unwrap().commit();
    let agents: Value = post(
        &format!("/local/repos/{id}/agents/sessions"),
        json!({"metadata_commit":agent_pin,"query":{"limit":20}}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(agents["data"]["sessions"].as_array().unwrap().len(), 1);
    let session_id = &agents["data"]["sessions"][0]["session_id"];
    let detail: Value = post(
        &format!("/local/repos/{id}/agents/session"),
        json!({"metadata_commit":agent_pin,"query":{"session_id":session_id}}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert!(!detail["activity"].as_array().unwrap().is_empty());
    let descriptor = &detail["transcripts"][0];
    let transcript: Value = post(&format!("/local/repos/{id}/agents/transcript"), json!({"metadata_commit":agent_pin,"query":{"session_id":session_id,"transcript_id":descriptor["transcript_id"],"source_instance_id":descriptor["source_instance_id"],"source_generation":descriptor["source_generation"],"limit":10,"max_bytes":1024}})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(
        transcript["data"]["chunks"][0]["content"],
        "Update the parser"
    );
    let relations: Value = post(&format!("/local/repos/{id}/agents/relations"), json!({"metadata_commit":agent_pin,"query":{"subject":{"subject":"session","session_id":session_id},"limit":20}})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    let old: Value = post(
        &format!("/local/repos/{id}/agents/sessions"),
        json!({"metadata_commit":snapshot["metadata_commit"],"query":{"limit":20}}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert!(old["data"]["sessions"].as_array().unwrap().is_empty());
    let event_url = format!("/local/repos/{id}/events");
    assert_eq!(
        read(&event_url)
            .header("Last-Event-ID", "forged")
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let mut stream = read(&event_url)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let ready_frame = String::from_utf8(
        tokio::time::timeout(std::time::Duration::from_secs(5), stream.chunk())
            .await
            .unwrap()
            .unwrap()
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    let ready: Value = serde_json::from_str(
        ready_frame
            .lines()
            .find_map(|line| line.strip_prefix("data: "))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(ready["snapshot"]["metadata_commit"], json!(agent_pin));
    jj(&folder, &["new", "-m", "External code change"]);
    let changed = tokio::time::timeout(std::time::Duration::from_secs(8), async {
        loop {
            let frame = String::from_utf8(stream.chunk().await.unwrap().unwrap().to_vec()).unwrap();
            if let Some(raw) = frame.lines().find_map(|line| line.strip_prefix("data: ")) {
                break serde_json::from_str::<Value>(raw).unwrap();
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(changed["type"], "changed");
    assert_ne!(changed["snapshot"]["heads"], ready["snapshot"]["heads"]);
    assert_eq!(
        changed["snapshot"]["metadata_commit"],
        ready["snapshot"]["metadata_commit"]
    );
    drop(stream);
    let mut review_samples = working_copy_acceptance::qualify(&endpoint, id, &folder).await;
    for (kind, exact) in [
        ("workspaces", None),
        ("workspaces", Some("default")),
        ("bookmarks", Some("missing")),
    ] {
        let response: Value = post(
            &format!("/local/repos/{id}/query"),
            json!({
                "operation":operation, "request":{"kind":"references","query":{
                    "operation":operation,"kind":kind,"query":"","exact":exact,"limit":1,
                    "anchor":null,"direction":"forward"
                }},"cursor":null
            }),
        )
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
        assert_eq!(response["data"]["kind"], "references");
        review_samples.push(
            json!({"path":"/v1/local/repos/{repo}/query","method":"post",
            "status":200,"media":"application/json","data":response}),
        );
    }

    review_samples.extend(file_reads.into_iter().map(|data| {
        json!({
            "path":"/v1/local/repos/{repo}/query", "method":"post", "status":200,
            "media":"application/json", "data":data
        })
    }));
    review_samples.extend(
        review_acceptance::qualify(
            &endpoint,
            id,
            other["binding"]["repository"].as_str().unwrap(),
            &folder,
            snapshot["metadata_commit"].clone(),
        )
        .await,
    );
    let query_info: Value = post(
        &format!("/local/repos/{id}/query"),
        json!({"operation":operation,"request":{"kind":"query_info"},"cursor":null}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert_eq!(query_info["data"]["data"]["jj_version"], "0.45.1");
    let filtered_operations: Value = post(&format!("/local/repos/{id}/query"), json!({"operation":operation,"request":{"kind":"graph","query":[{"graph":"operations_filtered","query":"from:@ snapshot:false"},{"limit":1,"after":null}]},"cursor":null})).send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
    assert_eq!(filtered_operations["data"]["kind"], "graph");
    if let Ok(path) = std::env::var("JJ_STORE_LOCAL_BROWSER_CONTRACT_CAPTURE") {
        let sample = |path: &str, method: &str, data: Value| json!({"path":path,"method":method,"status":200,"media":"application/json","data":data});
        fs::write(
            path,
            serde_json::to_vec(&[vec![
                sample("/v1/bootstrap", "get", bootstrap),
                sample("/v1/local/repositories/inspect", "post", inspected),
                sample("/v1/local/repositories", "post", result),
                sample("/v1/repositories", "get", repos),
                sample("/v1/local/repositories/{repo}/reconnect", "post", reconnected),
                sample("/v1/local/repos/{repo}/snapshot", "get", snapshot),
                json!({"path":"/v1/local/repos/{repo}/query","method":"post","status":409,"media":"application/json","data":cold}),
                sample("/v1/local/repos/{repo}/index/tasks", "post", task),
                sample("/v1/local/repos/{repo}/index/tasks/{task}", "get", ready_task),
                sample("/v1/local/repos/{repo}/index/tasks/{task}/cancel", "post", cancel_task),
                sample("/v1/local/repos/{repo}/query", "post", page),
                sample("/v1/local/repos/{repo}/query", "post", stack_page),
                sample("/v1/local/repos/{repo}/query", "post", next_stack_page),
                sample("/v1/local/repos/{repo}/query", "post", query_info),
                sample("/v1/local/repos/{repo}/query", "post", filtered_operations),
                sample("/v1/local/repos/{repo}/agents/sessions", "post", agents),
                sample("/v1/local/repos/{repo}/agents/session", "post", detail),
                sample("/v1/local/repos/{repo}/agents/transcript", "post", transcript),
                sample("/v1/local/repos/{repo}/agents/relations", "post", relations),
                json!({"path":"/v1/local/repos/{repo}/events","method":"get","status":200,"media":"text/event-stream","data":ready}),
                json!({"path":"/v1/local/repos/{repo}/events","method":"get","status":200,"media":"text/event-stream","data":changed}),
                sample("/v1/local/repos/{repo}/work/page", "post", work_page),
                sample("/v1/local/repos/{repo}/work/browse", "post", work_browse),
                sample("/v1/local/repos/{repo}/work/activity", "post", task_activity),
                sample("/v1/local/repos/{repo}/work/activity", "post", task_activity_resumed),
                sample("/v1/local/repos/{repo}/work/commands", "post", work_outcome),
                sample(
                    "/v1/local/repos/{repo}/work/commands/{request_id}",
                    "get",
                    work_status,
                ),
            ], review_samples].concat())
            .unwrap(),
        )
        .unwrap();
    }
    stop.send(()).unwrap();
    server.await.unwrap().unwrap();
}

#[path = "local_browser/history_query.rs"]
mod history_query_acceptance;
