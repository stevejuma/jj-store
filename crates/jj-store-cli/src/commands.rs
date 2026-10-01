use crate::{binding::Connection, credentials, presentation};
use clap::{Args, Subcommand, ValueEnum};
use jj_cli::{
    cli_util::CommandHelper,
    command_error::{CommandError, user_error},
    ui::Ui,
};
use jj_store_service::auth::BearerToken;
use serde_json::json;
use std::io::Read as _;
use zeroize::Zeroizing;

use crate::store_commands::StoreCommand;

#[derive(Clone, Debug, Subcommand)]
pub enum Extension {
    /// Capture repository-owned activity from supported coding agents
    #[command(long_about = include_str!("../../../docs/cli/agent.md"))]
    Agent(crate::agent_commands::AgentArgs),
    /// Read and update repository-owned review threads
    #[command(long_about = include_str!("../../../docs/cli/review.md"))]
    Review(crate::review_commands::ReviewArgs),
    /// Manage native project homes and repository links
    #[command(long_about = include_str!("../../../docs/cli/work.md"))]
    Project(crate::work_commands::ProjectArgs),
    /// Manage native tasks, subtasks and threaded comments
    #[command(long_about = include_str!("../../../docs/cli/work.md"))]
    Task(crate::work_commands::TaskArgs),
    /// Manage authenticated native JJ repositories
    #[command(long_about = include_str!("../../../docs/cli/store.md"), after_long_help = "Use 'jj store help -k commands' for the native command guide.")]
    Store(StoreArgs),
    /// Synchronize and explicitly reconcile a local native repository
    #[command(long_about = include_str!("../../../docs/cli/sync.md"))]
    Sync(crate::sync_commands::SyncArgs),
    /// Inspect configured writable-replica state
    Replica(crate::sync_commands::ReplicaArgs),
}
#[derive(Clone, Debug, Args)]
#[command(disable_help_subcommand = true)]
pub struct StoreArgs {
    #[arg(long, global = true, value_enum, default_value_t=Format::Text)]
    pub format: Format,
    #[command(subcommand)]
    pub(crate) command: Option<StoreCommand>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum Format {
    #[default]
    Text,
    Json,
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
    let args = match command {
        Extension::Agent(args) => return crate::agent_commands::run(ui, helper, args).await,
        Extension::Project(args) => {
            return crate::work_commands::run_project(ui, helper, args).await;
        }
        Extension::Task(args) => return crate::work_commands::run_task(ui, helper, args).await,
        Extension::Review(args) => return crate::review_commands::run(ui, helper, args).await,
        Extension::Sync(args) => return crate::sync_commands::run(ui, helper, args).await,
        Extension::Replica(args) => {
            return crate::sync_commands::run_replica(ui, helper, args).await;
        }
        Extension::Store(args) => args,
    };
    let Some(command) = args.command else {
        return presentation::help(ui, helper, &[]);
    };
    match command {
        StoreCommand::Git(_) => Err(user_error("Git attempts require native bridge dispatch")),
        StoreCommand::Token(token_args) => {
            let (value, text) = crate::token_commands::run(ui, token_args)
                .await
                .map_err(error)?;
            presentation::result(ui, helper, args.format, &value, &text)
        }
        StoreCommand::Init {
            destination,
            name,
            endpoint,
            unix_socket,
            insecure_loopback,
            ca_certificate,
            workspace,
            cache,
            state_dir,
        } => {
            let mut initialization =
                crate::initialization::Initialization::open(crate::initialization::Options {
                    endpoint,
                    unix_socket: unix_socket.map(|path| helper.cwd().join(path)),
                    insecure_loopback,
                    ca_certificate: ca_certificate.map(|p| helper.cwd().join(p)),
                    destination: helper.cwd().join(destination),
                    name,
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
            let mut attachment = if connection.unix_socket.is_some() {
                crate::attachment::Attachment::open_local(
                    connection,
                    helper.cwd().join(destination),
                    cache.map(|p| helper.cwd().join(p)),
                    workspace,
                    state_dir.map(|p| helper.cwd().join(p)),
                )
                .await
                .map_err(error)?
            } else {
                let token = credentials::resolve(&connection).map_err(error)?;
                crate::attachment::Attachment::open(
                    connection,
                    token,
                    helper.cwd().join(destination),
                    cache.map(|p| helper.cwd().join(p)),
                    workspace,
                    state_dir.map(|p| helper.cwd().join(p)),
                )
                .await
                .map_err(error)?
            };
            complete_attachment(ui, helper, args.format, &mut attachment).await
        }
        StoreCommand::MigrateLocal {
            unix_socket,
            cache,
            state_dir,
        } => {
            let loader = helper.workspace_loader()?;
            let source = Connection::read(loader.repo_path()).map_err(error)?;
            let socket = helper
                .cwd()
                .join(unix_socket)
                .canonicalize()
                .map_err(error)?;
            let local = Connection::new_local(socket, source.repository).map_err(error)?;
            let settings = helper.settings().clone();
            let mut migration = crate::migration::Migration::open(
                &settings,
                loader.workspace_root().to_owned(),
                loader.repo_path().to_owned(),
                local,
                cache.map(|path| helper.cwd().join(path)),
                state_dir.map(|path| helper.cwd().join(path)),
            )
            .await
            .map_err(error)?;
            let result = migration.run(&settings).await.map_err(error)?;
            presentation::result(
                ui,
                helper,
                args.format,
                &serde_json::to_value(&result).map_err(error)?,
                &format!(
                    "Migrated native repository {} at {} to LocalTrusted storage.\nWorkspace: {}\nRemote descriptor backup: {}\n",
                    result.repository,
                    result.path.display(),
                    result.workspace,
                    result.backup.display()
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
            if connection.unix_socket.is_some() {
                return Err(user_error(
                    "LocalTrusted connections do not use repository login credentials",
                ));
            }
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
            if connection.unix_socket.is_some() {
                return Err(user_error(
                    "LocalTrusted connections do not use repository login credentials",
                ));
            }
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
            "Native history: commit, operation and evolution graphs.\nUse --format json for command qualification and limits.\nExplicit Git bridge support is reported per command; destructive GC is unsupported.\n",
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
    json!({"version":1,"native_profile":jj_store_jj::factories::PROFILE,"transport":"jj_store.native.v1","qualification":"native RPC and pinned single-node RustFS cold TLS recovery on current Unix host; other deployment topologies and release platforms remain gated","commands":{
        "edit":"verified","describe":"verified","log":"verified","op log":"verified","evolog":"verified",
        "new":"verified","diff":"verified","bookmark create":"verified","bookmark set":"verified","bookmark delete":"verified","bookmark list":"verified",
        "abandon":"verified","rebase":"verified","squash":"verified","split":"verified","undo":"verified",
        "resolve --list":"verified","file show":"verified","file list":"verified","file chmod":"verified",
        "sign":"verified (SSH)","unsign":"verified (SSH)","operation reconciliation":"verified",
        "git remote add/list/set-url":"verified local bridge","git fetch":"verified local bounded scope","git push":"verified single bookmark/default tracked selection and dry-run","store git status/recover":"verified data-only status and no-repush recovery","git import/export":"unsupported","util gc":"unsupported"},"git_bridge":{"stage":"GB4 native Git facts slice","dispatch":"native backend-aware hook","native_transport":"approved device-local Git 2.55.0; data-only code origins with reducer 2","fetch":"one named remote; simple bookmark filters; commit tags retained","publication":"one bookmark with exact lease; tag/multi-ref/other selectors deferred","attempts":"durable prepare/start/result/apply; cache-loss and lost-ack repair; no exactly-once external claim"},"creation":"jj store init with durable retries","physical_state":"client-local","ordinary_commands":"upstream jj 0.45.1"})
}
