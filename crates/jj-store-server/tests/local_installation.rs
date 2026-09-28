use jj_store_server::{
    config::{Access, Config, Storage},
    installation::initialize_local,
};
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
fn local_initialization_rejects_checkouts_and_does_not_leave_partial_state() {
    let root = tempfile::tempdir().unwrap();
    let checkout = root.path().join("checkout");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    let state = checkout.join("state");
    assert!(initialize_local(&state, &std::env::current_exe().unwrap()).is_err());
    assert!(!state.exists());
}
