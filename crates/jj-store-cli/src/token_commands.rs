//! Shared typed token administration with durable, secret-free retry intents.
use crate::{
    credentials,
    store_commands::ConnectionArgs,
    token_journal::{Intent, Journal, execute, fingerprint, persist, read, read_grant},
};
use clap::{Args, Subcommand, ValueEnum};
use jj_cli::ui::Ui;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model as m;
use jj_store_service::auth::{BearerToken, Permission};
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    command: TokenCommand,
}
#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum TokenPermission {
    Read,
    Write,
    Admin,
}
impl From<TokenPermission> for Permission {
    fn from(value: TokenPermission) -> Self {
        match value {
            TokenPermission::Read => Self::Read,
            TokenPermission::Write => Self::Write,
            TokenPermission::Admin => Self::Admin,
        }
    }
}
#[derive(Clone, Debug, Subcommand)]
pub(crate) enum TokenCommand {
    /// Generate one manual token locally without storing it
    Generate,
    /// List current public grant metadata and the exact policy snapshot
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    List {
        #[command(flatten)]
        connection: ConnectionArgs,
    },
    /// Grant a client-generated credential at the selected snapshot
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Grant {
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        principal: m::PrincipalId,
        #[arg(long, value_enum)]
        permission: TokenPermission,
        #[arg(long)]
        expires_at: Option<u64>,
        #[arg(long)]
        token_stdin: bool,
        #[arg(long)]
        request: PathBuf,
    },
    /// Revoke one repository credential at the selected snapshot
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Revoke {
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        token_id: m::TokenId,
        #[arg(long)]
        request: PathBuf,
    },
    /// Read the committed receipt without publishing
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Status {
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        token_stdin: bool,
    },
    /// Reapply the exact original snapshot and intent
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Retry {
        #[command(flatten)]
        connection: ConnectionArgs,
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        token_stdin: bool,
    },
}
pub async fn run(ui: &Ui, args: TokenArgs) -> AuthorityResult<(Value, String)> {
    if matches!(args.command, TokenCommand::Generate) {
        let token = BearerToken::generate()?;
        let credential = token.expose();
        return Ok((
            json!({"version":1,"token_id":token.id(),"credential":credential.as_str()}),
            format!(
                "Token ID: {}\nCredential (shown once): {}\n",
                token.id(),
                credential.as_str()
            ),
        ));
    }
    let connection_args = match &args.command {
        TokenCommand::Generate => unreachable!(),
        TokenCommand::List { connection }
        | TokenCommand::Grant { connection, .. }
        | TokenCommand::Revoke { connection, .. }
        | TokenCommand::Status { connection, .. }
        | TokenCommand::Retry { connection, .. } => connection,
    };
    let connection = connection_args.connection_raw()?;
    let admin = credentials::resolve(&connection)?;
    let client = connection.connect_with(admin.clone()).await?;
    let listing = client.tokens().await?;
    let actor = listing
        .tokens
        .iter()
        .find(|t| t.id == admin.id())
        .ok_or("admin credential is absent from current token policy")?
        .principal;
    if matches!(args.command, TokenCommand::List { .. }) {
        let text = format!(
            "Repository: {}\nSnapshot: {}\n{}",
            connection.repository,
            listing.snapshot.position.0,
            listing
                .tokens
                .iter()
                .map(|t| format!(
                    "  {} principal {} permission {} expires {}\n",
                    t.id,
                    t.principal,
                    match t.permission {
                        Permission::Read => "read",
                        Permission::Write => "write",
                        Permission::Admin => "admin",
                        Permission::CreateRepository => "create-repository",
                        Permission::ManageIdentities => "manage-identities",
                    },
                    t.expires_at
                        .map_or_else(|| "never".into(), |v| format!("{v} (Unix seconds)"))
                ))
                .collect::<String>()
        );
        return Ok((
            json!({"version":1,"repository":connection.repository,"snapshot":listing.snapshot,"tokens":listing.tokens}),
            text,
        ));
    }
    let (path, proposed, mut token, status, stdin) = match args.command {
        TokenCommand::Generate | TokenCommand::List { .. } => unreachable!(),
        TokenCommand::Grant {
            connection: _,
            principal,
            permission,
            expires_at,
            token_stdin,
            request,
        } => {
            let token = read_grant(ui, token_stdin)?;
            let intent = Intent::Grant {
                token: token.id(),
                fingerprint: fingerprint(&token),
                principal,
                permission: permission.into(),
                expires_at,
            };
            (request, Some(intent), Some(token), false, false)
        }
        TokenCommand::Revoke {
            connection: _,
            token_id,
            request,
        } => (
            request,
            Some(Intent::Revoke { token: token_id }),
            None,
            false,
            false,
        ),
        TokenCommand::Status {
            connection: _,
            request,
            token_stdin,
        } => (request, None, None, true, token_stdin),
        TokenCommand::Retry {
            connection: _,
            request,
            token_stdin,
        } => (request, None, None, false, token_stdin),
    };
    let path = jj_store_server::policy::outside_checkout(&path)?;
    let parent = path.parent().ok_or("request file parent")?;
    if proposed.is_some() {
        fs::create_dir_all(parent)?;
    }
    let mut name = path.as_os_str().to_owned();
    name.push(".lock");
    let _lock = jj_store_server::policy::exclusive_lock(Path::new(&name))?;
    let journal = if path.exists() {
        let journal = read(&path)?;
        if journal.actor != actor
            || serde_json::to_value(&journal.connection)? != serde_json::to_value(&connection)?
            || proposed
                .as_ref()
                .is_some_and(|intent| intent != &journal.intent)
        {
            return Err("token retry differs from original repository, principal or intent".into());
        }
        journal
    } else {
        let intent = proposed.ok_or("token request journal is missing")?;
        let mut id = [0; 16];
        getrandom::fill(&mut id)?;
        let journal = Journal {
            format: 1,
            connection,
            actor,
            request: m::RequestId::new(id),
            expected: listing.snapshot,
            intent,
        };
        persist(&path, &journal)?;
        journal
    };
    if matches!(journal.intent, Intent::Grant { .. }) && token.is_none() {
        token = Some(read_grant(ui, stdin)?);
    }
    if matches!(journal.intent, Intent::Revoke { .. }) && stdin {
        return Err("revoke retry does not use credential input".into());
    }
    execute(&client, &journal, token, status, &path).await
}
