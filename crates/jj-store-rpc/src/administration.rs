//! Bounded typed administration codecs shared by the client and server.
use crate::wire;
use jj_store_model as m;
use jj_store_service::{
    administration as a,
    auth::{BearerToken, Permission, TokenDescription},
};
use std::collections::BTreeSet;
use tonic::Status;
use zeroize::{Zeroize as _, Zeroizing};

impl std::fmt::Debug for wire::CreateRepositoryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CreateRepositoryRequest([REDACTED])")
    }
}
impl Drop for wire::CreateRepositoryRequest {
    fn drop(&mut self) {
        self.admin_credential.zeroize();
    }
}
impl std::fmt::Debug for wire::TokenGrant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenGrant([REDACTED])")
    }
}
impl Drop for wire::TokenGrant {
    fn drop(&mut self) {
        self.credential.zeroize();
    }
}
fn invalid() -> Status {
    Status::invalid_argument("invalid administration request")
}
pub(crate) fn snapshot(value: Option<wire::Snapshot>) -> Result<a::AdministrationSnapshot, Status> {
    let value = value.ok_or_else(invalid)?;
    if value.heads.is_empty() || value.heads.len() > 256 {
        return Err(invalid());
    }
    let heads: BTreeSet<_> = value
        .heads
        .iter()
        .map(|id| m::OperationId::from_slice(id).map_err(|_| invalid()))
        .collect::<Result<_, _>>()?;
    if heads.len() != value.heads.len() {
        return Err(invalid());
    }
    Ok(a::AdministrationSnapshot {
        epoch: m::EpochId::from_slice(&value.epoch).map_err(|_| invalid())?,
        position: m::WalPosition(value.position),
        code_state_version: value.code_state_version,
        heads,
    })
}
pub(crate) fn wire_snapshot(value: a::AdministrationSnapshot) -> wire::Snapshot {
    wire::Snapshot {
        epoch: value.epoch.as_bytes().to_vec(),
        position: value.position.0,
        code_state_version: value.code_state_version,
        heads: value
            .heads
            .into_iter()
            .map(|id| id.as_bytes().to_vec())
            .collect(),
    }
}
fn permission(value: i32) -> Result<Permission, Status> {
    match wire::TokenPermission::try_from(value).map_err(|_| invalid())? {
        wire::TokenPermission::Read => Ok(Permission::Read),
        wire::TokenPermission::Write => Ok(Permission::Write),
        wire::TokenPermission::Admin => Ok(Permission::Admin),
        _ => Err(invalid()),
    }
}
fn wire_permission(value: Permission) -> Result<i32, Status> {
    Ok(match value {
        Permission::Read => wire::TokenPermission::Read,
        Permission::Write => wire::TokenPermission::Write,
        Permission::Admin => wire::TokenPermission::Admin,
        _ => return Err(invalid()),
    } as i32)
}
pub(crate) fn parse_secret(value: &mut String) -> Result<BearerToken, Status> {
    let value = Zeroizing::new(std::mem::take(value));
    BearerToken::parse(&value).map_err(|_| invalid())
}
pub(crate) fn request(value: wire::TokenRequest) -> Result<a::TokenRequest, Status> {
    let change = match value.change.ok_or_else(invalid)? {
        wire::token_request::Change::Grant(mut grant) => a::TokenChange::Grant {
            credential: parse_secret(&mut grant.credential)?,
            principal: m::PrincipalId::from_slice(&grant.principal).map_err(|_| invalid())?,
            permission: permission(grant.permission)?,
            expires_at: grant.expires_at,
        },
        wire::token_request::Change::Revoke(token) => a::TokenChange::Revoke {
            token: m::TokenId::from_slice(&token).map_err(|_| invalid())?,
        },
    };
    Ok(a::TokenRequest {
        request_id: m::RequestId::from_slice(&value.request_id).map_err(|_| invalid())?,
        expected: snapshot(value.expected)?,
        change,
    })
}
pub(crate) fn wire_request(
    repository: m::RepositoryId,
    value: &a::TokenRequest,
) -> Result<wire::TokenRequest, Status> {
    if value.expected.heads.is_empty() || value.expected.heads.len() > 256 {
        return Err(invalid());
    }
    let change = match &value.change {
        a::TokenChange::Grant {
            credential,
            principal,
            permission,
            expires_at,
        } => wire::token_request::Change::Grant(wire::TokenGrant {
            credential: credential.expose().to_string(),
            principal: principal.as_bytes().to_vec(),
            permission: wire_permission(*permission)?,
            expires_at: *expires_at,
        }),
        a::TokenChange::Revoke { token } => {
            wire::token_request::Change::Revoke(token.as_bytes().to_vec())
        }
    };
    Ok(wire::TokenRequest {
        repository: repository.as_bytes().to_vec(),
        request_id: value.request_id.as_bytes().to_vec(),
        expected: Some(wire_snapshot(value.expected.clone())),
        change: Some(change),
    })
}
fn description(value: wire::TokenDescription) -> Result<TokenDescription, Status> {
    Ok(TokenDescription {
        id: m::TokenId::from_slice(&value.id).map_err(|_| invalid())?,
        principal: m::PrincipalId::from_slice(&value.principal).map_err(|_| invalid())?,
        permission: permission(value.permission)?,
        expires_at: value.expires_at,
    })
}
fn wire_description(value: TokenDescription) -> Result<wire::TokenDescription, Status> {
    Ok(wire::TokenDescription {
        id: value.id.as_bytes().to_vec(),
        principal: value.principal.as_bytes().to_vec(),
        permission: wire_permission(value.permission)?,
        expires_at: value.expires_at,
    })
}
pub(crate) fn listing(value: wire::TokenListing) -> Result<a::TokenListing, Status> {
    if value.tokens.len() > 128 {
        return Err(invalid());
    }
    let tokens: Vec<_> = value
        .tokens
        .into_iter()
        .map(description)
        .collect::<Result<_, _>>()?;
    if tokens.windows(2).any(|pair| pair[0].id >= pair[1].id) {
        return Err(invalid());
    }
    Ok(a::TokenListing {
        snapshot: snapshot(value.snapshot)?,
        tokens,
    })
}
pub(crate) fn wire_listing(value: a::TokenListing) -> Result<wire::TokenListing, Status> {
    Ok(wire::TokenListing {
        snapshot: Some(wire_snapshot(value.snapshot)),
        tokens: value
            .tokens
            .into_iter()
            .map(wire_description)
            .collect::<Result<_, _>>()?,
    })
}
pub(crate) fn wire_reply(value: Option<a::TokenReceipt>) -> Result<wire::TokenReply, Status> {
    Ok(wire::TokenReply {
        receipt: value
            .map(|value| {
                let result = match value.result {
                    a::TokenResult::Granted { token } => {
                        wire::token_receipt::Result::Granted(wire_description(token)?)
                    }
                    a::TokenResult::Revoked { token } => {
                        wire::token_receipt::Result::Revoked(token.as_bytes().to_vec())
                    }
                };
                Ok::<_, Status>(wire::TokenReceipt {
                    request_id: value.request_id.as_bytes().to_vec(),
                    publication_id: value.publication_id.as_bytes().to_vec(),
                    accepted: Some(wire_snapshot(value.accepted)),
                    result: Some(result),
                })
            })
            .transpose()?,
    })
}
pub(crate) fn reply(
    value: wire::TokenReply,
    request: &a::TokenRequest,
) -> Result<Option<a::TokenReceipt>, Status> {
    let Some(value) = value.receipt else {
        return Ok(None);
    };
    let result = match value.result.ok_or_else(invalid)? {
        wire::token_receipt::Result::Granted(token) => a::TokenResult::Granted {
            token: description(token)?,
        },
        wire::token_receipt::Result::Revoked(token) => a::TokenResult::Revoked {
            token: m::TokenId::from_slice(&token).map_err(|_| invalid())?,
        },
    };
    let receipt = a::TokenReceipt {
        request_id: m::RequestId::from_slice(&value.request_id).map_err(|_| invalid())?,
        publication_id: m::PublicationId::from_slice(&value.publication_id)
            .map_err(|_| invalid())?,
        accepted: snapshot(value.accepted)?,
        result,
    };
    let expected_result = match &request.change {
        a::TokenChange::Grant {
            credential,
            principal,
            permission,
            expires_at,
        } => a::TokenResult::Granted {
            token: TokenDescription {
                id: credential.id(),
                principal: *principal,
                permission: *permission,
                expires_at: *expires_at,
            },
        },
        a::TokenChange::Revoke { token } => a::TokenResult::Revoked { token: *token },
    };
    if receipt.request_id != request.request_id
        || receipt.result != expected_result
        || receipt.accepted.epoch != request.expected.epoch
        || receipt.accepted.heads != request.expected.heads
        || request.expected.position.0.checked_add(1) != Some(receipt.accepted.position.0)
    {
        return Err(invalid());
    }
    Ok(Some(receipt))
}
pub(crate) fn bad_response(_: Status) -> Status {
    Status::data_loss("invalid administration response")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_receipt_validation_binds_action_request_and_exact_snapshot() {
        let request = a::TokenRequest {
            request_id: m::RequestId::new([1; 16]),
            expected: a::AdministrationSnapshot {
                epoch: m::EpochId::new([2; 16]),
                position: m::WalPosition(7),
                code_state_version: 0,
                heads: [m::OperationId::new([0; 64])].into(),
            },
            change: a::TokenChange::Revoke {
                token: m::TokenId::new([3; 16]),
            },
        };
        let receipt = a::TokenReceipt {
            request_id: request.request_id,
            publication_id: m::PublicationId::new([4; 16]),
            accepted: a::AdministrationSnapshot {
                position: m::WalPosition(8),
                ..request.expected.clone()
            },
            result: a::TokenResult::Revoked {
                token: m::TokenId::new([3; 16]),
            },
        };
        assert_eq!(
            reply(wire_reply(Some(receipt.clone())).unwrap(), &request).unwrap(),
            Some(receipt.clone())
        );
        assert!(
            reply(wire_reply(None).unwrap(), &request)
                .unwrap()
                .is_none()
        );
        for field in 0..5 {
            let mut changed = receipt.clone();
            match field {
                0 => changed.request_id = m::RequestId::new([9; 16]),
                1 => changed.accepted.epoch = m::EpochId::new([9; 16]),
                2 => changed.accepted.position = m::WalPosition(9),
                3 => changed.accepted.heads = [m::OperationId::new([9; 64])].into(),
                _ => {
                    changed.result = a::TokenResult::Revoked {
                        token: m::TokenId::new([9; 16]),
                    }
                }
            }
            assert!(reply(wire_reply(Some(changed)).unwrap(), &request).is_err());
        }
    }
    #[test]
    fn admin_decoding_clears_credentials_and_rejects_unbounded_or_duplicate_results() {
        let token = BearerToken::generate().unwrap();
        let mut secret = token.expose().to_string();
        assert_eq!(parse_secret(&mut secret).unwrap().id(), token.id());
        assert!(secret.is_empty());
        let mut invalid_secret = "not a token".to_owned();
        assert!(parse_secret(&mut invalid_secret).is_err());
        assert!(invalid_secret.is_empty());
        let snapshot = wire::Snapshot {
            epoch: vec![1; 16],
            position: 0,
            code_state_version: 0,
            heads: vec![vec![0; 64]],
        };
        let token = wire::TokenDescription {
            id: vec![2; 16],
            principal: vec![3; 16],
            permission: 1,
            expires_at: None,
        };
        for n in [2, 129] {
            assert!(
                listing(wire::TokenListing {
                    snapshot: Some(snapshot.clone()),
                    tokens: vec![token.clone(); n]
                })
                .is_err()
            );
        }
    }
}
