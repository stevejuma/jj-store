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
    let first = manager.register(request.clone()).await.unwrap();
    let RegistrationOutcome::Committed {
        binding, revision, ..
    } = first
    else {
        panic!("registration must commit")
    };
    assert_eq!(binding.availability, Availability::Ready);
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
    let renamed = temp.path().join("moved");
    fs::rename(&folder, &renamed).unwrap();
    assert_eq!(
        make().list().unwrap().bindings[0].availability,
        Availability::Unavailable
    );
    fs::rename(&renamed, &folder).unwrap();
    make()
        .remove(
            binding.repository,
            RemoveBinding {
                version: 1,
                expected_revision: revision,
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
