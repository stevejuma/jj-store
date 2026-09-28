//! Repository creation intent is persisted before any remote publication. The
//! initial bearer token is separately durable in the OS facility or injected.
use crate::{
    attachment::{self, Attachment},
    binding::Connection,
    credentials,
};
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_model as m;
use jj_store_service::auth::BearerToken;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write as _,
    path::PathBuf,
};

pub struct Options {
    pub endpoint: Option<String>,
    pub unix_socket: Option<PathBuf>,
    pub insecure_loopback: bool,
    pub ca_certificate: Option<PathBuf>,
    pub destination: PathBuf,
    pub name: Option<String>,
    pub cache: Option<PathBuf>,
    pub workspace: Option<String>,
    pub state: Option<PathBuf>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    format: u32,
    connection: Connection,
    epoch: m::EpochId,
    admin: Option<m::TokenId>,
    destination: PathBuf,
    cache: PathBuf,
    workspace: Option<String>,
    name: String,
    created: bool,
}
pub struct Initialization {
    record: Record,
    state: PathBuf,
    token: Option<BearerToken>,
    _lock: File,
}
impl Initialization {
    pub fn open(options: Options) -> AuthorityResult<Self> {
        let destination = attachment::target(&options.destination)?;
        let name = options.name.unwrap_or_else(|| {
            destination
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("repository")
                .to_owned()
        });
        let state = jj_store_server::policy::outside_checkout(&attachment::state_path(
            &destination,
            options.cache.as_deref(),
            options.state,
        )?)?;
        let cache = attachment::target(&options.cache.unwrap_or_else(|| state.join("cache")))?;
        if cache.starts_with(&destination)
            || destination.starts_with(&cache)
            || state.starts_with(&destination)
            || destination.starts_with(&state)
            || !jj_store_server::registry::valid_name(&name)
            || options.workspace.as_ref().is_some_and(|name| {
                name.is_empty()
                    || name.len() > 128
                    || name
                        .bytes()
                        .any(|c| !c.is_ascii_alphanumeric() && !b"-_".contains(&c))
            })
        {
            return Err("invalid initialization paths or workspace name".into());
        }
        // Validate endpoint/CA before allocating IDs, credentials or remote state.
        let ca = options
            .ca_certificate
            .map(|p| p.canonicalize())
            .transpose()?;
        let probe = match (options.endpoint.as_deref(), options.unix_socket) {
            (Some(endpoint), None) => Connection::new(
                endpoint,
                m::RepositoryId::new([0; 16]),
                options.insecure_loopback,
                ca,
            )?,
            (None, Some(socket)) if !options.insecure_loopback && ca.is_none() => {
                Connection::new_local(attachment::target(&socket)?, m::RepositoryId::new([0; 16]))?
            }
            _ => {
                return Err(
                    "select exactly one authenticated endpoint or local Unix socket".into(),
                );
            }
        };
        fs::create_dir_all(&state)?;
        let lock = jj_store_server::policy::exclusive_lock(&state.join("creation.lock"))?;
        let path = state.join("creation.json");
        let (record, token) = if path.exists() {
            let record: Record =
                serde_json::from_slice(&jj_store_server::policy::read_regular(&path, 64 * 1024)?)
                    .map_err(|_| "invalid repository creation journal")?;
            record.connection.validate()?;
            if record.format != 1
                || record.destination != destination
                || record.cache != cache
                || record.workspace != options.workspace
                || record.name != name
                || record.connection.endpoint != probe.endpoint
                || record.connection.insecure_loopback != probe.insecure_loopback
                || record.connection.ca_certificate != probe.ca_certificate
            {
                return Err("initialization retry differs from durable creation intent".into());
            }
            let token = if record.connection.unix_socket.is_some() {
                if record.admin.is_some() {
                    return Err("local creation intent contains a bearer credential binding".into());
                }
                None
            } else {
                let token = credentials::resolve(&record.connection)?;
                if Some(token.id()) != record.admin {
                    return Err("initial admin credential differs from creation intent".into());
                }
                Some(token)
            };
            (record, token)
        } else {
            attachment::empty(&destination)?;
            attachment::empty(&cache)?;
            if state.join("attachment.json").exists() {
                return Err("this state directory already owns an attachment".into());
            }
            let mut id = [0; 16];
            getrandom::fill(&mut id)?;
            let mut connection = probe;
            connection.repository = m::RepositoryId::new(id);
            getrandom::fill(&mut id)?;
            let epoch = m::EpochId::new(id);
            let token = if connection.unix_socket.is_some() {
                None
            } else {
                Some(credentials::provision(&connection)?)
            };
            (
                Record {
                    format: 1,
                    connection,
                    epoch,
                    admin: token.as_ref().map(BearerToken::id),
                    destination,
                    cache,
                    workspace: options.workspace,
                    name,
                    created: false,
                },
                token,
            )
        };
        let this = Self {
            record,
            state,
            token,
            _lock: lock,
        };
        this.save()?;
        Ok(this)
    }
    fn save(&self) -> AuthorityResult<()> {
        let mut file = tempfile::NamedTempFile::new_in(&self.state)?;
        file.write_all(&serde_json::to_vec_pretty(&self.record)?)?;
        file.as_file().sync_all()?;
        file.persist(self.state.join("creation.json"))?;
        #[cfg(unix)]
        File::open(&self.state)?.sync_all()?;
        Ok(())
    }
    pub async fn create(&mut self) -> AuthorityResult<()> {
        if !self.record.created {
            // A prior response may have been lost and the operator may since
            // have been revoked. Current admin access plus exact epoch proves
            // the intended repository already exists without another creation.
            if let Some(token) = &self.token {
                let admin = self.record.connection.connect_with(token.clone()).await?;
                if let Ok(listing) = admin.tokens().await {
                    if listing.snapshot.epoch != self.record.epoch {
                        return Err("existing repository epoch differs from creation intent".into());
                    }
                } else {
                    let operator = credentials::operator_resolve(&self.record.connection.endpoint)?;
                    let client = self.record.connection.connect_with(operator).await?;
                    client
                        .create_repository_named(self.record.epoch, token, &self.record.name)
                        .await?;
                }
            } else {
                let local = self.record.connection.connect().await?;
                match local.heads().await {
                    Ok(_) => {}
                    Err(_) => {
                        local
                            .create_repository_local_named(self.record.epoch, &self.record.name)
                            .await?;
                    }
                }
            }
            self.record.created = true;
            self.save()?;
        }
        Ok(())
    }
    pub async fn attachment(&self) -> AuthorityResult<Attachment> {
        if !self.record.created {
            return Err("repository creation has not been acknowledged".into());
        }
        match &self.token {
            Some(token) => {
                Attachment::open(
                    self.record.connection.clone(),
                    token.clone(),
                    self.record.destination.clone(),
                    Some(self.record.cache.clone()),
                    self.record.workspace.clone(),
                    Some(self.state.clone()),
                )
                .await
            }
            None => {
                Attachment::open_local(
                    self.record.connection.clone(),
                    self.record.destination.clone(),
                    Some(self.record.cache.clone()),
                    self.record.workspace.clone(),
                    Some(self.state.clone()),
                )
                .await
            }
        }
    }
}
