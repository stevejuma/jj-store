//! Standalone command presentation shares the native client's terminal policy.
use crate::{
    binding::{Connection, canonical_endpoint},
    commands::Format,
    credentials, presentation,
};
use clap::{CommandFactory as _, Parser, Subcommand};
use jj_cli::ui::Ui;
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PrincipalId, RepositoryId, TokenId};
use jj_store_server::{config::Config, policy::PolicyFile, process};
use serde_json::json;
use std::{io::Write as _, path::PathBuf};

#[derive(Debug, Parser)]
#[command(name="jj-store",disable_help_subcommand=true,about="Serve persistent local native JJ repositories",long_about=include_str!("../../../docs/cli/server.md"))]
pub struct ServerCli {
    #[arg(long,global=true,value_enum,default_value_t=Format::Text)]
    pub format: Format,
    #[arg(long, global = true)]
    pub no_pager: bool,
    #[arg(long,global=true,value_parser=["auto","always","never"],default_value="auto")]
    pub color: String,
    #[command(subcommand)]
    pub command: Option<ServerCommand>,
}
#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    /// Read, generate and rename retained conversation metadata on a local service
    ConversationMetadata(crate::conversation_metadata::MetadataArgs),
    /// Run local Stacks actions directly in an ordinary JJ workspace
    Stack(crate::stack_commands::StackArgs),
    /// Create a new private local installation with managed RustFS storage
    Init {
        #[arg(long, required = true)]
        local: bool,
        #[arg(long)]
        state_dir: PathBuf,
        #[arg(long)]
        rustfs_executable: Option<PathBuf>,
        /// Native embedded storage (default), or existing Git-backed JJ folders only
        #[arg(long, value_parser = ["native", "filesystem"], default_value = "native")]
        storage: String,
    },
    /// List, grant, revoke and recover repository token requests
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Token(crate::token_commands::TokenArgs),
    /// Administer installation-owned profile identities
    Identity(crate::identity_commands::IdentityArgs),
    /// Verify repository authority and update LocalTrusted discovery metadata
    Repository {
        #[command(subcommand)]
        command: RepositoryCommand,
    },
    /// Initialize installation-owned discovery metadata for authenticated serving
    Registry {
        #[command(subcommand)]
        command: RegistryCommand,
    },
    /// Run the configured service in the foreground
    #[command(long_about=include_str!("../../../docs/cli/server-run.md"))]
    Run {
        #[arg(long)]
        config: PathBuf,
        /// Require a literal loopback bind address
        #[arg(long)]
        loopback_only: bool,
        #[command(flatten)]
        desktop: crate::server_desktop::DesktopOptions,
    },
    /// Print or validate configuration
    #[command(long_about=include_str!("../../../docs/cli/server-config.md"))]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Check current repository metadata and authorization
    #[command(long_about=include_str!("../../../docs/cli/server-doctor.md"))]
    Doctor {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        repo_id: RepositoryId,
    },
    /// Administer the local repository-creation policy
    #[command(long_about=include_str!("../../../docs/cli/server-operator.md"))]
    Operator {
        #[command(subcommand)]
        command: OperatorCommand,
    },
    /// Print version and build information
    Version,
    /// Generate shell completions
    Completion { shell: clap_complete::Shell },
    /// Read command help and keyword guides
    Help {
        command: Vec<String>,
        #[arg(short='k',long,conflicts_with="command",value_parser=["commands","config","credentials","pagination"])]
        keyword: Option<String>,
    },
}
#[derive(Debug, Subcommand)]
pub enum RepositoryCommand {
    /// Register a verified repository in the local installation catalog
    Register {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        repo_id: RepositoryId,
        #[arg(long)]
        name: Option<String>,
    },
}
#[derive(Debug, Subcommand)]
pub enum RegistryCommand {
    /// Create a private empty installation registry
    Init {
        #[arg(long)]
        state_dir: PathBuf,
    },
}
#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Print an example configuration
    Example,
    /// Validate syntax, boundaries and resource limits without contacting storage
    Check {
        #[arg(long)]
        config: PathBuf,
    },
}
#[derive(Debug, Subcommand)]
pub enum OperatorCommand {
    /// Initialize a creation policy and provision its credential
    Init {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        endpoint: String,
        #[arg(long)]
        insecure_loopback: bool,
        #[arg(long)]
        principal: PrincipalId,
        #[arg(long, value_enum, default_value = "create-repository")]
        permission: OperatorPermission,
    },
    /// Add a selected operator credential to an existing policy
    Grant {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        endpoint: String,
        #[arg(long)]
        insecure_loopback: bool,
        #[arg(long)]
        principal: PrincipalId,
        #[arg(long, value_enum, default_value = "create-repository")]
        permission: OperatorPermission,
    },
    /// List public operator grant metadata
    List {
        #[arg(long)]
        policy: PathBuf,
    },
    /// Revoke one operator credential
    Revoke {
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        token_id: TokenId,
    },
}
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OperatorPermission {
    CreateRepository,
    ManageIdentities,
}
impl From<OperatorPermission> for jj_store_service::auth::Permission {
    fn from(value: OperatorPermission) -> Self {
        match value {
            OperatorPermission::CreateRepository => Self::CreateRepository,
            OperatorPermission::ManageIdentities => Self::ManageIdentities,
        }
    }
}
fn ui(color: &str) -> AuthorityResult<Ui> {
    let mut config = StackedConfig::empty();
    for layer in jj_cli::config::default_config_layers() {
        config.add_layer(layer);
    }
    let color = if color == "auto" && std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()) {
        "never"
    } else {
        color
    };
    config.add_layer(ConfigLayer::parse(
        ConfigSource::CommandArg,
        &format!("ui.color = '{color}'"),
    )?);
    Ui::with_config(&config).map_err(|_| "terminal configuration failed".into())
}
pub fn main() -> u8 {
    let args: Vec<_> = std::env::args_os().collect();
    let parsed = ServerCli::try_parse_from(&args);
    let cli = match parsed {
        Ok(cli) => cli,
        Err(error) => {
            if error.kind() == clap::error::ErrorKind::DisplayHelp {
                let probe: Vec<_> = args
                    .iter()
                    .filter(|a| *a != "-h" && *a != "--help")
                    .cloned()
                    .collect();
                let matches = ServerCli::command()
                    .ignore_errors(true)
                    .try_get_matches_from(probe)
                    .ok();
                let color = matches
                    .as_ref()
                    .and_then(|m| m.get_one::<String>("color"))
                    .map(String::as_str)
                    .unwrap_or("auto");
                let no_pager = matches
                    .as_ref()
                    .and_then(|m| m.get_one::<bool>("no_pager"))
                    .copied()
                    .unwrap_or(false);
                if let Ok(ui) = ui(color) {
                    let raw = if ui.color() {
                        error.render().ansi().to_string()
                    } else {
                        error.render().to_string()
                    };
                    return if page(&ui, &presentation::markdown(&raw, ui.color()), no_pager).is_ok()
                    {
                        0
                    } else {
                        1
                    };
                }
            }
            let _ = error.print();
            eprintln!("Hint: Use 'jj-store help -k commands'.");
            return 2;
        }
    };
    let stack_command = matches!(&cli.command, Some(ServerCommand::Stack(_)));
    let result = (|| -> AuthorityResult<()> {
        let ui = ui(&cli.color)?;
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()?;
        runtime.block_on(run(&ui, cli))
    })();
    match result {
        Ok(()) => 0,
        Err(error) => {
            if stack_command {
                eprintln!(
                    "Error: {error}\nHint: Use 'jj-store stack status --no-snapshot' and 'jj-store help stack'."
                );
                crate::stack_commands::error_code(&error)
            } else {
                eprintln!(
                    "Error: {error}\nHint: Use 'jj-store help -k config'; verify paths, credentials and the selected repository."
                );
                1
            }
        }
    }
}
pub(crate) fn emit(
    ui: &Ui,
    format: Format,
    no_pager: bool,
    value: serde_json::Value,
    text: &str,
) -> AuthorityResult<()> {
    if format == Format::Json {
        writeln!(ui.stdout(), "{value}")?;
    } else {
        page(ui, text, no_pager)?;
    }
    Ok(())
}
pub(crate) fn help(ui: &Ui, no_pager: bool, path: &[String]) -> AuthorityResult<()> {
    let mut app = ServerCli::command();
    app.build();
    let mut command = &mut app;
    for name in path {
        command = command
            .find_subcommand_mut(name)
            .ok_or("unknown server help command")?;
    }
    let styled = command.render_long_help();
    let raw = if ui.color() {
        styled.ansi().to_string()
    } else {
        styled.to_string()
    };
    page(ui, &presentation::markdown(&raw, ui.color()), no_pager)?;
    Ok(())
}
pub async fn serve(
    ui: &Ui,
    path: &std::path::Path,
    format: Format,
    local: bool,
) -> AuthorityResult<()> {
    crate::server_process::serve(ui, path, format, local, Default::default()).await
}

async fn run(ui: &Ui, cli: ServerCli) -> AuthorityResult<()> {
    let ServerCli {
        format,
        no_pager,
        command,
        ..
    } = cli;
    let Some(command) = command else {
        return help(ui, no_pager, &[]);
    };
    match command {
        ServerCommand::ConversationMetadata(args) => {
            crate::conversation_metadata::run(ui, args).await
        }
        ServerCommand::Stack(args) => crate::stack_commands::run(ui, format, no_pager, args).await,
        ServerCommand::Init {
            local,
            state_dir,
            rustfs_executable,
            storage,
        } => {
            if !local {
                return Err("only explicit --local initialization is supported".into());
            }
            let initialized = if storage == "filesystem" {
                if rustfs_executable.is_some() {
                    return Err("filesystem storage cannot configure RustFS".into());
                }
                jj_store_server::installation::initialize_filesystem(&state_dir)?
            } else {
                let runtime = embedded_runtime(rustfs_executable)?;
                jj_store_server::installation::initialize_local(&state_dir, &runtime)?
            };
            let text = format!(
                "Local installation created at {}\nConfiguration: {}\nBackups: disabled\n",
                initialized.state_dir.display(),
                initialized.config.display()
            );
            emit(
                ui,
                format,
                no_pager,
                serde_json::to_value(initialized)?,
                &text,
            )
        }
        ServerCommand::Run {
            config,
            loopback_only,
            desktop,
        } => crate::server_process::serve(ui, &config, format, loopback_only, desktop).await,
        ServerCommand::Identity(args) => {
            let (value, text) = crate::identity_commands::run(ui, args).await?;
            emit(ui, format, no_pager, value, &text)
        }
        ServerCommand::Token(args) => {
            let (value, text) = crate::token_commands::run(ui, args).await?;
            emit(ui, format, no_pager, value, &text)
        }
        ServerCommand::Repository {
            command:
                RepositoryCommand::Register {
                    config,
                    repo_id,
                    name,
                },
        } => {
            let config = Config::load(&config)?;
            let (installation, catalog) = match &config.access {
                jj_store_server::config::Access::LocalTrusted {
                    installation,
                    catalog,
                    ..
                }
                | jj_store_server::config::Access::Authenticated {
                    installation,
                    catalog,
                } => (installation, catalog),
            };
            let identity = jj_store_server::installation::Identity::load(installation)?;
            jj_store_server::installation::Catalog::load(catalog, &identity)?;
            let storage = jj_store_server::storage::AuthoritativeStorage::start(&config).await?;
            let snapshot =
                jj_store_wal::publication::ManifestSnapshot::load(storage.store(), repo_id).await?;
            if snapshot.manifest().repository != repo_id {
                storage.shutdown().await?;
                return Err("repository authority identity mismatch".into());
            }
            let name = name.unwrap_or_else(|| repo_id.to_string());
            let updated = jj_store_server::installation::Catalog::register(
                catalog,
                &identity,
                repo_id,
                name.clone(),
            )?;
            storage.shutdown().await?;
            emit(
                ui,
                format,
                no_pager,
                json!({"version":1,"repository":repo_id,"name":name,"catalog_entries":updated.repositories.len()}),
                &format!("Registered repository {name} ({repo_id})\n"),
            )
        }
        ServerCommand::Registry {
            command: RegistryCommand::Init { state_dir },
        } => {
            let initialized = jj_store_server::installation::initialize_registry(&state_dir)?;
            let text = format!(
                "Initialized authenticated repository registry at {}.\n",
                initialized.state_dir.display()
            );
            emit(
                ui,
                format,
                no_pager,
                serde_json::to_value(initialized)?,
                &text,
            )
        }
        ServerCommand::Config {
            command: ConfigCommand::Example,
        } => emit(
            ui,
            format,
            no_pager,
            json!({"version":1,"toml":jj_store_server::config::EXAMPLE}),
            jj_store_server::config::EXAMPLE,
        ),
        ServerCommand::Config {
            command: ConfigCommand::Check { config },
        } => {
            let config = Config::load(&config)?;
            emit(
                ui,
                format,
                no_pager,
                json!({"version":1,"valid":true,"configuration":config,"storage_readiness":"not checked"}),
                "Configuration is valid; storage readiness has not been checked.\n",
            )
        }
        ServerCommand::Doctor { config, repo_id } => {
            let config = Config::load(&config)?;
            match &config.access {
                jj_store_server::config::Access::LocalTrusted { rpc_socket, .. } => {
                    let heads = Connection::new_local(rpc_socket.clone(), repo_id)?
                        .connect()
                        .await?
                        .heads()
                        .await?;
                    let search = jj_store_server::search::diagnose_local(&config, repo_id).await;
                    let text = format!(
                        "Authoritative repository metadata is accessible. Search: {}.\n",
                        search.state.replace('_', " ")
                    );
                    emit(
                        ui,
                        format,
                        no_pager,
                        json!({"version":1,"repository":repo_id,"storage_ready":true,"heads":heads,"adapters":if search.configured {"search configured"} else {"not configured"},"search":search}),
                        &text,
                    )
                }
                jj_store_server::config::Access::Authenticated { .. } => {
                    let connection = Connection::new(
                        &config.public_url,
                        repo_id,
                        config.insecure_loopback,
                        None,
                    )?;
                    let report =
                        process::doctor(&config, repo_id, credentials::resolve(&connection)?)
                            .await?;
                    let text = format!(
                        "Authoritative repository metadata is accessible. Search: {}.\n",
                        report.search.state.replace('_', " ")
                    );
                    emit(ui, format, no_pager, serde_json::to_value(report)?, &text)
                }
            }
        }
        ServerCommand::Operator { command } => {
            let initialize = matches!(&command, OperatorCommand::Init { .. });
            match command {
                OperatorCommand::Init {
                    policy,
                    endpoint,
                    insecure_loopback,
                    principal,
                    permission,
                }
                | OperatorCommand::Grant {
                    policy,
                    endpoint,
                    insecure_loopback,
                    principal,
                    permission,
                } => {
                    let endpoint = canonical_endpoint(&endpoint, insecure_loopback)?;
                    let file = PolicyFile::new(policy)?;
                    let token = credentials::operator_provision(&endpoint)?;
                    if initialize {
                        file.initialize_permission(principal, &token, permission.into())?;
                    } else {
                        file.grant_permission(principal, &token, permission.into())?;
                    }
                    emit(
                        ui,
                        format,
                        no_pager,
                        json!({"version":1,"principal":principal,"token_id":token.id(),"endpoint":endpoint}),
                        &format!(
                            "Operator verifier stored. Principal: {principal}\nToken ID: {}\n",
                            token.id()
                        ),
                    )
                }
                OperatorCommand::List { policy } => {
                    let policy = PolicyFile::new(policy)?.read()?;
                    let tokens = policy.tokens();
                    let text = format!(
                        "Operator grants: {}\n{}",
                        tokens.len(),
                        tokens
                            .iter()
                            .map(|t| format!("  {} principal {}\n", t.id, t.principal))
                            .collect::<String>()
                    );
                    emit(
                        ui,
                        format,
                        no_pager,
                        json!({"version":1,"tokens":tokens}),
                        &text,
                    )
                }
                OperatorCommand::Revoke { policy, token_id } => {
                    let removed = PolicyFile::new(policy)?.revoke(token_id)?;
                    emit(
                        ui,
                        format,
                        no_pager,
                        json!({"version":1,"token_id":token_id,"removed":removed}),
                        &format!(
                            "Operator token {token_id}: {}\n",
                            if removed { "revoked" } else { "already absent" }
                        ),
                    )
                }
            }
        }
        ServerCommand::Version => emit(
            ui,
            format,
            no_pager,
            json!({"server":{"name":"jj-store","version":env!("CARGO_PKG_VERSION"),"upstream_jj":"0.45.1","source_revision":env!("JJ_STORE_REVISION"),"build_target":env!("JJ_STORE_TARGET"),"build_profile":env!("JJ_STORE_PROFILE")}}),
            &format!(
                "Server: jj-store {}\nUpstream JJ: 0.45.1\nSource revision: {}\nBuild target: {}\nBuild profile: {}\n",
                env!("CARGO_PKG_VERSION"),
                env!("JJ_STORE_REVISION"),
                env!("JJ_STORE_TARGET"),
                env!("JJ_STORE_PROFILE")
            ),
        ),
        ServerCommand::Completion { shell } => {
            clap_complete::generate(
                shell,
                &mut ServerCli::command(),
                "jj-store",
                &mut std::io::stdout(),
            );
            Ok(())
        }
        ServerCommand::Help { command, keyword } => {
            if let Some(keyword) = keyword {
                let doc = match keyword.as_str() {
                    "commands" => include_str!("../../../docs/cli/server-commands.md"),
                    "config" => include_str!("../../../docs/cli/server-config.md"),
                    "credentials" => include_str!("../../../docs/cli/server-operator.md"),
                    "pagination" => include_str!("../../../docs/cli/pagination.md"),
                    _ => unreachable!(),
                };
                page(ui, &presentation::markdown(doc, ui.color()), no_pager)?;
                Ok(())
            } else {
                help(ui, no_pager, &command)
            }
        }
    }
}

fn embedded_runtime(explicit: Option<PathBuf>) -> AuthorityResult<PathBuf> {
    let executable = std::env::current_exe()?;
    let mut candidates = Vec::new();
    if let Some(path) = explicit {
        candidates.push(path);
    }
    if let Some(path) = std::env::var_os("JJ_STORE_RUSTFS_RUNTIME") {
        candidates.push(path.into());
    }
    if let Some(parent) = executable.parent() {
        candidates.push(parent.join("jj-store-rustfs"));
        if let Some(root) = parent.parent().and_then(std::path::Path::parent) {
            candidates.push(root.join("runtime/embedded-rustfs/target/debug/jj-store-rustfs"));
        }
    }
    candidates
        .into_iter()
        .find_map(|path| path.canonicalize().ok().filter(|path| path.is_file()))
        .ok_or_else(|| {
            "embedded RustFS runtime not found; install the paired executable or pass --rustfs-executable"
                .into()
        })
}

fn page(ui: &Ui, text: &str, no_pager: bool) -> AuthorityResult<()> {
    presentation::page(ui, text, no_pager).map_err(|_| "terminal output failed".into())
}

/// Upstream CliRunner installs a process-exiting signal handler. On Unix,
/// replacing the process resets that handler and lets the foreground server own
/// graceful shutdown. The sibling binary is resolved from this build, not PATH.
pub fn serve_local(ui: &Ui, path: &std::path::Path, format: Format) -> AuthorityResult<()> {
    let config = Config::load(path)?;
    if !config.bind.ip().is_loopback() {
        return Err("serve-local requires a literal loopback listener".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        let executable = std::env::current_exe()?
            .parent()
            .ok_or("missing executable directory")?
            .join("jj-store");
        let error = std::process::Command::new(executable)
            .args([
                "--no-pager",
                "--color",
                if ui.color() { "always" } else { "never" },
                "--format",
                if format == Format::Json {
                    "json"
                } else {
                    "text"
                },
                "run",
                "--loopback-only",
                "--config",
            ])
            .arg(path)
            .exec();
        Err(format!(
            "could not start paired jj-store binary: {error}; build both development executables"
        )
        .into())
    }
    #[cfg(not(unix))]
    {
        let _ = (ui, format);
        Err("serve-local process handoff is not qualified on this platform; run the paired jj-store run --loopback-only command".into())
    }
}
