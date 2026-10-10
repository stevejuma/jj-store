//! Explicit browser authorization with a bounded, state-bound loopback callback.
use super::callback::Callback;
use super::*;
use axum::routing::get;
impl Supervisor {
    pub(in crate::agent_execution) async fn chatgpt_connect(
        &self,
        request: RequestId,
        selected: Option<String>,
    ) -> Result<()> {
        if self.0.stopping.load(std::sync::atomic::Ordering::Acquire)
            || (!cfg!(target_os = "macos") && !cfg!(test))
        {
            return Err(Error::Unavailable);
        }
        let digest = journal::digest(&("connect", &selected))?;
        let id = request.to_string();
        let mut attempts = self
            .0
            .chatgpt
            .attempts
            .lock()
            .map_err(|_| Error::Unavailable)?;
        if let Some((old, _)) = self.read(|s| Ok(s.chatgpt_requests.get(&request).cloned()))? {
            return if old == digest {
                Ok(())
            } else {
                Err(Error::Conflict)
            };
        }
        let record = selected
            .as_deref()
            .map(|id| self.chatgpt_record(id))
            .transpose()?;
        let account = selected.unwrap_or_else(|| id.clone());
        let generation = record.as_ref().map(|r| r.generation).unwrap_or(0);
        if attempts
            .values()
            .any(|p| p.task.is_some() && p.view.account == account)
        {
            return Err(Error::Busy);
        }
        if attempts.len() >= 32 {
            let done = attempts
                .iter()
                .find(|(_, p)| p.task.is_none())
                .map(|(id, _)| id.clone());
            if let Some(id) = done {
                attempts.remove(&id);
            } else {
                return Err(Error::Limit);
            }
        }
        self.update(|s| {
            if s.chatgpt_requests.len() >= 4096 {
                return Err(Error::Limit);
            }
            if s.chatgpt_accounts.len() >= 64 && !s.chatgpt_accounts.contains_key(&account) {
                return Err(Error::Limit);
            }
            s.chatgpt_requests.insert(
                request,
                (
                    digest.clone(),
                    contract::Attempt {
                        id: id.clone(),
                        account: account.clone(),
                        state: "interrupted".into(),
                        error: Some("authorization_interrupted_reconnect_required".into()),
                    },
                ),
            );
            Ok(())
        })?;
        attempts.insert(
            id.clone(),
            Pending {
                view: contract::Attempt {
                    id: id.clone(),
                    account: account.clone(),
                    state: "authorizing".into(),
                    error: None,
                },
                task: None,
            },
        );
        let this = self.clone();
        let task_id = id.clone();
        let task = tokio::spawn(async move {
            let result = this
                .chatgpt_authorize(&task_id, &account, record, generation)
                .await;
            let _ = this.chatgpt_attempt(
                &task_id,
                if result.is_ok() {
                    "connected"
                } else {
                    "failed"
                },
                result.err().map(|e| match e {
                    Error::Denied => "authorization_denied",
                    Error::Stale => "account_changed",
                    Error::Invalid => "authorization_invalid",
                    Error::Busy => "privacy_not_ready",
                    _ => "authorization_unavailable",
                }),
            );
        });
        attempts.get_mut(&id).ok_or(Error::Unavailable)?.task = Some(task.abort_handle());
        Ok(())
    }
    async fn chatgpt_authorize(
        &self,
        attempt: &str,
        account: &str,
        record: Option<Record>,
        generation: u64,
    ) -> Result<()> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|_| Error::Unavailable)?;
        let redirect = format!(
            "http://127.0.0.1:{}/auth/callback",
            listener
                .local_addr()
                .map_err(|_| Error::Unavailable)?
                .port()
        );
        let state = oauth::random()?;
        let nonce = oauth::random()?;
        let verifier = oauth::random()?;
        let host = self.update(|s| {
            if s.chatgpt_host.is_none() {
                let mut bytes = [0u8; 16];
                getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
                bytes[6] = (bytes[6] & 15) | 64;
                bytes[8] = (bytes[8] & 63) | 128;
                let h = hex::encode(bytes);
                s.chatgpt_host = Some(format!(
                    "urn:uuid:{}-{}-{}-{}-{}",
                    &h[..8],
                    &h[8..12],
                    &h[12..16],
                    &h[16..20],
                    &h[20..]
                ));
            }
            Ok(s.chatgpt_host.clone().unwrap_or_default())
        })?;
        let url = oauth::authorize(record.as_ref(), &host, &redirect, &state, &nonce, &verifier)?;
        let (tx, rx) = tokio::sync::oneshot::channel();
        let router = axum::Router::new()
            .route("/auth/callback", get(super::callback::receive))
            .with_state(Arc::new(Callback {
                state,
                sender: Mutex::new(Some(tx)),
            }));
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, router).await;
        });
        struct Guard(tokio::task::AbortHandle);
        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.abort();
            }
        }
        let _guard = Guard(server.abort_handle());
        self.open_chatgpt_browser(url).await?;
        let fields = tokio::time::timeout(Duration::from_secs(300), rx)
            .await
            .map_err(|_| Error::Unavailable)?
            .map_err(|_| Error::Unavailable)?;
        if fields.contains_key("error") {
            return Err(Error::Denied);
        }
        let client = oauth::client_id(
            record.as_ref().map(|r| r.client_id.as_str()),
            fields.get("client_id").map(String::as_str),
        )?;
        self.update(|s| {
            if s.chatgpt_accounts
                .get(account)
                .is_some_and(|r| r.generation != generation)
            {
                return Err(Error::Stale);
            }
            s.chatgpt_accounts.entry(account.into()).or_insert(Record {
                view: contract::Account {
                    id: account.into(),
                    label: format!("ChatGPT · {}", &account[..8.min(account.len())]),
                    email: None,
                    connected: false,
                    plan_enabled: false,
                },
                client_id: client.clone(),
                subject: String::new(),
                generation,
            });
            Ok(())
        })?;
        let code = fields
            .get("code")
            .filter(|c| !c.is_empty() && c.len() <= 4096)
            .ok_or(Error::Invalid)?;
        let http = oauth::http()?;
        let value = oauth::json(
            http.post(self.oauth_url("/api/accounts/oauth/token"))
                .form(&[
                    ("grant_type", "authorization_code"),
                    ("client_id", client.as_str()),
                    ("code", code.as_str()),
                    ("code_verifier", verifier.as_str()),
                    ("redirect_uri", redirect.as_str()),
                    ("resource", oauth::RESOURCE),
                ])
                .send()
                .await
                .map_err(|_| Error::Unavailable)?,
        )
        .await?;
        let tokens: oauth::Tokens = serde_json::from_value(value).map_err(|_| Error::Invalid)?;
        if !tokens.valid() {
            return Err(Error::Invalid);
        }
        let keys: jsonwebtoken::jwk::JwkSet = serde_json::from_value(
            oauth::json(
                http.get(self.oauth_url("/.well-known/jwks.json"))
                    .send()
                    .await
                    .map_err(|_| Error::Unavailable)?,
            )
            .await?,
        )
        .map_err(|_| Error::Invalid)?;
        let identity = oauth::identity(&tokens.id_token, &keys, &client, &nonce)?;
        if record
            .as_ref()
            .is_some_and(|r| !r.subject.is_empty() && r.subject != identity.sub)
        {
            return Err(Error::Stale);
        }
        let slot = self.0.chatgpt.slot(account)?;
        let mut cache = slot.lock().await;
        if self.chatgpt_record(account)?.generation != generation {
            return Err(Error::Stale);
        }
        self.register_oauth_secrets(&tokens)?;
        let expires = now().saturating_add(tokens.expires_in);
        let mut attempts = self
            .0
            .chatgpt
            .attempts
            .lock()
            .map_err(|_| Error::Unavailable)?;
        let pending = attempts
            .get_mut(attempt)
            .filter(|p| p.view.state == "authorizing")
            .ok_or(Error::Stale)?;
        self.update(|s| {
            let saved = s.chatgpt_accounts.get_mut(account).ok_or(Error::Stale)?;
            if saved.generation != generation {
                return Err(Error::Stale);
            }
            self.save_chatgpt_tokens(account, &tokens, expires, generation)?;
            saved.subject = identity.sub;
            saved.client_id = client;
            saved.view.email = identity.email;
            saved.view.connected = true;
            saved.view.plan_enabled = tokens.plan();
            s.chatgpt_notices.remove(account);
            for (_, old) in s.chatgpt_requests.values_mut() {
                if old.account == account {
                    old.error = None;
                }
            }

            let (_, receipt) = s
                .chatgpt_requests
                .values_mut()
                .find(|(_, v)| v.id == attempt)
                .ok_or(Error::Invalid)?;
            receipt.state = "connected".into();
            receipt.error = None;
            Ok(())
        })?;
        *cache = Some(Cached {
            tokens,
            expires,
            generation,
        });
        pending.view.state = "connected".into();
        pending.view.error = None;
        pending.task = None;
        for entry in attempts.values_mut() {
            if entry.view.account == account {
                entry.view.error = None;
            }
        }
        Ok(())
    }
}
