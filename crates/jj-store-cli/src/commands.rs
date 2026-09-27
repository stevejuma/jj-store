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
use std::{fs, io::Read as _, path::PathBuf};
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
fn random_name() -> Result<String, CommandError> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes).map_err(|_| user_error("Secure identity generation failed"))?;
    Ok(hex::encode(bytes))
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
        StoreCommand::ServeLocal { server_config } => {
            crate::server_commands::serve_local(ui, &helper.cwd().join(server_config), args.format)
                .map_err(error)
        }
        StoreCommand::Attach {
            destination,
            connection,
            workspace,
            cache,
        } => {
            let connection = connection.connection()?;
            let destination = helper.cwd().join(destination);
            if destination.exists() && fs::read_dir(&destination).map_err(error)?.next().is_some() {
                return Err(user_error("Native attachment requires an empty destination").hinted("Choose a new directory; existing Git/JJ repositories are not migrated by attach."));
            }
            let workspace = workspace.unwrap_or(format!("client-{}", random_name()?));
            if workspace.is_empty()
                || workspace.len() > 128
                || workspace
                    .bytes()
                    .any(|c| !c.is_ascii_alphanumeric() && !b"-_".contains(&c))
            {
                return Err(user_error("Invalid workspace name")
                    .hinted("Use 1–128 ASCII letters, digits, hyphens or underscores."));
            }
            let cache = match cache {
                Some(path) => helper.cwd().join(path),
                None => dirs::data_local_dir()
                    .ok_or_else(|| user_error("No local client data directory"))?
                    .join("jj-store/workspaces")
                    .join(random_name()?),
            };
            // A read-only or revoked credential must fail before creating any
            // physical checkout or cache state.
            let client = connection.connect().await.map_err(error)?;
            client.authorize(true).await.map_err(error)?;
            let stores = crate::remote::stores(
                client,
                connection.max_blob_bytes,
                jj_store_jj::backend::BackendLimits::default(),
                jj_store_jj::operation_stores::OperationLimits::default(),
            )
            .map_err(error)?;
            fs::create_dir_all(&destination).map_err(error)?;
            fs::create_dir_all(&cache).map_err(error)?;
            let destination = destination.canonicalize().map_err(error)?;
            let cache = cache.canonicalize().map_err(error)?;
            if cache.starts_with(&destination) || destination.starts_with(&cache) {
                return Err(
                    user_error("Client cache must be outside the physical checkout")
                        .hinted("Choose a separate empty --cache directory."),
                );
            }
            let settings = helper.settings_for_new_workspace(ui, &destination)?.0;
            stores
                .initialize_cache(&settings, &cache)
                .await
                .map_err(error)?;
            connection.write(&cache).map_err(error)?;
            let (_, repo) = stores
                .attach_workspace(&settings, &destination, &cache, workspace.clone().into())
                .await
                .map_err(error)?;
            use jj_lib::object_id::ObjectId as _;
            let result = json!({"version":1,"repository":connection.repository,"workspace":workspace,"path":destination,"cache":cache,"operation":repo.op_id().hex()});
            presentation::result(
                ui,
                helper,
                args.format,
                &result,
                &format!(
                    "Attached native repository {} at {}\nWorkspace: {workspace}\n",
                    connection.repository,
                    destination.display()
                ),
            )
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
    json!({"version":1,"native_profile":jj_store_jj::factories::PROFILE,"transport":"jj_store.native.v1","qualification":"cold executable scenarios over loopback RPC; deployment and full semantic matrix remain gated","commands":{
        "edit":"verified","describe":"verified","log":"verified","op log":"verified","evolog":"verified",
        "new":"verified","diff":"verified","bookmark create":"verified","bookmark set":"verified","bookmark delete":"verified","bookmark list":"verified",
        "abandon":"unqualified","rebase":"unqualified","squash":"unqualified",
        "git":"unsupported","util gc":"unsupported"},"creation":"server administration pending","physical_state":"client-local","ordinary_commands":"upstream jj 0.45.1"})
}
