//! OpenAI public-client OAuth wire and signed identity validation.
use super::*;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use sha2::{Digest, Sha256};
pub(super) const ISSUER: &str = "https://auth.openai.com";
pub(super) const RESOURCE: &str = "https://api.openai.com/v1";
pub(super) const PLAN_SCOPE: &str = "chatgpt.tokens.use.direct";
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Tokens {
    pub access_token: String,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub refresh_token: String,
    #[serde(default)]
    pub id_token: String,
    pub expires_in: u64,
    #[serde(default)]
    pub scope: String,
}
impl Tokens {
    pub fn valid(&self) -> bool {
        [&self.access_token, &self.refresh_token, &self.id_token]
            .iter()
            .all(|s| {
                (8..=16384).contains(&s.len())
                    && !s.chars().any(|c| c.is_control() || c.is_whitespace())
            })
            && self.expires_in > 0
            && self.expires_in <= 86400
            && self.scope.len() <= 4096
            && self
                .token_type
                .as_deref()
                .is_none_or(|kind| kind.eq_ignore_ascii_case("Bearer"))
    }
    pub fn plan(&self) -> bool {
        self.scope.split_whitespace().any(|s| s == PLAN_SCOPE)
    }
}
#[derive(Deserialize)]
pub(super) struct Identity {
    pub sub: String,
    pub email: Option<String>,
    pub nonce: String,
    #[serde(default)]
    pub azp: Option<String>,
}
pub(super) fn identity(token: &str, keys: &JwkSet, client: &str, nonce: &str) -> Result<Identity> {
    let header = decode_header(token).map_err(|_| Error::Invalid)?;
    if header.alg != Algorithm::RS256 {
        return Err(Error::Invalid);
    }
    let key = keys
        .find(header.kid.as_deref().ok_or(Error::Invalid)?)
        .ok_or(Error::Invalid)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[ISSUER]);
    validation.set_audience(&[client]);
    validation.validate_nbf = true;
    validation.leeway = 0;
    let claims = decode::<Identity>(
        token,
        &DecodingKey::from_jwk(key).map_err(|_| Error::Invalid)?,
        &validation,
    )
    .map_err(|_| Error::Invalid)?
    .claims;
    if claims.nonce != nonce
        || claims.sub.is_empty()
        || claims.sub.len() > 512
        || claims.sub.chars().any(char::is_control)
        || claims
            .email
            .as_ref()
            .is_some_and(|s| s.len() > 512 || s.chars().any(char::is_control))
        || claims.azp.as_deref().is_some_and(|s| s != client)
    {
        return Err(Error::Invalid);
    }
    Ok(claims)
}
pub(super) fn random() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}
pub(super) fn authorize(
    client: Option<&Record>,
    host: &str,
    redirect: &str,
    state: &str,
    nonce: &str,
    verifier: &str,
) -> Result<url::Url> {
    let mut url = url::Url::parse("https://auth.openai.com/api/accounts/authorize")
        .map_err(|_| Error::Invalid)?;
    let mut q = url.query_pairs_mut();
    q.append_pair(
        "client_id",
        client
            .map(|c| c.client_id.as_str())
            .unwrap_or("dynamic_agent_client"),
    );
    if client.is_none() {
        q.append_pair("agent_name_hint", "JJ Agent");
    }
    if let Some(email) = client.and_then(|c| c.view.email.as_deref()) {
        q.append_pair("login_hint", email);
    }
    q.append_pair("ext_agent_host_id", host)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirect)
        .append_pair(
            "scope",
            "openid profile email offline_access resource.invoke chatgpt.tokens.use.direct",
        )
        .append_pair("resource", RESOURCE)
        .append_pair("state", state)
        .append_pair("nonce", nonce)
        .append_pair("code_challenge_method", "S256")
        .append_pair(
            "code_challenge",
            &URL_SAFE_NO_PAD.encode(Sha256::digest(verifier)),
        );
    drop(q);
    Ok(url)
}
pub(super) fn client_id(saved: Option<&str>, returned: Option<&str>) -> Result<String> {
    let value = match (saved, returned) {
        (Some(s), None) => s,
        (Some(s), Some(r)) if s == r => s,
        (None, Some(r)) => r,
        _ => return Err(Error::Invalid),
    };
    if value == "dynamic_agent_client"
        || value.len() > 512
        || value.is_empty()
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
    {
        return Err(Error::Invalid);
    }
    Ok(value.into())
}
pub(super) fn http() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(20))
        .build()
        .map_err(|_| Error::Unavailable)
}
pub(super) async fn json(response: reqwest::Response) -> Result<Value> {
    let status = response.status();
    let mut response = response;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| Error::Unavailable)? {
        if bytes.len() + chunk.len() > 65536 {
            return Err(Error::Limit);
        }
        bytes.extend_from_slice(&chunk);
    }
    if !status.is_success() {
        let terminal = serde_json::from_slice::<Value>(&bytes)
            .ok()
            .is_some_and(|v| {
                let code = v["error"]
                    .as_str()
                    .or_else(|| v["error"]["code"].as_str())
                    .unwrap_or("");
                matches!(
                    code,
                    "invalid_grant"
                        | "invalid_refresh_token"
                        | "token_expired"
                        | "refresh_token_expired"
                        | "refresh_token_invalidated"
                        | "refresh_token_reused"
                )
            });
        return Err(if terminal && matches!(status.as_u16(), 400 | 401) {
            Error::Denied
        } else {
            Error::Unavailable
        });
    }
    serde_json::from_slice(&bytes).map_err(|_| Error::Invalid)
}

impl Drop for Tokens {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.access_token.zeroize();
        self.refresh_token.zeroize();
        self.id_token.zeroize();
    }
}
