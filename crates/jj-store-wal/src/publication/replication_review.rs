//! Deterministic data-only ReviewStore import reduction.
use super::replication_review_graph::{event_parent_map, graph_is_ancestor};
use super::*;

impl Coordinator {
    pub(super) async fn reduce_imported_review_facts(
        &self,
        current_root: RecordRef,
        authored_root: RecordRef,
        delta: &ReviewDeltaV1,
    ) -> Result<RecordRef> {
        let authored = self.review_store(authored_root).await?;
        for edit in &delta.edits {
            if authored.lookup(edit.catalog, edit.key).await? != edit.replacement {
                return Err(Error::Corrupt("origin review facts selection"));
            }
            if let Some(replacement) = edit.replacement {
                authored.read_object(replacement).await?;
            }
        }
        let current = self.review_store(current_root).await?;
        let mut edits = Vec::new();
        for edit in delta.edits.iter().filter(|edit| {
            matches!(
                edit.catalog,
                ReviewCatalogKind::Threads
                    | ReviewCatalogKind::Comments
                    | ReviewCatalogKind::ThreadEvents
            )
        }) {
            let selected = current.lookup(edit.catalog, edit.key).await?;
            if selected == edit.replacement {
                continue;
            }
            let replacement = if selected == edit.expected {
                edit.replacement
            } else {
                let (Some(selected), Some(incoming)) = (selected, edit.replacement) else {
                    return Err(Error::Corrupt("concurrent review deletion conflict"));
                };
                Some(match edit.catalog {
                    ReviewCatalogKind::Threads => {
                        self.merge_review_thread_versions(selected, incoming)
                            .await?
                    }
                    ReviewCatalogKind::Comments => {
                        let (thread_id, comment_id) =
                            self.review_comment_identity(selected, incoming).await?;
                        self.merge_review_comment_versions(
                            thread_id, comment_id, selected, incoming,
                        )
                        .await?
                    }
                    ReviewCatalogKind::ThreadEvents => {
                        self.merge_review_event_roots(selected, incoming).await?
                    }
                    _ => unreachable!(),
                })
            };
            edits.push(ReviewCatalogEditV1 {
                catalog: edit.catalog,
                key: edit.key,
                expected: selected,
                replacement,
            });
        }
        if edits.is_empty() {
            return Ok(current_root);
        }
        self.rebuild_imported_review_indexes(&current, edits).await
    }

    async fn merge_review_thread_versions(
        &self,
        current_ref: ReviewObjectRef,
        incoming_ref: ReviewObjectRef,
    ) -> Result<ReviewObjectRef> {
        let current = self.load_replicated_thread(current_ref).await?;
        let incoming = self.load_replicated_thread(incoming_ref).await?;
        if current.thread_id != incoming.thread_id
            || current.original_anchor != incoming.original_anchor
        {
            return Err(Error::Corrupt("concurrent review thread identity"));
        }
        if graph_contains(&current.graph, &incoming.graph) {
            return Ok(current_ref);
        }
        if graph_contains(&incoming.graph, &current.graph) {
            return Ok(incoming_ref);
        }
        let graph = self
            .merge_review_event_graphs(current.graph, incoming.graph)
            .await?;
        let comments = self
            .merge_review_comments(current.thread_id, current.comments, incoming.comments)
            .await?;
        let graph_root = self.stage_review_event_graph(graph.clone()).await?;
        let (status, selected_workflow_actions, content_version, workflow_version) =
            self.reduce_review_workflow(&graph).await?;
        self.stage_review_object(&ReviewObjectV1::ThreadMerge(Box::new(
            ReviewThreadMergeV1 {
                schema_version: 1,
                thread_id: current.thread_id,
                original_anchor: current.original_anchor,
                comments,
                event_graph_root: graph_root,
                status,
                selected_workflow_actions,
                thread_version: graph.nodes.len() as u64,
                content_version,
                workflow_version,
            },
        )))
        .await
    }

    async fn merge_review_event_roots(
        &self,
        current_ref: ReviewObjectRef,
        incoming_ref: ReviewObjectRef,
    ) -> Result<ReviewObjectRef> {
        let current = self.load_review_event_graph(current_ref).await?;
        let incoming = self.load_review_event_graph(incoming_ref).await?;
        if graph_contains(&current, &incoming) {
            return Ok(current_ref);
        }
        if graph_contains(&incoming, &current) {
            return Ok(incoming_ref);
        }
        let graph = self.merge_review_event_graphs(current, incoming).await?;
        self.stage_review_event_graph(graph).await
    }

    async fn review_comment_identity(
        &self,
        current: ReviewObjectRef,
        incoming: ReviewObjectRef,
    ) -> Result<(ReviewThreadId, CommentId)> {
        let current = self.review_comment_object_identity(current).await?;
        let incoming = self.review_comment_object_identity(incoming).await?;
        if current == incoming {
            Ok(current)
        } else {
            Err(Error::Corrupt("concurrent review comment identity"))
        }
    }

    async fn review_comment_object_identity(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<(ReviewThreadId, CommentId)> {
        match self.read_review_object(reference).await? {
            ReviewObjectV1::CommentRevision(revision) => {
                Ok((revision.thread_id, revision.comment_id))
            }
            ReviewObjectV1::CommentConflict(conflict) => {
                Ok((conflict.thread_id, conflict.comment_id))
            }
            _ => Err(Error::Corrupt("review comment selection kind")),
        }
    }

    pub(super) async fn reduce_review_workflow(
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

    async fn rebuild_imported_review_indexes(
        &self,
        store: &ReviewStoreSnapshot,
        mut edits: Vec<ReviewCatalogEditV1>,
    ) -> Result<RecordRef> {
        let mut threads = self
            .all_review_catalog_selections(store, ReviewCatalogKind::Threads)
            .await?;
        for edit in edits
            .iter()
            .filter(|edit| edit.catalog == ReviewCatalogKind::Threads)
        {
            match edit.replacement {
                Some(replacement) => {
                    threads.insert(edit.key, replacement);
                }
                None => {
                    threads.remove(&edit.key);
                }
            }
        }
        let mut anchors = BTreeMap::<_, BTreeMap<_, _>>::new();
        let mut artifacts = BTreeMap::<_, BTreeMap<_, _>>::new();
        let mut statuses = BTreeMap::<_, BTreeMap<_, _>>::new();
        for (_, reference) in threads {
            let (thread_id, anchor_ref, status) =
                self.review_thread_index_fields(reference).await?;
            let ReviewObjectV1::Anchor(anchor) = self.read_review_object(anchor_ref).await? else {
                return Err(Error::Corrupt("review relation anchor kind"));
            };
            anchors
                .entry(anchor.anchor_id().map_err(super::review::review_error)?)
                .or_default()
                .insert(thread_id, reference);
            artifacts
                .entry(anchor.artifact_id().map_err(super::review::review_error)?)
                .or_default()
                .insert(thread_id, reference);
            statuses
                .entry(status)
                .or_default()
                .insert(thread_id, reference);
        }
        let mut desired = BTreeMap::new();
        for (anchor_id, entries) in anchors {
            let subject = ReviewRelationSubjectV1::OriginalAnchor { anchor_id };
            desired.insert(
                (
                    ReviewCatalogKind::AnchorRelations,
                    subject.catalog_key().map_err(super::review::review_error)?,
                ),
                self.stage_replication_relation_pages(subject, entries)
                    .await?,
            );
        }
        for (artifact_id, entries) in artifacts {
            let subject = ReviewRelationSubjectV1::Artifact { artifact_id };
            desired.insert(
                (
                    ReviewCatalogKind::ArtifactRelations,
                    subject.catalog_key().map_err(super::review::review_error)?,
                ),
                self.stage_replication_relation_pages(subject, entries)
                    .await?,
            );
        }
        for (status, entries) in statuses {
            let key = jj_review::review_status_key(status).map_err(super::review::review_error)?;
            desired.insert(
                (ReviewCatalogKind::Statuses, key),
                self.stage_replication_status_pages(status, entries).await?,
            );
        }
        for catalog in [
            ReviewCatalogKind::AnchorRelations,
            ReviewCatalogKind::ArtifactRelations,
            ReviewCatalogKind::Statuses,
        ] {
            let existing = self.all_review_catalog_selections(store, catalog).await?;
            let keys = existing
                .keys()
                .copied()
                .chain(
                    desired
                        .keys()
                        .filter(|(kind, _)| *kind == catalog)
                        .map(|(_, key)| *key),
                )
                .collect::<BTreeSet<_>>();
            for key in keys {
                let expected = existing.get(&key).copied();
                let replacement = desired.get(&(catalog, key)).copied();
                if expected != replacement {
                    edits.push(ReviewCatalogEditV1 {
                        catalog,
                        key,
                        expected,
                        replacement,
                    });
                }
            }
        }
        self.stage_review_delta(
            store,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits,
            },
        )
        .await
    }

    async fn review_thread_index_fields(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<(ReviewThreadId, ReviewObjectRef, ThreadStatusV1)> {
        match self.read_review_object(reference).await? {
            ReviewObjectV1::ThreadSnapshot(snapshot) => Ok((
                snapshot.thread_id,
                snapshot.original_anchor,
                snapshot.status,
            )),
            ReviewObjectV1::ThreadMerge(snapshot) => Ok((
                snapshot.thread_id,
                snapshot.original_anchor,
                snapshot.status,
            )),
            _ => Err(Error::Corrupt("review thread index selection")),
        }
    }

    async fn all_review_catalog_selections(
        &self,
        store: &ReviewStoreSnapshot,
        catalog: ReviewCatalogKind,
    ) -> Result<BTreeMap<ReviewCatalogKey, ReviewObjectRef>> {
        let mut selected = BTreeMap::new();
        let mut after = None;
        loop {
            let page = store.list_catalog(catalog, after, 256).await?;
            for (key, value) in page.selections {
                selected.insert(key, value);
            }
            let Some(next) = page.next_after else {
                return Ok(selected);
            };
            after = Some(next);
        }
    }

    async fn stage_replication_relation_pages(
        &self,
        subject: ReviewRelationSubjectV1,
        entries: BTreeMap<ReviewThreadId, ReviewObjectRef>,
    ) -> Result<ReviewObjectRef> {
        let entries = relation_entries(entries);
        let mut next = None;
        for chunk in entries.rchunks(256) {
            next = Some(
                self.stage_review_object(&ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
                    schema_version: 1,
                    subject: subject.clone(),
                    entries: chunk.to_vec(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or(Error::Corrupt("empty review relation merge"))
    }

    async fn stage_replication_status_pages(
        &self,
        status: ThreadStatusV1,
        entries: BTreeMap<ReviewThreadId, ReviewObjectRef>,
    ) -> Result<ReviewObjectRef> {
        let entries = relation_entries(entries);
        let mut next = None;
        for chunk in entries.rchunks(256) {
            next = Some(
                self.stage_review_object(&ReviewObjectV1::StatusPage(ReviewStatusPageV1 {
                    schema_version: 1,
                    status,
                    entries: chunk.to_vec(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or(Error::Corrupt("empty review status merge"))
    }
}

fn graph_contains(container: &ReviewEventGraphV1, candidate: &ReviewEventGraphV1) -> bool {
    if container.thread_id != candidate.thread_id {
        return false;
    }
    let selected = container
        .nodes
        .iter()
        .map(|node| (node.event, &node.parents))
        .collect::<BTreeMap<_, _>>();
    candidate.nodes.iter().all(|node| {
        selected
            .get(&node.event)
            .is_some_and(|parents| **parents == node.parents)
    })
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

fn relation_entries(
    entries: BTreeMap<ReviewThreadId, ReviewObjectRef>,
) -> Vec<ReviewThreadRelationV1> {
    entries
        .into_iter()
        .map(|(thread_id, snapshot)| ReviewThreadRelationV1 {
            thread_id,
            snapshot,
        })
        .collect()
}
