use super::Result;
use super::*;
use jj_review::*;
pub(super) fn admit(value: &ReviewObjectV1) -> Result<()> {
    value.validate()?;
    if !value.native_dependencies()?.is_empty()
        || !value.agent_dependencies()?.is_empty()
        || matches!(value,ReviewObjectV1::Anchor(v) if v.target!=ArtifactTargetV1::Repository)
    {
        return Err("code or agent review evidence requires a qualified conversion carrier".into());
    }
    Ok(())
}
fn translate(
    value: &mut ReviewObjectV1,
    map: &BTreeMap<ReviewObjectRef, ReviewObjectRef>,
    anchors: &BTreeMap<AnchorId, AnchorId>,
    repository: m::RepositoryId,
) -> Result<()> {
    let remap = |r: &mut ReviewObjectRef| -> Result<()> {
        *r = mapped(map, *r)?;
        Ok(())
    };
    match value {
        ReviewObjectV1::Anchor(v) => v.repository_id = repository,
        ReviewObjectV1::CommentBody(_) => {}
        ReviewObjectV1::IndexPage(_) => return Err("conversion rebuilds review catalogs".into()),
        ReviewObjectV1::CommentRevision(v) => {
            for r in v
                .previous_revision
                .iter_mut()
                .chain(v.superseded_revisions.iter_mut())
            {
                remap(r)?
            }
            v.superseded_revisions.sort();
            if let CommentRevisionContentV1::Visible { body } = &mut v.content {
                remap(body)?
            }
        }
        ReviewObjectV1::CommentConflict(v) => {
            for r in &mut v.alternatives {
                remap(r)?
            }
            v.alternatives.sort();
        }
        ReviewObjectV1::ThreadEvent(v) => match &mut v.event {
            ReviewThreadEventKindV1::Created {
                anchor,
                initial_revision,
                ..
            } => {
                remap(anchor)?;
                remap(initial_revision)?
            }
            ReviewThreadEventKindV1::ReplyAdded { revision, .. } => remap(revision)?,
            ReviewThreadEventKindV1::CommentRevised {
                previous_revision,
                revision,
                ..
            } => {
                remap(previous_revision)?;
                remap(revision)?
            }
            ReviewThreadEventKindV1::CommentConflictResolved {
                superseded_revisions,
                revision,
                ..
            } => {
                for r in superseded_revisions.iter_mut() {
                    remap(r)?
                }
                superseded_revisions.sort();
                remap(revision)?
            }
            ReviewThreadEventKindV1::Resolved { .. } | ReviewThreadEventKindV1::Reopened { .. } => {
            }
        },
        ReviewObjectV1::EventPage(v) => {
            for e in &mut v.entries {
                remap(&mut e.event)?
            }
            if let Some(r) = &mut v.next {
                remap(r)?
            }
        }
        ReviewObjectV1::EventGraph(v) => {
            for n in &mut v.nodes {
                remap(&mut n.event)?;
                for r in &mut n.parents {
                    remap(r)?
                }
                n.parents.sort();
            }
            v.nodes.sort_by_key(|n| n.event);
            for r in &mut v.heads {
                remap(r)?
            }
            v.heads.sort();
        }
        ReviewObjectV1::ThreadSnapshot(v) => {
            remap(&mut v.original_anchor)?;
            remap(&mut v.event_page_root)?;
            for e in &mut v.comments {
                remap(&mut e.revision)?
            }
        }
        ReviewObjectV1::ThreadMerge(v) => {
            remap(&mut v.original_anchor)?;
            remap(&mut v.event_graph_root)?;
            for e in &mut v.comments {
                remap(&mut e.revision)?
            }
        }
        ReviewObjectV1::RelationPage(v) => {
            if let ReviewRelationSubjectV1::OriginalAnchor { anchor_id } = &mut v.subject {
                *anchor_id = mapped(anchors, *anchor_id)?
            }
            for e in &mut v.entries {
                remap(&mut e.snapshot)?
            }
            if let Some(r) = &mut v.next {
                remap(r)?
            }
        }
        ReviewObjectV1::StatusPage(v) => {
            for e in &mut v.entries {
                remap(&mut e.snapshot)?
            }
            if let Some(r) = &mut v.next {
                remap(r)?
            }
        }
    }
    value.validate()?;
    Ok(())
}
fn catalog(
    source: &BTreeMap<ReviewObjectRef, ReviewObjectV1>,
    root: Option<ReviewObjectRef>,
) -> Result<BTreeMap<ReviewCatalogKey, ReviewObjectRef>> {
    let mut pending: Vec<_> = root.into_iter().collect();
    let mut seen = BTreeSet::new();
    let mut entries = BTreeMap::new();
    while let Some(reference) = pending.pop() {
        if !seen.insert(reference) {
            return Err("review catalog cycle or duplicate".into());
        }
        match source.get(&reference) {
            Some(ReviewObjectV1::IndexPage(ReviewIndexPageV1::Leaf { entries: leaf, .. })) => {
                for (key, value) in leaf {
                    if entries.insert(*key, *value).is_some() {
                        return Err("review catalog duplicate".into());
                    }
                }
            }
            Some(ReviewObjectV1::IndexPage(ReviewIndexPageV1::Branch { children, .. })) => {
                pending.extend(children.values().copied())
            }
            _ => return Err("incomplete review catalog".into()),
        }
    }
    Ok(entries)
}
pub async fn transfer_review(
    source: &ReviewGraphV1,
    destination: &dyn ReviewObjectIo,
) -> Result<(ReviewRootV1, BTreeMap<ReviewObjectRef, ReviewObjectRef>)> {
    let all: BTreeMap<_, _> = source.objects.iter().cloned().collect();
    if all.len() != source.objects.len() || all.len() > 100_000 {
        return Err("review conversion inventory".into());
    }
    let mut pending = all
        .iter()
        .filter(|(_, v)| !matches!(v, ReviewObjectV1::IndexPage(_)))
        .map(|(r, v)| (*r, v.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut map = BTreeMap::new();
    let mut anchors = BTreeMap::new();
    let mut keys = BTreeMap::new();
    while !pending.is_empty() {
        let mut ready = Vec::new();
        for (reference, value) in &pending {
            admit(value)?;
            if reference.kind != value.kind() {
                return Err("review conversion kind".into());
            }
            if value
                .review_dependencies()?
                .iter()
                .all(|r| map.contains_key(r))
            {
                ready.push(*reference)
            }
        }
        if ready.is_empty() {
            return Err("incomplete or cyclic review conversion graph".into());
        }
        for reference in ready {
            let mut value = pending.remove(&reference).unwrap();
            let old = if let ReviewObjectV1::Anchor(v) = &value {
                Some(v.anchor_id()?)
            } else {
                None
            };
            translate(&mut value, &map, &anchors, destination.repository())?;
            if let (Some(old), ReviewObjectV1::Anchor(v)) = (old, &value) {
                let next = v.anchor_id()?;
                anchors.insert(old, next);
                keys.insert(
                    ReviewCatalogKey::derive("anchor-relation", old.as_bytes())?,
                    ReviewCatalogKey::derive("anchor-relation", next.as_bytes())?,
                );
            }
            map.insert(reference, destination.stage_review_object(&value).await?);
        }
    }
    let mut root = source.root.clone();
    for field in [
        &mut root.threads,
        &mut root.comments,
        &mut root.thread_events,
        &mut root.anchor_relations,
        &mut root.artifact_relations,
        &mut root.statuses,
        &mut root.work_requests,
        &mut root.aliases,
        &mut root.sources,
    ] {
        let entries = catalog(&all, *field)?
            .into_iter()
            .map(|(key, value)| Ok((keys.get(&key).copied().unwrap_or(key), mapped(&map, value)?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        let old = *field;
        *field = ReviewCatalogEditor::new(destination)
            .create_conversion_catalog(entries)
            .await?;
        if let (Some(old), Some(new)) = (old, *field) {
            map.insert(old, new);
        }
    }
    root.validate()?;
    Ok((root, map))
}
