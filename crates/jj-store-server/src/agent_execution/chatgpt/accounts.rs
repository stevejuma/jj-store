//! Per-registration serialized renewal and explicit sign-out.
use super::*;
impl Supervisor {
    pub(in crate::agent_execution) async fn chatgpt_token(
        &self,
        id: &str,
        rejected: Option<&str>,
    ) -> Result<String> {
        let record = self.chatgpt_record(id)?;
        if !record.view.connected || !record.view.plan_enabled {
            return Err(Error::Denied);
        }
        let slot = self.0.chatgpt.slot(id)?;
        let mut cached = slot.lock().await;
        if cached
            .as_ref()
            .is_none_or(|c| c.generation != record.generation)
        {
            *cached = Some(self.load_chatgpt_tokens(id, record.generation)?);
        }
        let entry = cached.as_mut().ok_or(Error::Unavailable)?;
        if entry.expires <= now() + 60 || rejected.is_some_and(|r| r == entry.tokens.access_token) {
            let http = oauth::http()?;
            let response = http
                .post(self.oauth_url("/api/accounts/oauth/token"))
                .form(&[
                    ("grant_type", "refresh_token"),
                    ("client_id", record.client_id.as_str()),
                    ("refresh_token", entry.tokens.refresh_token.as_str()),
                    ("resource", oauth::RESOURCE),
                ])
                .send()
                .await
                .map_err(|_| Error::Unavailable)?;
            let value = match oauth::json(response).await {
                Ok(value) => value,
                Err(Error::Denied) => {
                    self.chatgpt_invalidate(id)?;
                    self.clear_chatgpt_tokens(id)?;
                    *cached = None;
                    return Err(Error::Denied);
                }
                Err(error) => return Err(error),
            };
            let mut tokens: oauth::Tokens =
                serde_json::from_value(value).map_err(|_| Error::Invalid)?;
            if tokens.refresh_token.is_empty() {
                tokens.refresh_token = entry.tokens.refresh_token.clone();
            }
            if tokens.scope.is_empty() {
                tokens.scope = entry.tokens.scope.clone();
            }
            if tokens.id_token.is_empty() {
                tokens.id_token = entry.tokens.id_token.clone();
            }
            if !tokens.valid() || !tokens.plan() {
                self.chatgpt_invalidate(id)?;
                self.clear_chatgpt_tokens(id)?;
                *cached = None;
                return Err(Error::Denied);
            }
            if self.chatgpt_record(id)?.generation != record.generation {
                return Err(Error::Stale);
            }
            self.register_oauth_secrets(&tokens)?;
            let expires = now().saturating_add(tokens.expires_in);
            self.update(|s| {
                if s.chatgpt_accounts
                    .get(id)
                    .is_none_or(|r| r.generation != record.generation)
                {
                    return Err(Error::Stale);
                }
                self.save_chatgpt_tokens(id, &tokens, expires, record.generation)
            })?;
            *entry = Cached {
                tokens,
                expires,
                generation: record.generation,
            };
        }
        if self.chatgpt_record(id)?.generation != record.generation {
            return Err(Error::Stale);
        }
        self.register_oauth_secrets(&entry.tokens)?;
        Ok(entry.tokens.access_token.clone())
    }
    fn chatgpt_invalidate(&self, id: &str) -> Result<()> {
        self.update(|s| {
            let account = s.chatgpt_accounts.get_mut(id).ok_or(Error::Invalid)?;
            account.generation = account.generation.checked_add(1).ok_or(Error::Limit)?;
            account.view.connected = false;
            Ok(())
        })?;
        let workers = {
            let mut leases = self
                .0
                .chatgpt
                .leases
                .lock()
                .map_err(|_| Error::Unavailable)?;
            let workers = leases
                .values()
                .filter(|v| v.account == id)
                .map(|v| v.worker.clone())
                .collect::<std::collections::BTreeSet<_>>();
            leases.retain(|_, v| {
                if v.account == id {
                    let _ = v.cancel.send(true);
                    false
                } else {
                    true
                }
            });
            workers
        };
        for worker in workers {
            let _ = self.stop_worker(&worker);
        }
        Ok(())
    }
    pub(in crate::agent_execution) async fn chatgpt_sign_out(&self, id: &str) -> Result<()> {
        let record = self.chatgpt_record(id)?;
        let pending = self
            .0
            .chatgpt
            .attempts
            .lock()
            .map_err(|_| Error::Unavailable)?
            .values()
            .filter(|p| p.view.account == id && p.view.state == "authorizing")
            .map(|p| p.view.id.clone())
            .collect::<Vec<_>>();
        for attempt in pending {
            self.chatgpt_cancel(&attempt)?;
        }
        self.chatgpt_invalidate(id)?;
        self.update(|s| {
            s.chatgpt_notices.insert(
                id.into(),
                contract::Attempt {
                    id: format!("sign-out-{id}"),
                    account: id.into(),
                    state: "signed_out".into(),
                    error: Some("remote_revocation_unconfirmed".into()),
                },
            );
            Ok(())
        })?;
        let slot = self.0.chatgpt.slot(id)?;
        let mut cached = slot.lock().await;
        let tokens = match cached.take() {
            Some(c) => Some(c),
            None => self.load_chatgpt_tokens(id, record.generation).ok(),
        };
        self.clear_chatgpt_tokens(id)?;
        let revoked = if let Some(entry) = tokens {
            let result = async {
                let http = oauth::http()?;
                let config = oauth::json(
                    http.get(self.oauth_url("/.well-known/openid-configuration"))
                        .send()
                        .await
                        .map_err(|_| Error::Unavailable)?,
                )
                .await?;
                let endpoint = config["revocation_endpoint"]
                    .as_str()
                    .ok_or(Error::Unavailable)?;
                let url = url::Url::parse(endpoint).map_err(|_| Error::Invalid)?;
                if !valid_revocation_url(&self.oauth_url(""), &url) {
                    return Err(Error::Invalid);
                }
                let response = http
                    .post(url)
                    .form(&[
                        ("token", entry.tokens.refresh_token.as_str()),
                        ("token_type_hint", "refresh_token"),
                        ("client_id", record.client_id.as_str()),
                    ])
                    .send()
                    .await
                    .map_err(|_| Error::Unavailable)?;
                if response.status() != reqwest::StatusCode::OK {
                    return Err(Error::Unavailable);
                }
                Ok(())
            }
            .await;
            result.is_ok()
        } else {
            false
        };
        self.update(|s| {
            if revoked {
                s.chatgpt_notices.remove(id);
            } else {
                s.chatgpt_notices.insert(
                    id.into(),
                    contract::Attempt {
                        id: format!("sign-out-{id}"),
                        account: id.into(),
                        state: "signed_out".into(),
                        error: Some("remote_revocation_unconfirmed".into()),
                    },
                );
            }
            Ok(())
        })?;
        Ok(())
    }
    pub(in crate::agent_execution) async fn chatgpt_models(
        &self,
        id: &str,
    ) -> Result<jj_store_service::agent_execution::catalog::Catalog> {
        let token = self.chatgpt_token(id, None).await?;
        let http = oauth::http()?;
        let mut response = http
            .get(self.resource_url("/models"))
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|_| Error::Unavailable)?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            let renewed = self.chatgpt_token(id, Some(&token)).await?;
            response = http
                .get(self.resource_url("/models"))
                .bearer_auth(renewed)
                .send()
                .await
                .map_err(|_| Error::Unavailable)?;
        }
        let value = oauth::json(response).await?;
        let rows = value["models"].as_array().ok_or(Error::Invalid)?;
        if rows.len() > 1024 {
            return Err(Error::Limit);
        }
        let mut models = Vec::new();
        for row in rows.iter().filter(|r| r["visibility"] == "list") {
            let id = row["slug"]
                .as_str()
                .filter(|v| label(v, 512))
                .ok_or(Error::Invalid)?;
            let name = row["display_name"]
                .as_str()
                .filter(|v| label(v, 512))
                .ok_or(Error::Invalid)?;
            models.push(json!({"modelId":id,"name":name}));
        }
        Ok(jj_store_service::agent_execution::catalog::Catalog {
            runner: "local".into(),
            status: Some(if models.is_empty() { "empty" } else { "ready" }.into()),
            facts: json!({"models":{"availableModels":models},"modes":{"currentModeId":"ask","availableModes":[{"id":"ask","name":"Ask before tools"}]}}),
        })
    }
}

fn valid_revocation_url(origin: &str, url: &url::Url) -> bool {
    let Ok(expected) = url::Url::parse(origin) else {
        return false;
    };
    url.origin() == expected.origin()
        && url.username().is_empty()
        && url.password().is_none()
        && url.fragment().is_none()
}
