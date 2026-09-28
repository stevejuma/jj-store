//! Shared typed token administration with durable, secret-free retry intents.
use crate::{binding::Connection, credentials, store_commands::ConnectionArgs};
use clap::{Args, Subcommand, ValueEnum};
use jj_cli::ui::Ui;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model as m;
use jj_store_rpc::client::NativeClient;
use jj_store_service::{
    administration::{AdministrationSnapshot, TokenChange, TokenRequest},
    auth::{BearerToken, Permission},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::{
    fs::{self, File},
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
};
use zeroize::Zeroizing;

#[derive(Clone, Debug, Args)]
pub struct TokenArgs {
    #[command(flatten)]
    connection: ConnectionArgs,
    #[command(subcommand)]
    command: TokenCommand,
}
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum TokenPermission {
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
pub enum TokenCommand {
    /// List current public grant metadata and the exact policy snapshot
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    List,
    /// Grant a client-generated credential at the selected snapshot
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Grant {
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
        #[arg(long)]
        token_id: m::TokenId,
        #[arg(long)]
        request: PathBuf,
    },
    /// Read the committed receipt without publishing
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Status {
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        token_stdin: bool,
    },
    /// Reapply the exact original snapshot and intent
    #[command(long_about=include_str!("../../../docs/cli/tokens.md"))]
    Retry {
        #[arg(long)]
        request: PathBuf,
        #[arg(long)]
        token_stdin: bool,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
enum Intent {
    Grant {
        token: m::TokenId,
        fingerprint: String,
        principal: m::PrincipalId,
        permission: Permission,
        expires_at: Option<u64>,
    },
    Revoke {
        token: m::TokenId,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    format: u32,
    connection: Connection,
    actor: m::PrincipalId,
    request: m::RequestId,
    expected: AdministrationSnapshot,
    intent: Intent,
}
fn fingerprint(token: &BearerToken) -> String {
    let mut hash = Sha256::new_with_prefix(b"jj-store/cli-grant-fingerprint/v1");
    hash.update(token.expose().as_bytes());
    hex::encode(hash.finalize())
}
fn read_grant(ui: &Ui, stdin: bool) -> AuthorityResult<BearerToken> {
    let text = if stdin {
        if std::env::var_os("JJ_STORE_GRANT_TOKEN").is_some() {
            return Err("choose stdin or JJ_STORE_GRANT_TOKEN, not both".into());
        }
        let mut bytes = Zeroizing::new(Vec::new());
        std::io::stdin().take(105).read_to_end(&mut bytes)?;
        if bytes.len() > 104 {
            return Err("grant credential input exceeds limit".into());
        }
        Zeroizing::new(
            std::str::from_utf8(&bytes)
                .map_err(|_| "invalid grant credential encoding")?
                .to_owned(),
        )
    } else if let Some(value) = std::env::var_os("JJ_STORE_GRANT_TOKEN") {
        Zeroizing::new(
            value
                .into_string()
                .map_err(|_| "invalid grant credential encoding")?,
        )
    } else {
        Zeroizing::new(ui.prompt_password("Grant credential: ").map_err(
            |_| "could not read hidden grant credential; use --token-stdin or JJ_STORE_GRANT_TOKEN",
        )?)
    };
    BearerToken::parse(text.trim()).map_err(|_| "invalid grant credential".into())
}
fn persist(path: &Path, journal: &Journal) -> AuthorityResult<()> {
    let parent = path.parent().ok_or("request file parent")?;
    let bytes = serde_json::to_vec_pretty(journal)?;
    if bytes.len() > 64 * 1024 {
        return Err("token request journal exceeds limit".into());
    }
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    file.write_all(&bytes)?;
    file.as_file().sync_all()?;
    file.persist_noclobber(path)?;
    #[cfg(unix)]
    File::open(parent)?.sync_all()?;
    Ok(())
}
fn read(path: &Path) -> AuthorityResult<Journal> {
    let journal: Journal =
        serde_json::from_slice(&jj_store_server::policy::read_regular(path, 64 * 1024)?)
            .map_err(|_| "invalid token request journal")?;
    journal.connection.validate()?;
    if journal.format != 1
        || journal.expected.heads.is_empty()
        || journal.expected.heads.len() > 256
    {
        return Err("invalid token request journal bounds".into());
    }
    if let Intent::Grant {
        fingerprint,
        permission,
        ..
    } = &journal.intent
        && (fingerprint.len() != 64
            || !fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
            || *permission == Permission::CreateRepository)
    {
        return Err("invalid token grant journal".into());
    }
    Ok(journal)
}
fn request(journal: &Journal, token: Option<BearerToken>) -> AuthorityResult<TokenRequest> {
    let change = match &journal.intent {
        Intent::Grant {
            token: id,
            fingerprint: expected,
            principal,
            permission,
            expires_at,
        } => {
            let credential = token.ok_or("grant retry requires the original credential")?;
            if credential.id() != *id || fingerprint(&credential) != *expected {
                return Err("grant credential differs from original request".into());
            }
            TokenChange::Grant {
                credential,
                principal: *principal,
                permission: *permission,
                expires_at: *expires_at,
            }
        }
        Intent::Revoke { token } => TokenChange::Revoke { token: *token },
    };
    Ok(TokenRequest {
        request_id: journal.request,
        expected: journal.expected.clone(),
        change,
    })
}
async fn execute(
    client: &NativeClient,
    journal: &Journal,
    token: Option<BearerToken>,
    status: bool,
    path: &Path,
) -> AuthorityResult<(Value, String)> {
    let request = request(journal, token)?;
    let receipt = if status {
        client.token_status(&request).await?
    } else {
        client.change_token(&request).await.map_err(|error|format!("token mutation failed ({:?}); outcome may be unknown; retain the request file and use token status/retry",error.code()))?
    };
    if receipt.is_none() && !status {
        return Err(
            "token mutation outcome unknown; retain the request file and use token status/retry"
                .into(),
        );
    }
    let state = if receipt.is_some() {
        "committed"
    } else {
        "unknown"
    };
    let text = if let Some(receipt) = &receipt {
        format!(
            "Token request {} committed at {}.\n",
            receipt.request_id, receipt.accepted.position.0
        )
    } else {
        format!(
            "Token request {}: no receipt observed; outcome remains unknown.\n",
            journal.request
        )
    };
    Ok((
        json!({"version":1,"state":state,"request_id":journal.request,"request_file":path,"receipt":receipt}),
        text,
    ))
}
pub async fn run(ui: &Ui, args: TokenArgs) -> AuthorityResult<(Value, String)> {
    let connection = args.connection.connection_raw()?;
    let admin = credentials::resolve(&connection)?;
    let client = connection.connect_with(admin.clone()).await?;
    let listing = client.tokens().await?;
    let actor = listing
        .tokens
        .iter()
        .find(|t| t.id == admin.id())
        .ok_or("admin credential is absent from current token policy")?
        .principal;
    if matches!(args.command, TokenCommand::List) {
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
        TokenCommand::List => unreachable!(),
        TokenCommand::Grant {
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
        TokenCommand::Revoke { token_id, request } => (
            request,
            Some(Intent::Revoke { token: token_id }),
            None,
            false,
            false,
        ),
        TokenCommand::Status {
            request,
            token_stdin,
        } => (request, None, None, true, token_stdin),
        TokenCommand::Retry {
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
