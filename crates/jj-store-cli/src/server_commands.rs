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
use jj_store_server::{
    config::Config,
    policy::PolicyFile,
    process::{self, PreparedServer},
};
use serde_json::json;
use std::{io::Write as _, path::PathBuf};

#[derive(Debug, Parser)]
#[command(name="jj-store",disable_help_subcommand=true,about="Serve authenticated native JJ repositories",long_about=include_str!("../../../docs/cli/server.md"))]
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
    /// Run the configured service in the foreground
    #[command(long_about=include_str!("../../../docs/cli/server-run.md"))]
    Run {
        #[arg(long)]
        config: PathBuf,
        /// Require a literal loopback bind address
        #[arg(long)]
        loopback_only: bool,
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
            eprintln!(
                "Error: {error}\nHint: Use 'jj-store help -k config'; verify paths, credentials and the selected repository."
            );
            1
        }
    }
}
fn emit(
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
fn help(ui: &Ui, no_pager: bool, path: &[String]) -> AuthorityResult<()> {
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
    let config = Config::load(path)?;
    let server = PreparedServer::prepare(&config, local).await?;
    let address = server.address()?;
    emit(
        ui,
        format,
        true,
        json!({"version":1,"event":"listening","address":address,"transport":if config.tls.is_some(){"tls"}else{"explicit-loopback"},"storage_readiness":"unverified"}),
        &format!(
            "Listening on {address}\nStorage readiness: use jj-store doctor for a known repository.\n"
        ),
    )?;
    ui.stdout().flush()?;
    server.run(process::shutdown_signal()).await?;
    Ok(())
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
        ServerCommand::Run {
            config,
            loopback_only,
        } => serve(ui, &config, format, loopback_only).await,
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
            let connection =
                Connection::new(&config.public_url, repo_id, config.insecure_loopback, None)?;
            let report =
                process::doctor(&config, repo_id, credentials::resolve(&connection)?).await?;
            emit(
                ui,
                format,
                no_pager,
                serde_json::to_value(report)?,
                "Authoritative repository metadata is accessible. Adapters: not configured.\n",
            )
        }
        ServerCommand::Operator { command } => {
            let initialize = matches!(&command, OperatorCommand::Init { .. });
            match command {
                OperatorCommand::Init {
                    policy,
                    endpoint,
                    insecure_loopback,
                    principal,
                }
                | OperatorCommand::Grant {
                    policy,
                    endpoint,
                    insecure_loopback,
                    principal,
                } => {
                    let endpoint = canonical_endpoint(&endpoint, insecure_loopback)?;
                    let file = PolicyFile::new(policy)?;
                    let token = credentials::operator_provision(&endpoint)?;
                    if initialize {
                        file.initialize(principal, &token)?;
                    } else {
                        file.grant(principal, &token)?;
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
