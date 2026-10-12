//! Explicit operator enrollment; journals contain only public intent metadata.
use clap::{Args, Subcommand};
use jj_cli::ui::Ui;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{PrincipalId, RequestId, TokenId};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};
use std::path::PathBuf;
#[derive(Debug, Args)]
pub struct IdentityArgs {
    #[arg(long)]
    endpoint: String,
    #[arg(long)]
    insecure_loopback: bool,
    #[arg(long)]
    ca_certificate: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    List,
    Grant {
        #[arg(long)]
        principal: PrincipalId,
        #[arg(long)]
        expires_at: Option<u64>,
        #[arg(long)]
        token_stdin: bool,
        #[arg(long)]
        request: PathBuf,
    },
    Revoke {
        #[arg(long)]
        token_id: TokenId,
        #[arg(long)]
        request: PathBuf,
    },
    Status {
        #[arg(long)]
        request: PathBuf,
    },
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Journal {
    format: u32,
    endpoint: String,
    request_id: RequestId,
    intent: Value,
}
pub async fn run(ui: &Ui, args: IdentityArgs) -> AuthorityResult<(Value, String)> {
    let url = url::Url::parse(&args.endpoint)?;
    let loopback = match url.host() {
        Some(url::Host::Ipv4(v)) => v.is_loopback(),
        Some(url::Host::Ipv6(v)) => v.is_loopback(),
        _ => false,
    };
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || !(url.scheme() == "https"
            || (url.scheme() == "http" && loopback && args.insecure_loopback))
    {
        return Err("invalid identity endpoint".into());
    }
    let endpoint = url.origin().ascii_serialization();
    let operator = crate::credentials::operator_resolve(&endpoint)?;
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30));
    if let Some(path) = args.ca_certificate {
        builder =
            builder.add_root_certificate(reqwest::Certificate::from_pem(&std::fs::read(path)?)?);
    }
    let client = builder.build()?;
    let (path, intent, secret, status) = match args.command {
        Command::List => (None, None, None, false),
        Command::Grant {
            principal,
            expires_at,
            token_stdin,
            request,
        } => {
            let token = crate::token_journal::read_grant(ui, token_stdin)?;
            let fingerprint = hex::encode(Sha256::digest(token.expose().as_bytes()));
            (
                Some(request),
                Some(
                    json!({"type":"grant","principal":principal,"token":token.id(),"fingerprint":fingerprint,"expires_at":expires_at}),
                ),
                Some(token),
                false,
            )
        }
        Command::Revoke { token_id, request } => (
            Some(request),
            Some(json!({"type":"revoke","token":token_id})),
            None,
            false,
        ),
        Command::Status { request } => (Some(request), None, None, true),
    };
    let mut body: Option<zeroize::Zeroizing<Vec<u8>>> = None;
    let route = if let Some(path) = path {
        let path = jj_store_server::policy::outside_checkout(&path)?;
        let parent = path.parent().ok_or("identity journal parent")?;
        if intent.is_some() {
            std::fs::create_dir_all(parent)?;
        }
        let _lease = jj_store_server::policy::exclusive_lock(&path.with_extension("lock"))?;
        let journal = if path.exists() {
            let bytes = jj_store_server::policy::read_regular(&path, 16 * 1024)?;
            let journal: Journal = serde_json::from_slice(&bytes)?;
            if journal.format != 1
                || journal.endpoint != endpoint
                || intent.as_ref().is_some_and(|i| i != &journal.intent)
            {
                return Err("identity retry differs from original intent".into());
            }
            journal
        } else {
            let intent = intent.ok_or("identity journal missing")?;
            let mut id = [0u8; 16];
            getrandom::fill(&mut id)?;
            let journal = Journal {
                format: 1,
                endpoint: endpoint.clone(),
                request_id: RequestId::new(id),
                intent,
            };
            use std::io::Write as _;
            let mut file = tempfile::NamedTempFile::new_in(parent)?;
            file.write_all(&serde_json::to_vec(&journal)?)?;
            file.as_file().sync_all()?;
            file.persist(&path)?;
            std::fs::File::open(parent)?.sync_all()?;
            journal
        };
        if status {
            format!("commands/{}", journal.request_id)
        } else if journal.intent["type"] == "grant" {
            let credential = secret.ok_or("grant credential missing")?.expose();
            #[derive(Serialize)]
            struct Enrollment<'a> {
                request_id: RequestId,
                principal: &'a Value,
                expires_at: &'a Value,
                credential: &'a str,
            }
            body = Some(zeroize::Zeroizing::new(serde_json::to_vec(&Enrollment {
                request_id: journal.request_id,
                principal: &journal.intent["principal"],
                expires_at: &journal.intent["expires_at"],
                credential: credential.as_str(),
            })?));
            "identities".into()
        } else {
            body = Some(zeroize::Zeroizing::new(serde_json::to_vec(
                &json!({"request_id":journal.request_id,"token":journal.intent["token"]}),
            )?));
            "identity-revoke".into()
        }
    } else {
        "identities".into()
    };
    let mut request = client
        .request(
            if body.is_some() {
                reqwest::Method::POST
            } else {
                reqwest::Method::GET
            },
            format!("{endpoint}/v1/directory/{route}"),
        )
        .bearer_auth(operator.expose().as_str())
        .header("x-jj-store-protocol", "1");
    if let Some(bytes) = body {
        request = request
            .header("content-type", "application/json")
            .body(bytes.to_vec());
    }
    let response = request.send().await?;
    if !response.status().is_success() {
        return Err(format!(
            "identity request failed (HTTP {}); retain the request journal for status/retry",
            response.status()
        )
        .into());
    }
    let mut response = response;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > 4 * 1024 * 1024 {
            return Err("identity response limit".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    let value: Value = serde_json::from_slice(&bytes)?;
    let text = serde_json::to_string_pretty(&value)? + "\n";
    Ok((value, text))
}
