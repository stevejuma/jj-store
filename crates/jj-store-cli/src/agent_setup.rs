use crate::{
    agent_integrations::{
        AgentDeviceStatusV1, AgentIntegrationRegistry, HarnessSupportV1, IntegrationSetupV1,
        IntegrationStateV1,
    },
    agent_setup_files::{
        digest, install_path, persist_record, read_record, read_target, validate_target_parent,
        write_target,
    },
    agent_setup_templates,
};
use jj_agent::IntegrationId;
use jj_store_jj::operation_stores::AuthorityResult;
use serde::Serialize;
use std::path::{Path, PathBuf};

pub use crate::agent_removal::{RemoveOutcomeV1, RemoveRequestV1, remove};

#[derive(Clone, Debug)]
pub struct SetupRequestV1 {
    pub integration: IntegrationId,
    pub executable: PathBuf,
    pub dry_run: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SetupOutcomeV1 {
    pub integration: IntegrationId,
    pub target: PathBuf,
    pub changed: bool,
    pub dry_run: bool,
    pub setup: IntegrationSetupV1,
}

pub fn setup(
    registry: &mut AgentIntegrationRegistry,
    request: SetupRequestV1,
) -> AuthorityResult<SetupOutcomeV1> {
    require_setup_ready(registry, request.integration)?;
    let target = target_path(&registry.status().workspace, request.integration)?;
    validate_target_parent(&registry.status().workspace, &target)?;
    let current = read_target(&target)?;
    let intended = agent_setup_templates::render(
        request.integration,
        current.bytes.as_deref(),
        &request.executable,
    )?;
    let owned_sha256 = digest(&intended);
    let install = install_path(registry.state_directory(), request.integration)?;
    if install.exists() {
        let record = read_record(&install, request.integration, &target)?;
        if record.owned_sha256 != owned_sha256 {
            registry.set_setup(request.integration, IntegrationSetupV1::Drifted)?;
            return Err("agent setup intent differs from the owned installation".into());
        }
        let current_sha256 = current.bytes.as_deref().map(digest);
        if current_sha256.as_deref() == Some(&record.owned_sha256) {
            registry.set_setup(request.integration, IntegrationSetupV1::Installed)?;
            return Ok(outcome(
                request,
                target,
                false,
                IntegrationSetupV1::Installed,
            ));
        }
        if current_sha256 != record.original_sha256 {
            registry.set_setup(request.integration, IntegrationSetupV1::Drifted)?;
            return Err("agent integration configuration has drifted".into());
        }
        if request.dry_run {
            return Ok(outcome(
                request,
                target,
                true,
                IntegrationSetupV1::NotInstalled,
            ));
        }
        write_target(
            &registry.status().workspace,
            &target,
            &intended,
            current.permissions,
        )?;
        registry.set_setup(request.integration, IntegrationSetupV1::Installed)?;
        return Ok(outcome(
            request,
            target,
            true,
            IntegrationSetupV1::Installed,
        ));
    }
    if request.dry_run {
        return Ok(outcome(
            request,
            target,
            true,
            IntegrationSetupV1::NotInstalled,
        ));
    }
    persist_record(
        registry.state_directory(),
        request.integration,
        &target,
        current.bytes.as_deref(),
        &owned_sha256,
    )?;
    write_target(
        &registry.status().workspace,
        &target,
        &intended,
        current.permissions,
    )?;
    registry.set_setup(request.integration, IntegrationSetupV1::Installed)?;
    Ok(outcome(
        request,
        target,
        true,
        IntegrationSetupV1::Installed,
    ))
}

pub(crate) fn require_setup_ready(
    registry: &AgentIntegrationRegistry,
    integration: IntegrationId,
) -> AuthorityResult<()> {
    let status = registry
        .status()
        .integration(integration)
        .ok_or("unsupported agent integration")?;
    if status.support != HarnessSupportV1::Supported {
        return Err("agent integration version is not supported".into());
    }
    if status.state != IntegrationStateV1::Enabled {
        return Err("agent integration must be enabled before setup".into());
    }
    Ok(())
}

pub fn audit_installations(
    registry: &mut AgentIntegrationRegistry,
) -> AuthorityResult<AgentDeviceStatusV1> {
    for integration in [
        IntegrationId::Codex,
        IntegrationId::Cursor,
        IntegrationId::OpenCode,
    ] {
        let setup = installation_state(registry.status(), registry.state_directory(), integration)?;
        registry.set_setup(integration, setup)?;
    }
    Ok(registry.status().clone())
}

pub(crate) fn installation_state(
    status: &AgentDeviceStatusV1,
    state: &Path,
    integration: IntegrationId,
) -> AuthorityResult<IntegrationSetupV1> {
    let target = target_path(&status.workspace, integration)?;
    validate_target_parent(&status.workspace, &target)?;
    let install = install_path(state, integration)?;
    if !install.exists() {
        return Ok(IntegrationSetupV1::NotInstalled);
    }
    let record = read_record(&install, integration, &target)?;
    let current = read_target(&target)?.bytes;
    Ok(
        if current.as_deref().map(digest).as_deref() == Some(&record.owned_sha256) {
            IntegrationSetupV1::Installed
        } else {
            IntegrationSetupV1::Drifted
        },
    )
}

fn outcome(
    request: SetupRequestV1,
    target: PathBuf,
    changed: bool,
    setup: IntegrationSetupV1,
) -> SetupOutcomeV1 {
    SetupOutcomeV1 {
        integration: request.integration,
        target,
        changed,
        dry_run: request.dry_run,
        setup,
    }
}

pub(crate) fn target_path(
    workspace: &Path,
    integration: IntegrationId,
) -> AuthorityResult<PathBuf> {
    let relative = match integration {
        IntegrationId::Codex => ".codex/hooks.json",
        IntegrationId::Cursor => ".cursor/hooks.json",
        IntegrationId::OpenCode => ".opencode/plugins/jj-store-agent.ts",
        IntegrationId::OwnedController => {
            return Err("owned controller has no harness setup".into());
        }
    };
    Ok(workspace.join(relative))
}
