use crate::{binding::Connection, credentials, presentation};
use clap::{Args, Subcommand, ValueEnum};
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
    ui::Ui,
};
use jj_store_model::RepositoryId;
use jj_store_service::auth::BearerToken;
use serde_json::json;
use std::{io::Read as _, path::PathBuf};
use zeroize::Zeroizing;

#[derive(Clone, Debug, Subcommand)]
pub enum Extension {
    /// Manage authenticated native JJ repositories
    #[command(long_about = include_str!("../../../docs/cli/store.md"), after_long_help = "Use 'jj store help -k commands' for the native command guide.")]
    Store(StoreArgs),
}
#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct StoreArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub command: Option<StoreCommand>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum Format {
    #[default]
    Text,
    Json,
}
#[derive(Clone, Debug, Args)]
pub struct ConnectionArgs {
    #[arg(long)]
    endpoint: String,
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
    fn connection(&self) -> Result<Connection, CommandError> {
        let ca = self
            .ca_certificate
            .as_ref()
            .map(|path| path.canonicalize())
            .transpose()
            .map_err(|_| user_error("Could not read deployment CA path"))?;
        Connection::new(&self.endpoint, self.repo_id, self.insecure_loopback, ca).map_err(error)
    }
}
#[derive(Clone, Debug, Subcommand)]
pub enum StoreCommand {
    /// Create a new native repository and its first physical workspace
    #[command(long_about = include_str!("../../../docs/cli/init.md"))]
    Init {
        destination: PathBuf,
        #[arg(long)]
        endpoint: String,
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
pub fn error(error: impl std::fmt::Display) -> CommandError {
    user_error(error.to_string()).hinted(
        "Use 'jj store help -k commands'; verify the endpoint, repository ID and credentials.",
    )
}
async fn complete_attachment(
    ui: &mut Ui,
    helper: &CommandHelper,
    format: Format,
    attachment: &mut crate::attachment::Attachment,
) -> Result<(), CommandError> {
    // The upstream new-workspace helper assumes .jj/repo is a directory.
    // A completed native attachment uses a pointer file instead. Its
    // retry creates no new operation and can use the current settings.
    let settings = if attachment.destination().join(".jj").exists() {
        helper.settings().clone()
    } else {
        helper
            .settings_for_new_workspace(ui, attachment.destination())?
            .0
    };
    attachment.prepare(&settings).await.map_err(error)?;
    let result = attachment.finish(&settings).await.map_err(error)?;
    presentation::result(
        ui,
        helper,
        format,
        &serde_json::to_value(&result).map_err(error)?,
        &format!(
            "Attached native repository {} at {}\nWorkspace: {}\n",
            result.repository,
            result.path.display(),
            result.workspace
        ),
    )
}
pub async fn run(
    ui: &mut Ui,
    helper: &CommandHelper,
    command: Extension,
) -> Result<(), CommandError> {
    let Extension::Store(args) = command;
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &[]);
    };
    match command {
        StoreCommand::Init {
            destination,
            endpoint,
            insecure_loopback,
            ca_certificate,
            workspace,
            cache,
            state_dir,
        } => {
            let mut initialization =
                crate::initialization::Initialization::open(crate::initialization::Options {
                    endpoint,
                    insecure_loopback,
                    ca_certificate: ca_certificate.map(|p| helper.cwd().join(p)),
                    destination: helper.cwd().join(destination),
                    cache: cache.map(|p| helper.cwd().join(p)),
                    workspace,
                    state: state_dir.map(|p| helper.cwd().join(p)),
                })
                .map_err(error)?;
            initialization.create().await.map_err(error)?;
            let mut attachment = initialization.attachment().await.map_err(error)?;
            complete_attachment(ui, helper, args.format, &mut attachment).await
        }
        StoreCommand::ServeLocal { server_config } => {
            crate::server_commands::serve_local(ui, &helper.cwd().join(server_config), args.format)
                .map_err(error)
        }
        StoreCommand::Attach {
            destination,
            connection,
            workspace,
            cache,
            state_dir,
        } => {
            let connection = connection.connection()?;
            let token = credentials::resolve(&connection).map_err(error)?;
            let mut attachment = crate::attachment::Attachment::open(
                connection,
                token,
                helper.cwd().join(destination),
                cache.map(|p| helper.cwd().join(p)),
                workspace,
                state_dir.map(|p| helper.cwd().join(p)),
            )
            .await
            .map_err(error)?;
            complete_attachment(ui, helper, args.format, &mut attachment).await
        }

        StoreCommand::Status | StoreCommand::Doctor => {
            let path = helper.workspace_loader()?.repo_path();
            let connection = Connection::read(path).map_err(error)?;
            let client = connection.connect().await.map_err(error)?;
            let heads = client.heads().await.map_err(error)?;
            let result = json!({"version":1,"ok":true,"repository":connection.repository,"endpoint":connection.endpoint,"profile":connection.profile,"heads":heads});
            let text = format!(
                "Repository: {}\nEndpoint: {}\nOperation heads:\n{}",
                connection.repository,
                connection.endpoint,
                heads
                    .iter()
                    .map(|head| format!("  {head}\n"))
                    .collect::<String>()
            );
            presentation::result(ui, helper, args.format, &result, &text)
        }
        StoreCommand::Login {
            connection,
            token_stdin,
        } => {
            let connection = connection.connection()?;
            let text = if token_stdin {
                let mut bytes = Zeroizing::new(Vec::new());
                std::io::stdin()
                    .take(105)
                    .read_to_end(&mut bytes)
                    .map_err(|_| user_error("Could not read credential from stdin"))?;
                if bytes.len() > 104 {
                    return Err(user_error("Credential input exceeds limit"));
                }
                Zeroizing::new(
                    std::str::from_utf8(&bytes)
                        .map_err(|_| user_error("Invalid credential encoding"))?
                        .to_owned(),
                )
            } else {
                Zeroizing::new(ui.prompt_password("Repository token: ").map_err(|_| {
                    user_error("Could not read hidden credential")
                        .hinted("Use --token-stdin for non-interactive provisioning.")
                })?)
            };
            let token = BearerToken::parse(text.trim()).map_err(|_| {
                user_error("Invalid repository token")
                    .hinted("Provide one complete jst1 token through stdin or the hidden prompt.")
            })?;
            connection
                .connect_with(token.clone())
                .await
                .map_err(error)?
                .authorize(false)
                .await
                .map_err(error)?;
            credentials::save(&connection, &token).map_err(error)?;
            presentation::result(
                ui,
                helper,
                args.format,
                &json!({"version":1,"saved":true,"repository":connection.repository,"environment_override":credentials::environment_present()}),
                "Credential saved to the OS facility.\n",
            )
        }
        StoreCommand::Logout { connection } => {
            let connection = connection.connection()?;
            credentials::remove(&connection).map_err(error)?;
            let override_present = credentials::environment_present();
            let text = if override_present {
                "Stored credential removed. JJ_STORE_TOKEN still overrides the OS facility; clear it separately.\n"
            } else {
                "Stored credential removed.\n"
            };
            presentation::result(
                ui,
                helper,
                args.format,
                &json!({"version":1,"removed":true,"repository":connection.repository,"environment_override":override_present}),
                text,
            )
        }
        StoreCommand::Capabilities => presentation::result(
            ui,
            helper,
            args.format,
            &capabilities(),
            "Native history: commit, operation and evolution graphs.\nUse --format json for command qualification and limits.\nGit transport and destructive GC are unsupported on native repositories.\n",
        ),
        StoreCommand::Version => {
            let result = json!({"client":{"name":"jj-store custom jj","version":env!("CARGO_PKG_VERSION"),"upstream_jj":"0.45.1","source_revision":env!("JJ_STORE_REVISION"),"build_target":env!("JJ_STORE_TARGET"),"build_profile":env!("JJ_STORE_PROFILE")}});
            let text = format!(
                "Client:\n  Name: jj-store custom jj\n  Version: {}\n  Upstream JJ: 0.45.1\n  Git commit: {}\n  Build target: {}\n  Build profile: {}\n",
                env!("CARGO_PKG_VERSION"),
                env!("JJ_STORE_REVISION"),
                env!("JJ_STORE_TARGET"),
                env!("JJ_STORE_PROFILE")
            );
            presentation::result(ui, helper, args.format, &result, &text)
        }
        StoreCommand::Help { command, keyword } => {
            if let Some(keyword) = keyword {
                let markdown = match keyword.as_str() {
                    "commands" => include_str!("../../../docs/cli/commands.md"),
                    "credentials" => include_str!("../../../docs/cli/credentials.md"),
                    "pagination" => include_str!("../../../docs/cli/pagination.md"),
                    _ => unreachable!("clap validates keyword"),
                };
                presentation::text(ui, helper, &presentation::markdown(markdown, ui.color()))
            } else {
                presentation::help(ui, helper, &command)
            }
        }
    }
}
pub fn capabilities() -> serde_json::Value {
    json!({"version":1,"native_profile":jj_store_jj::factories::PROFILE,"transport":"jj_store.native.v1","qualification":"loopback RPC and pinned single-node RustFS cold recovery; shared deployment and full semantic matrix remain gated","commands":{
        "edit":"verified","describe":"verified","log":"verified","op log":"verified","evolog":"verified",
        "new":"verified","diff":"verified","bookmark create":"verified","bookmark set":"verified","bookmark delete":"verified","bookmark list":"verified",
        "abandon":"unqualified","rebase":"unqualified","squash":"unqualified",
        "git":"unsupported","util gc":"unsupported"},"creation":"jj store init with durable retries","physical_state":"client-local","ordinary_commands":"upstream jj 0.45.1"})
}
