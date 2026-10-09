//! Explicit LocalTrusted metadata administration. Unknown commands are never retried.
use clap::{Args, Subcommand};
use jj_cli::ui::Ui;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model::{RepositoryId, RequestId};
use jj_store_service::agent_execution::metadata;
use std::{
    io::{Read, Write},
    path::PathBuf,
};
#[derive(Debug, Args)]
pub struct MetadataArgs {
    /// Explicit literal-loopback LocalTrusted server URL with a trailing slash.
    #[arg(long)]
    endpoint: String,
    #[command(subcommand)]
    command: MetadataCommand,
}
#[derive(Debug, Subcommand)]
enum MetadataCommand {
    /// Read selected conversation details without starting inference.
    Read {
        /// Managed execution session ID, or captured session ID with --repository.
        #[arg(long)]
        session: String,
        /// Repository owning the captured session; independently authorized.
        #[arg(long)]
        repository: Option<RepositoryId>,
    },
    /// Submit one typed metadata command with its original request identity.
    Command {
        /// Bounded version-1 metadata command JSON file. Unknown outcomes are retained.
        #[arg(long)]
        input: PathBuf,
    },
    /// Inspect a retained metadata request without replaying inference.
    Job {
        /// Original metadata request ID.
        #[arg(long)]
        request: RequestId,
    },
}
pub async fn run(ui: &Ui, args: MetadataArgs) -> AuthorityResult<()> {
    let endpoint = url::Url::parse(&args.endpoint)?;
    if endpoint.scheme() != "http"
        || !matches!(endpoint.host_str(), Some("127.0.0.1" | "[::1]" | "::1"))
        || endpoint.port().is_none()
        || !endpoint.username().is_empty()
        || endpoint.password().is_some()
        || endpoint.query().is_some()
        || endpoint.fragment().is_some()
        || endpoint.path() != "/"
    {
        return Err("metadata requires an explicit literal-loopback LocalTrusted endpoint".into());
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(35))
        .build()?;
    let path = endpoint.join("v1/local/agents/metadata")?;
    let request = match args.command {
        MetadataCommand::Read {
            session,
            repository,
        } => {
            let q = metadata::Query {
                session,
                repository,
            };
            client.get(path).query(&q)
        }
        MetadataCommand::Command { input } => {
            let mut bytes = Vec::new();
            std::fs::File::open(input)?
                .take(32 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            if bytes.len() > 32 * 1024 {
                return Err("metadata command exceeds limit".into());
            }
            let command: metadata::Command = serde_json::from_slice(&bytes)?;
            client.post(path).json(&command)
        }
        MetadataCommand::Job { request } => client
            .get(endpoint.join("v1/local/agents/metadata/job")?)
            .query(&[("request", request.to_string())]),
    };
    let mut response = request.header("x-jj-store-protocol", "1").send().await?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!(
            "metadata command refused ({status}); retain the original request identity"
        )
        .into());
    }
    if response.content_length().is_some_and(|n| n > 1024 * 1024) {
        return Err("metadata response exceeds limit".into());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > 1024 * 1024 {
            return Err("metadata response exceeds limit".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    writeln!(
        ui.stdout_formatter(),
        "{}",
        serde_json::to_string_pretty(&value)?
    )?;
    Ok(())
}
