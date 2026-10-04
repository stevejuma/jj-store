use super::*;
pub(super) fn admit(value: &AgentObjectV1) -> Result<()> {
    value.validate()?;
    if !value.native_dependencies()?.is_empty()
        || !value.code_evidence()?.is_empty()
        || matches!(
            value,
            AgentObjectV1::Association(_)
                | AgentObjectV1::CheckpointVersion(_)
                | AgentObjectV1::PortableAssociation(_)
                | AgentObjectV1::PortableCheckpointVersion(_)
        )
    {
        return Err("agent code provenance requires a qualified conversion carrier".into());
    }
    Ok(())
}
fn translate(
    value: &mut AgentObjectV1,
    map: &BTreeMap<AgentObjectRef, AgentObjectRef>,
) -> Result<()> {
    let remap = |r: &mut AgentObjectRef| -> Result<()> {
        *r = mapped(map, *r)?;
        Ok(())
    };
    match value {
        AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf { entries, .. }) => {
            for r in entries.values_mut() {
                remap(r)?
            }
        }
        AgentObjectV1::IndexPage(AgentIndexPageV1::Branch { children, .. }) => {
            for r in children.values_mut() {
                remap(r)?
            }
        }
        AgentObjectV1::SessionSnapshot(v) => {
            for r in v
                .evidence
                .iter_mut()
                .chain(v.event_batches.iter_mut())
                .chain(v.transcripts.iter_mut())
                .chain(v.tools.values_mut())
            {
                remap(r)?
            }
        }
        AgentObjectV1::ToolSnapshot(v) => {
            for r in v
                .input
                .iter_mut()
                .chain(v.output.iter_mut())
                .chain(v.evidence.iter_mut())
            {
                remap(r)?
            }
        }
        AgentObjectV1::EventBatch(v) => {
            remap(&mut v.evidence)?;
            for e in &mut v.events {
                if let AgentEventV1::MessageCompleted { content, .. } = e {
                    remap(content)?
                }
            }
        }
        AgentObjectV1::TranscriptManifest(v) => {
            for stream in &mut v.streams {
                remap(&mut stream.ordered_chunk_root)?
            }
        }
        AgentObjectV1::TranscriptTailPage(v) => {
            for r in v.chunks.iter_mut().chain(v.prefix.iter_mut()) {
                remap(r)?
            }
        }
        AgentObjectV1::RelationPage(v) => {
            for r in v.selections.iter_mut().chain(v.next.iter_mut()) {
                remap(r)?
            }
        }
        AgentObjectV1::ProjectionCatalog(v) => {
            for r in v
                .sessions
                .iter_mut()
                .chain(v.event_batches.iter_mut())
                .chain(v.source_states.iter_mut())
            {
                remap(r)?
            }
        }
        AgentObjectV1::CatalogConflict(v) => {
            for r in &mut v.alternatives {
                remap(r)?
            }
            v.alternatives.sort();
        }
        AgentObjectV1::ObservationBatch(_)
        | AgentObjectV1::TranscriptChunk(_)
        | AgentObjectV1::Content(_)
        | AgentObjectV1::Alias(_)
        | AgentObjectV1::SourceState(_) => {}
        AgentObjectV1::Association(_)
        | AgentObjectV1::CheckpointVersion(_)
        | AgentObjectV1::PortableAssociation(_)
        | AgentObjectV1::PortableCheckpointVersion(_) => {
            return Err("unsupported agent conversion representation".into());
        }
    }
    value.validate()?;
    Ok(())
}
pub async fn transfer_agent(
    source: &AgentGraphV1,
    destination: &dyn AgentObjectIo,
) -> Result<(AgentRootV1, BTreeMap<AgentObjectRef, AgentObjectRef>)> {
    let mut pending: BTreeMap<_, _> = source.objects.iter().cloned().collect();
    if pending.len() != source.objects.len() || pending.len() > 4096 {
        return Err("agent conversion inventory".into());
    }
    let mut map = BTreeMap::new();
    while !pending.is_empty() {
        let mut ready = Vec::new();
        for (reference, value) in &pending {
            admit(value)?;
            if value.kind() != reference.kind {
                return Err("agent conversion kind".into());
            }
            if value.dependencies()?.iter().all(|r| map.contains_key(r)) {
                ready.push(*reference)
            }
        }
        if ready.is_empty() {
            return Err("incomplete or cyclic agent conversion graph".into());
        }
        for reference in ready {
            let mut value = pending.remove(&reference).unwrap();
            translate(&mut value, &map)?;
            let next = destination.stage_agent_object(&value).await?;
            map.insert(reference, next);
        }
    }
    let mut root = source.root.clone();
    for r in root
        .sessions
        .iter_mut()
        .chain(root.aliases.iter_mut())
        .chain(root.checkpoints.iter_mut())
        .chain(root.associations.iter_mut())
        .chain(root.relations.iter_mut())
        .chain(root.sources.iter_mut())
        .chain(root.projections.iter_mut())
    {
        *r = mapped(&map, *r)?;
    }
    root.validate()?;
    Ok((root, map))
}
