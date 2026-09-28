use jj_agent::{DeviceId, IntegrationId};
use jj_store_cli::agent_integrations::{
    AgentIntegrationRegistry, HarnessSupportV1, IntegrationStateV1,
};
use jj_store_model::RepositoryId;
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
#[test]
fn registry_probes_pinned_harnesses_and_persists_explicit_enablement() {
    let temporary = tempfile::tempdir().unwrap();
    let bin = temporary.path().join("bin");
    let state = temporary.path().join("state");
    let workspace = temporary.path().join("workspace");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&workspace).unwrap();
    executable(&bin.join("codex"), "codex-cli 0.156.1");
    executable(&bin.join("cursor-agent"), "2026.09.26-dd393fe");
    executable(&bin.join("opencode"), "1.18.30");
    let repository = RepositoryId::new([1; 16]);

    let mut registry = AgentIntegrationRegistry::open(&state, &workspace, repository).unwrap();
    let device = registry.status().device_id;
    assert_ne!(device, DeviceId::new([0; 16]));
    let status = registry.refresh(std::slice::from_ref(&bin)).unwrap();
    assert_eq!(
        status.integration(IntegrationId::Codex).unwrap().support,
        HarnessSupportV1::Supported
    );
    assert_eq!(
        status.integration(IntegrationId::Cursor).unwrap().support,
        HarnessSupportV1::UnsupportedVersion
    );
    assert_eq!(
        status.integration(IntegrationId::OpenCode).unwrap().support,
        HarnessSupportV1::Supported
    );

    let preview = registry
        .enable(&[IntegrationId::Codex, IntegrationId::OpenCode], true)
        .unwrap();
    assert_eq!(
        preview.integration(IntegrationId::Codex).unwrap().state,
        IntegrationStateV1::Enabled
    );
    assert_eq!(
        registry
            .status()
            .integration(IntegrationId::Codex)
            .unwrap()
            .state,
        IntegrationStateV1::Disabled
    );
    registry
        .enable(&[IntegrationId::Codex, IntegrationId::OpenCode], false)
        .unwrap();
    assert!(registry.enable(&[IntegrationId::Cursor], false).is_err());
    drop(registry);

    let registry = AgentIntegrationRegistry::open(&state, &workspace, repository).unwrap();
    assert_eq!(registry.status().device_id, device);
    assert_eq!(
        registry
            .status()
            .integration(IntegrationId::Codex)
            .unwrap()
            .state,
        IntegrationStateV1::Enabled
    );
    drop(registry);
    assert!(
        AgentIntegrationRegistry::open(&state, &workspace, RepositoryId::new([2; 16])).is_err()
    );
}

#[cfg(unix)]
#[test]
fn registry_reports_missing_and_failed_probes_without_forwarding_output() {
    let temporary = tempfile::tempdir().unwrap();
    let bin = temporary.path().join("bin");
    let state = temporary.path().join("state");
    let workspace = temporary.path().join("workspace");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&workspace).unwrap();
    fs::write(
        bin.join("codex"),
        "#!/bin/sh\nprintf 'synthetic-secret' >&2\nexit 9\n",
    )
    .unwrap();
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(bin.join("codex"), fs::Permissions::from_mode(0o700)).unwrap();

    let mut registry =
        AgentIntegrationRegistry::open(&state, &workspace, RepositoryId::new([3; 16])).unwrap();
    let status = registry.refresh(&[bin]).unwrap();
    let codex = status.integration(IntegrationId::Codex).unwrap();
    assert_eq!(codex.support, HarnessSupportV1::ProbeFailed);
    assert!(
        !serde_json::to_string(codex)
            .unwrap()
            .contains("synthetic-secret")
    );
    assert_eq!(
        status.integration(IntegrationId::OpenCode).unwrap().support,
        HarnessSupportV1::Absent
    );
}

#[test]
fn agent_command_requires_explicit_noninteractive_enablement() {
    use clap::Parser as _;
    let parsed = Cli::try_parse_from([
        "jj",
        "agent",
        "enable",
        "--non-interactive",
        "--agent",
        "codex",
        "--agent",
        "opencode",
        "--dry-run",
    ])
    .unwrap();
    let jj_store_cli::commands::Extension::Agent(args) = parsed.command else {
        panic!("expected agent extension");
    };
    let Some(jj_store_cli::agent_commands::AgentCommand::Enable {
        agents,
        dry_run,
        non_interactive,
    }) = args.command
    else {
        panic!("expected enable command");
    };
    assert_eq!(agents.len(), 2);
    assert!(dry_run);
    assert!(non_interactive);
}
