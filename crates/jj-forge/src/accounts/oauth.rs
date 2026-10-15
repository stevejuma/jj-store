use super::*;
use zeroize::Zeroizing;

pub(super) struct Flow {
    pub public: Authorization,
    device_code: Zeroizing<String>,
    next_poll: u64,
    polling: bool,
}
pub(super) fn credential(value: &Value, now: u64) -> Result<Credential> {
    if value.get("error").is_some() {
        return Err(Error::Denied);
    }
    let token = value
        .get("access_token")
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty() && v.len() <= 4096 && !v.contains(['\n', '\r', '\0']))
        .ok_or(Error::Denied)?;
    Ok(Credential {
        access_token: token.to_string(),
        refresh_token: value
            .get("refresh_token")
            .and_then(Value::as_str)
            .map(str::to_string),
        expires_at: value
            .get("expires_in")
            .and_then(Value::as_u64)
            .map(|v| now.saturating_add(v)),
    })
}
impl Manager {
    pub(super) async fn begin(&self, now: u64) -> Result<Authorization> {
        {
            let mut flows = self.flows.lock().map_err(|_| Error::State)?;
            flows.retain(|_, f| f.public.expires_at > now);
            if flows.len() >= 8 {
                return Err(Error::Limit);
            }
        }
        let response = self
            .transport
            .request(
                "https://github.com/login/device/code",
                None,
                Some(
                    json!({"client_id":self.client_id.as_ref().ok_or(Error::Unsupported)?,
                "scope":"repo workflow read:user user:email offline_access"}),
                ),
            )
            .await?;
        let field = |key| {
            response
                .get(key)
                .and_then(Value::as_str)
                .ok_or(Error::Invalid)
        };
        let device = field("device_code")?;
        if device.len() > 256 {
            return Err(Error::Invalid);
        }
        let uri = field("verification_uri")?;
        if uri != "https://github.com/login/device" {
            return Err(Error::Invalid);
        }
        let user_code = field("user_code")?;
        if user_code.len() > 32 {
            return Err(Error::Invalid);
        }
        let expires = response
            .get("expires_in")
            .and_then(Value::as_u64)
            .filter(|v| *v > 0 && *v <= 1800)
            .ok_or(Error::Invalid)?;
        let interval = response
            .get("interval")
            .and_then(Value::as_u64)
            .unwrap_or(5)
            .clamp(5, 60);
        let mut flows = self.flows.lock().map_err(|_| Error::State)?;
        if flows.len() >= 8 {
            return Err(Error::Limit);
        }
        // A private random handle is separate from GitHub's secret device code.
        let file = tempfile::Builder::new()
            .prefix("oauth-")
            .tempfile()
            .map_err(|_| Error::State)?;
        let session = store::account_id(&file.path().to_string_lossy(), device, &Method::Oauth);
        let public = Authorization {
            session: session.clone(),
            user_code: user_code.to_string(),
            verification_uri: uri.to_string(),
            expires_at: now + expires,
            interval,
            state: "pending".into(),
        };
        flows.insert(
            session,
            Flow {
                public: public.clone(),
                device_code: Zeroizing::new(device.to_string()),
                next_poll: now + interval,
                polling: false,
            },
        );
        Ok(public)
    }
    pub(super) async fn poll(&self, session: &str, now: u64) -> Result<Authorization> {
        let device = {
            let mut flows = self.flows.lock().map_err(|_| Error::State)?;
            let flow = flows.get_mut(session).ok_or(Error::Stale)?;
            if flow.public.state != "pending" {
                return Ok(flow.public.clone());
            }
            if flow.public.expires_at <= now {
                flow.public.state = "expired".into();
                return Ok(flow.public.clone());
            }
            if flow.polling || flow.next_poll > now {
                return Ok(flow.public.clone());
            }
            flow.polling = true;
            flow.next_poll = now + flow.public.interval;
            flow.device_code.clone()
        };
        let result = self.transport.request("https://github.com/login/oauth/access_token", None,
            Some(json!({"client_id":self.client_id.as_ref().ok_or(Error::Unsupported)?,
                "device_code":device.as_str(), "grant_type":"urn:ietf:params:oauth:grant-type:device_code"}))).await;
        let response = match result {
            Ok(value) => value,
            Err(error) => {
                if let Some(flow) = self
                    .flows
                    .lock()
                    .map_err(|_| Error::State)?
                    .get_mut(session)
                {
                    flow.polling = false;
                }
                return Err(error);
            }
        };
        let error = response.get("error").and_then(Value::as_str);
        if error.is_none() {
            let verified = async {
                let credential = credential(&response, now)?;
                let user = self
                    .user("https://api.github.com", &credential.access_token)
                    .await?;
                Ok::<_, Error>((credential, user))
            }
            .await;
            // Hold the session lease while saving; cancellation wins before this point.
            let mut flows = self.flows.lock().map_err(|_| Error::State)?;
            let flow = flows.get_mut(session).ok_or(Error::Stale)?;
            flow.polling = false;
            if flow.public.expires_at <= now {
                flow.public.state = "expired".into();
                return Err(Error::Stale);
            }
            let (credential, user) = verified.inspect_err(|_| {
                flow.public.state = "failed".into();
            })?;
            self.persist(
                "https://api.github.com".into(),
                Method::Oauth,
                user,
                &credential,
            )
            .inspect_err(|_| {
                flow.public.state = "failed".into();
            })?;
            flow.public.state = "complete".into();
            flow.polling = false;
            flow.device_code.clear();
            return Ok(flow.public.clone());
        }
        let mut flows = self.flows.lock().map_err(|_| Error::State)?;
        let flow = flows.get_mut(session).ok_or(Error::Stale)?;
        flow.polling = false;
        match error {
            Some("authorization_pending") => {}
            Some("slow_down") => {
                flow.public.interval = (flow.public.interval + 5).min(300);
                flow.next_poll = now + flow.public.interval;
            }
            Some("expired_token" | "token_expired") => flow.public.state = "expired".into(),
            Some("access_denied") => flow.public.state = "denied".into(),
            _ => {
                flow.public.state = "failed".into();
            }
        }
        Ok(flow.public.clone())
    }
}
