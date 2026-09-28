use crate::{
    agent_integrations::{AgentDeviceStatusV1, AgentIntegrationRegistry},
    agent_sanitization::warm_kingfisher_cache,
    agent_setup::{SetupRequestV1, audit_installations, require_setup_ready, setup},
    commands::{Format, error},
    presentation,
};
use clap::{Args, Subcommand, ValueEnum};
use jj_agent::IntegrationId;
use jj_cli::{cli_util::CommandHelper, command_error::CommandError, ui::Ui};
use sha2::{Digest as _, Sha256};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct AgentArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    /// Override device-local state; intended for isolated qualification
    #[arg(long, global = true)]
    pub state_dir: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<AgentCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum AgentCommand {
    /// Probe integrations and report desired versus installed state
    Status,
    /// Enable explicit supported integrations in the device registry
    Enable {
        #[arg(long = "agent", required = true, value_enum)]
        agents: Vec<AgentName>,
        /// Preview the durable registry result without writing it
        #[arg(long)]
        dry_run: bool,
        /// Confirm that no interactive integration selection is requested
        #[arg(long)]
        non_interactive: bool,
    },
    /// Install one owned project hook or plugin without replacing unrelated configuration
    Setup {
        #[arg(value_enum)]
        agent: AgentName,
        /// Preview the target change without writing configuration or ownership state
        #[arg(long)]
        dry_run: bool,
    },
    /// Receive one bounded project hook event
    #[command(hide = true)]
    Hook {
        #[arg(value_enum)]
        agent: AgentName,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum AgentName {
    Codex,
    Cursor,
    Opencode,
}

impl From<AgentName> for IntegrationId {
    fn from(value: AgentName) -> Self {
        match value {
            AgentName::Codex => Self::Codex,
            AgentName::Cursor => Self::Cursor,
            AgentName::Opencode => Self::OpenCode,
        }
    }
}

pub async fn run(ui: &mut Ui, helper: &CommandHelper, args: AgentArgs) -> Result<(), CommandError> {
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &["agent".into()]);
    };
    let loader = helper.workspace_loader()?;
    let connection = crate::binding::Connection::read(loader.repo_path()).map_err(error)?;
    let workspace = workspace_root(helper.cwd()).map_err(error)?;
    let state = args
        .state_dir
        .map(|path| helper.cwd().join(path))
        .unwrap_or_else(|| default_state(&workspace, connection.repository));
    if let AgentCommand::Hook { agent } = &command {
        crate::agent_hook::ingest_stdin(
            &workspace,
            &state,
            connection.repository,
            IntegrationId::from(*agent),
        )
        .map_err(error)?;
        crate::agent_hook::publish_pending_best_effort(&state, &connection).await;
        return Ok(());
    }
    let mut registry =
        AgentIntegrationRegistry::open(&state, &workspace, connection.repository).map_err(error)?;
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
        AgentCommand::Hook { .. } => return Err(error("agent hook dispatch failed")),
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

fn workspace_root(cwd: &Path) -> Result<PathBuf, &'static str> {
    let cwd = cwd
        .canonicalize()
        .map_err(|_| "workspace path unavailable")?;
    cwd.ancestors()
        .find(|path| path.join(".jj").exists())
        .map(Path::to_owned)
        .ok_or("agent commands require a JJ workspace")
}

fn default_state(workspace: &Path, repository: jj_store_model::RepositoryId) -> PathBuf {
    let digest = Sha256::digest(workspace.as_os_str().as_encoded_bytes());
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("jj-store/agent")
        .join(repository.to_string())
        .join(hex::encode(&digest[..16]))
}
