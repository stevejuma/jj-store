use jj_store_server::config::{Access, Config, EXAMPLE};
use jj_store_service::auth::Permission;

#[test]
fn documented_remote_agents_require_explicit_authenticated_allowlists() {
    let source = include_str!("fixtures/server-config.md");
    let snippet = source
        .split_once("## Authenticated remote agents")
        .unwrap()
        .1
        .split_once("```toml\n")
        .unwrap()
        .1
        .split_once("\n```")
        .unwrap()
        .0;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("server.toml");
    let load = |text: &str| {
        std::fs::write(&path, text).unwrap();
        Config::load(&path)
    };
    let mut config = load(&format!("{EXAMPLE}\n{snippet}")).unwrap();
    let remote = config.remote_agents.as_ref().unwrap();
    assert_eq!(remote.agents.len(), 1);
    assert_eq!(remote.agents[0].setup.id, "builder");
    assert_eq!(remote.agents[0].principals.len(), 1);
    assert!(!config.state_dir.exists());
    for invalid in [
        snippet.replace("11111111111111111111111111111111", "not-a-principal"),
        snippet.replace("[\"11111111111111111111111111111111\"]", "[]"),
        snippet.replace("inherit_defaults = false", "inherit_defaults = true"),
        snippet.replace("runner = \"codex\"", "runner = \"arbitrary-executable\""),
    ] {
        assert!(load(&format!("{EXAMPLE}\n{invalid}")).is_err());
    }
    config.access = Access::LocalTrusted {
        installation: root.path().join("installation.json"),
        catalog: root.path().join("repositories.json"),
        rpc_socket: root.path().join("native.sock"),
        permission: Permission::Admin,
    };
    assert!(config.validate().is_err());
    let disabled = load(EXAMPLE).unwrap();
    assert!(disabled.remote_agents.is_none());
}
