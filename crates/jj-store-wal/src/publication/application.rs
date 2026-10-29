//! Original typed intent is distinct from nondeterministic native preparation.
use super::*;

const MAX_COMMAND_BYTES: usize = 512 * 1024;

/// Constructed by the authenticated application service from validated typed
/// inputs. This is data, not a publication capability or a deserializable request.
#[derive(Clone, Debug, Serialize)]
pub struct ApplicationIntent {
    principal: PrincipalId,
    request_id: RequestId,
    publication_id: PublicationId,
    expected: Snapshot,
    kind: SemanticKind,
    canonical_command: Vec<u8>,
}
impl ApplicationIntent {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        expected: Snapshot,
        kind: SemanticKind,
        canonical_command: Vec<u8>,
    ) -> Result<Self> {
        if canonical_command.is_empty()
            || canonical_command.len() > MAX_COMMAND_BYTES
            || expected.heads.is_empty()
            || expected.heads.len() > MAX_HEADS
        {
            return Err(Error::Limit("application intent"));
        }
        if !matches!(
            kind,
            SemanticKind::OperationPublished | SemanticKind::HeadsNormalized
        ) {
            return Err(Error::Corrupt("application intent kind"));
        }
        Ok(Self {
            principal,
            request_id,
            publication_id,
            expected,
            kind,
            canonical_command,
        })
    }
    /// Includes every stable user input; native IDs/result bytes are candidate
    /// realization and deliberately excluded. The native request profile is intact.
    pub fn digest(&self, repository: RepositoryId) -> Result<IntentDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/application-intent/v1\0");
        hash.update(records::encode(&(1u32, repository, self), 4 * 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
    fn request(&self, heads: BTreeSet<OperationId>, result: Vec<u8>) -> Request {
        Request {
            principal: self.principal,
            request_id: self.request_id,
            publication_id: self.publication_id,
            transition: Some(Transition::Strict {
                expected: self.expected.clone(),
                heads,
            }),
            settings: None,
            result,
            kind: self.kind,
        }
    }
}

impl Coordinator {
    pub async fn publish_application(
        &self,
        intent: &ApplicationIntent,
        heads: BTreeSet<OperationId>,
        result: Vec<u8>,
    ) -> Result<PublishOutcome> {
        let request = intent.request(heads, result);
        request.validate()?;
        self.publish_with_intent(&request, intent.digest(self.records.repository)?)
            .await
    }
    /// Reauthorizes against current policy. None is pending/unknown, not rejection.
    /// No candidate reconstruction or mutation execution is needed.
    pub async fn status_application(&self, intent: &ApplicationIntent) -> Result<Option<Receipt>> {
        let request = intent.request(intent.expected.heads.clone(), Vec::new());
        let snapshot = self.read().await?;
        self.guard.authorize(&snapshot, &request).await?;
        self.lookup(
            &snapshot.manifest,
            &request,
            intent.digest(self.records.repository)?,
        )
        .await
    }
    /// Internal selected-record reader, analogous to settings_at. The service
    /// must authenticate the current principal before/after calling this method.
    /// Missing entries never establish rejection or cancellation of a request.
    pub async fn receipt_at(
        &self,
        snapshot: &ManifestSnapshot,
        principal: PrincipalId,
        request_id: RequestId,
    ) -> Result<Option<Receipt>> {
        snapshot.manifest.validate(self.records.repository)?;
        let Some(reference) = self
            .catalog
            .get(
                snapshot.manifest.receipts,
                self.request_key(principal, request_id),
            )
            .await?
        else {
            return Ok(None);
        };
        let receipt: Receipt = self.records.get(reference, RecordKind::Receipt).await?;
        if receipt.principal != principal || receipt.request_id != request_id {
            return Err(Error::Corrupt("receipt request identity"));
        }
        self.validate_receipt_position(&snapshot.manifest, &receipt)?;
        let companion = self
            .catalog
            .get(
                snapshot.manifest.receipts,
                self.publication_key(receipt.publication_id),
            )
            .await?;
        if companion != Some(reference) {
            return Err(Error::Corrupt("receipt companion index missing"));
        }
        Ok(Some(receipt))
    }
}

impl Coordinator {
    /// Validate an unpublished preview with the publication guard and complete
    /// dependency traversal. This method writes no inventory, receipt or manifest.
    pub async fn validate_application(
        &self,
        intent: &ApplicationIntent,
        heads: BTreeSet<OperationId>,
    ) -> Result<()> {
        let request = intent.request(heads.clone(), Vec::new());
        request.validate()?;
        tokio::time::timeout(self.limits.timeout, async {
            let before = self.read().await?;
            self.guard.authorize(&before, &request).await?;
            if !same_code_state(&before.manifest.snapshot(), &intent.expected) {
                return Err(Error::Stale);
            }
            let after = Snapshot {
                epoch: intent.expected.epoch,
                position: WalPosition(
                    intent
                        .expected
                        .position
                        .0
                        .checked_add(1)
                        .ok_or(Error::Limit("publication sequence"))?,
                ),
                code_state_version: if before.manifest.format >= 2 {
                    intent
                        .expected
                        .code_state_version
                        .checked_add(1)
                        .ok_or(Error::Limit("code state version"))?
                } else {
                    0
                },
                heads,
            };
            self.guard
                .validate_transition(&before, &request, &after)
                .await?;
            self.verify_delta(
                before.manifest.inventory,
                after.heads.iter().copied().map(ObjectKey::Operation),
            )
            .await?;
            let latest = self.read().await?;
            self.guard.authorize(&latest, &request).await?;
            if !same_code_state(&latest.manifest.snapshot(), &before.manifest.snapshot()) {
                return Err(Error::Stale);
            }
            Ok(())
        })
        .await
        .map_err(|_| Error::Limit("preview validation timeout"))?
    }
}

impl Coordinator {
    /// Exact domain selection accepted by a selected original receipt. This
    /// bounded historical lookup neither stages records nor follows current roots.
    pub async fn receipt_domain_at(
        &self,
        snapshot: &ManifestSnapshot,
        receipt: &Receipt,
        domain: Domain,
    ) -> Result<DomainState> {
        if self
            .receipt_at(snapshot, receipt.principal, receipt.request_id)
            .await?
            .as_ref()
            != Some(receipt)
        {
            return Err(Error::IntentConflict);
        }
        let after = receipt
            .accepted
            .position
            .0
            .checked_sub(1)
            .ok_or(Error::Corrupt("receipt position"))?;
        let rows = snapshot
            .selected_records(self.records.store.clone(), WalPosition(after), 1)
            .await?;
        let tx = rows
            .first()
            .ok_or(Error::Corrupt("receipt transaction missing"))?
            .transaction();
        if tx.after != receipt.accepted || tx.repository != self.records.repository {
            return Err(Error::Corrupt("receipt transaction identity"));
        }
        let original: Receipt = self.records.get(tx.receipt, RecordKind::Receipt).await?;
        if original != *receipt {
            return Err(Error::Corrupt("receipt transaction binding"));
        }
        // The qualified compound native command is Agent capture plus Thread
        // assignment. Older transactions encode it as DomainsStateChanged.
        let bound_capture =
            if tx.kind == SemanticKind::DomainsStateChanged && domain == Domain::Agent {
                match (tx.before_domains, tx.after_domains) {
                    (Some(before), Some(after)) => {
                        let mut expected = before;
                        expected.agent = after.agent;
                        expected.thread = after.thread;
                        expected == after
                            && before.agent.version.checked_add(1) == Some(after.agent.version)
                            && after.thread.is_some_and(|thread| {
                                before.thread.map_or(0, |s| s.version).checked_add(1)
                                    == Some(thread.version)
                            })
                    }
                    _ => false,
                }
            } else {
                false
            };
        if tx.kind != domain.semantic_kind() && !bound_capture {
            return Err(Error::IntentConflict);
        }
        let selected = tx.after_domains.ok_or(Error::Corrupt("receipt domains"))?;
        match domain {
            Domain::Agent => Some(selected.agent),
            Domain::Review => Some(selected.review),
            Domain::Work => selected.work,
            Domain::Workflow => selected.workflow,
            Domain::Git => selected.git,
            Domain::Thread => selected.thread,
            Domain::Reaction => selected.reaction,
        }
        .ok_or(Error::Corrupt("receipt domain selection"))
    }
}
