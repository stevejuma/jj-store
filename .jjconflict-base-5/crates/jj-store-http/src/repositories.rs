//! Deployment-aware repository discovery with repository-policy reauthorization.
use crate::{
    Access, Error, Result, State,
    dto::{RepositoryPage, RepositorySummary},
};
use axum::http::{HeaderMap, header};
use hmac::{Hmac, Mac as _};
use jj_store_model::{RepositoryId, TokenId};
use jj_store_service::{
    auth::{BearerToken, Permission},
    registry::EffectivePermission,
};
use sha2::Sha256;

const CURSOR_PREFIX: &str = "jrr1_";
const CURSOR_BYTES: usize = 72;

pub(crate) async fn list(
    state: &State,
    headers: &HeaderMap,
    cursor: Option<&str>,
    limit: usize,
) -> Result<RepositoryPage> {
    if !(1..=100).contains(&limit) {
        return Err(Error::invalid());
    }
    let registry = state
        .config
        .registry
        .as_ref()
        .ok_or_else(Error::unavailable)?;
    let token = match state.config.access {
        Access::Authenticated => Some(bearer(headers)?),
        Access::LocalTrusted { .. } => None,
    };
    let token_id = token.as_ref().map(BearerToken::id);
    let mut snapshot = registry
        .snapshot(token_id)
        .await
        .map_err(|_| Error::unavailable())?;
    let local = if token.is_none() {
        if let Some(manager) = &state.config.local_repositories {
            let manager = manager.clone();
            Some(
                tokio::task::spawn_blocking(move || manager.list())
                    .await
                    .map_err(|_| Error::unavailable())?
                    .map_err(|_| Error::unavailable())?,
            )
        } else {
            None
        }
    } else {
        None
    };
    if let Some(local) = &local {
        snapshot.generation = snapshot
            .generation
            .checked_add(local.revision)
            .ok_or_else(Error::limit)?;
        snapshot
            .repositories
            .extend(local.bindings.iter().map(|binding| {
                jj_store_service::registry::RepositoryEntry {
                    id: binding.repository,
                    name: binding.name.clone(),
                }
            }));
        snapshot.repositories.sort_by_key(|r| r.id);
        if snapshot.repositories.windows(2).any(|w| w[0].id == w[1].id) {
            return Err(Error::unavailable());
        }
    }
    let after = cursor
        .map(|value| {
            decode_cursor(
                &state.repository_cursor_key,
                value,
                snapshot.generation,
                token_id,
            )
        })
        .transpose()?;
    let start = after
        .map(|id| {
            snapshot
                .repositories
                .partition_point(|entry| entry.id <= id)
        })
        .unwrap_or(0);
    let mut repositories = Vec::with_capacity(limit);
    let mut scanned = None;
    for entry in snapshot.repositories.iter().skip(start) {
        scanned = Some(entry.id);
        let (permission, principal) = match (&state.config.access, &token) {
            (
                Access::LocalTrusted {
                    permission,
                    principal,
                },
                None,
            ) => (
                EffectivePermission::try_from(*permission).map_err(|_| Error::denied())?,
                *principal,
            ),
            (Access::Authenticated, Some(token)) => {
                let endpoint = state
                    .host
                    .as_ref()
                    .ok_or_else(Error::unavailable)?
                    .endpoint(entry.id)
                    .map_err(|_| Error::unavailable())?;
                let session = endpoint
                    .bind(token.clone())
                    .map_err(|_| {
                        Error::new(axum::http::StatusCode::UNAUTHORIZED, "invalid_credential")
                    })?
                    .into_session();
                let Ok(principal) = session.principal(Permission::Read).await else {
                    continue;
                };
                (
                    EffectivePermission::try_from(principal.permission())
                        .map_err(|_| Error::denied())?,
                    principal.id(),
                )
            }
            _ => return Err(Error::denied()),
        };
        let binding = local
            .as_ref()
            .and_then(|l| l.bindings.iter().find(|b| b.repository == entry.id));
        repositories.push(RepositorySummary {
            profile: Some(if binding.is_some() {
                jj_store_model::portable::CodeProfileV1::GitSha1Jj045
            } else {
                jj_store_model::portable::CodeProfileV1::NativeV1
            }),
            availability: Some(
                binding.map_or(jj_store_service::local_browser::Availability::Ready, |b| {
                    b.availability.clone()
                }),
            ),
            id: entry.id,
            name: entry.name.clone(),
            permission,
            principal,
        });
        if repositories.len() == limit {
            break;
        }
    }
    if token.is_some() && cursor.is_none() && repositories.is_empty() {
        return Err(Error::new(
            axum::http::StatusCode::UNAUTHORIZED,
            "invalid_credential",
        ));
    }
    let next_cursor = scanned.and_then(|last| {
        (snapshot
            .repositories
            .last()
            .is_some_and(|entry| entry.id > last))
        .then(|| {
            encode_cursor(
                &state.repository_cursor_key,
                snapshot.generation,
                token_id,
                last,
            )
        })
    });
    Ok(RepositoryPage {
        version: 1,
        generation: snapshot.generation,
        repositories,
        next_cursor,
    })
}

pub(crate) fn bearer(headers: &HeaderMap) -> Result<BearerToken> {
    if headers.get_all(header::AUTHORIZATION).iter().count() != 1 {
        return Err(Error::new(
            axum::http::StatusCode::UNAUTHORIZED,
            "bearer_required",
        ));
    }
    let value = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| Error::new(axum::http::StatusCode::UNAUTHORIZED, "bearer_required"))?;
    BearerToken::parse(value)
        .map_err(|_| Error::new(axum::http::StatusCode::UNAUTHORIZED, "invalid_credential"))
}

fn encode_cursor(
    key: &[u8; 32],
    generation: u64,
    token: Option<TokenId>,
    last: RepositoryId,
) -> String {
    let mut bytes = Vec::with_capacity(CURSOR_BYTES);
    bytes.extend_from_slice(&generation.to_be_bytes());
    bytes.extend_from_slice(token.map(|id| *id.as_bytes()).unwrap_or([0; 16]).as_slice());
    bytes.extend_from_slice(last.as_bytes());
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("fixed HMAC key");
    mac.update(&bytes);
    bytes.extend_from_slice(&mac.finalize().into_bytes());
    format!("{CURSOR_PREFIX}{}", hex::encode(bytes))
}

fn decode_cursor(
    key: &[u8; 32],
    cursor: &str,
    generation: u64,
    token: Option<TokenId>,
) -> Result<RepositoryId> {
    let text = cursor
        .strip_prefix(CURSOR_PREFIX)
        .ok_or_else(Error::invalid)?;
    let bytes = hex::decode(text).map_err(|_| Error::invalid())?;
    if bytes.len() != CURSOR_BYTES {
        return Err(Error::invalid());
    }
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("fixed HMAC key");
    mac.update(&bytes[..40]);
    mac.verify_slice(&bytes[40..])
        .map_err(|_| Error::invalid())?;
    let cursor_generation =
        u64::from_be_bytes(bytes[..8].try_into().map_err(|_| Error::invalid())?);
    if cursor_generation != generation {
        return Err(Error::new(axum::http::StatusCode::CONFLICT, "cursor_reset"));
    }
    let expected = token.map(|id| *id.as_bytes()).unwrap_or([0; 16]);
    if bytes[8..24] != expected {
        return Err(Error::invalid());
    }
    RepositoryId::from_slice(&bytes[24..40]).map_err(|_| Error::invalid())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse as _;

    #[tokio::test]
    async fn repository_cursor_resets_after_registry_generation_changes() {
        let key = [7; 32];
        let token = TokenId::new([8; 16]);
        let cursor = encode_cursor(&key, 12, Some(token), RepositoryId::new([9; 16]));
        let response = decode_cursor(&key, &cursor, 13, Some(token))
            .unwrap_err()
            .into_response();
        assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap()["code"],
            "cursor_reset"
        );
    }
}
