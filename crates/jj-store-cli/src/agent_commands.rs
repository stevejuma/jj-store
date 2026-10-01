use crate::{
    agent_integrations::{AgentDeviceStatusV1, AgentIntegrationRegistry},
    agent_sanitization::warm_kingfisher_cache,
    agent_setup::{
        RemoveRequestV1, SetupRequestV1, audit_installations, remove, require_setup_ready, setup,
    },
    commands::{Format, error},
    presentation,
};
use jj_agent::IntegrationId;
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};
mod backend;
mod provenance;
mod types;
use backend::{Backend, default_state, workspace_root};
pub use types::{AgentArgs, AgentCommand, AgentName};

pub async fn run(ui: &mut Ui, helper: &CommandHelper, args: AgentArgs) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["agent".into()]);
    };
    let backend = Backend::open(helper)?;
    let workspace = workspace_root(helper.cwd()).map_err(error)?;
    if backend.repository()?.is_none()
        && matches!(command, AgentCommand::Status | AgentCommand::Enable { .. })
    {
        let mut value = backend.uninitialized_status()?;
        if let AgentCommand::Enable {
            agents,
            non_interactive,
            ..
        } = &command
        {
            if !non_interactive || agents.is_empty() || agents.len() > 3 {
                return Err(error("agent enable requires --non-interactive"));
            }
            for agent in agents {
                let key = serde_json::to_value(IntegrationId::from(*agent)).map_err(error)?;
                let status = &mut value["integrations"]
                    [key.as_str().ok_or_else(|| error("integration identity"))?];
                if status["support"] != "supported" {
                    return Err(error("agent integration is unsupported or unavailable"));
                }
                status["state"] = serde_json::json!("enabled");
            }
            if let AgentCommand::Enable { dry_run: false, .. } = &command {
                backend.ensure_repository().await?;
            } else {
                value["dry_run"] = serde_json::json!(true);
            }
        }
        if backend.repository()?.is_none() {
            return presentation::result(
                ui,
                helper,
                args.format,
                &value,
                "Local agent metadata is not initialized.\n",
            );
        }
    }
    let selected_repository = backend.repository()?;
    let state = args
        .state_dir
        .map(|path| helper.cwd().join(path))
        .unwrap_or_else(|| default_state(&workspace, selected_repository));
    if let Some(value) = backend.provenance_action(helper, &command, &state).await? {
        let text = format!("{}\n", serde_json::to_string_pretty(&value).map_err(error)?);
        return presentation::result(ui, helper, args.format, &value, &text);
    }
    if let Some(value) = backend.local_action(&command, &state).await? {
        let text = format!("{}\n", serde_json::to_string_pretty(&value).map_err(error)?);
        return presentation::result(ui, helper, args.format, &value, &text);
    }
    let repository = backend
        .repository()?
        .ok_or_else(|| error("enable an integration or initialize local agent metadata first"))?;
    if let AgentCommand::Hook { agent } = &command {
        crate::agent_hook::ingest_stdin(
            &workspace,
            &state,
            repository,
            IntegrationId::from(*agent),
        )
        .map_err(error)?;
        backend.publish_pending(&state).await;
        return Ok(());
    }
    let mut registry =
        AgentIntegrationRegistry::open(&state, &workspace, repository).map_err(error)?;
    registry.refresh_environment().map_err(error)?;
    audit_installations(&mut registry).map_err(error)?;
    let (status, action) = match command {
        AgentCommand::Status => (registry.status().clone(), "probed"),
        AgentCommand::Enable {
            agents,
            dry_run,
            non_interactive,
        } => {
            if !non_interactive {
                return Err(error(
                    "agent enable currently requires --non-interactive with explicit --agent values",
                ));
            }
            let agents = agents
                .into_iter()
                .map(IntegrationId::from)
                .collect::<Vec<_>>();
            (
                registry.enable(&agents, dry_run).map_err(error)?,
                if dry_run { "previewed" } else { "enabled" },
            )
        }
        AgentCommand::Setup { agent, dry_run } => {
            let integration = IntegrationId::from(agent);
            require_setup_ready(&registry, integration).map_err(error)?;
            if !dry_run {
                warm_kingfisher_cache(&state.join("kingfisher-cache")).map_err(error)?;
            }
            let result = setup(
                &mut registry,
                SetupRequestV1 {
                    integration,
                    executable: std::env::current_exe().map_err(error)?,
                    dry_run,
                },
            )
            .map_err(error)?;
            return presentation::result(
                ui,
                helper,
                args.format,
                &serde_json::to_value(&result).map_err(error)?,
                &format!(
                    "Agent integration {:?}: {:?} at {}{}\n",
                    result.integration,
                    result.setup,
                    result.target.display(),
                    if result.dry_run { " (dry run)" } else { "" }
                ),
            );
        }
        AgentCommand::Remove { agent, dry_run } => {
            let result = remove(
                &mut registry,
                RemoveRequestV1 {
                    integration: IntegrationId::from(agent),
                    dry_run,
                },
            )
            .map_err(error)?;
            return presentation::result(
                ui,
                helper,
                args.format,
                &serde_json::to_value(&result).map_err(error)?,
                &format!(
                    "Agent integration {:?}: {} at {}{}\n",
                    result.integration,
                    if result.changed { "removed" } else { "absent" },
                    result.target.display(),
                    if result.dry_run { " (dry run)" } else { "" }
                ),
            );
        }
        AgentCommand::Hook { .. } => return Err(error("agent hook dispatch failed")),
        AgentCommand::Initialize { .. }
        | AgentCommand::Sessions { .. }
        | AgentCommand::Receipt { .. }
        | AgentCommand::Drain => return Err(error("agent local command dispatch failed")),
        AgentCommand::CodeEvidence { .. }
        | AgentCommand::Associate { .. }
        | AgentCommand::Checkpoint { .. }
        | AgentCommand::RetryProvenance { .. }
        | AgentCommand::Associations { .. }
        | AgentCommand::Checkpoints { .. } => {
            return Err(error("agent provenance dispatch failed"));
        }
    };
    present(ui, helper, args.format, &status, action)
}

fn present(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
    status: &AgentDeviceStatusV1,
    action: &str,
) -> Result<(), CommandError> {
    let integrations = status
        .integrations
        .values()
        .map(|value| {
            format!(
                "  {:?}: {:?}; {:?}; {:?}\n",
                value.integration, value.support, value.state, value.setup
            )
        })
        .collect::<String>();
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(status).map_err(error)?,
        &format!(
            "Agent integrations {action}.\nRepository: {}\nWorkspace: {}\n{integrations}",
            status.repository,
            status.workspace.display()
        ),
    )
}
