//! Causal ReviewStore graph and comment-version reduction.
use super::*;

const MAX_REVIEW_FACTS: usize = 4096;

#[derive(Clone)]
pub(super) struct ReplicatedThreadState {
    pub thread_id: ReviewThreadId,
    pub original_anchor: ReviewObjectRef,
    pub comments: Vec<CommentSelectionV1>,
    pub graph: ReviewEventGraphV1,
}

impl Coordinator {
    pub(super) async fn load_replicated_thread(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<ReplicatedThreadState> {
        match self.read_review_object(reference).await? {
            ReviewObjectV1::ThreadSnapshot(snapshot) => Ok(ReplicatedThreadState {
                thread_id: snapshot.thread_id,
                original_anchor: snapshot.original_anchor,
                comments: snapshot.comments,
                graph: self
                    .load_review_event_graph(snapshot.event_page_root)
                    .await?,
            }),
            ReviewObjectV1::ThreadMerge(snapshot) => {
                let graph = self
                    .load_review_event_graph(snapshot.event_graph_root)
                    .await?;
                if graph.thread_id != snapshot.thread_id {
                    return Err(Error::Corrupt("merged review graph thread"));
                }
                Ok(ReplicatedThreadState {
                    thread_id: snapshot.thread_id,
                    original_anchor: snapshot.original_anchor,
                    comments: snapshot.comments,
                    graph,
                })
            }
            _ => Err(Error::Corrupt("review thread selection kind")),
        }
    }

    pub(super) async fn load_review_event_graph(
        &self,
        root: ReviewObjectRef,
    ) -> Result<ReviewEventGraphV1> {
        match self.read_review_object(root).await? {
            ReviewObjectV1::EventGraph(graph) => Ok(graph),
            ReviewObjectV1::EventPage(first) => {
                let thread_id = first.thread_id;
                let mut entries = first.entries;
                let mut next = first.next;
                let mut pages = BTreeSet::from([root]);
                while let Some(reference) = next {
                    if !pages.insert(reference) || pages.len() > MAX_REVIEW_FACTS {
                        return Err(Error::Corrupt("review event page cycle"));
                    }
                    let ReviewObjectV1::EventPage(page) =
                        self.read_review_object(reference).await?
                    else {
                        return Err(Error::Corrupt("review event page kind"));
                    };
                    if page.thread_id != thread_id {
                        return Err(Error::Corrupt("review event page thread"));
                    }
                    entries.extend(page.entries);
                    next = page.next;
                }
                if entries.is_empty() || entries.len() > MAX_REVIEW_FACTS {
                    return Err(Error::Limit("review event graph"));
                }
                entries.sort_by_key(|entry| entry.sequence);
                for (index, entry) in entries.iter().enumerate() {
                    if entry.sequence != index as u64 + 1 {
                        return Err(Error::Corrupt("review event history sequence"));
                    }
                }
                let events = entries
                    .into_iter()
                    .map(|entry| entry.event)
                    .collect::<Vec<_>>();
                let mut nodes = events
                    .iter()
                    .enumerate()
                    .map(|(index, event)| ReviewEventNodeV1 {
                        event: *event,
                        parents: index
                            .checked_sub(1)
                            .map(|parent| vec![events[parent]])
                            .unwrap_or_default(),
                    })
                    .collect::<Vec<_>>();
                nodes.sort_by_key(|node| node.event);
                Ok(ReviewEventGraphV1 {
                    schema_version: 1,
                    thread_id,
                    nodes,
                    heads: vec![
                        *events
                            .last()
                            .ok_or(Error::Corrupt("empty review event history"))?,
                    ],
                })
            }
            _ => Err(Error::Corrupt("review event graph kind")),
        }
    }

    pub(super) async fn merge_review_event_graphs(
        &self,
        current: ReviewEventGraphV1,
        incoming: ReviewEventGraphV1,
    ) -> Result<ReviewEventGraphV1> {
        if current.thread_id != incoming.thread_id {
            return Err(Error::Corrupt("review event graph identity"));
        }
        let mut nodes = current
            .nodes
            .into_iter()
            .map(|node| (node.event, node.parents))
            .collect::<BTreeMap<_, _>>();
        for node in incoming.nodes {
            match nodes.insert(node.event, node.parents.clone()) {
                Some(previous) if previous != node.parents => {
                    return Err(Error::Corrupt("review event causal identity"));
                }
                _ => {}
            }
        }
        if nodes.len() > MAX_REVIEW_FACTS {
            return Err(Error::Limit("review event graph"));
        }
        let parented = nodes.values().flatten().copied().collect::<BTreeSet<_>>();
        let heads = nodes
            .keys()
            .filter(|event| !parented.contains(event))
            .copied()
            .collect::<Vec<_>>();
        let graph = ReviewEventGraphV1 {
            schema_version: 1,
            thread_id: current.thread_id,
            nodes: nodes
                .into_iter()
                .map(|(event, parents)| ReviewEventNodeV1 { event, parents })
                .collect(),
            heads,
        };
        graph
            .validate()
            .map_err(|_| Error::Corrupt("review event graph"))?;
        self.audit_review_event_graph(&graph).await?;
        Ok(graph)
    }

    pub(super) async fn stage_review_event_graph(
        &self,
        graph: ReviewEventGraphV1,
    ) -> Result<ReviewObjectRef> {
        self.audit_review_event_graph(&graph).await?;
        self.stage_review_object(&ReviewObjectV1::EventGraph(graph))
            .await
    }

    pub(super) async fn merge_review_comments(
        &self,
        thread_id: ReviewThreadId,
        current: Vec<CommentSelectionV1>,
        incoming: Vec<CommentSelectionV1>,
    ) -> Result<Vec<CommentSelectionV1>> {
        let mut comments = BTreeMap::new();
        for selection in current.into_iter().chain(incoming) {
            match comments.entry(selection.comment_id) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((selection.parent_comment_id, selection.revision));
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    if entry.get().0 != selection.parent_comment_id {
                        return Err(Error::Corrupt("review comment parent conflict"));
                    }
                    let merged = self
                        .merge_review_comment_versions(
                            thread_id,
                            selection.comment_id,
                            entry.get().1,
                            selection.revision,
                        )
                        .await?;
                    entry.get_mut().1 = merged;
                }
            }
        }
        order_review_comments(comments)
    }

    pub(super) async fn merge_review_comment_versions(
        &self,
        thread_id: ReviewThreadId,
        comment_id: CommentId,
        current: ReviewObjectRef,
        incoming: ReviewObjectRef,
    ) -> Result<ReviewObjectRef> {
        if current == incoming {
            return Ok(current);
        }
        let mut alternatives = self
            .review_comment_alternatives(thread_id, comment_id, current)
            .await?;
        alternatives.extend(
            self.review_comment_alternatives(thread_id, comment_id, incoming)
                .await?,
        );
        let candidates = alternatives.iter().copied().collect::<Vec<_>>();
        for candidate in &candidates {
            for descendant in &candidates {
                if candidate != descendant
                    && self
                        .review_revision_is_ancestor(*candidate, *descendant)
                        .await?
                {
                    alternatives.remove(candidate);
                    break;
                }
            }
        }
        match alternatives.len() {
            0 => Err(Error::Corrupt("empty review comment conflict")),
            1 => Ok(*alternatives
                .first()
                .ok_or(Error::Corrupt("empty review comment conflict"))?),
            _ => {
                self.stage_review_object(&ReviewObjectV1::CommentConflict(Box::new(
                    ReviewCommentConflictV1 {
                        schema_version: 1,
                        thread_id,
                        comment_id,
                        alternatives: alternatives.into_iter().collect(),
                    },
                )))
                .await
            }
        }
    }

    async fn review_comment_alternatives(
        &self,
        thread_id: ReviewThreadId,
        comment_id: CommentId,
        reference: ReviewObjectRef,
    ) -> Result<BTreeSet<ReviewObjectRef>> {
        match self.read_review_object(reference).await? {
            ReviewObjectV1::CommentRevision(revision) => {
                if revision.thread_id != thread_id || revision.comment_id != comment_id {
                    return Err(Error::Corrupt("review revision identity"));
                }
                Ok(BTreeSet::from([reference]))
            }
            ReviewObjectV1::CommentConflict(conflict) => {
                if conflict.thread_id != thread_id || conflict.comment_id != comment_id {
                    return Err(Error::Corrupt("review comment conflict identity"));
                }
                for alternative in &conflict.alternatives {
                    if matches!(
                        self.read_review_object(*alternative).await?,
                        ReviewObjectV1::CommentConflict(_)
                    ) {
                        return Err(Error::Corrupt("nested review comment conflict"));
                    }
                }
                Ok(conflict.alternatives.into_iter().collect())
            }
            _ => Err(Error::Corrupt("review comment selection kind")),
        }
    }

    async fn review_revision_is_ancestor(
        &self,
        ancestor: ReviewObjectRef,
        descendant: ReviewObjectRef,
    ) -> Result<bool> {
        let mut pending = vec![descendant];
        let mut seen = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) || seen.len() > MAX_REVIEW_FACTS {
                continue;
            }
            if reference == ancestor {
                return Ok(true);
            }
            match self.read_review_object(reference).await? {
                ReviewObjectV1::CommentRevision(revision) => {
                    pending.extend(revision.previous_revision);
                    pending.extend(revision.superseded_revisions);
                }
                ReviewObjectV1::CommentConflict(conflict) => {
                    pending.extend(conflict.alternatives);
                }
                _ => return Err(Error::Corrupt("review revision ancestry kind")),
            }
        }
        Ok(false)
    }

    pub(super) async fn audit_review_event_graph(&self, graph: &ReviewEventGraphV1) -> Result<()> {
        graph
            .validate()
            .map_err(|_| Error::Corrupt("review event graph"))?;
        let parents = graph
            .nodes
            .iter()
            .map(|node| {
                (
                    node.event,
                    node.parents.iter().copied().collect::<BTreeSet<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut resolved = BTreeSet::new();
        loop {
            let before = resolved.len();
            for (event, dependencies) in &parents {
                if dependencies.is_subset(&resolved) {
                    resolved.insert(*event);
                }
            }
            if resolved.len() == parents.len() {
                break;
            }
            if resolved.len() == before {
                return Err(Error::Corrupt("review event graph cycle"));
            }
        }
        for node in &graph.nodes {
            let ReviewObjectV1::ThreadEvent(event) = self.read_review_object(node.event).await?
            else {
                return Err(Error::Corrupt("review event graph selection"));
            };
            if event.thread_id != graph.thread_id {
                return Err(Error::Corrupt("review event graph thread"));
            }
        }
        Ok(())
    }
}

fn order_review_comments(
    mut comments: BTreeMap<CommentId, (Option<CommentId>, ReviewObjectRef)>,
) -> Result<Vec<CommentSelectionV1>> {
    let mut ordered = Vec::with_capacity(comments.len());
    let mut selected = BTreeSet::new();
    while !comments.is_empty() {
        let ready = comments
            .iter()
            .filter(|(_, (parent, _))| parent.is_none_or(|value| selected.contains(&value)))
            .map(|(comment_id, _)| *comment_id)
            .collect::<Vec<_>>();
        if ready.is_empty() {
            return Err(Error::Corrupt("review comment parent cycle"));
        }
        for comment_id in ready {
            let (parent_comment_id, revision) = comments
                .remove(&comment_id)
                .ok_or(Error::Corrupt("review comment ordering"))?;
            selected.insert(comment_id);
            ordered.push(CommentSelectionV1 {
                comment_id,
                revision,
                parent_comment_id,
            });
        }
    }
    Ok(ordered)
}

pub(super) fn event_parent_map(
    graph: &ReviewEventGraphV1,
) -> BTreeMap<ReviewObjectRef, BTreeSet<ReviewObjectRef>> {
    graph
        .nodes
        .iter()
        .map(|node| (node.event, node.parents.iter().copied().collect()))
        .collect()
}

pub(super) fn graph_is_ancestor(
    parents: &BTreeMap<ReviewObjectRef, BTreeSet<ReviewObjectRef>>,
    ancestor: ReviewObjectRef,
    descendant: ReviewObjectRef,
) -> bool {
    let mut pending = vec![descendant];
    let mut seen = BTreeSet::new();
    while let Some(event) = pending.pop() {
        if event == ancestor {
            return true;
        }
        if seen.insert(event) {
            pending.extend(parents.get(&event).into_iter().flatten().copied());
        }
    }
    false
}
