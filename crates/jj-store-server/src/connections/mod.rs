//! Registered-origin gateway. No browser CORS or LocalTrusted authority upstream.
#[cfg(test)]
mod event_tests;
mod events;
#[cfg(test)]
mod tests;
mod transport;
use jj_store_service::auth::BearerToken;
use jj_store_service::connections::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Manager(Arc<Inner>);
struct Inner {
    root: PathBuf,
    loopback: bool,
    client: reqwest::Client,
    state: Mutex<State>,
    slots: Arc<tokio::sync::Semaphore>,
}
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    connections: BTreeMap<String, Connection>,
}
struct Session {
    connection: String,
    token: BearerToken,
    closed: tokio::sync::watch::Sender<bool>,
}
struct State {
    saved: Saved,
    sessions: BTreeMap<String, Arc<Session>>,
}
impl Manager {
    pub fn open(root: PathBuf, loopback: bool) -> Result<Self> {
        std::fs::create_dir_all(&root).map_err(|_| Error::Unavailable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
                .map_err(|_| Error::Unavailable)?;
        }
        let file = root.join("connections.json");
        let saved: Saved = match std::fs::read(file) {
            Ok(bytes) if bytes.len() <= 65536 => {
                serde_json::from_slice(&bytes).map_err(|_| Error::Unavailable)?
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Saved::default(),
            _ => return Err(Error::Unavailable),
        };
        if saved.connections.len() > 16 {
            return Err(Error::Limit);
        }
        for (id, value) in &saved.connections {
            if id != &value.id
                || !valid_id(id)
                || !valid_id(&value.installation)
                || origin(&value.origin, loopback)? != value.origin
                || !valid_name(&value.name)
            {
                return Err(Error::Invalid);
            }
        }
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|_| Error::Unavailable)?;
        Ok(Self(Arc::new(Inner {
            root,
            loopback,
            client,
            state: Mutex::new(State {
                saved,
                sessions: BTreeMap::new(),
            }),
            slots: Arc::new(tokio::sync::Semaphore::new(16)),
        })))
    }
    fn persist(&self, saved: &Saved) -> Result<()> {
        use std::io::Write;
        let mut temp =
            tempfile::NamedTempFile::new_in(&self.0.root).map_err(|_| Error::Unavailable)?;
        serde_json::to_writer(&mut temp, saved).map_err(|_| Error::Unavailable)?;
        temp.flush().map_err(|_| Error::Unavailable)?;
        temp.as_file().sync_all().map_err(|_| Error::Unavailable)?;
        temp.persist(self.0.root.join("connections.json"))
            .map_err(|_| Error::Unavailable)?;
        std::fs::File::open(&self.0.root)
            .and_then(|f| f.sync_all())
            .map_err(|_| Error::Unavailable)
    }
}
fn valid_id(id: &str) -> bool {
    id.len() == 32
        && id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn valid_name(name: &str) -> bool {
    !name.trim().is_empty() && name.len() <= 128 && !name.chars().any(char::is_control)
}
fn identity() -> Result<String> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
    Ok(hex::encode(bytes))
}
fn origin(value: &str, loopback: bool) -> Result<String> {
    let url = url::Url::parse(value).map_err(|_| Error::Invalid)?;
    let local = url.host_str().is_some_and(|host| {
        host.trim_matches(['[', ']'])
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback())
    });
    if !(url.scheme() == "https" || loopback && local && url.scheme() == "http")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || url.host_str().is_none()
    {
        return Err(Error::Invalid);
    }
    Ok(url.origin().ascii_serialization())
}
#[async_trait::async_trait]
impl Connections for Manager {
    async fn list(&self) -> Result<Vec<Connection>> {
        Ok(self
            .0
            .state
            .lock()
            .await
            .saved
            .connections
            .values()
            .cloned()
            .collect())
    }
    async fn connect(&self, request: Connect) -> Result<Connected> {
        if !valid_name(&request.name) {
            return Err(Error::Invalid);
        }
        let target = origin(&request.origin, self.0.loopback)?;
        let token = BearerToken::parse(&request.token).map_err(|_| Error::Denied)?;
        let _permit = self
            .0
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| Error::Limit)?;
        let bootstrap = self.probe(&target, "bootstrap", None).await?;
        if bootstrap["access_mode"] != "authenticated" || bootstrap["version"] != 1 {
            return Err(Error::Denied);
        }
        let installation = bootstrap["installation_id"]
            .as_str()
            .filter(|v| valid_id(v))
            .ok_or(Error::Invalid)?
            .to_owned();
        let profile = self
            .probe(&target, "directory/profile", Some(&token))
            .await?;
        let principal = profile["principal"]
            .as_str()
            .ok_or(Error::Denied)?
            .parse()
            .map_err(|_| Error::Denied)?;
        let mut state = self.0.state.lock().await;
        let id = if let Some(id) = request.id {
            let old = state.saved.connections.get(&id).ok_or(Error::Invalid)?;
            if old.origin != target || old.installation != installation {
                return Err(Error::Identity);
            }
            id
        } else {
            if state.saved.connections.len() >= 16
                || state.saved.connections.values().any(|c| c.origin == target)
            {
                return Err(Error::Limit);
            }
            identity()?
        };
        if state.sessions.len() >= 64 {
            return Err(Error::Limit);
        }
        let connection = Connection {
            id: id.clone(),
            name: request.name.trim().into(),
            origin: target,
            installation,
        };
        let mut saved = Saved {
            connections: state.saved.connections.clone(),
        };
        saved.connections.insert(id.clone(), connection.clone());
        self.persist(&saved)?;
        state.saved = saved;
        let handle = identity()?;
        let (closed, _) = tokio::sync::watch::channel(false);
        state.sessions.insert(
            handle.clone(),
            Arc::new(Session {
                connection: id,
                token,
                closed,
            }),
        );
        Ok(Connected {
            connection,
            handle,
            principal,
        })
    }
    async fn edit(&self, request: Edit) -> Result<Vec<Connection>> {
        let mut state = self.0.state.lock().await;
        let mut saved = Saved {
            connections: state.saved.connections.clone(),
        };
        match request {
            Edit::Rename { id, name } => {
                if !valid_name(&name) {
                    return Err(Error::Invalid);
                }
                saved.connections.get_mut(&id).ok_or(Error::Invalid)?.name = name.trim().into();
            }
            Edit::Remove { id } => {
                saved.connections.remove(&id).ok_or(Error::Invalid)?;
            }
        }
        self.persist(&saved)?;
        state.saved = saved;
        let valid = state
            .saved
            .connections
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        state.sessions.retain(|_, session| {
            let keep = valid.contains(&session.connection);
            if !keep {
                session.closed.send_replace(true);
            }
            keep
        });
        Ok(state.saved.connections.values().cloned().collect())
    }
    async fn disconnect(&self, handle: &str) -> Result<()> {
        if let Some(session) = self.0.state.lock().await.sessions.remove(handle) {
            session.closed.send_replace(true);
        }
        Ok(())
    }
    async fn forward(&self, request: Forward) -> Result<Response> {
        self.transfer(request).await
    }
    async fn events(
        &self,
        connection: &str,
        handle: &str,
        after: Option<String>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        self.follow(connection, handle, after).await
    }
}
