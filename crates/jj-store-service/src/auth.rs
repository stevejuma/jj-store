//! Bounded policies and scope-bound high-entropy bearer credentials.
use jj_store_model::{PrincipalId, RepositoryId, TokenId};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::BTreeMap,
    fmt,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use subtle::ConstantTimeEq as _;
use zeroize::Zeroizing;

const MAX_TOKENS: usize = 128;
const MAX_POLICY_BYTES: usize = 64 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid or unauthorized credential")]
    Denied,
    #[error("invalid authentication policy")]
    Policy,
    #[error("authentication resource limit exceeded")]
    Limit,
    #[error("secure credential generation failed")]
    Entropy,
    #[error("shared listeners require TLS; unauthenticated development requires loopback")]
    Transport,
}
pub type Result<T> = std::result::Result<T, Error>;

/// No Serialize implementation: credentials are returned once and live only in
/// request/client memory. Callers deliberately expose them only to a credential
/// facility or the authenticated transport, never to checkout files or logs.
#[derive(Clone)]
pub struct BearerToken {
    id: TokenId,
    secret: Zeroizing<[u8; 32]>,
}
impl fmt::Debug for BearerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BearerToken([REDACTED])")
    }
}
impl BearerToken {
    fn generate() -> Result<Self> {
        let mut id = [0; 16];
        let mut secret = Zeroizing::new([0; 32]);
        getrandom::fill(&mut id).map_err(|_| Error::Entropy)?;
        getrandom::fill(secret.as_mut()).map_err(|_| Error::Entropy)?;
        Ok(Self {
            id: TokenId::new(id),
            secret,
        })
    }
    pub fn parse(text: &str) -> Result<Self> {
        if text.len() != 102
            || !text.is_ascii()
            || !text.starts_with("jst1_")
            || &text[37..38] != "_"
        {
            return Err(Error::Denied);
        }
        let id = text[5..37].parse().map_err(|_| Error::Denied)?;
        let tail = &text[38..];
        if tail
            .bytes()
            .any(|c| !c.is_ascii_digit() && !(b'a'..=b'f').contains(&c))
        {
            return Err(Error::Denied);
        }
        let mut secret = Zeroizing::new([0; 32]);
        hex::decode_to_slice(tail, secret.as_mut()).map_err(|_| Error::Denied)?;
        Ok(Self { id, secret })
    }
    pub fn id(&self) -> TokenId {
        self.id
    }
    pub fn expose(&self) -> Zeroizing<String> {
        // Encode directly into the zeroizing output rather than leaving a
        // temporary formatted copy of the secret allocation behind.
        let mut output = Zeroizing::new(String::with_capacity(102));
        output.push_str(&format!("jst1_{}_", self.id));
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for byte in self.secret.iter().copied() {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 15) as usize] as char);
        }
        output
    }
    fn verifier(&self, scope: Scope) -> [u8; 32] {
        let mut digest = Sha256::new_with_prefix(b"jj-store/token-verifier/v1");
        match scope {
            Scope::Repository(id) => {
                digest.update(b"repository");
                digest.update(id.as_bytes());
            }
            Scope::Operator => digest.update(b"operator"),
        }
        digest.update(self.id.as_bytes());
        digest.update(self.secret.as_ref());
        digest.finalize().into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "repository",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Scope {
    Repository(RepositoryId),
    Operator,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Read,
    Write,
    Admin,
    CreateRepository,
}
impl Permission {
    fn allows(self, required: Self) -> bool {
        self == required
            || matches!(
                (self, required),
                (Self::Admin, Self::Read | Self::Write) | (Self::Write, Self::Read)
            )
    }
    fn valid_for(self, scope: Scope) -> bool {
        matches!(
            (scope, self),
            (Scope::Operator, Self::CreateRepository)
                | (Scope::Repository(_), Self::Read | Self::Write | Self::Admin)
        )
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Grant {
    principal: PrincipalId,
    permission: Permission,
    expires_at: Option<u64>,
    verifier: String,
}
/// Trusted issuance tools build the policy. Transports only use encoded policies
/// selected by authority and must never accept a caller-supplied policy.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenPolicy {
    format: u32,
    scope: Scope,
    tokens: BTreeMap<TokenId, Grant>,
}
impl fmt::Debug for TokenPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenPolicy")
            .field("scope", &self.scope)
            .field("token_count", &self.tokens.len())
            .finish()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Principal {
    id: PrincipalId,
    token: TokenId,
    scope: Scope,
    permission: Permission,
}
impl Principal {
    pub fn id(self) -> PrincipalId {
        self.id
    }
    pub fn token(self) -> TokenId {
        self.token
    }
    pub fn scope(self) -> Scope {
        self.scope
    }
    pub fn permission(self) -> Permission {
        self.permission
    }
}
impl TokenPolicy {
    pub fn new(scope: Scope) -> Self {
        Self {
            format: 1,
            scope,
            tokens: BTreeMap::new(),
        }
    }
    pub fn scope(&self) -> Scope {
        self.scope
    }
    pub fn issue(
        &mut self,
        principal: PrincipalId,
        permission: Permission,
        expires_at: Option<u64>,
    ) -> Result<BearerToken> {
        if !permission.valid_for(self.scope) {
            return Err(Error::Policy);
        }
        if self.tokens.len() >= MAX_TOKENS {
            return Err(Error::Limit);
        }
        let token = BearerToken::generate()?;
        if self.tokens.contains_key(&token.id) {
            return Err(Error::Entropy);
        }
        let grant = Grant {
            principal,
            permission,
            expires_at,
            verifier: hex::encode(token.verifier(self.scope)),
        };
        self.tokens.insert(token.id, grant);
        if self.encode().is_err() {
            self.tokens.remove(&token.id);
            return Err(Error::Limit);
        }
        Ok(token)
    }
    /// Idempotent removal. An empty policy denies everyone; it cannot create an
    /// implicit unauthenticated mode or manufacture an operator capability.
    pub fn revoke(&mut self, id: TokenId) -> bool {
        self.tokens.remove(&id).is_some()
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Policy)?;
        if bytes.len() > MAX_POLICY_BYTES {
            return Err(Error::Limit);
        }
        Ok(bytes)
    }
    pub fn decode(bytes: &[u8], expected: Scope) -> Result<Self> {
        if bytes.len() > MAX_POLICY_BYTES {
            return Err(Error::Limit);
        }
        let policy: Self = serde_json::from_slice(bytes).map_err(|_| Error::Policy)?;
        if policy.format != 1
            || policy.scope != expected
            || policy.tokens.len() > MAX_TOKENS
            || policy.encode()? != bytes
        {
            return Err(Error::Policy);
        }
        if policy.tokens.values().any(|g| {
            !g.permission.valid_for(expected)
                || g.verifier.len() != 64
                || g.verifier
                    .bytes()
                    .any(|c| !c.is_ascii_digit() && !(b'a'..=b'f').contains(&c))
        }) {
            return Err(Error::Policy);
        }
        Ok(policy)
    }
    pub fn authorize(
        &self,
        token: &BearerToken,
        required: Permission,
        now: u64,
    ) -> Result<Principal> {
        if !required.valid_for(self.scope) {
            return Err(Error::Denied);
        }
        let grant = self.tokens.get(&token.id).ok_or(Error::Denied)?;
        let mut expected = [0; 32];
        hex::decode_to_slice(&grant.verifier, &mut expected).map_err(|_| Error::Policy)?;
        if !bool::from(expected.ct_eq(&token.verifier(self.scope)))
            || !grant.permission.allows(required)
            || grant.expires_at.is_some_and(|deadline| now >= deadline)
        {
            return Err(Error::Denied);
        }
        Ok(Principal {
            id: grant.principal,
            token: token.id,
            scope: self.scope,
            permission: grant.permission,
        })
    }
}

pub trait Clock: Send + Sync {
    fn unix_seconds(&self) -> Result<u64>;
}
pub struct SystemClock;
impl Clock for SystemClock {
    fn unix_seconds(&self) -> Result<u64> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| t.as_secs())
            .map_err(|_| Error::Denied)
    }
}
/// Configuration validation; actual TLS and explicit development routing are
/// wired by the server, never inferred from the absence of a bearer credential.
pub fn validate_listener(
    bind: SocketAddr,
    tls: bool,
    unauthenticated_development: bool,
) -> Result<()> {
    if (unauthenticated_development && !bind.ip().is_loopback())
        || (!bind.ip().is_loopback() && !tls)
    {
        return Err(Error::Transport);
    }
    Ok(())
}
