use super::{
    LocalError, LocalProfile, LocalRepository, LocalResult, collab_external::ExternalV1,
    collab_graph,
};
use jj_collab::*;
use jj_store_jj::local_metadata::{Domain, GitOid, Snapshot};
use jj_store_model as m;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalCollabSnapshot {
    pub profile: LocalProfile,
    pub repo_id: Option<m::RepositoryId>,
    pub metadata_commit: Option<GitOid>,
    pub state_version: u64,
}
pub struct LocalThreadStore {
    pub snapshot: LocalCollabSnapshot,
    pub state: ThreadStateV1,
    pub(crate) graph: CollabGraphV1,
    pub(crate) external: ExternalV1,
}
pub struct LocalReactionStore {
    pub snapshot: LocalCollabSnapshot,
    pub(crate) state: ReactionStateV1,
    pub(crate) graph: CollabGraphV1,
    pub(crate) external: ExternalV1,
}
impl LocalThreadStore {
    pub fn object(&self, reference: CollabObjectRef) -> LocalResult<CollabObjectV1> {
        self.graph
            .objects
            .get(&reference)
            .cloned()
            .ok_or(LocalError::Stale)
    }
}
impl LocalReactionStore {
    pub fn emojis(
        &self,
        after: Option<CustomEmojiId>,
        limit: u32,
    ) -> LocalResult<LocalEmojiPageV1> {
        if limit == 0 || limit > 256 {
            return Err(LocalError::Invalid);
        }
        let mut definitions = Vec::new();
        for (id, reference) in &self.state.emojis {
            if after.is_some_and(|a| *id <= a) {
                continue;
            }
            let CollabObjectV1::CustomEmojiVersion(value) = self.graph.get(*reference)? else {
                return Err(LocalError::Corrupt);
            };
            definitions.push(value.clone());
            if definitions.len() > limit as usize {
                break;
            }
        }
        let more = definitions.len() > limit as usize;
        definitions.truncate(limit as usize);
        let next_id = if more {
            definitions.last().map(|v| v.emoji_id)
        } else {
            None
        };
        Ok(LocalEmojiPageV1 {
            snapshot: self.snapshot.clone(),
            definitions,
            next_id,
        })
    }
    pub fn object(&self, reference: CollabObjectRef) -> LocalResult<CollabObjectV1> {
        if !matches!(
            reference.kind,
            CollabObjectKind::CustomEmojiVersion | CollabObjectKind::ImageAsset
        ) {
            return Err(LocalError::Invalid);
        }
        self.graph
            .objects
            .get(&reference)
            .cloned()
            .ok_or(LocalError::Stale)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalEmojiPageV1 {
    pub snapshot: LocalCollabSnapshot,
    pub definitions: Vec<CustomEmojiVersionV1>,
    pub next_id: Option<CustomEmojiId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalReactionRowV1 {
    pub actor: m::PrincipalId,
    pub active: std::collections::BTreeSet<CollabObjectRef>,
    pub value: Option<ReactionValueV1>,
    pub conflict: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalReactionPageV1 {
    pub snapshot: LocalCollabSnapshot,
    pub target: ReactionTargetV1,
    pub revision: TargetRevisionV1,
    pub rows: Vec<LocalReactionRowV1>,
    pub next_actor: Option<m::PrincipalId>,
}
impl LocalRepository {
    /// Expected metadata prevents a continuation from silently changing its roots.
    /// Local filesystem admission authorizes this repository and its stored actors.
    pub async fn reaction_query(
        &self,
        target: &ReactionTargetV1,
        after: Option<m::PrincipalId>,
        limit: u32,
        expected: Option<GitOid>,
    ) -> LocalResult<LocalReactionPageV1> {
        if limit == 0 || limit > 256 || (after.is_some() && expected.is_none()) {
            return Err(LocalError::Invalid);
        }
        let pin = self.metadata.read()?;
        if expected.is_some() && expected != pin.as_ref().map(|s| s.commit()) {
            return Err(LocalError::Stale);
        }
        let repository = pin
            .as_ref()
            .map_or(m::RepositoryId::new([0; 16]), |s| s.repository());
        let threads = self.thread_store_at(pin.as_ref()).await?;
        let reactions = self.reaction_store_at(pin.as_ref(), repository).await?;
        let agents = self.agent_store_at(pin.as_ref(), repository).await?;
        let reviews = self.review_store_at(pin.as_ref(), repository).await?;
        let work = self.work_state(pin.as_ref())?;
        let reader = jj_store_wal::publication::CollabTargetReaderV1::new(
            repository,
            Some((threads.state.root, &threads.graph)),
            Some(&agents),
            Some(&reviews),
        )?
        .with_work(repository, &work)?;
        let resolved = resolve_reaction_target(&reader, target).await?;
        let mut matching = std::collections::BTreeMap::new();
        for reference in reactions.state.slots.values() {
            let CollabObjectV1::ReactionSlot(slot) = reactions.graph.get(*reference)? else {
                return Err(LocalError::Corrupt);
            };
            if slot.target.exact_key()? != resolved.exact_key
                || after.is_some_and(|actor| slot.actor <= actor)
            {
                continue;
            }
            if matching.insert(slot.actor, slot).is_some() {
                return Err(LocalError::Corrupt);
            }
        }
        let mut rows = Vec::new();
        let mut bytes = 4096usize;
        let mut more = false;
        for slot in matching.into_values() {
            let selection = slot.resolve(&reactions.graph.objects)?;
            if selection.active.is_empty() {
                continue;
            }
            let row = LocalReactionRowV1 {
                actor: slot.actor,
                conflict: selection.is_conflict(),
                active: selection.active,
                value: selection.value,
            };
            let size = serde_json::to_vec(&row)
                .map_err(|_| LocalError::Corrupt)?
                .len();
            if rows.len() >= limit as usize || bytes.saturating_add(size) > 4 * 1024 * 1024 {
                if rows.is_empty() {
                    return Err(LocalError::Limit);
                }
                more = true;
                break;
            }
            bytes += size;
            rows.push(row);
        }
        let next_actor = if more {
            rows.last().map(|r| r.actor)
        } else {
            None
        };
        Ok(LocalReactionPageV1 {
            snapshot: reactions.snapshot,
            target: target.clone(),
            revision: resolved.revision,
            rows,
            next_actor,
        })
    }
}
fn snapshot(selected: Option<&Snapshot>, domain: Domain) -> LocalCollabSnapshot {
    LocalCollabSnapshot {
        profile: LocalProfile::LocalJjGit,
        repo_id: selected.map(|s| s.repository()),
        metadata_commit: selected.map(|s| s.commit()),
        state_version: selected
            .and_then(|s| s.selection(domain))
            .map_or(0, |s| s.version),
    }
}
impl LocalRepository {
    pub async fn thread_store(&self) -> LocalResult<LocalThreadStore> {
        let pin = self.metadata.read()?;
        self.thread_store_at(pin.as_ref()).await
    }
    pub async fn reaction_store(&self) -> LocalResult<LocalReactionStore> {
        let pin = self.metadata.read()?;
        self.reaction_store_at(pin.as_ref(), m::RepositoryId::new([0; 16]))
            .await
    }
    pub(crate) async fn thread_store_at(
        &self,
        pin: Option<&Snapshot>,
    ) -> LocalResult<LocalThreadStore> {
        let bytes = pin
            .map(|s| self.metadata.read_domain(s, Domain::Thread))
            .transpose()?
            .flatten();
        let (root, graph, external) = match bytes {
            Some(bytes) => {
                collab_graph::read::<ThreadRootV1>(self, pin.ok_or(LocalError::Corrupt)?, &bytes)
                    .await?
            }
            None => (
                ThreadRootV1::empty(),
                CollabGraphV1::default(),
                ExternalV1::default(),
            ),
        };
        let state = ThreadStateV1::read(root, &graph).map_err(|_| LocalError::Corrupt)?;
        Ok(LocalThreadStore {
            snapshot: snapshot(pin, Domain::Thread),
            state,
            graph,
            external,
        })
    }
    pub(crate) async fn reaction_store_at(
        &self,
        pin: Option<&Snapshot>,
        proposed: m::RepositoryId,
    ) -> LocalResult<LocalReactionStore> {
        let bytes = pin
            .map(|s| self.metadata.read_domain(s, Domain::Reaction))
            .transpose()?
            .flatten();
        let (root, graph, external) = match bytes {
            Some(bytes) => {
                collab_graph::read::<ReactionRootV1>(self, pin.ok_or(LocalError::Corrupt)?, &bytes)
                    .await?
            }
            None => (
                ReactionRootV1::empty(),
                CollabGraphV1::default(),
                ExternalV1::default(),
            ),
        };
        let state = ReactionStateV1::read(pin.map_or(proposed, |s| s.repository()), root, &graph)
            .map_err(|_| LocalError::Corrupt)?;
        Ok(LocalReactionStore {
            snapshot: snapshot(pin, Domain::Reaction),
            state,
            graph,
            external,
        })
    }
}
