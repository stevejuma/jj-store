//! Durable, secret-free token request journals and exact retry execution.
use crate::binding::Connection;
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
    fs::File,
    io::{Read as _, Write as _},
    path::Path,
};
use zeroize::Zeroizing;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Intent {
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
pub(crate) struct Journal {
    pub(crate) format: u32,
    pub(crate) connection: Connection,
    pub(crate) actor: m::PrincipalId,
    pub(crate) request: m::RequestId,
    pub(crate) expected: AdministrationSnapshot,
    pub(crate) intent: Intent,
}

pub(crate) fn fingerprint(token: &BearerToken) -> String {
    let mut hash = Sha256::new_with_prefix(b"jj-store/cli-grant-fingerprint/v1");
    hash.update(token.expose().as_bytes());
    hex::encode(hash.finalize())
}

pub(crate) fn read_grant(ui: &Ui, stdin: bool) -> AuthorityResult<BearerToken> {
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

pub(crate) fn persist(path: &Path, journal: &Journal) -> AuthorityResult<()> {
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

pub(crate) fn read(path: &Path) -> AuthorityResult<Journal> {
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

pub(crate) async fn execute(
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
