//! Owner-only local IPC; each random capability grants one registration to one worker.
use super::*;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
#[derive(Clone)]
pub(super) struct Lease {
    pub account: String,
    pub worker: String,
    pub generation: u64,
    pub cancel: tokio::sync::watch::Sender<bool>,
}
pub(in crate::agent_execution) struct Guard {
    owner: Supervisor,
    capability: String,
    cancelled: tokio::sync::watch::Receiver<bool>,
}
impl Drop for Guard {
    fn drop(&mut self) {
        if let Ok(mut leases) = self.owner.0.chatgpt.leases.lock() {
            leases.remove(&self.capability);
        }
    }
}
impl Supervisor {
    pub(in crate::agent_execution) async fn chatgpt_launch(
        &self,
        command: &mut tokio::process::Command,
        setup: &Setup,
        worker: &str,
        generation: u64,
    ) -> Result<Option<Guard>> {
        if setup.runner != "local" || setup.options.provider.as_deref() != Some("chatgpt-plan") {
            return Ok(None);
        }
        let account = setup.options.account.clone().ok_or(Error::Invalid)?;
        let record = self.chatgpt_record(&account)?;
        if !record.view.connected || !record.view.plan_enabled {
            return Err(Error::Denied);
        }
        let mut broker = self.0.chatgpt.broker.lock().await;
        let path = if let Some(path) = broker.as_ref() {
            path.clone()
        } else {
            let directory = tempfile::Builder::new()
                .prefix("jj-cg-")
                .tempdir()
                .map_err(|_| Error::Unavailable)?;
            let path = directory.path().join("token.sock");
            let listener = tokio::net::UnixListener::bind(&path).map_err(|_| Error::Unavailable)?;
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
                .map_err(|_| Error::Unavailable)?;
            let this = self.clone();
            let task = tokio::spawn(async move {
                let _directory = directory;
                let slots = Arc::new(tokio::sync::Semaphore::new(8));
                let mut clients = tokio::task::JoinSet::new();
                loop {
                    tokio::select! {
                        accepted=listener.accept()=>{
                            let Ok((stream,_))=accepted else{break};let Ok(permit)=slots.clone().try_acquire_owned()else{continue};let this=this.clone();
                            clients.spawn(async move{let _permit=permit;let _=tokio::time::timeout(Duration::from_secs(30),serve(this,stream)).await;});
                        },
                        _=clients.join_next(),if !clients.is_empty()=>{}
                    }
                }
            });
            self.0
                .runtime_tasks
                .lock()
                .map_err(|_| Error::Unavailable)?
                .insert("chatgpt-broker".into(), task.abort_handle());
            *broker = Some(path.clone());
            path
        };
        let capability = oauth::random()?;
        let mut leases = self
            .0
            .chatgpt
            .leases
            .lock()
            .map_err(|_| Error::Unavailable)?;
        if leases.len() >= 64 {
            return Err(Error::Limit);
        }
        let (cancel, cancelled) = tokio::sync::watch::channel(false);
        leases.insert(
            capability.clone(),
            Lease {
                account,
                worker: worker.into(),
                generation,
                cancel,
            },
        );
        command
            .env("JJ_STORE_CHATGPT_SOCKET", path)
            .env("JJ_STORE_CHATGPT_BROKER_TOKEN", &capability);
        Ok(Some(Guard {
            owner: self.clone(),
            capability,
            cancelled,
        }))
    }
}
async fn serve(this: Supervisor, stream: tokio::net::UnixStream) -> Result<()> {
    let (read, mut write) = stream.into_split();
    let mut text = zeroize::Zeroizing::new(String::new());
    BufReader::new(read)
        .take(65537)
        .read_line(&mut text)
        .await
        .map_err(|_| Error::Unavailable)?;
    if text.len() > 65536 || !text.ends_with('\n') {
        return Err(Error::Limit);
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Request {
        capability: String,
        rejected: Option<String>,
    }
    let request: Request = serde_json::from_str(&text).map_err(|_| Error::Invalid)?;
    let lease = this
        .0
        .chatgpt
        .leases
        .lock()
        .map_err(|_| Error::Unavailable)?
        .get(&request.capability)
        .cloned()
        .ok_or(Error::Denied)?;
    let result = this
        .chatgpt_token(&lease.account, request.rejected.as_deref())
        .await;
    // Revalidate after renewal: sign-out/worker retirement can race network I/O.
    let live = this
        .0
        .chatgpt
        .leases
        .lock()
        .map_err(|_| Error::Unavailable)?
        .contains_key(&request.capability);
    let value = match result {
        Ok(token) if live => json!({"access_token":token}),
        _ => json!({"error":"chatgpt_authentication_required"}),
    };
    let mut bytes =
        zeroize::Zeroizing::new(serde_json::to_vec(&value).map_err(|_| Error::Invalid)?);
    bytes.push(b'\n');
    write
        .write_all(&bytes)
        .await
        .map_err(|_| Error::Unavailable)?;
    Ok(())
}

pub(in crate::agent_execution) async fn cancelled(lease: &mut Option<Guard>) {
    if let Some(lease) = lease {
        if !*lease.cancelled.borrow() {
            let _ = lease.cancelled.changed().await;
        }
    } else {
        std::future::pending::<()>().await;
    }
}
