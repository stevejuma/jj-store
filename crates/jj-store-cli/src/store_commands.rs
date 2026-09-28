use crate::{binding::Connection, commands::error};
use clap::{Args, Subcommand};
use jj_cli::command_error::CommandError;
use jj_store_model::RepositoryId;
use std::path::PathBuf;

#[derive(Clone, Debug, Args)]
pub(crate) struct ConnectionArgs {
    #[arg(
        long,
        required_unless_present = "unix_socket",
        conflicts_with = "unix_socket"
    )]
    endpoint: Option<String>,
    /// User-restricted LocalTrusted native RPC socket
    #[arg(long, conflicts_with_all = ["endpoint", "insecure_loopback", "ca_certificate"])]
    unix_socket: Option<PathBuf>,
    #[arg(long)]
    repo_id: RepositoryId,
    /// Allow plaintext only for a literal loopback IP address
    #[arg(long)]
    insecure_loopback: bool,
    /// Additional deployment CA in PEM format
    #[arg(long)]
    ca_certificate: Option<PathBuf>,
}

impl ConnectionArgs {
    pub(crate) fn connection_raw(
        &self,
    ) -> jj_store_jj::operation_stores::AuthorityResult<Connection> {
        let ca = self
            .ca_certificate
            .as_ref()
            .map(|path| path.canonicalize())
            .transpose()
            .map_err(|_| "Could not read deployment CA path")?;
        match (&self.endpoint, &self.unix_socket) {
            (Some(endpoint), None) => {
                Connection::new(endpoint, self.repo_id, self.insecure_loopback, ca)
            }
            (None, Some(socket)) if !self.insecure_loopback && ca.is_none() => {
                Connection::new_local(socket.canonicalize()?, self.repo_id)
            }
            _ => Err("select exactly one authenticated endpoint or local Unix socket".into()),
        }
    }

    pub(crate) fn connection(&self) -> Result<Connection, CommandError> {
        self.connection_raw().map_err(error)
    }
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum StoreCommand {
    /// List, grant, revoke and recover repository token requests
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Token(crate::token_commands::TokenArgs),
    /// Create a new native repository and its first physical workspace
    #[command(long_about = include_str!("../../../docs/cli/init.md"))]
    Init {
        destination: PathBuf,
        #[arg(
            long,
            required_unless_present = "unix_socket",
            conflicts_with = "unix_socket"
        )]
        endpoint: Option<String>,
        /// User-restricted LocalTrusted native RPC socket
        #[arg(long, conflicts_with_all = ["endpoint", "insecure_loopback", "ca_certificate"])]
        unix_socket: Option<PathBuf>,
        #[arg(long)]
        insecure_loopback: bool,
        #[arg(long)]
        ca_certificate: Option<PathBuf>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        cache: Option<PathBuf>,
        #[arg(long)]
        state_dir: Option<PathBuf>,
    },
    /// Run the configured native server on a literal loopback listener
    #[command(long_about = include_str!("../../../docs/cli/serve-local.md"))]
    ServeLocal {
        #[arg(long = "server-config")]
        server_config: PathBuf,
    },
    /// Attach a new physical workspace to a native repository
    #[command(long_about = include_str!("../../../docs/cli/attach.md"))]
    Attach {
        destination: PathBuf,
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        cache: Option<PathBuf>,
        /// Durable attachment journal outside the checkout
        #[arg(long)]
        state_dir: Option<PathBuf>,
    },
    /// Inspect remote heads without snapshotting or reconciling
    #[command(long_about = include_str!("../../../docs/cli/status.md"))]
    Status,
    /// Check binding, authentication and remote access
    #[command(long_about = include_str!("../../../docs/cli/doctor.md"))]
    Doctor,
    /// Report native command qualification
    #[command(long_about = include_str!("../../../docs/cli/capabilities.md"))]
    Capabilities,
    /// Verify and save a token to the OS credential facility
    #[command(long_about = include_str!("../../../docs/cli/credentials.md"))]
    Login {
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        token_stdin: bool,
    },
    /// Delete one OS credential account
    #[command(long_about = include_str!("../../../docs/cli/credentials.md"))]
    Logout {
        #[command(flatten)]
        connection: ConnectionArgs,
    },
    /// Print build and upstream version information
    #[command(long_about = include_str!("../../../docs/cli/version.md"))]
    Version,
    /// Read native command help and keyword documents
    Help {
        command: Vec<String>,
        #[arg(short='k', long, conflicts_with="command", value_parser=["commands", "credentials", "pagination"])]
        keyword: Option<String>,
    },
}
