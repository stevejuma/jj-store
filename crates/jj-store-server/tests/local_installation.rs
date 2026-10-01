use jj_store_server::{
    config::{Access, Config, Storage},
    installation::{Catalog, Identity, initialize_local, initialize_registry},
    registry::Registry,
};
use jj_store_service::registry::RepositoryRegistry as _;
use std::fs;

#[test]
fn local_initialization_is_atomic_private_and_restart_stable() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local-state");
    let runtime = std::env::current_exe().unwrap();
    let initialized = initialize_local(&state, &runtime).unwrap();
    let state = state.canonicalize().unwrap();
    assert_eq!(initialized.config, state.join("server.toml"));
    assert_eq!(initialized.state_dir, state);
    assert!(initialize_local(&state, &runtime).is_err());

    let config = Config::load(&initialized.config).unwrap();
    assert_eq!(config.format, 2);
    assert!(matches!(config.access, Access::LocalTrusted { .. }));
    assert!(matches!(config.storage, Storage::EmbeddedRustfs { .. }));
    assert!(config.backup.is_empty());
    assert!(
        !fs::read_to_string(&initialized.config)
            .unwrap()
            .contains("secret_key")
    );

    for name in [
        "installation.json",
        "repositories.json",
        "rustfs-credentials.json",
        "storage-owner.json",
        "operators.json",
    ] {
        assert!(state.join(name).is_file(), "missing {name}");
    }
    assert!(state.join("rustfs").is_dir());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(
            fs::metadata(&state).unwrap().permissions().mode() & 0o077,
            0
        );
        assert_eq!(
            fs::metadata(state.join("rustfs-credentials.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o077,
            0
        );
    }
}

#[test]
fn authenticated_registry_initialization_is_private_empty_and_atomic() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("authenticated-state");
    let initialized = initialize_registry(&state).unwrap();
    assert_eq!(initialized.state_dir, state.canonicalize().unwrap());
    let identity = Identity::load(&initialized.installation_file).unwrap();
    assert_eq!(identity.installation, initialized.installation);
    assert!(
        Catalog::load(&initialized.catalog, &identity)
            .unwrap()
            .repositories
            .is_empty()
    );
    assert!(initialize_registry(&state).is_err());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(
            fs::metadata(&state).unwrap().permissions().mode() & 0o077,
            0
        );
        assert_eq!(
            fs::metadata(&initialized.catalog)
                .unwrap()
                .permissions()
                .mode()
                & 0o077,
            0
        );
    }
}

#[test]
fn local_catalog_is_identity_bound_sorted_and_atomically_repairable() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local-state");
    let initialized = initialize_local(&state, &std::env::current_exe().unwrap()).unwrap();
    let identity = Identity::load(&state.join("installation.json")).unwrap();
    let path = state.join("repositories.json");
    assert!(
        Catalog::load(&path, &identity)
            .unwrap()
            .repositories
            .is_empty()
    );
    let high = jj_store_model::RepositoryId::new([9; 16]);
    let low = jj_store_model::RepositoryId::new([1; 16]);
    Catalog::register(&path, &identity, high, "lapis/high".into()).unwrap();
    let catalog = Catalog::register(&path, &identity, low, "lapis/low".into()).unwrap();
    assert_eq!(catalog.repositories[0].id, low);
    assert_eq!(catalog.repositories[1].id, high);
    let catalog = Catalog::register(&path, &identity, low, "lapis/renamed".into()).unwrap();
    assert_eq!(catalog.repositories.len(), 2);
    assert_eq!(catalog.repositories[0].name, "lapis/renamed");
    assert!(Catalog::register(&path, &identity, high, "lapis/renamed".into()).is_err());
    for invalid in [
        " /repo",
        "/repo",
        "repo/",
        "repo//child",
        "repo/../child",
        "repo\\child",
    ] {
        assert!(Catalog::register(&path, &identity, high, invalid.into()).is_err());
    }
    let other = Identity {
        version: 1,
        installation: "00".repeat(16),
    };
    assert!(Catalog::load(&path, &other).is_err());
    assert!(Catalog::register(&path, &identity, low, "\n".into()).is_err());
    assert!(initialized.config.is_file());
}

#[tokio::test]
async fn registry_indexes_token_candidates_without_conferring_access() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local-state");
    initialize_local(&state, &std::env::current_exe().unwrap()).unwrap();
    let identity = Identity::load(&state.join("installation.json")).unwrap();
    let path = state.join("repositories.json");
    let first = jj_store_model::RepositoryId::new([1; 16]);
    let second = jj_store_model::RepositoryId::new([2; 16]);
    Catalog::register(&path, &identity, first, "lapis/first".into()).unwrap();
    Catalog::register(&path, &identity, second, "lapis/second".into()).unwrap();
    let registry = Registry::new(path, identity).unwrap();
    let token = jj_store_model::TokenId::new([3; 16]);
    registry.token_candidate(first, token, true).await.unwrap();
    registry.token_candidate(second, token, true).await.unwrap();
    let candidates = registry.snapshot(Some(token)).await.unwrap();
    assert_eq!(
        candidates
            .repositories
            .iter()
            .map(|entry| entry.id)
            .collect::<Vec<_>>(),
        vec![first, second]
    );
    registry.token_candidate(first, token, false).await.unwrap();
    assert_eq!(
        registry.snapshot(Some(token)).await.unwrap().repositories[0].id,
        second
    );
    assert_eq!(registry.snapshot(None).await.unwrap().repositories.len(), 2);
}

#[tokio::test]
async fn registry_reserves_aliases_before_exposing_published_repositories() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("local-state");
    initialize_local(&state, &std::env::current_exe().unwrap()).unwrap();
    let identity = Identity::load(&state.join("installation.json")).unwrap();
    let path = state.join("repositories.json");
    let registry = Registry::new(path, identity).unwrap();
    let pending = jj_store_model::RepositoryId::new([31; 16]);
    let conflict = jj_store_model::RepositoryId::new([32; 16]);

    registry
        .reserve(pending, "lapis/reserved".into())
        .await
        .unwrap();
    registry
        .reserve(pending, "lapis/reserved".into())
        .await
        .unwrap();
    assert!(
        registry
            .snapshot(None)
            .await
            .unwrap()
            .repositories
            .is_empty()
    );
    assert!(
        registry
            .reserve(conflict, "lapis/reserved".into())
            .await
            .is_err()
    );
    assert!(
        registry
            .reserve(pending, "lapis/changed".into())
            .await
            .is_err()
    );

    registry
        .register(
            pending,
            "lapis/reserved".into(),
            Some(jj_store_model::TokenId::new([33; 16])),
        )
        .await
        .unwrap();
    let visible = registry.snapshot(None).await.unwrap();
    assert_eq!(visible.repositories.len(), 1);
    assert_eq!(visible.repositories[0].name, "lapis/reserved");
    assert_eq!(
        registry
            .snapshot(Some(jj_store_model::TokenId::new([33; 16])))
            .await
            .unwrap()
            .repositories[0]
            .id,
        pending
    );
}

#[test]
fn local_initialization_rejects_checkouts_and_does_not_leave_partial_state() {
    let root = tempfile::tempdir().unwrap();
    let checkout = root.path().join("checkout");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    let state = checkout.join("state");
    assert!(initialize_local(&state, &std::env::current_exe().unwrap()).is_err());
    assert!(!state.exists());
}

#[test]
fn filesystem_initialization_is_private_and_rejects_nonlocal_authority() {
    let root = tempfile::tempdir().unwrap();
    let state = root.path().join("filesystem");
    let initialized = jj_store_server::installation::initialize_filesystem(&state).unwrap();
    let mut config = Config::load(&initialized.config).unwrap();
    assert!(matches!(config.storage, Storage::Filesystem));
    assert!(matches!(config.access, Access::LocalTrusted { .. }));
    assert!(!state.join("rustfs").exists());
    assert!(!state.join("rustfs-credentials.json").exists());
    assert!(!state.join("storage-owner.json").exists());
    assert!(!state.join("native.sock").exists());
    assert!(jj_store_server::installation::initialize_filesystem(&state).is_err());
    config.bind = "0.0.0.0:7443".parse().unwrap();
    assert!(config.validate().is_err());
    config.bind = "127.0.0.1:7443".parse().unwrap();
    let Access::LocalTrusted {
        ref mut permission, ..
    } = config.access
    else {
        panic!()
    };
    *permission = jj_store_service::auth::Permission::Read;
    assert!(config.validate().is_err());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        for file in [
            "installation.json",
            "repositories.json",
            "operators.json",
            "server.toml",
        ] {
            assert_eq!(
                fs::metadata(state.join(file)).unwrap().permissions().mode() & 0o077,
                0
            );
        }
    }
}
