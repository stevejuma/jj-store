use jj_store_desktop::{installation::Resources, runtime::ManagedService};
use std::{net::TcpListener, path::PathBuf};

#[test]
#[ignore = "requires explicitly staged production binaries and web assets"]
fn real_managed_runtime_restarts_preserves_origin_and_rejects_occupied_port() {
    let resource_root = PathBuf::from(
        std::env::var_os("JJ_STORE_DESKTOP_TEST_RESOURCES").expect("explicit resources"),
    );
    let resources = Resources::from_root(&resource_root).unwrap();
    let state = tempfile::tempdir().unwrap();
    let mut first = ManagedService::start(state.path(), &resources).unwrap();
    let origin = first.origin.clone();
    let identity = std::fs::read(state.path().join("installation/installation.json")).unwrap();
    let config = std::fs::read(state.path().join("installation/server.toml")).unwrap();
    assert!(first.running().unwrap());
    first.stop().unwrap();
    let occupied = TcpListener::bind(("127.0.0.1", origin.port().unwrap())).unwrap();
    assert!(ManagedService::start(state.path(), &resources).is_err());
    drop(occupied);
    let mut second = ManagedService::start(state.path(), &resources).unwrap();
    assert_eq!(second.origin, origin);
    second.stop().unwrap();
    assert_eq!(
        std::fs::read(state.path().join("installation/installation.json")).unwrap(),
        identity
    );
    assert_eq!(
        std::fs::read(state.path().join("installation/server.toml")).unwrap(),
        config
    );
    assert!(!state.path().join("installation/native.sock").exists());
}
