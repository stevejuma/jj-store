use super::Result;
use super::*;
use jj_collab::*;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollabGraphArchiveV1 {
    pub thread: Option<ThreadRootV1>,
    pub reaction: Option<ReactionRootV1>,
    pub objects: Vec<(CollabObjectRef, CollabObjectV1)>,
}
fn target(
    v: &mut ReactionTargetV1,
    c: &BTreeMap<CollabObjectRef, CollabObjectRef>,
    a: &BTreeMap<AgentObjectRef, AgentObjectRef>,
    r: &BTreeMap<ReviewObjectRef, ReviewObjectRef>,
) -> Result<()> {
    match v {
        ReactionTargetV1::HumanMessage { revision, .. } => *revision = mapped(c, *revision)?,
        ReactionTargetV1::AgentMessage { batch, content, .. } => {
            *batch = mapped(a, *batch)?;
            *content = mapped(a, *content)?
        }
        ReactionTargetV1::ToolInvocation { snapshot, .. }
        | ReactionTargetV1::AgentCheckpoint {
            version: snapshot, ..
        } => *snapshot = mapped(a, *snapshot)?,
        ReactionTargetV1::ToolOutput {
            snapshot, content, ..
        } => {
            *snapshot = mapped(a, *snapshot)?;
            *content = mapped(a, *content)?
        }
        ReactionTargetV1::ReviewComment { revision, .. } => *revision = mapped(r, *revision)?,
        ReactionTargetV1::RepositoryReviewComment { .. } => {
            return Err("qualified review conversion is not supported".into());
        }
        ReactionTargetV1::Thread { .. }
        | ReactionTargetV1::ReviewThread { .. }
        | ReactionTargetV1::WorkComment { .. } => {}
    }
    Ok(())
}
fn translate(
    v: &mut CollabObjectV1,
    c: &BTreeMap<CollabObjectRef, CollabObjectRef>,
    a: &BTreeMap<AgentObjectRef, AgentObjectRef>,
    r: &BTreeMap<ReviewObjectRef, ReviewObjectRef>,
    repository: m::RepositoryId,
) -> Result<()> {
    let remap = |v: &mut CollabObjectRef| -> Result<()> {
        *v = mapped(c, *v)?;
        Ok(())
    };
    match v {
        CollabObjectV1::IndexPage(_) => {
            return Err("conversion rebuilds collaboration catalogs".into());
        }
        CollabObjectV1::ConversationMetadata(v) => {
            if let Some(previous) = &mut v.previous {
                remap(previous)?;
            }
            for source in v.value.agent_title_sources.values_mut() {
                source.snapshot = mapped(a, source.snapshot)?;
            }
            if let Some(g) = &mut v.value.generated {
                for source in &mut g.sources {
                    source.snapshot = mapped(a, source.snapshot)?;
                }
            }
        }
        CollabObjectV1::ThreadVersion(v) => {
            if let Some(source) = &mut v.comment_source {
                target(&mut source.target, c, a, r)?;
            }
            if let Some(t) = &mut v.agent_message {
                target(t, c, a, r)?;
            }
            if v.baseline.is_some() {
                return Err("Thread code baseline requires qualified conversion evidence".into());
            }
            if let Some(p) = &mut v.previous {
                remap(p)?
            }
            remap(&mut v.event)?
        }
        CollabObjectV1::ThreadEvent(v) => {
            if let Some(p) = &mut v.previous {
                remap(p)?
            }
            match &mut v.event {
                ThreadEventKindV1::Message { revision }
                | ThreadEventKindV1::Link { version: revision }
                | ThreadEventKindV1::Metadata { version: revision } => remap(revision)?,
                _ => {}
            }
        }
        CollabObjectV1::HumanMessageRevision(v) => {
            for p in v.previous.iter_mut().chain(v.reply_to.iter_mut()) {
                remap(p)?
            }
        }
        CollabObjectV1::ThreadLink(v) => {
            if let Some(p) = &mut v.previous {
                remap(p)?
            }
            match &mut v.target {
                ThreadLinkTargetV1::CommentReply { .. } => {
                    return Err(
                        "Conversation answer links require qualified conversion evidence".into(),
                    );
                }
                ThreadLinkTargetV1::CommentBatch {
                    comments,
                    message,
                    reply_to,
                    ..
                } => {
                    for reference in comments {
                        target(reference, c, a, r)?;
                    }
                    if let Some(reference) = reply_to {
                        remap(reference)?;
                    }
                    if let Some(reference) = message {
                        remap(reference)?;
                    }
                }
                ThreadLinkTargetV1::CaptureGap {
                    source_state,
                    event_batch,
                    binding,
                } => {
                    *source_state = mapped(a, *source_state)?;
                    *event_batch = mapped(a, *event_batch)?;
                    remap(&mut binding.observed_thread)?;
                    if let Some(p) = &mut binding.observed_assignment {
                        remap(p)?
                    }
                }
                ThreadLinkTargetV1::Session { snapshot, .. } => *snapshot = mapped(a, *snapshot)?,
                ThreadLinkTargetV1::Review { snapshot, .. } => *snapshot = mapped(r, *snapshot)?,
                ThreadLinkTargetV1::Code { .. } => {
                    return Err("Thread code link requires qualified conversion evidence".into());
                }
            }
        }
        CollabObjectV1::Reaction(v) => {
            v.repository = repository;
            target(&mut v.target, c, a, r)?;
            if let ReactionValueV1::Custom { version, .. } = &mut v.value {
                remap(version)?
            }
        }
        CollabObjectV1::ReactionRetraction(v) => {
            v.repository = repository;
            target(&mut v.target, c, a, r)?;
            v.observed = v
                .observed
                .iter()
                .map(|v| mapped(c, *v))
                .collect::<Result<_>>()?
        }
        CollabObjectV1::ReactionSlot(v) => {
            v.repository = repository;
            target(&mut v.target, c, a, r)?;
            v.additions = v
                .additions
                .iter()
                .map(|v| mapped(c, *v))
                .collect::<Result<_>>()?;
            v.retractions = v
                .retractions
                .iter()
                .map(|v| mapped(c, *v))
                .collect::<Result<_>>()?
        }
        CollabObjectV1::CustomEmojiVersion(v) => {
            remap(&mut v.asset)?;
            remap(&mut v.preview)?;
            if let Some(p) = &mut v.previous {
                remap(p)?
            }
        }
        CollabObjectV1::ImageAsset(_) => {}
    }
    v.validate()?;
    Ok(())
}
pub fn transfer_collab(
    source: &CollabGraphArchiveV1,
    repository: m::RepositoryId,
    agents: &BTreeMap<AgentObjectRef, AgentObjectRef>,
    reviews: &BTreeMap<ReviewObjectRef, ReviewObjectRef>,
) -> Result<(
    CollabGraphArchiveV1,
    CollabGraphV1,
    BTreeMap<CollabObjectRef, CollabObjectRef>,
)> {
    let original = CollabGraphV1 {
        objects: source.objects.iter().cloned().collect(),
    };
    if original.objects.len() != source.objects.len() || source.objects.len() > 100_000 {
        return Err("collaboration conversion inventory".into());
    }
    let mut pending = original
        .objects
        .iter()
        .filter(|(_, v)| !matches!(v, CollabObjectV1::IndexPage(_)))
        .map(|(r, v)| (*r, v.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut graph = CollabGraphV1::default();
    let mut map = BTreeMap::new();
    let mut keys = BTreeMap::new();
    while !pending.is_empty() {
        let mut ready = Vec::new();
        for (reference, value) in &pending {
            if value.reference()? != *reference {
                return Err("collaboration conversion identity".into());
            }
            if value.dependencies()?.iter().all(|r| map.contains_key(r)) {
                ready.push(*reference)
            }
        }
        if ready.is_empty() {
            return Err("incomplete or cyclic collaboration conversion graph".into());
        }
        for reference in ready {
            let mut value = pending.remove(&reference).unwrap();
            let old = if let CollabObjectV1::ReactionSlot(v) = &value {
                Some(v.key()?)
            } else {
                None
            };
            translate(&mut value, &map, agents, reviews, repository)?;
            if let (Some(old), CollabObjectV1::ReactionSlot(v)) = (old, &value) {
                keys.insert(old, v.key()?);
            }
            map.insert(reference, graph.stage(value)?);
        }
    }
    let mut thread = source.thread.clone();
    let mut reaction = source.reaction.clone();
    let mut fields = Vec::new();
    if let Some(t) = &mut thread {
        fields.extend([
            &mut t.threads,
            &mut t.messages,
            &mut t.links,
            &mut t.assignments,
            &mut t.metadata,
        ]);
    }
    if let Some(r) = &mut reaction {
        fields.extend([&mut r.slots, &mut r.emojis]);
    }
    for field in fields {
        let old = *field;
        let entries = original
            .catalog(old)?
            .into_iter()
            .map(|(key, value)| Ok((keys.get(&key).copied().unwrap_or(key), mapped(&map, value)?)))
            .collect::<Result<_>>()?;
        *field = graph.catalog_update(None, entries)?;
        if let (Some(old), Some(new)) = (old, *field) {
            map.insert(old, new);
        }
    }
    Ok((
        CollabGraphArchiveV1 {
            thread,
            reaction,
            objects: graph.objects.iter().map(|(r, v)| (*r, v.clone())).collect(),
        },
        graph,
        map,
    ))
}
