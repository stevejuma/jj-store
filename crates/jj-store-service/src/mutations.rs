//! Durable typed commands. Only the manifest-selected WAL receipt is commitment.
use crate::{
    auth::Permission,
    commands::{self, Command, Executor, Prepared},
    native::NativeSession,
};
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{ApplicationIntent, PublishOutcome, SemanticKind, Snapshot},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("mutation is not authorized")]
    Denied,
    #[error("mutation changes protected native history")]
    Protected,
    #[error("mutation requires the exact current snapshot")]
    Stale,
    #[error("request identity was already used for different intent")]
    IntentConflict,
    #[error("mutation resource limit exceeded")]
    Limit,
    #[error("invalid mutation request")]
    Invalid,
    #[error("mutation receipt failed integrity validation")]
    Corrupt,
    #[error("mutation authority is unavailable; retain the original request ID")]
    Unavailable,
    #[error(transparent)]
    Preparation(commands::Error),
}

#[cfg(test)]
mod report_compatibility {
    use super::*;

    #[test]
    fn conflict_report_decodes_legacy_selected_v1_receipt_without_changing_bytes() {
        let repository = m::RepositoryId::new([1; 16]);
        let principal = m::PrincipalId::new([2; 16]);
        let request_id = m::RequestId::new([3; 16]);
        let operation = m::OperationId::new([4; 64]);
        let epoch = m::EpochId::new([5; 16]);
        let request = Request {
            version: 1,
            request_id,
            expected: Snapshot {
                epoch,
                position: m::WalPosition(7),
                heads: BTreeSet::from([m::OperationId::new([6; 64])]),
            },
            command: Command::ResolveConflicts {
                commit: m::CommitId::new([7; 64]),
                paths: vec![commands::ConflictResolution {
                    path: "dir/file".into(),
                    fingerprint: m::ConflictFingerprint::new([8; 64]),
                    resolution: commands::Resolution::Delete,
                }],
            },
        };
        // Explicit pre-extension field order: do not construct Prepared and
        // serialize it here, which would silently follow new encoding changes.
        let legacy_result = format!(
            concat!(
                "{{\"version\":1,\"operation\":\"{}\",\"created\":[],\"rewrites\":[],",
                "\"bookmarks\":{{}},\"workspaces\":{{}},",
                "\"stats\":{{\"metadata_nodes\":0,\"predicate_file_bytes\":0,",
                "\"predicate_lines\":0,\"object_reads\":0,\"object_bytes\":0,\"work\":0}},",
                "\"staged_objects\":0,\"staged_bytes\":0}}"
            ),
            operation
        );
        let bytes = format!(
            "{{\"version\":1,\"repository\":\"{repository}\",\"principal\":\"{principal}\",\"request\":{},\"result\":{legacy_result}}}",
            serde_json::to_string(&request).unwrap()
        )
        .into_bytes();
        let stored: StoredResult = serde_json::from_slice(&bytes).unwrap();
        assert!(stored.result.conflict_resolution.is_none());
        assert_eq!(encode(&stored, 1024 * 1024).unwrap(), bytes);
        let receipt = wal::publication::Receipt {
            principal,
            request_id,
            publication_id: publication_id(repository, principal, request_id),
            intent: intent(repository, principal, &request)
                .unwrap()
                .digest(repository)
                .unwrap(),
            accepted: Snapshot {
                epoch,
                position: m::WalPosition(8),
                heads: BTreeSet::from([operation]),
            },
            result: bytes,
        };
        let decoded = decode(repository, principal, request_id, receipt).unwrap();
        assert!(decoded.result.conflict_resolution.is_none());
        assert_eq!(
            serde_json::to_string(&decoded.result).unwrap(),
            legacy_result
        );
    }
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Stale => Self::Stale,
            wal::Error::IntentConflict => Self::IntentConflict,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::Corrupt(_) | wal::Error::Json(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}
impl From<commands::Error> for Error {
    fn from(error: commands::Error) -> Self {
        use jj_store_jj::query::Error as Q;
        match error {
            commands::Error::Stale => Self::Stale,
            commands::Error::Query(Q::Denied) => Self::Denied,
            commands::Error::Query(Q::Limit) => Self::Limit,
            commands::Error::Query(Q::Unavailable) => Self::Unavailable,
            _ => Self::Preparation(error),
        }
    }
}
fn semantic(error: wal::Error) -> Error {
    match error {
        wal::Error::Denied => Error::Protected,
        error => error.into(),
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub request_id: m::RequestId,
    pub expected: Snapshot,
    pub command: Command,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct Receipt {
    pub version: u32,
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub request_digest: m::IntentDigest,
    pub accepted: Snapshot,
    pub result: Prepared,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
pub struct PreviewReport {
    pub version: u32,
    pub request_id: m::RequestId,
    pub request_digest: m::IntentDigest,
    pub expected: Snapshot,
    /// Provisional native IDs; apply revalidates and may produce different IDs.
    pub result: Prepared,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Preview {
    Prepared { preview: PreviewReport },
    Committed { receipt: Receipt },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Committed { receipt: Box<Receipt> },
    OutcomeUnknown { request_id: m::RequestId },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Status {
    Committed { receipt: Box<Receipt> },
    NotKnown { request_id: m::RequestId },
    OutcomeUnknown { request_id: m::RequestId },
}
/// Retained in the same selected receipt as the head transition, not a mutable
/// candidate registry. No accepted sequence or future WAL digest is embedded.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredResult {
    version: u32,
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request: Request,
    result: Prepared,
}
fn encode(value: &impl Serialize, limit: usize) -> Result<Vec<u8>> {
    struct Bounded {
        bytes: Vec<u8>,
        limit: usize,
    }
    impl std::io::Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
                return Err(std::io::Error::other("mutation result limit"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut out = Bounded {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut out, value).map_err(|error| {
        if error.is_io() {
            Error::Limit
        } else {
            Error::Corrupt
        }
    })?;
    Ok(out.bytes)
}
async fn reauthorize(
    session: &NativeSession,
    principal: m::PrincipalId,
    epoch: m::EpochId,
) -> Result<()> {
    if session.principal(Permission::Write).await?.id() != principal {
        return Err(Error::Denied);
    }
    if session.snapshot().await?.manifest().epoch != epoch {
        return Err(Error::Stale);
    }
    if session.principal(Permission::Write).await?.id() != principal {
        return Err(Error::Denied);
    }
    Ok(())
}
fn publication_id(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request: m::RequestId,
) -> m::PublicationId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/application-publication/v1\0");
    hash.update(repository.as_bytes());
    hash.update(principal.as_bytes());
    hash.update(request.as_bytes());
    let bytes = hash.finalize();
    let mut id = [0; 16];
    id.copy_from_slice(&bytes[..16]);
    m::PublicationId::new(id)
}
fn intent(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request: &Request,
) -> Result<ApplicationIntent> {
    if request.version != 1 {
        return Err(Error::Invalid);
    }
    request.command.validate()?;
    Ok(ApplicationIntent::new(
        principal,
        request.request_id,
        publication_id(repository, principal, request.request_id),
        request.expected.clone(),
        if matches!(request.command, Command::ReconcileOperations { .. }) {
            SemanticKind::HeadsNormalized
        } else {
            SemanticKind::OperationPublished
        },
        encode(request, 512 * 1024)?,
    )?)
}
fn decode(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request_id: m::RequestId,
    receipt: wal::publication::Receipt,
) -> Result<Receipt> {
    if receipt.principal != principal
        || receipt.request_id != request_id
        || receipt.publication_id != publication_id(repository, principal, request_id)
    {
        return Err(Error::IntentConflict);
    }
    if receipt.result.len() > 1024 * 1024 {
        return Err(Error::Corrupt);
    }
    let stored: StoredResult =
        serde_json::from_slice(&receipt.result).map_err(|_| Error::Corrupt)?;
    if stored.version != 1
        || stored.repository != repository
        || stored.principal != principal
        || stored.request.request_id != request_id
        || stored.result.version != 1
        || encode(&stored, 1024 * 1024).map_err(|_| Error::Corrupt)? != receipt.result
        || intent(repository, principal, &stored.request)
            .map_err(|_| Error::Corrupt)?
            .digest(repository)
            .map_err(|_| Error::Corrupt)?
            != receipt.intent
        || receipt.accepted.epoch != stored.request.expected.epoch
        || stored.request.expected.position.0.checked_add(1) != Some(receipt.accepted.position.0)
        || receipt.accepted.heads != BTreeSet::from([stored.result.operation])
    {
        return Err(Error::Corrupt);
    }
    Ok(Receipt {
        version: 1,
        request_id,
        publication_id: receipt.publication_id,
        request_digest: receipt.intent,
        accepted: receipt.accepted,
        result: stored.result,
    })
}
#[derive(Clone)]
pub struct CommandService {
    executor: Executor,
}
impl CommandService {
    /// The serving composition shares the executor's admission across requests.
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }
    async fn prior(
        &self,
        session: &NativeSession,
        principal: m::PrincipalId,
        request: &Request,
        intent: &ApplicationIntent,
    ) -> Result<Option<Receipt>> {
        let prior = session.coordinator.status_application(intent).await?;
        reauthorize(session, principal, request.expected.epoch).await?;
        prior
            .map(|receipt| decode(session.repository, principal, request.request_id, receipt))
            .transpose()
    }
    pub async fn preview(&self, session: Arc<NativeSession>, request: &Request) -> Result<Preview> {
        tokio::time::timeout(session.limits.timeout, async {
            let principal = session.principal(Permission::Write).await?.id();
            let intent = intent(session.repository, principal, request)?;
            if let Some(receipt) = self.prior(&session, principal, request, &intent).await? {
                return Ok(Preview::Committed { receipt });
            }
            let prepared = match session
                .prepare_command(&self.executor, &request.expected, request.command.clone())
                .await
            {
                Ok(prepared) => prepared,
                Err(commands::Error::Stale) => {
                    if let Some(receipt) = self.prior(&session, principal, request, &intent).await?
                    {
                        return Ok(Preview::Committed { receipt });
                    }
                    return Err(Error::Stale);
                }
                Err(error) => return Err(error.into()),
            };
            session
                .coordinator
                .validate_application(&intent, BTreeSet::from([prepared.operation]))
                .await
                .map_err(semantic)?;
            reauthorize(&session, principal, request.expected.epoch).await?;
            Ok(Preview::Prepared {
                preview: PreviewReport {
                    version: 1,
                    request_id: request.request_id,
                    request_digest: intent.digest(session.repository)?,
                    expected: request.expected.clone(),
                    result: prepared,
                },
            })
        })
        .await
        .map_err(|_| Error::Limit)?
    }
    pub async fn apply(&self, session: Arc<NativeSession>, request: &Request) -> Result<Outcome> {
        match tokio::time::timeout(
            session.limits.timeout,
            self.apply_inner(session.clone(), request),
        )
        .await
        {
            Ok(Err(Error::Unavailable)) | Err(_) => Ok(Outcome::OutcomeUnknown {
                request_id: request.request_id,
            }),
            Ok(result) => result,
        }
    }
    async fn apply_inner(&self, session: Arc<NativeSession>, request: &Request) -> Result<Outcome> {
        let principal = session.principal(Permission::Write).await?.id();
        let intent = intent(session.repository, principal, request)?;
        if let Some(receipt) = self.prior(&session, principal, request, &intent).await? {
            return Ok(Outcome::Committed {
                receipt: Box::new(receipt),
            });
        }
        let prepared = match session
            .prepare_command(&self.executor, &request.expected, request.command.clone())
            .await
        {
            Ok(prepared) => prepared,
            Err(commands::Error::Stale) => {
                if let Some(receipt) = self.prior(&session, principal, request, &intent).await? {
                    return Ok(Outcome::Committed {
                        receipt: Box::new(receipt),
                    });
                }
                return Err(Error::Stale);
            }
            Err(error) => return Err(error.into()),
        };
        let heads = BTreeSet::from([prepared.operation]);
        let result = encode(
            &StoredResult {
                version: 1,
                repository: session.repository,
                principal,
                request: request.clone(),
                result: prepared,
            },
            1024 * 1024,
        )?;
        match session
            .coordinator
            .publish_application(&intent, heads, result)
            .await
            .map_err(semantic)?
        {
            PublishOutcome::Committed(receipt) => {
                session.events.published(session.repository);
                reauthorize(&session, principal, receipt.accepted.epoch).await?;
                Ok(Outcome::Committed {
                    receipt: Box::new(decode(
                        session.repository,
                        principal,
                        request.request_id,
                        receipt,
                    )?),
                })
            }
            PublishOutcome::OutcomeUnknown { .. } => Ok(Outcome::OutcomeUnknown {
                request_id: request.request_id,
            }),
        }
    }
    pub async fn status(
        &self,
        session: Arc<NativeSession>,
        request_id: m::RequestId,
    ) -> Result<Status> {
        match tokio::time::timeout(session.limits.timeout, async {
            let principal = session.principal(Permission::Write).await?.id();
            let current = session.snapshot().await?;
            let receipt = session
                .coordinator
                .receipt_at(&current, principal, request_id)
                .await?;
            reauthorize(&session, principal, current.manifest().epoch).await?;
            Ok(match receipt {
                Some(receipt) => Status::Committed {
                    receipt: Box::new(decode(session.repository, principal, request_id, receipt)?),
                },
                None => Status::NotKnown { request_id },
            })
        })
        .await
        {
            Ok(Err(Error::Unavailable)) | Err(_) => Ok(Status::OutcomeUnknown { request_id }),
            Ok(result) => result,
        }
    }
}
