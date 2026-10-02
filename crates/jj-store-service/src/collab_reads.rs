//! Credential-free collaboration pins; every query reauthorizes its current viewer.
mod materialization;
use crate::{
    auth::Permission,
    collab_mutations::{Error, Result},
    native::NativeSession,
};
use jj_collab::*;
use jj_store_model as m;
use jj_store_wal::{
    self as wal,
    publication::{AgentStoreSelection, CollabTargetReaderV1, ReviewStoreSelection},
};
pub(crate) use materialization::Cache;
pub use materialization::ReadScope;
use materialization::{Data, ReactionData, ThreadData};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

const MAX_PAGE: u32 = 256;
const MAX_BYTES: usize = 4 * 1024 * 1024;

/// A trusted repository policy filters actors before counts or identities exist.
pub trait ReactionActorVisibility: Send + Sync {
    fn can_disclose(
        &self,
        repository: m::RepositoryId,
        viewer: m::PrincipalId,
        actor: m::PrincipalId,
    ) -> bool;
}
/// The initial repository-scoped policy has no private actor subresources. A
/// caller must first hold current repository Read access; another repo is denied.
/// The current repository access profile has no private actor subresources.
/// Pinned reads reauthorize repository admission before consulting this policy.
pub struct RepositoryMemberReactionActors;
impl ReactionActorVisibility for RepositoryMemberReactionActors {
    fn can_disclose(&self, _: m::RepositoryId, _: m::PrincipalId, _: m::PrincipalId) -> bool {
        true
    }
}
pub struct RepositoryReactionActors(pub m::RepositoryId);
impl ReactionActorVisibility for RepositoryReactionActors {
    fn can_disclose(
        &self,
        repository: m::RepositoryId,
        _: m::PrincipalId,
        _: m::PrincipalId,
    ) -> bool {
        repository == self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CollabReadSnapshotV1 {
    pub version: u32,
    pub repo_id: m::RepositoryId,
    pub epoch: m::EpochId,
    pub manifest_digest: m::RecordDigest,
    pub published_seq: m::WalPosition,
    pub code_state_version: u64,
    pub operation_heads: BTreeSet<m::OperationId>,
    pub thread_root: Option<m::RecordDigest>,
    pub thread_state_version: u64,
    pub reaction_root: Option<m::RecordDigest>,
    pub reaction_state_version: u64,
    pub agent_root: Option<m::RecordDigest>,
    pub review_root: Option<m::RecordDigest>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadDirectoryMetadata {
    pub message_count: u32,
    pub reply_count: u32,
    pub excerpt: String,
    pub participants: Vec<m::PrincipalId>,
    pub last_activity_at_millis: i64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadSummaryV1 {
    pub reference: CollabObjectRef,
    pub value: ThreadVersionV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<ThreadDirectoryMetadata>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadDetailV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub thread: ThreadSummaryV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct HumanMessageDetailV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub target: ReactionTargetV1,
    pub message: HumanMessageRevisionV1,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageDetailV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub reference: CollabObjectRef,
    pub asset: ImageAssetV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadPageV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub threads: Vec<ThreadSummaryV1>,
    pub next_id: Option<ThreadId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadEventItemV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ThreadLinkV1>,
    pub message: Option<HumanMessageRevisionV1>,
    pub reference: CollabObjectRef,
    pub value: ThreadEventV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ThreadEventPageV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub thread_id: ThreadId,
    /// Newest-first local Thread history. Source links are immutable references;
    /// these entries are not a copied agent transcript or a global causal order.
    pub events: Vec<ThreadEventItemV1>,
    pub next_event: Option<CollabObjectRef>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReactionActorRowV1 {
    pub actor: m::PrincipalId,
    pub active: BTreeSet<CollabObjectRef>,
    pub value: Option<ReactionValueV1>,
    pub conflict: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReactionAggregateV1 {
    pub value: ReactionValueV1,
    pub count: u32,
    pub viewer_reacted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReactionPageV1 {
    pub snapshot: CollabReadSnapshotV1,
    pub target: ReactionTargetV1,
    pub revision: TargetRevisionV1,
    pub reactions: Vec<ReactionAggregateV1>,
    pub conflict_count: u32,
    pub actors: Vec<ReactionActorRowV1>,
    pub next_actor: Option<m::PrincipalId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct EmojiPageV1 {
    pub versions: BTreeMap<CustomEmojiId, CollabObjectRef>,
    pub snapshot: CollabReadSnapshotV1,
    pub definitions: Vec<CustomEmojiVersionV1>,
    pub next_id: Option<CustomEmojiId>,
}

/// Immutable, bounded source closure without a stored session or credential.
/// Transport continuation registries own its capacity, expiry and query binding.
pub struct PinnedCollabRead {
    snapshot: CollabReadSnapshotV1,
    viewer: m::PrincipalId,
    thread: Arc<ThreadData>,
    reaction: Arc<ReactionData>,
    agents: Option<Arc<AgentStoreSelection>>,
    reviews: Option<Arc<ReviewStoreSelection>>,
    actors: Arc<dyn ReactionActorVisibility>,
}
impl PinnedCollabRead {
    pub async fn open(
        session: Arc<NativeSession>,
        actors: Arc<dyn ReactionActorVisibility>,
    ) -> Result<Self> {
        Self::open_scoped(session, actors, ReadScope::All, None).await
    }
    pub(crate) async fn open_scoped(
        session: Arc<NativeSession>,
        actors: Arc<dyn ReactionActorVisibility>,
        scope: ReadScope,
        cache: Option<&Cache>,
    ) -> Result<Self> {
        query(session.limits.timeout, async {
            let viewer = session.principal(Permission::Read).await?.id();
            let pin = session.snapshot().await?;
            let manifest = pin.manifest();
            let snapshot = CollabReadSnapshotV1 {
                version: 1,
                repo_id: session.repository,
                epoch: manifest.epoch,
                manifest_digest: pin.digest(),
                published_seq: manifest.published,
                code_state_version: manifest.code_state_version,
                operation_heads: manifest.heads.clone(),
                thread_root: manifest.thread_root.map(|r| r.digest),
                thread_state_version: manifest.thread_state_version,
                reaction_root: manifest.reaction_root.map(|r| r.digest),
                reaction_state_version: manifest.reaction_state_version,
                agent_root: manifest.agent_root.map(|r| r.digest),
                review_root: manifest.review_root.map(|r| r.digest),
            };
            let domains = scope.domains();
            let roots = [
                manifest.thread_root,
                manifest.reaction_root,
                manifest.agent_root,
                manifest.review_root,
            ];
            let session_ref = session.as_ref();
            let epoch = manifest.epoch;
            let read = |index: usize| async move {
                match roots[index].filter(|_| domains[index]) {
                    Some(root) => match cache {
                        Some(cache) => cache.get(session_ref, epoch, root).await.map(Some),
                        None => materialization::load(session_ref, root).await.map(Some),
                    },
                    None => Ok(None),
                }
            };
            let (threads, reactions, agents, reviews) =
                tokio::try_join!(read(0), read(1), read(2), read(3))?;
            let thread = match threads.as_deref() {
                Some(Data::Thread(d)) => d.clone(),
                None => Arc::new(ThreadData::empty()),
                _ => return Err(Error::Corrupt),
            };
            let reaction = match reactions.as_deref() {
                Some(Data::Reaction(d)) => d.clone(),
                None => Arc::new(ReactionData::empty()),
                _ => return Err(Error::Corrupt),
            };
            let agents = match agents.as_deref() {
                Some(Data::Agent(d)) => Some(d.clone()),
                None => None,
                _ => return Err(Error::Corrupt),
            };
            let reviews = match reviews.as_deref() {
                Some(Data::Review(d)) => Some(d.clone()),
                None => None,
                _ => return Err(Error::Corrupt),
            };
            let result = Self {
                snapshot,
                viewer,
                thread,
                reaction,
                agents,
                reviews,
                actors,
            };
            if result.object_count() > session.limits.inventory_nodes {
                return Err(Error::Limit);
            }
            result.authorize(&session).await?;
            Ok(result)
        })
        .await
    }
    pub fn object_count(&self) -> usize {
        self.thread.graph.objects.len()
            + self.reaction.graph.objects.len()
            + self.agents.as_ref().map_or(0, |s| s.object_count())
            + self.reviews.as_ref().map_or(0, |s| s.object_count())
    }
    /// Encoded owned collaboration graphs retained in memory. Agent/Review pins
    /// retain membership references and load bodies through their owned reader.
    pub fn encoded_collab_bytes(&self) -> Result<usize> {
        self.thread
            .bytes
            .checked_add(self.reaction.bytes)
            .ok_or(Error::Limit)
    }

    pub(crate) async fn authorize(&self, session: &NativeSession) -> Result<()> {
        if session.repository != self.snapshot.repo_id
            || session.principal(Permission::Read).await?.id() != self.viewer
        {
            return Err(Error::Denied);
        }
        if session.snapshot().await?.manifest().epoch != self.snapshot.epoch {
            return Err(Error::Stale);
        }
        if session.principal(Permission::Read).await?.id() != self.viewer {
            return Err(Error::Denied);
        }
        Ok(())
    }
    fn thread_value(&self, id: ThreadId) -> Result<ThreadSummaryV1> {
        let reference = *self.thread.state.threads.get(&id).ok_or(Error::NotFound)?;
        let CollabObjectV1::ThreadVersion(value) =
            self.thread.graph.get(reference).map_err(model)?
        else {
            return Err(Error::Corrupt);
        };
        if value.status == ThreadStatusV1::Tombstoned {
            return Err(Error::NotFound);
        }
        Ok(ThreadSummaryV1 {
            reference,
            value: value.clone(),
            directory: self.thread.directory.get(&id).cloned(),
        })
    }
    async fn threads_inner(
        &self,
        session: Arc<NativeSession>,
        after: Option<ThreadId>,
        limit: u32,
    ) -> Result<ThreadPageV1> {
        limit_check(limit)?;
        self.authorize(&session).await?;
        let mut values = Vec::new();
        for id in self.thread.state.threads.keys() {
            if after.is_some_and(|a| *id <= a) {
                continue;
            }
            match self.thread_value(*id) {
                Ok(value) => values.push(value),
                Err(Error::NotFound) => continue,
                Err(e) => return Err(e),
            }
            if values.len() > limit as usize {
                break;
            }
        }
        let more = values.len() > limit as usize;
        values.truncate(limit as usize);
        let page = ThreadPageV1 {
            snapshot: self.snapshot.clone(),
            next_id: more.then(|| values.last().expect("nonempty page").value.thread_id),
            threads: values,
        };
        bounded(&page)?;
        self.authorize(&session).await?;
        Ok(page)
    }
    async fn thread_inner(
        &self,
        session: Arc<NativeSession>,
        id: ThreadId,
    ) -> Result<ThreadSummaryV1> {
        self.authorize(&session).await?;
        let value = self.thread_value(id)?;
        self.authorize(&session).await?;
        Ok(value)
    }
    async fn events_inner(
        &self,
        session: Arc<NativeSession>,
        id: ThreadId,
        before: Option<CollabObjectRef>,
        limit: u32,
    ) -> Result<ThreadEventPageV1> {
        limit_check(limit)?;
        self.authorize(&session).await?;
        let selected = self.thread_value(id)?;
        let mut cursor = Some(selected.value.event);
        let mut history = BTreeSet::new();
        // A supplied position must be reachable from this selected Thread, not
        // merely an object in another Thread's retained closure.
        while cursor != before && before.is_some() {
            let reference = cursor.ok_or(Error::NotFound)?;
            if !history.insert(reference) || history.len() > 4096 {
                return Err(Error::Limit);
            }
            let CollabObjectV1::ThreadEvent(event) =
                self.thread.graph.get(reference).map_err(model)?
            else {
                return Err(Error::Corrupt);
            };
            if event.thread_id != id {
                return Err(Error::Corrupt);
            }
            cursor = event.previous;
        }
        let mut events = Vec::new();
        while let Some(reference) = cursor {
            if events.len() == limit as usize {
                break;
            }
            if !history.insert(reference) {
                return Err(Error::Corrupt);
            }
            let CollabObjectV1::ThreadEvent(value) =
                self.thread.graph.get(reference).map_err(model)?
            else {
                return Err(Error::Corrupt);
            };
            if value.thread_id != id {
                return Err(Error::Corrupt);
            }
            let message = if let ThreadEventKindV1::Message { revision } = value.event {
                let CollabObjectV1::HumanMessageRevision(message) =
                    self.thread.graph.get(revision).map_err(model)?
                else {
                    return Err(Error::Corrupt);
                };
                if message.thread_id != id {
                    return Err(Error::Corrupt);
                }
                Some(message.clone())
            } else {
                None
            };
            let link = if let ThreadEventKindV1::Link { version } = value.event {
                let CollabObjectV1::ThreadLink(link) =
                    self.thread.graph.get(version).map_err(|_| Error::Corrupt)?
                else {
                    return Err(Error::Corrupt);
                };
                if link.thread_id != id {
                    return Err(Error::Corrupt);
                }
                Some(link.clone())
            } else {
                None
            };
            events.push(ThreadEventItemV1 {
                link,
                message,
                reference,
                value: value.clone(),
            });
            cursor = value.previous;
        }
        let page = ThreadEventPageV1 {
            snapshot: self.snapshot.clone(),
            thread_id: id,
            events,
            next_event: cursor,
        };
        bounded(&page)?;
        self.authorize(&session).await?;
        Ok(page)
    }
    async fn message_inner(
        &self,
        session: Arc<NativeSession>,
        target: &ReactionTargetV1,
    ) -> Result<HumanMessageRevisionV1> {
        self.authorize(&session).await?;
        let ReactionTargetV1::HumanMessage { revision, .. } = target else {
            return Err(Error::Invalid);
        };
        self.resolve(&session, target).await?;
        let CollabObjectV1::HumanMessageRevision(value) =
            self.thread.graph.get(*revision).map_err(model)?
        else {
            return Err(Error::Corrupt);
        };
        let value = value.clone();
        self.authorize(&session).await?;
        Ok(value)
    }
    async fn resolve(
        &self,
        session: &NativeSession,
        target: &ReactionTargetV1,
    ) -> Result<ResolvedReactionTargetV1> {
        let agents = self
            .agents
            .as_ref()
            .map(|s| session.coordinator.selected_agent_reader(s))
            .transpose()?;
        let reviews = self
            .reviews
            .as_ref()
            .map(|s| session.coordinator.selected_review_reader(s))
            .transpose()?;
        let reader = CollabTargetReaderV1::new(
            self.snapshot.repo_id,
            Some((self.thread.state.root.clone(), &self.thread.graph)),
            agents
                .as_ref()
                .map(|s| s as &dyn wal::publication::AgentSnapshotRead),
            reviews
                .as_ref()
                .map(|s| s as &dyn wal::publication::ReviewSnapshotRead),
        )?;
        resolve_reaction_target(&reader, target)
            .await
            .map_err(model)
    }
    async fn reactions_inner(
        &self,
        session: Arc<NativeSession>,
        target: &ReactionTargetV1,
        after: Option<m::PrincipalId>,
        limit: u32,
    ) -> Result<ReactionPageV1> {
        limit_check(limit)?;
        self.authorize(&session).await?;
        let resolved = self.resolve(&session, target).await?;
        let mut rows = BTreeMap::new();
        let mut aggregates = BTreeMap::<ReactionValueV1, (u32, bool)>::new();
        let mut conflicts = 0u32;
        for reference in self
            .reaction
            .targets
            .get(&resolved.exact_key)
            .into_iter()
            .flatten()
        {
            let CollabObjectV1::ReactionSlot(slot) =
                self.reaction.graph.get(*reference).map_err(model)?
            else {
                return Err(Error::Corrupt);
            };
            if !self
                .actors
                .can_disclose(self.snapshot.repo_id, self.viewer, slot.actor)
            {
                continue;
            }
            let selected = slot.resolve(&self.reaction.graph.objects).map_err(model)?;
            if selected.active.is_empty() {
                continue;
            }
            let conflict = selected.is_conflict();
            if conflict {
                conflicts = conflicts.checked_add(1).ok_or(Error::Limit)?;
            }
            if let Some(value) = &selected.value {
                let entry = aggregates.entry(value.clone()).or_default();
                entry.0 = entry.0.checked_add(1).ok_or(Error::Limit)?;
                entry.1 |= slot.actor == self.viewer;
            }
            if after.is_none_or(|a| slot.actor > a)
                && rows
                    .insert(
                        slot.actor,
                        ReactionActorRowV1 {
                            actor: slot.actor,
                            active: selected.active,
                            value: selected.value,
                            conflict,
                        },
                    )
                    .is_some()
            {
                return Err(Error::Corrupt);
            }
        }
        let mut page = ReactionPageV1 {
            snapshot: self.snapshot.clone(),
            target: target.clone(),
            revision: resolved.revision,
            reactions: aggregates
                .into_iter()
                .map(|(value, (count, viewer_reacted))| ReactionAggregateV1 {
                    value,
                    count,
                    viewer_reacted,
                })
                .collect(),
            conflict_count: conflicts,
            next_actor: None,
            actors: Vec::new(),
        };
        let mut bytes = serde_json::to_vec(&page)
            .map_err(|_| Error::Corrupt)?
            .len()
            .checked_add(128)
            .ok_or(Error::Limit)?;
        for row in rows.into_values() {
            let size = serde_json::to_vec(&row).map_err(|_| Error::Corrupt)?.len() + 1;
            if page.actors.len() >= limit as usize || bytes.saturating_add(size) > MAX_BYTES {
                if page.actors.is_empty() {
                    return Err(Error::Limit);
                }
                page.next_actor = page.actors.last().map(|row| row.actor);
                break;
            }
            bytes += size;
            page.actors.push(row);
        }
        bounded(&page)?;
        self.authorize(&session).await?;
        Ok(page)
    }
    async fn emojis_inner(
        &self,
        session: Arc<NativeSession>,
        after: Option<CustomEmojiId>,
        limit: u32,
    ) -> Result<EmojiPageV1> {
        limit_check(limit)?;
        self.authorize(&session).await?;
        let mut definitions = Vec::new();
        for (id, reference) in &self.reaction.state.emojis {
            if after.is_some_and(|a| *id <= a) {
                continue;
            }
            let CollabObjectV1::CustomEmojiVersion(value) =
                self.reaction.graph.get(*reference).map_err(model)?
            else {
                return Err(Error::Corrupt);
            };
            definitions.push(value.clone());
            if definitions.len() > limit as usize {
                break;
            }
        }
        let more = definitions.len() > limit as usize;
        definitions.truncate(limit as usize);
        let versions = definitions
            .iter()
            .map(|value| (value.emoji_id, self.reaction.state.emojis[&value.emoji_id]))
            .collect();
        let page = EmojiPageV1 {
            versions,
            snapshot: self.snapshot.clone(),
            next_id: more.then(|| definitions.last().expect("nonempty page").emoji_id),
            definitions,
        };
        bounded(&page)?;
        self.authorize(&session).await?;
        Ok(page)
    }
    async fn image_inner(
        &self,
        session: Arc<NativeSession>,
        reference: CollabObjectRef,
    ) -> Result<ImageAssetV1> {
        self.authorize(&session).await?;
        if reference.kind != CollabObjectKind::ImageAsset {
            return Err(Error::Invalid);
        }
        let CollabObjectV1::ImageAsset(value) = self
            .reaction
            .graph
            .objects
            .get(&reference)
            .ok_or(Error::NotFound)?
        else {
            return Err(Error::Corrupt);
        };
        let value = value.clone();
        self.authorize(&session).await?;
        Ok(value)
    }
}
fn limit_check(value: u32) -> Result<()> {
    if !(1..=MAX_PAGE).contains(&value) {
        Err(Error::Invalid)
    } else {
        Ok(())
    }
}
fn bounded(value: &impl Serialize) -> Result<()> {
    if serde_json::to_vec(value).map_err(|_| Error::Corrupt)?.len() > MAX_BYTES {
        Err(Error::Limit)
    } else {
        Ok(())
    }
}
fn model(error: jj_collab::Error) -> Error {
    CollabCommandError::Model(error).into()
}
use wal::publication::CollabCommandError;

impl PinnedCollabRead {
    pub fn snapshot(&self) -> &CollabReadSnapshotV1 {
        &self.snapshot
    }
    pub async fn threads(
        &self,
        session: Arc<NativeSession>,
        after: Option<ThreadId>,
        limit: u32,
    ) -> Result<ThreadPageV1> {
        query(
            session.limits.timeout,
            self.threads_inner(session, after, limit),
        )
        .await
    }
    pub async fn thread(
        &self,
        session: Arc<NativeSession>,
        id: ThreadId,
    ) -> Result<ThreadSummaryV1> {
        query(session.limits.timeout, self.thread_inner(session, id)).await
    }
    pub async fn events(
        &self,
        session: Arc<NativeSession>,
        id: ThreadId,
        before: Option<CollabObjectRef>,
        limit: u32,
    ) -> Result<ThreadEventPageV1> {
        query(
            session.limits.timeout,
            self.events_inner(session, id, before, limit),
        )
        .await
    }
    pub async fn message(
        &self,
        session: Arc<NativeSession>,
        target: &ReactionTargetV1,
    ) -> Result<HumanMessageRevisionV1> {
        query(session.limits.timeout, self.message_inner(session, target)).await
    }
    pub async fn reactions(
        &self,
        session: Arc<NativeSession>,
        target: &ReactionTargetV1,
        after: Option<m::PrincipalId>,
        limit: u32,
    ) -> Result<ReactionPageV1> {
        query(
            session.limits.timeout,
            self.reactions_inner(session, target, after, limit),
        )
        .await
    }
    pub async fn emojis(
        &self,
        session: Arc<NativeSession>,
        after: Option<CustomEmojiId>,
        limit: u32,
    ) -> Result<EmojiPageV1> {
        query(
            session.limits.timeout,
            self.emojis_inner(session, after, limit),
        )
        .await
    }
    pub async fn image(
        &self,
        session: Arc<NativeSession>,
        reference: CollabObjectRef,
    ) -> Result<ImageAssetV1> {
        query(session.limits.timeout, self.image_inner(session, reference)).await
    }
}
async fn query<T>(
    timeout: std::time::Duration,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    let started = tokio::time::Instant::now();
    let result = tokio::time::timeout(timeout, future)
        .await
        .map_err(|_| Error::Limit)?;
    if started.elapsed() >= timeout {
        return Err(Error::Limit);
    }
    result
}
