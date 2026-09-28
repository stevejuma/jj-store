use jj_agent::IntegrationId;
use jj_store_cli::{
    agent_integrations::{AgentIntegrationRegistry, IntegrationSetupV1},
    agent_setup::{SetupRequestV1, audit_installations, setup},
};
use jj_store_model::RepositoryId;
use serde_json::Value;
use std::{fs, path::Path};

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: jj_store_cli::commands::Extension,
}

#[cfg(unix)]
fn executable(path: &Path, version: &str) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::write(path, format!("#!/bin/sh\nprintf '%s\\n' '{version}'\n")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
}

#[cfg(unix)]
fn registry(state: &Path, workspace: &Path, bin: &Path) -> AgentIntegrationRegistry {
    let mut registry =
        AgentIntegrationRegistry::open(state, workspace, RepositoryId::new([7; 16])).unwrap();
    registry.refresh(&[bin.to_path_buf()]).unwrap();
    registry
        .enable(
            &[
                IntegrationId::Codex,
                IntegrationId::Cursor,
                IntegrationId::OpenCode,
            ],
            false,
        )
        .unwrap();
    registry
}

#[cfg(unix)]
#[test]
fn setup_merges_only_owned_hooks_and_preserves_original_backups() {
    let temporary = tempfile::tempdir().unwrap();
    let workspace = temporary.path().join("workspace");
    let state = temporary.path().join("state");
    let bin = temporary.path().join("bin");
    fs::create_dir_all(workspace.join(".codex")).unwrap();
    fs::create_dir_all(workspace.join(".cursor")).unwrap();
    fs::create_dir_all(&bin).unwrap();
    executable(&bin.join("codex"), "codex-cli 0.156.1");
    executable(&bin.join("cursor-agent"), "3.22.7");
    executable(&bin.join("opencode"), "1.18.30");

    let codex_original = br#"{
  "description": "keep me",
  "hooks": {
    "PreToolUse": [{"matcher":"^Bash$","hooks":[{"type":"command","command":"./keep-codex"}]}]
  }
}"#;
    let cursor_original = br#"{
  "version": 1,
  "hooks": {"beforeShellExecution":[{"command":"./keep-cursor"}]}
}"#;
    fs::write(workspace.join(".codex/hooks.json"), codex_original).unwrap();
    fs::write(workspace.join(".cursor/hooks.json"), cursor_original).unwrap();

    let mut registry = registry(&state, &workspace, &bin);
    let preview = setup(
        &mut registry,
        SetupRequestV1 {
            integration: IntegrationId::Codex,
            executable: Path::new("/opt/jj store/bin/jj").into(),
            dry_run: true,
        },
    )
    .unwrap();
    assert!(preview.changed);
    assert!(preview.dry_run);
    assert_eq!(
        fs::read(workspace.join(".codex/hooks.json")).unwrap(),
        codex_original
    );

    for integration in [
        IntegrationId::Codex,
        IntegrationId::Cursor,
        IntegrationId::OpenCode,
    ] {
        let outcome = setup(
            &mut registry,
            SetupRequestV1 {
                integration,
                executable: Path::new("/opt/jj store/bin/jj").into(),
                dry_run: false,
            },
        )
        .unwrap();
        assert!(outcome.changed);
        assert_eq!(outcome.setup, IntegrationSetupV1::Installed);
    }

    let codex: Value =
        serde_json::from_slice(&fs::read(workspace.join(".codex/hooks.json")).unwrap()).unwrap();
    assert_eq!(codex["description"], "keep me");
    assert_eq!(codex["hooks"]["PreToolUse"][0]["matcher"], "^Bash$");
    assert!(
        codex["hooks"]["SessionStart"]
            .to_string()
            .contains("agent hook codex")
    );
    assert!(codex.to_string().contains("'/opt/jj store/bin/jj'"));

    let cursor: Value =
        serde_json::from_slice(&fs::read(workspace.join(".cursor/hooks.json")).unwrap()).unwrap();
    assert_eq!(cursor["version"], 1);
    assert_eq!(
        cursor["hooks"]["beforeShellExecution"][0]["command"],
        "./keep-cursor"
    );
    assert!(
        cursor["hooks"]["sessionStart"]
            .to_string()
            .contains("agent hook cursor")
    );

    let plugin = fs::read_to_string(workspace.join(".opencode/plugins/jj-store-agent.ts")).unwrap();
    assert!(plugin.contains("@opencode-ai/plugin"));
    assert!(plugin.contains("\"agent\", \"hook\", \"opencode\""));
    assert!(plugin.contains("tool.execute.after"));
    assert_eq!(
        fs::read(state.join("install/codex/original.bin")).unwrap(),
        codex_original
    );
    assert_eq!(
        fs::read(state.join("install/cursor/original.bin")).unwrap(),
        cursor_original
    );
}

#[cfg(unix)]
#[test]
fn setup_recovers_owned_state_and_refuses_drifted_configuration() {
    let temporary = tempfile::tempdir().unwrap();
    let workspace = temporary.path().join("workspace");
    let state = temporary.path().join("state");
    let bin = temporary.path().join("bin");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(&bin).unwrap();
    executable(&bin.join("codex"), "0.156.1");
    executable(&bin.join("cursor-agent"), "3.22.7");
    executable(&bin.join("opencode"), "1.18.30");
    let mut registry = registry(&state, &workspace, &bin);
    let request = SetupRequestV1 {
        integration: IntegrationId::Codex,
        executable: Path::new("/usr/local/bin/jj").into(),
        dry_run: false,
    };
    setup(&mut registry, request.clone()).unwrap();
    let target = workspace.join(".codex/hooks.json");
    let owned = fs::read(&target).unwrap();
    drop(registry);

    let mut registry =
        AgentIntegrationRegistry::open(&state, &workspace, RepositoryId::new([7; 16])).unwrap();
    audit_installations(&mut registry).unwrap();
    assert_eq!(
        registry
            .status()
            .integration(IntegrationId::Codex)
            .unwrap()
            .setup,
        IntegrationSetupV1::Installed
    );
    let repeated = setup(&mut registry, request.clone()).unwrap();
    assert!(!repeated.changed);

    fs::write(&target, [owned.as_slice(), b"\n "].concat()).unwrap();
    audit_installations(&mut registry).unwrap();
    assert_eq!(
        registry
            .status()
            .integration(IntegrationId::Codex)
            .unwrap()
            .setup,
        IntegrationSetupV1::Drifted
    );
    let drifted = fs::read(&target).unwrap();
    assert!(setup(&mut registry, request).is_err());
    assert_eq!(fs::read(&target).unwrap(), drifted);
}

#[cfg(unix)]
#[test]
fn setup_rejects_project_configuration_symlinks() {
    use std::os::unix::fs::symlink;
    let temporary = tempfile::tempdir().unwrap();
    let workspace = temporary.path().join("workspace");
    let outside = temporary.path().join("outside");
    let state = temporary.path().join("state");
    let bin = temporary.path().join("bin");
    fs::create_dir_all(&workspace).unwrap();
    fs::create_dir_all(&outside).unwrap();
    fs::create_dir_all(&bin).unwrap();
    symlink(&outside, workspace.join(".codex")).unwrap();
    executable(&bin.join("codex"), "0.156.1");
    executable(&bin.join("cursor-agent"), "3.22.7");
    executable(&bin.join("opencode"), "1.18.30");
    let mut registry = registry(&state, &workspace, &bin);

    assert!(
        setup(
            &mut registry,
            SetupRequestV1 {
                integration: IntegrationId::Codex,
                executable: Path::new("/usr/local/bin/jj").into(),
                dry_run: false,
            },
        )
        .is_err()
    );
    assert!(!outside.join("hooks.json").exists());
    assert!(!state.join("install/codex").exists());
}

#[test]
fn setup_command_selects_one_explicit_integration() {
    use clap::Parser as _;
    let parsed = Cli::try_parse_from(["jj", "agent", "setup", "opencode", "--dry-run"]).unwrap();
    let jj_store_cli::commands::Extension::Agent(args) = parsed.command else {
        panic!("expected agent extension");
    };
    let Some(jj_store_cli::agent_commands::AgentCommand::Setup { agent, dry_run }) = args.command
    else {
        panic!("expected setup command");
    };
    assert_eq!(agent, jj_store_cli::agent_commands::AgentName::Opencode);
    assert!(dry_run);
}
