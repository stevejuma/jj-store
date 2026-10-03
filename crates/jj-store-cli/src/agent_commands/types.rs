use crate::commands::Format;
use clap::{Args, Subcommand, ValueEnum};
use jj_agent::IntegrationId;
use std::path::PathBuf;

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
    /// Manage one explicitly selected local server's automatic transcript capture
    Capture(super::capture::CaptureArgs),
    /// Persistent repository Threads; execution remains separate
    Thread {
        #[command(subcommand)]
        command: super::collab::ThreadCliCommand,
    },
    /// Exact-target rich reactions and the retained custom emoji library
    Reaction {
        #[command(subcommand)]
        command: super::collab::ReactionCliCommand,
    },
    /// Inspect exact source evidence for one explicit Git-backed JJ operation
    CodeEvidence {
        #[arg(long)]
        operation: jj_store_model::OperationId,
    },
    /// Publish one complete typed portable association document
    Associate {
        #[arg(long)]
        request_id: jj_store_model::RequestId,
        #[arg(long)]
        record: PathBuf,
        #[arg(long)]
        expected_version: Option<jj_agent::AgentObjectId>,
    },
    /// Publish one complete typed portable checkpoint document
    Checkpoint {
        #[arg(long)]
        request_id: jj_store_model::RequestId,
        #[arg(long)]
        record: PathBuf,
        #[arg(long)]
        expected_version: Option<jj_agent::AgentObjectId>,
    },
    /// Retry an exact privately journaled provenance publication
    RetryProvenance {
        request_id: jj_store_model::RequestId,
    },
    /// List one pinned page of local code associations
    Associations {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// List one pinned page of local checkpoints
    Checkpoints {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Initialize companion AgentStore metadata in a local Git-backed JJ repository
    Initialize {
        #[arg(long)]
        request_id: jj_store_model::RequestId,
    },
    /// List one pinned page of local Git-backed AgentStore sessions
    Sessions {
        #[arg(long, default_value_t = 50)]
        limit: usize,
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Read a local capture's original durable receipt
    Receipt {
        request_id: jj_store_model::RequestId,
    },
    /// Retry pending sanitized captures in source order
    Drain,
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
    /// Remove one exact jj-store-owned project hook or plugin
    Remove {
        #[arg(value_enum)]
        agent: AgentName,
        /// Preview exact restoration without changing project or registry state
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
