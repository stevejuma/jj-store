use super::*;
use hmac::{Hmac, Mac as _};
use jj_store_model::portable::CodeProfileV1;
use sha2::Sha256;
pub(super) struct Authorization {
    token: Option<BearerToken>,
    scope: String,
    sessions: BTreeMap<m::RepositoryId, Option<Arc<NativeSession>>>,
    local: bool,
    viewers: BTreeMap<m::RepositoryId, m::PrincipalId>,
}
impl Authorization {
    pub fn new(state: &State, headers: &HeaderMap) -> Result<Self> {
        let token = match state.config.access {
            Access::Authenticated => Some(crate::repositories::bearer(headers)?),
            _ => None,
        };
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&state.repository_cursor_key).expect("fixed key");
        if let Some(token) = &token {
            mac.update(token.expose().as_bytes());
        } else {
            mac.update(b"local-trusted");
        }
        Ok(Self {
            token,
            scope: hex::encode(&mac.finalize().into_bytes()[..16]),
            sessions: BTreeMap::new(),
            viewers: BTreeMap::new(),
            local: matches!(
                state.config.access,
                Access::LocalTrusted {
                    permission: Permission::Admin,
                    ..
                }
            ),
        })
    }
    pub async fn repositories(&mut self, state: &State) -> Result<Vec<(m::RepositoryId, bool)>> {
        let mut headers = HeaderMap::new();
        if let Some(token) = &self.token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", token.expose().as_str())
                    .parse()
                    .map_err(|_| Error::invalid())?,
            );
        }
        let mut cursor = None;
        let mut repositories = Vec::new();
        let mut sessions = BTreeMap::new();
        let mut viewers = BTreeMap::new();
        loop {
            let page = crate::repositories::list(state, &headers, cursor.as_deref(), 100).await?;
            for repository in page.repositories {
                if repositories.len() >= 256 {
                    return Err(Error::limit());
                }
                let local = repository.profile == Some(CodeProfileV1::GitSha1Jj045);
                let session = if local {
                    None
                } else {
                    let endpoint = state
                        .host
                        .as_ref()
                        .ok_or_else(Error::unavailable)?
                        .endpoint(repository.id)?;
                    let request = match (&state.config.access, &self.token) {
                        (_, Some(token)) => endpoint.bind(token.clone())?,
                        (
                            Access::LocalTrusted {
                                principal,
                                permission,
                            },
                            _,
                        ) => endpoint.bind_local_trusted(*principal, *permission)?,
                        _ => return Err(Error::denied()),
                    };
                    Some(request.into_session())
                };
                viewers.insert(repository.id, repository.principal);
                repositories.push((repository.id, local));
                sessions.insert(repository.id, session);
            }
            cursor = page.next_cursor;
            if cursor.is_none() {
                break;
            }
        }
        if viewers
            .iter()
            .any(|(id, principal)| self.viewers.get(id).is_some_and(|old| old != principal))
        {
            return Err(Error::denied());
        }
        self.sessions = sessions;
        self.viewers = viewers;
        Ok(repositories)
    }
    pub fn visible(&self, change: &Change) -> bool {
        if change.audience.is_some_and(|viewer| {
            change
                .repository
                .and_then(|r| self.viewers.get(&r).copied())
                != Some(viewer)
        }) {
            return false;
        }
        change.repository.map_or(
            self.local || matches!(change.domain, Domain::Discovery | Domain::Repository),
            |id| self.sessions.contains_key(&id),
        )
    }
    pub async fn authorize(
        &self,
        _state: &State,
        repository: Option<m::RepositoryId>,
    ) -> Result<()> {
        for (id, session) in &self.sessions {
            if repository.is_some_and(|r| r != *id) {
                continue;
            }
            if let Some(session) = session {
                let principal = session
                    .principal(Permission::Read)
                    .await
                    .map_err(reads::Error::from)?;
                if self.viewers.get(id) != Some(&principal.id()) {
                    return Err(Error::denied());
                }
            } else if !self.local {
                return Err(Error::denied());
            }
        }
        if repository.is_some_and(|r| !self.sessions.contains_key(&r)) {
            return Err(Error::denied());
        }
        Ok(())
    }
    pub fn cursor(&self, state: &State, revision: u64) -> String {
        format!("jse1:{}:{}:{revision}", state.event_generation, self.scope)
    }
    pub fn resume(&self, state: &State, cursor: &str) -> Result<Option<u64>> {
        if cursor.len() > 128 {
            return Err(Error::invalid());
        }
        let parts: Vec<_> = cursor.split(':').collect();
        let ["jse1", generation, scope, revision] = parts.as_slice() else {
            return Err(Error::invalid());
        };
        if generation.len() != 32
            || scope.len() != 32
            || !generation
                .bytes()
                .chain(scope.bytes())
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(Error::invalid());
        }
        let revision: u64 = revision.parse().map_err(|_| Error::invalid())?;
        if self.cursor(state, revision) != cursor
            && (*generation == state.event_generation && *scope == self.scope)
        {
            return Err(Error::invalid());
        }
        Ok((*generation == state.event_generation && *scope == self.scope).then_some(revision))
    }
}
