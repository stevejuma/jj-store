//! Canonical collaboration graphs inside repository-bound immutable records.
use super::*;
use jj_collab::{
    CollabGraphV1, CollabObjectRef, CollabObjectV1, MAX_COLLAB_OBJECTS, ReactionRootV1,
    ThreadRootV1, ThreadStateV1,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ThreadRecordV1 {
    schema_version: u32,
    root: ThreadRootV1,
    objects: Option<RecordRef>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReactionRecordV1 {
    schema_version: u32,
    root: ReactionRootV1,
    objects: Option<RecordRef>,
}
pub struct NativeThreadStoreV1 {
    pub reference: RecordRef,
    pub state: ThreadStateV1,
    pub graph: CollabGraphV1,
    pub(crate) native: BTreeSet<ObjectKey>,
    pub(crate) records: BTreeSet<RecordRef>,
}
pub struct NativeReactionStoreV1 {
    pub reference: RecordRef,
    pub root: ReactionRootV1,
    pub graph: CollabGraphV1,
    pub(crate) native: BTreeSet<ObjectKey>,
    pub(crate) records: BTreeSet<RecordRef>,
}
impl Coordinator {
    pub async fn native_thread_store(&self, reference: RecordRef) -> Result<NativeThreadStoreV1> {
        let record: ThreadRecordV1 = self.records.get(reference, RecordKind::ThreadRoot).await?;
        if record.schema_version != 1 {
            return Err(Error::Corrupt("native Thread root format"));
        }
        let (graph, mut records) = self.read_collab_graph(record.objects).await?;
        let state = ThreadStateV1::read(record.root.clone(), &graph).map_err(collab_error)?;
        verify_exact_closure(&graph, record.root.dependencies())?;
        let (native, external) = self.audit_collab_external(&graph).await?;
        records.extend(external);
        records.insert(reference);
        Ok(NativeThreadStoreV1 {
            reference,
            state,
            graph,
            native,
            records,
        })
    }
    pub async fn native_reaction_store(
        &self,
        reference: RecordRef,
    ) -> Result<NativeReactionStoreV1> {
        let record: ReactionRecordV1 = self
            .records
            .get(reference, RecordKind::ReactionRoot)
            .await?;
        if record.schema_version != 1 {
            return Err(Error::Corrupt("native Reaction root format"));
        }
        record.root.validate().map_err(collab_error)?;
        let (graph, mut records) = self.read_collab_graph(record.objects).await?;
        verify_exact_closure(&graph, record.root.dependencies())?;
        for (key, reference) in graph.catalog(record.root.slots).map_err(collab_error)? {
            let CollabObjectV1::ReactionSlot(slot) = graph.get(reference).map_err(collab_error)?
            else {
                return Err(Error::Corrupt("reaction slot catalog kind"));
            };
            if slot.repository != self.records.repository
                || slot.key().map_err(collab_error)? != key
            {
                return Err(Error::Corrupt("reaction slot repository or key"));
            }
        }
        let mut names = BTreeSet::new();
        for (key, reference) in graph.catalog(record.root.emojis).map_err(collab_error)? {
            let CollabObjectV1::CustomEmojiVersion(emoji) =
                graph.get(reference).map_err(collab_error)?
            else {
                return Err(Error::Corrupt("emoji catalog kind"));
            };
            if jj_collab::entity_key("emoji", emoji.emoji_id.as_bytes()).map_err(collab_error)?
                != key
                || (!emoji.retired && !names.insert(emoji.shortcode.clone()))
            {
                return Err(Error::Corrupt("emoji catalog key or duplicate shortcode"));
            }
        }
        let (native, external) = self.audit_collab_external(&graph).await?;
        records.extend(external);
        records.insert(reference);
        Ok(NativeReactionStoreV1 {
            reference,
            root: record.root,
            graph,
            native,
            records,
        })
    }
    pub async fn stage_native_thread_root(
        &self,
        root: &ThreadRootV1,
        graph: &CollabGraphV1,
    ) -> Result<RecordRef> {
        ThreadStateV1::read(root.clone(), graph).map_err(collab_error)?;
        let objects = self.stage_collab_graph(root.dependencies(), graph).await?;
        let reference = self
            .records
            .put(
                RecordKind::ThreadRoot,
                None,
                &ThreadRecordV1 {
                    schema_version: 1,
                    root: root.clone(),
                    objects,
                },
            )
            .await?;
        self.native_thread_store(reference).await?;
        Ok(reference)
    }
    pub async fn stage_native_reaction_root(
        &self,
        root: &ReactionRootV1,
        graph: &CollabGraphV1,
    ) -> Result<RecordRef> {
        root.validate().map_err(collab_error)?;
        let objects = self.stage_collab_graph(root.dependencies(), graph).await?;
        let reference = self
            .records
            .put(
                RecordKind::ReactionRoot,
                None,
                &ReactionRecordV1 {
                    schema_version: 1,
                    root: root.clone(),
                    objects,
                },
            )
            .await?;
        self.native_reaction_store(reference).await?;
        Ok(reference)
    }
    async fn read_collab_graph(
        &self,
        root: Option<RecordRef>,
    ) -> Result<(CollabGraphV1, BTreeSet<RecordRef>)> {
        let closure = self.catalog.closure(root, MAX_COLLAB_OBJECTS).await?;
        let mut records = closure.pages;
        let mut graph = CollabGraphV1::default();
        let mut bytes = 0usize;
        for (key, reference) in closure.entries {
            let object: CollabObjectV1 = self
                .records
                .get(reference, RecordKind::CollabObject)
                .await?;
            bytes = bytes
                .checked_add(
                    jj_collab::encode_object(&object)
                        .map_err(collab_error)?
                        .len(),
                )
                .ok_or(Error::Limit("collaboration bytes"))?;
            if bytes > 256 * 1024 * 1024 {
                return Err(Error::Limit("collaboration bytes"));
            }
            let canonical = object.reference().map_err(collab_error)?;
            if *canonical.id.as_bytes() != *key.as_bytes() {
                return Err(Error::Corrupt("collaboration record inventory identity"));
            }
            graph.objects.insert(canonical, object);
            records.insert(reference);
        }
        Ok((graph, records))
    }
    async fn stage_collab_graph(
        &self,
        roots: Vec<CollabObjectRef>,
        graph: &CollabGraphV1,
    ) -> Result<Option<RecordRef>> {
        let closure = graph.closure(roots).map_err(collab_error)?;
        let mut entries = BTreeMap::new();
        for canonical in closure {
            let object = graph.get(canonical).map_err(collab_error)?;
            let reference = self
                .records
                .put(RecordKind::CollabObject, None, object)
                .await?;
            entries.insert(RecordDigest::new(*canonical.id.as_bytes()), reference);
        }
        self.catalog.update(None, entries).await
    }
    async fn audit_collab_external(
        &self,
        graph: &CollabGraphV1,
    ) -> Result<(BTreeSet<ObjectKey>, BTreeSet<RecordRef>)> {
        let mut native = BTreeSet::new();
        let mut records = BTreeSet::new();
        let mut agent = BTreeSet::new();
        let mut review = BTreeSet::new();
        for object in graph.objects.values() {
            if !object.code_evidence().is_empty() {
                return Err(Error::Unsupported("native portable capsule carrier"));
            }
            native.extend(object.native_dependencies());
            agent.extend(object.agent_dependencies());
            review.extend(object.review_dependencies());
            match object {
                CollabObjectV1::Reaction(v) if v.repository != self.records.repository => {
                    return Err(Error::Corrupt("reaction repository binding"));
                }
                CollabObjectV1::ReactionRetraction(v)
                    if v.repository != self.records.repository =>
                {
                    return Err(Error::Corrupt("retraction repository binding"));
                }
                _ => {}
            }
        }
        let mut seen_review = BTreeSet::new();
        while let Some(reference) = review.pop_first() {
            if !seen_review.insert(reference) {
                continue;
            }
            if seen_review.len() > MAX_COLLAB_OBJECTS {
                return Err(Error::Limit("collaboration review closure"));
            }
            let object = self.read_review_object(reference).await?;
            native.extend(
                object
                    .native_dependencies()
                    .map_err(super::review::review_error)?,
            );
            agent.extend(
                object
                    .agent_dependencies()
                    .map_err(super::review::review_error)?,
            );
            review.extend(
                object
                    .review_dependencies()
                    .map_err(super::review::review_error)?,
            );
            records.insert(super::review::review_record(reference));
        }
        let mut seen_agent = BTreeSet::new();
        while let Some(reference) = agent.pop_first() {
            if !seen_agent.insert(reference) {
                continue;
            }
            if seen_agent.len() > MAX_COLLAB_OBJECTS {
                return Err(Error::Limit("collaboration agent closure"));
            }
            let object = self.read_agent_object(reference).await?;
            native.extend(
                object
                    .native_dependencies()
                    .map_err(super::agent::agent_error)?,
            );
            agent.extend(object.dependencies().map_err(super::agent::agent_error)?);
            records.insert(super::agent::agent_record(reference));
        }
        self.verify_delta(None, native.iter().copied()).await?;
        Ok((native, records))
    }
}
fn verify_exact_closure(graph: &CollabGraphV1, roots: Vec<CollabObjectRef>) -> Result<()> {
    let closure = graph.closure(roots).map_err(collab_error)?;
    let provided: BTreeSet<_> = graph.objects.keys().copied().collect();
    if closure != provided {
        return Err(Error::Corrupt("unselected collaboration inventory records"));
    }
    Ok(())
}
pub(super) fn collab_error(error: jj_collab::Error) -> Error {
    match error {
        jj_collab::Error::Stale => Error::Stale,
        jj_collab::Error::Limit => Error::Limit("collaboration model"),
        _ => Error::Corrupt("collaboration model"),
    }
}
