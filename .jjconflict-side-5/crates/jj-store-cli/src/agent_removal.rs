//! Drift-safe removal of project hooks and plugins owned by jj-store.
use crate::{
    agent_capture_files::sync_dir,
    agent_integrations::{AgentIntegrationRegistry, IntegrationSetupV1, IntegrationStateV1},
    agent_setup::target_path,
    agent_setup_files::{
        digest, install_path, original_bytes, read_record, read_target, recover_removed_record,
        remove_record, validate_target_parent, write_target,
    },
};
use jj_agent::IntegrationId;
use jj_store_jj::operation_stores::AuthorityResult;
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct RemoveRequestV1 {
    pub integration: IntegrationId,
    pub dry_run: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RemoveOutcomeV1 {
    pub integration: IntegrationId,
    pub target: PathBuf,
    pub changed: bool,
    pub dry_run: bool,
    pub setup: IntegrationSetupV1,
}

pub fn remove(
    registry: &mut AgentIntegrationRegistry,
    request: RemoveRequestV1,
) -> AuthorityResult<RemoveOutcomeV1> {
    let target = target_path(&registry.status().workspace, request.integration)?;
    recover_removed_record(registry.state_directory(), request.integration)?;
    let install = install_path(registry.state_directory(), request.integration)?;
    if !install.exists() {
        let status = registry
            .status()
            .integration(request.integration)
            .ok_or("unsupported agent integration")?;
        let changed = status.state != IntegrationStateV1::Disabled
            || status.setup != IntegrationSetupV1::NotInstalled;
        if !request.dry_run && changed {
            registry.set_removed(request.integration)?;
        }
        return Ok(outcome(request, target, changed));
    }

    validate_target_parent(&registry.status().workspace, &target)?;
    let record = read_record(&install, request.integration, &target)?;
    let current = read_target(&target)?;
    let current_sha256 = current.bytes.as_deref().map(digest);
    let matches_owned = current_sha256.as_deref() == Some(record.owned_sha256.as_str());
    let matches_original = current_sha256 == record.original_sha256;
    if !matches_owned && !matches_original {
        if !request.dry_run {
            registry.set_setup(request.integration, IntegrationSetupV1::Drifted)?;
        }
        return Err("agent integration configuration has drifted".into());
    }
    if request.dry_run {
        return Ok(outcome(request, target, true));
    }
    if matches_owned {
        match original_bytes(&install, &record)? {
            Some(original) => write_target(
                &registry.status().workspace,
                &target,
                &original,
                current.permissions,
            )?,
            None => {
                fs::remove_file(&target)?;
                sync_dir(target.parent().ok_or("agent setup target has no parent")?)?;
            }
        }
    }
    remove_record(registry.state_directory(), request.integration)?;
    registry.set_removed(request.integration)?;
    Ok(outcome(request, target, true))
}

fn outcome(request: RemoveRequestV1, target: PathBuf, changed: bool) -> RemoveOutcomeV1 {
    RemoveOutcomeV1 {
        integration: request.integration,
        target,
        changed,
        dry_run: request.dry_run,
        setup: IntegrationSetupV1::NotInstalled,
    }
}
