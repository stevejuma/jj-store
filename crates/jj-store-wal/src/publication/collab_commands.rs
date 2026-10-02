//! Receipt-first collaboration commands resolved against one captured manifest.
use super::*;
use jj_collab::{
    CollabCommandResultV1, CollabCommandV1, CollabGraphV1, ReactionRootV1, ReactionStateV1,
    ThreadCodeAnchorV1, ThreadCommandV1, ThreadLinkTargetV1, ThreadRootV1, resolve_reaction_target,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollabCommandRequestV1 {
    pub schema_version: u32,
    /// The service derives this identity from its authenticated caller.
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub command: CollabCommandV1,
}
impl CollabCommandRequestV1 {
    pub fn validate(&self) -> std::result::Result<(), CollabCommandError> {
        if self.schema_version != 1 {
            return Err(jj_collab::Error::Invalid("collaboration request version").into());
        }
        self.command.validate()?;
        Ok(())
    }
    pub fn intent_digest(
        &self,
        repository: RepositoryId,
    ) -> std::result::Result<IntentDigest, CollabCommandError> {
        self.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/collab-command/v1\0");
        hash.update(records::encode(&(repository, self), 16 * 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
    fn domain(&self) -> Domain {
        match &self.command {
            CollabCommandV1::Thread { .. } => Domain::Thread,
            _ => Domain::Reaction,
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum CollabCommandError {
    #[error(transparent)]
    Publication(#[from] Error),
    #[error(transparent)]
    Model(#[from] jj_collab::Error),
}
/// Canonical typed result retained in the existing repository receipt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollabCommandReceiptV1 {
    pub schema_version: u32,
    pub accepted_at_millis: i64,
    pub result: CollabCommandResultV1,
}
impl CollabCommandReceiptV1 {
    pub fn encode(&self) -> std::result::Result<Vec<u8>, CollabCommandError> {
        if self.schema_version != 1 || self.accepted_at_millis < 0 {
            return Err(Error::Corrupt("collaboration receipt format").into());
        }
        self.result.validate()?;
        Ok(records::encode(self, 1024 * 1024)?)
    }
    pub fn decode(bytes: &[u8]) -> std::result::Result<Self, CollabCommandError> {
        if bytes.len() > 1024 * 1024 {
            return Err(Error::Limit("collaboration receipt").into());
        }
        let value: Self = serde_json::from_slice(bytes).map_err(Error::from)?;
        if value.encode()? != bytes {
            return Err(Error::Corrupt("noncanonical collaboration receipt").into());
        }
        Ok(value)
    }
}
impl Coordinator {
    pub async fn publish_collab_command(
        &self,
        request: &CollabCommandRequestV1,
    ) -> std::result::Result<PublishOutcome, CollabCommandError> {
        let intent = request.intent_digest(self.records.repository)?;
        let at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Corrupt("collaboration command time"))?
            .as_millis()
            .try_into()
            .map_err(|_| Error::Limit("collaboration command time"))?;
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_collab_command_inner(request, intent, at),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(CollabCommandError::Publication(Error::OutcomeUnknown))) | Err(_) => {
                Ok(PublishOutcome::OutcomeUnknown {
                    request_id: request.request_id,
                    publication_id: request.publication_id,
                })
            }
            Ok(Err(error)) => Err(error),
        }
    }
    pub(super) async fn collab_domain_state(
        &self,
        base: &ManifestSnapshot,
        domain: Domain,
    ) -> Result<DomainState> {
        if let Some(state) = base.manifest.domain_state(domain) {
            return Ok(state);
        }
        // Explicit first feature use selects an empty independent domain; this
        // staging step never alters the manifest or backfills sessions.
        let graph = CollabGraphV1::default();
        let root = match domain {
            Domain::Thread => {
                self.stage_native_thread_root(&ThreadRootV1::empty(), &graph)
                    .await?
            }
            Domain::Reaction => {
                self.stage_native_reaction_root(&ReactionRootV1::empty(), &graph)
                    .await?
            }
            _ => return Err(Error::Corrupt("missing collaboration source domain")),
        };
        Ok(DomainState { root, version: 0 })
    }
    async fn publish_collab_command_inner(
        &self,
        request: &CollabCommandRequestV1,
        intent: IntentDigest,
        at: i64,
    ) -> std::result::Result<Receipt, CollabCommandError> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            if base.manifest.domains().is_none() {
                return Err(
                    Error::Unsupported("explicit collaboration enablement required").into(),
                );
            }
            let domain = request.domain();
            let expected = self.collab_domain_state(&base, domain).await?;
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                domain,
                expected,
                expected.root,
                Vec::new(),
            )?;
            self.guard.authorize_domain(&base, &probe).await?;
            if let Some(receipt) = self
                .lookup_identity(
                    &base.manifest,
                    request.principal,
                    request.request_id,
                    request.publication_id,
                    intent,
                )
                .await?
            {
                return Ok(receipt);
            }
            let (root, result) = match &request.command {
                CollabCommandV1::Thread { intent: command } => {
                    self.validate_thread_sources(&base, command).await?;
                    let mut threads = self.native_thread_store(expected.root).await?;
                    let (root, result) = threads.state.apply(
                        command,
                        request.principal,
                        request.request_id,
                        at,
                        &mut threads.graph,
                    )?;
                    (
                        self.stage_native_thread_root(&root, &threads.graph).await?,
                        CollabCommandResultV1::Thread { result },
                    )
                }
                CollabCommandV1::Reaction { intent: command } => {
                    let thread_state = self.collab_domain_state(&base, Domain::Thread).await?;
                    let threads = self.native_thread_store(thread_state.root).await?;
                    let agents = self
                        .agent_store(
                            base.manifest
                                .agent_root
                                .ok_or(Error::Corrupt("agent source root"))?,
                        )
                        .await?;
                    let reviews = self
                        .review_store(
                            base.manifest
                                .review_root
                                .ok_or(Error::Corrupt("review source root"))?,
                        )
                        .await?;
                    let work = match base.manifest.work_root {
                        Some(root) => Some(self.work_store(root).await?),
                        None => None,
                    };
                    let mut reader = CollabTargetReaderV1::new(
                        self.records.repository,
                        Some((threads.state.root, &threads.graph)),
                        Some(&agents),
                        Some(&reviews),
                    )?;
                    if let Some(work) = &work {
                        reader = reader.with_work(self.records.repository, work.state())?;
                    }
                    resolve_reaction_target(&reader, command.target()).await?;
                    let mut reactions = self.native_reaction_store(expected.root).await?;
                    for reference in threads
                        .graph
                        .closure(command.target().collab_dependencies())?
                    {
                        if reactions
                            .graph
                            .stage(threads.graph.get(reference)?.clone())?
                            != reference
                        {
                            return Err(Error::Corrupt("reaction target identity").into());
                        }
                    }
                    let state = ReactionStateV1::read(
                        self.records.repository,
                        reactions.root,
                        &reactions.graph,
                    )?;
                    let (root, result) = state.apply(
                        command,
                        self.records.repository,
                        request.principal,
                        request.request_id,
                        at,
                        &mut reactions.graph,
                    )?;
                    (
                        self.stage_native_reaction_root(&root, &reactions.graph)
                            .await?,
                        CollabCommandResultV1::Reaction { result },
                    )
                }
                CollabCommandV1::Emoji { intent: command } => {
                    let mut reactions = self.native_reaction_store(expected.root).await?;
                    let state = ReactionStateV1::read(
                        self.records.repository,
                        reactions.root,
                        &reactions.graph,
                    )?;
                    let (root, version) =
                        state.apply_emoji(command, request.principal, at, &mut reactions.graph)?;
                    (
                        self.stage_native_reaction_root(&root, &reactions.graph)
                            .await?,
                        CollabCommandResultV1::Emoji { version },
                    )
                }
            };
            let result = CollabCommandReceiptV1 {
                schema_version: 1,
                accepted_at_millis: at,
                result,
            }
            .encode()?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                domain,
                expected,
                root,
                result,
            )?;
            match self
                .publish_domain_at_manifest(&domain, intent, Some(base.digest))
                .await
            {
                Err(Error::Stale) => continue,
                Ok(receipt) => return Ok(receipt),
                Err(error) => return Err(error.into()),
            }
        }
        Err(Error::Contended.into())
    }
    async fn validate_thread_sources(
        &self,
        base: &ManifestSnapshot,
        command: &ThreadCommandV1,
    ) -> std::result::Result<(), CollabCommandError> {
        if let ThreadCommandV1::Create {
            baseline: Some(anchor),
            ..
        } = command
        {
            self.validate_thread_code(base, anchor).await?;
        }
        let ThreadCommandV1::Link { target, .. } = command else {
            return Ok(());
        };
        match target {
            ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                ..
            } => {
                let agents = self
                    .agent_store(
                        base.manifest
                            .agent_root
                            .ok_or(Error::Corrupt("agent source root"))?,
                    )
                    .await?;
                let key = AgentCatalogKey::derive("session", session_id.as_bytes())
                    .map_err(super::agent::agent_error)?;
                if agents.lookup(AgentCatalogKind::Sessions, key).await? != Some(*snapshot) {
                    return Err(jj_collab::Error::Stale.into());
                }
                let AgentObjectV1::SessionSnapshot(session) = agents.read_object(*snapshot).await?
                else {
                    return Err(Error::Corrupt("session link kind").into());
                };
                if session.session_id != *session_id {
                    return Err(Error::Corrupt("session link identity").into());
                }
            }
            ThreadLinkTargetV1::Review {
                review_id,
                snapshot,
            } => {
                let reviews = self
                    .review_store(
                        base.manifest
                            .review_root
                            .ok_or(Error::Corrupt("review source root"))?,
                    )
                    .await?;
                let key = ReviewCatalogKey::derive("thread", review_id.as_bytes())
                    .map_err(super::review::review_error)?;
                if reviews.lookup(ReviewCatalogKind::Threads, key).await? != Some(*snapshot) {
                    return Err(jj_collab::Error::Stale.into());
                }
                let id = match reviews.read_object(*snapshot).await? {
                    ReviewObjectV1::ThreadSnapshot(t) => t.thread_id,
                    ReviewObjectV1::ThreadMerge(t) => t.thread_id,
                    _ => return Err(Error::Corrupt("review link kind").into()),
                };
                if id != *review_id {
                    return Err(Error::Corrupt("review link identity").into());
                }
            }
            ThreadLinkTargetV1::Code {
                anchor,
                evidence_method,
                evidence,
            } => {
                self.validate_thread_code(base, anchor).await?;
                if *evidence_method != EvidenceMethod::ManualAssertion {
                    return Err(Error::Unsupported("qualified Thread code attribution").into());
                }
                if let Some(reference) = evidence {
                    let agents = self
                        .agent_store(
                            base.manifest
                                .agent_root
                                .ok_or(Error::Corrupt("agent source root"))?,
                        )
                        .await?;
                    agents.read_object(*reference).await?;
                }
            }
        }
        Ok(())
    }
    async fn validate_thread_code(
        &self,
        base: &ManifestSnapshot,
        anchor: &ThreadCodeAnchorV1,
    ) -> Result<()> {
        let ThreadCodeAnchorV1::Native(anchor) = anchor else {
            return Err(Error::Unsupported("native portable capsule carrier"));
        };
        let accepted = self.accepted_code_operations(base).await?;
        if !anchor.operations.is_subset(&accepted) {
            return Err(Error::Stale);
        }
        if !anchor.commits.is_empty() {
            let keys: BTreeSet<_> = self
                .inventory(base, self.limits.inventory.max_nodes)
                .await?
                .into_iter()
                .map(|v| v.key)
                .collect();
            if anchor
                .commits
                .iter()
                .any(|id| !keys.contains(&ObjectKey::Commit(*id)))
            {
                return Err(Error::Stale);
            }
        }
        Ok(())
    }
}
