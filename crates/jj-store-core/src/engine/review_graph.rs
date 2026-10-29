//! Shared retained event graphs and deterministic workflow reduction.
use super::replication_review_graph::{event_parent_map, graph_is_ancestor};
use super::*;
const MAX_REVIEW_FACTS: usize = 4096;
impl ReviewProjector<'_> {
    pub async fn load_review_event_graph(
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
    pub async fn stage_review_event_graph(
        &self,
        graph: ReviewEventGraphV1,
    ) -> Result<ReviewObjectRef> {
        self.audit_review_event_graph(&graph).await?;
        self.stage_review_object(&ReviewObjectV1::EventGraph(graph))
            .await
    }
    pub async fn audit_review_event_graph(&self, graph: &ReviewEventGraphV1) -> Result<()> {
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
    pub async fn reduce_review_workflow(
        &self,
        graph: &ReviewEventGraphV1,
    ) -> Result<(ThreadStatusV1, Vec<ReviewActionId>, u64, u64)> {
        let parents = event_parent_map(graph);
        let mut activity = Vec::new();
        let mut workflows = Vec::new();
        for node in &graph.nodes {
            let ReviewObjectV1::ThreadEvent(event) = self.read_review_object(node.event).await?
            else {
                return Err(Error::Corrupt("review event graph selection"));
            };
            match event.event {
                ReviewThreadEventKindV1::Created { .. }
                | ReviewThreadEventKindV1::ReplyAdded { .. }
                | ReviewThreadEventKindV1::CommentRevised { .. }
                | ReviewThreadEventKindV1::CommentConflictResolved { .. } => {
                    activity.push(node.event)
                }
                ReviewThreadEventKindV1::Resolved { .. }
                | ReviewThreadEventKindV1::Reopened { .. } => workflows.push(node.event),
            }
        }
        let activity_heads = maximal_events(&parents, &activity);
        let workflow_heads = maximal_events(&parents, &workflows);
        let mut actions = Vec::new();
        let mut resolved = !workflow_heads.is_empty();
        for reference in &workflow_heads {
            let ReviewObjectV1::ThreadEvent(event) = self.read_review_object(*reference).await?
            else {
                return Err(Error::Corrupt("review workflow selection"));
            };
            match event.event {
                ReviewThreadEventKindV1::Resolved { action_id, .. } => {
                    actions.push(action_id);
                    if activity_heads
                        .iter()
                        .any(|activity| !graph_is_ancestor(&parents, *activity, *reference))
                    {
                        resolved = false;
                    }
                }
                ReviewThreadEventKindV1::Reopened { .. } => resolved = false,
                _ => return Err(Error::Corrupt("review workflow event kind")),
            }
        }
        actions.sort();
        actions.dedup();
        if !resolved {
            actions.clear();
        }
        Ok((
            if resolved {
                ThreadStatusV1::Resolved
            } else {
                ThreadStatusV1::Open
            },
            actions,
            activity.len() as u64,
            workflows.len() as u64 + 1,
        ))
    }
}

fn maximal_events(
    parents: &BTreeMap<ReviewObjectRef, BTreeSet<ReviewObjectRef>>,
    events: &[ReviewObjectRef],
) -> Vec<ReviewObjectRef> {
    events
        .iter()
        .copied()
        .filter(|candidate| {
            !events.iter().any(|descendant| {
                candidate != descendant && graph_is_ancestor(parents, *candidate, *descendant)
            })
        })
        .collect()
}
