use jj_agent as a;
use jj_collab::*;
use jj_review as r;
use jj_store_model::PrincipalId;
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Reader {
    graph: CollabGraphV1,
    work: Option<jj_work::WorkCommentV1>,
    threads: BTreeMap<ThreadId, CollabObjectRef>,
    messages: BTreeMap<(ThreadId, HumanMessageId), CollabObjectRef>,
    sessions: BTreeMap<a::AgentSessionId, a::AgentObjectRef>,
    checkpoints: BTreeMap<a::AgentCheckpointId, a::AgentObjectRef>,
    agents: BTreeMap<a::AgentObjectRef, a::AgentObjectV1>,
    reviews: BTreeMap<r::ReviewThreadId, r::ReviewObjectRef>,
    review_objects: BTreeMap<r::ReviewObjectRef, r::ReviewObjectV1>,
}
#[async_trait::async_trait]
impl ReactionTargetRead for Reader {
    async fn work_comment(
        &self,
        _: jj_work::TaskId,
        _: jj_work::WorkCommentId,
    ) -> Result<Option<jj_work::WorkCommentV1>> {
        Ok(self.work.clone())
    }
    async fn thread_selection(&self, id: ThreadId) -> Result<Option<CollabObjectRef>> {
        Ok(self.threads.get(&id).copied())
    }
    async fn message_selection(
        &self,
        t: ThreadId,
        m: HumanMessageId,
    ) -> Result<Option<CollabObjectRef>> {
        Ok(self.messages.get(&(t, m)).copied())
    }
    async fn collab_object(&self, reference: CollabObjectRef) -> Result<Option<CollabObjectV1>> {
        Ok(self.graph.objects.get(&reference).cloned())
    }
    async fn session_selection(&self, id: a::AgentSessionId) -> Result<Option<a::AgentObjectRef>> {
        Ok(self.sessions.get(&id).copied())
    }
    async fn checkpoint_selection(
        &self,
        id: a::AgentCheckpointId,
    ) -> Result<Option<a::AgentObjectRef>> {
        Ok(self.checkpoints.get(&id).copied())
    }
    async fn agent_object(&self, reference: a::AgentObjectRef) -> Result<Option<a::AgentObjectV1>> {
        let object = self.agents.get(&reference).cloned();
        if let Some(object) = &object
            && reference.id.as_bytes()
                != &<[u8; 32]>::from(Sha256::digest(a::encode_object(object).unwrap()))
        {
            return Err(Error::Invalid("fixture agent identity"));
        }
        Ok(object)
    }
    async fn review_selection(&self, id: r::ReviewThreadId) -> Result<Option<r::ReviewObjectRef>> {
        Ok(self.reviews.get(&id).copied())
    }
    async fn review_object(
        &self,
        reference: r::ReviewObjectRef,
    ) -> Result<Option<r::ReviewObjectV1>> {
        let object = self.review_objects.get(&reference).cloned();
        if let Some(object) = &object
            && reference.id.as_bytes()
                != &<[u8; 32]>::from(Sha256::digest(r::encode_object(object).unwrap()))
        {
            return Err(Error::Invalid("fixture review identity"));
        }
        Ok(object)
    }
}
fn tid() -> ThreadId {
    ThreadId::new([1; 16])
}
fn mid() -> HumanMessageId {
    HumanMessageId::new([2; 16])
}
fn sid() -> a::AgentSessionId {
    a::AgentSessionId::new([3; 16])
}
fn rid() -> r::ReviewThreadId {
    r::ReviewThreadId::new([4; 16])
}
fn cid() -> r::CommentId {
    r::CommentId::new([5; 16])
}
fn actor() -> PrincipalId {
    PrincipalId::new([6; 16])
}
fn aref(kind: a::AgentObjectKind) -> a::AgentObjectRef {
    a::AgentObjectRef::new(kind, a::AgentObjectId::new([0; 32]))
}
fn rref(kind: r::ReviewObjectKind) -> r::ReviewObjectRef {
    r::ReviewObjectRef::new(kind, r::ReviewObjectId::new([0; 32]))
}
fn coverage() -> a::CoverageV1 {
    a::CoverageV1 {
        completeness: a::CompletenessV1::Unknown,
        gaps: 0,
        redacted_fields: 0,
        truncated: false,
    }
}
impl Reader {
    fn agent(&mut self, object: a::AgentObjectV1) -> a::AgentObjectRef {
        let reference = a::AgentObjectRef::new(
            object.kind(),
            a::AgentObjectId::new(Sha256::digest(a::encode_object(&object).unwrap()).into()),
        );
        self.agents.insert(reference, object);
        reference
    }
    fn review(&mut self, object: r::ReviewObjectV1) -> r::ReviewObjectRef {
        let reference = r::ReviewObjectRef::new(
            object.kind(),
            r::ReviewObjectId::new(Sha256::digest(r::encode_object(&object).unwrap()).into()),
        );
        self.review_objects.insert(reference, object);
        reference
    }
    fn thread(&mut self, status: ThreadStatusV1) {
        let event = self
            .graph
            .stage(CollabObjectV1::ThreadEvent(ThreadEventV1 {
                schema_version: 1,
                id: ThreadEventId::new([1; 16]),
                thread_id: tid(),
                actor: actor(),
                accepted_at_millis: 1,
                previous: None,
                event: ThreadEventKindV1::Created,
            }))
            .unwrap();
        let reference = self
            .graph
            .stage(CollabObjectV1::ThreadVersion(ThreadVersionV1 {
                schema_version: 1,
                thread_id: tid(),
                title: "Tests".into(),
                status,
                actor: actor(),
                accepted_at_millis: 1,
                previous: None,
                event,
                baseline: None,
            }))
            .unwrap();
        self.threads.insert(tid(), reference);
    }
    fn message(
        &mut self,
        previous: Option<CollabObjectRef>,
        body: Option<&str>,
    ) -> CollabObjectRef {
        self.graph
            .stage(CollabObjectV1::HumanMessageRevision(
                HumanMessageRevisionV1 {
                    reply_to: None,
                    schema_version: 1,
                    message_id: mid(),
                    thread_id: tid(),
                    actor: actor(),
                    accepted_at_millis: 1,
                    previous,
                    body: body.map(str::to_owned),
                },
            ))
            .unwrap()
    }
    fn session(
        &mut self,
        batches: Vec<a::AgentObjectRef>,
        tools: BTreeMap<a::ToolCallId, a::AgentObjectRef>,
    ) -> a::AgentObjectRef {
        let reference = self.agent(a::AgentObjectV1::SessionSnapshot(
            a::AgentSessionSnapshotV1 {
                schema_version: 1,
                session_id: sid(),
                integration: a::IntegrationId::Codex,
                state: a::SessionState::Ended,
                parent_session_id: None,
                evidence: vec![],
                event_batches: batches,
                transcripts: vec![],
                tools,
                coverage: coverage(),
            },
        ));
        self.sessions.insert(sid(), reference);
        reference
    }
    fn content(&mut self, text: &str) -> a::AgentObjectRef {
        self.agent(a::AgentObjectV1::Content(a::AgentContentV1 {
            schema_version: 1,
            payload: a::SanitizedPayloadV1::new(a::SanitizedMediaType::Text, text.as_bytes())
                .unwrap(),
        }))
    }
    fn comment(
        &mut self,
        previous: Option<r::ReviewObjectRef>,
        content: r::CommentRevisionContentV1,
    ) -> r::ReviewObjectRef {
        self.review(r::ReviewObjectV1::CommentRevision(Box::new(
            r::ReviewCommentRevisionV1 {
                schema_version: 1,
                thread_id: rid(),
                comment_id: cid(),
                author: actor(),
                actor: actor(),
                created_at_millis: 1,
                previous_revision: previous,
                superseded_revisions: vec![],
                content,
            },
        )))
    }
    fn body(&mut self, text: &str) -> r::ReviewObjectRef {
        self.review(r::ReviewObjectV1::CommentBody(r::ReviewCommentBodyV1 {
            schema_version: 1,
            format: r::ReviewBodyFormatV1::PlainText,
            text: text.into(),
            sanitizer_version: "v1".into(),
        }))
    }
    fn select_review(&mut self, revision: r::ReviewObjectRef, merged: bool) {
        let comments = vec![r::CommentSelectionV1 {
            comment_id: cid(),
            revision,
            parent_comment_id: None,
        }];
        let object = if merged {
            r::ReviewObjectV1::ThreadMerge(Box::new(r::ReviewThreadMergeV1 {
                schema_version: 1,
                thread_id: rid(),
                original_anchor: rref(r::ReviewObjectKind::Anchor),
                comments,
                event_graph_root: rref(r::ReviewObjectKind::EventPage),
                status: r::ThreadStatusV1::Open,
                selected_workflow_actions: vec![],
                thread_version: 1,
                content_version: 1,
                workflow_version: 1,
            }))
        } else {
            r::ReviewObjectV1::ThreadSnapshot(Box::new(r::ReviewThreadSnapshotV1 {
                schema_version: 1,
                thread_id: rid(),
                original_anchor: rref(r::ReviewObjectKind::Anchor),
                comments,
                event_page_root: rref(r::ReviewObjectKind::EventPage),
                last_event_sequence: 1,
                status: r::ThreadStatusV1::Open,
                selected_workflow_action: None,
                thread_version: 1,
                content_version: 1,
                workflow_version: 1,
            }))
        };
        let reference = self.review(object);
        self.reviews.insert(rid(), reference);
    }
}
fn human_target(revision: CollabObjectRef) -> ReactionTargetV1 {
    ReactionTargetV1::HumanMessage {
        thread_id: tid(),
        message_id: mid(),
        revision,
    }
}
fn review_target(revision: r::ReviewObjectRef) -> ReactionTargetV1 {
    ReactionTargetV1::ReviewComment {
        thread_id: rid(),
        comment_id: cid(),
        revision,
    }
}

#[tokio::test]
async fn human_targets_require_selected_history_and_current_visibility() {
    let mut reader = Reader::default();
    reader.thread(ThreadStatusV1::Active);
    let old = reader.message(None, Some("before"));
    let orphan = reader.message(None, Some("unrelated retained bytes"));
    let current = reader.message(Some(old), Some("after"));
    reader.messages.insert((tid(), mid()), current);
    assert_eq!(
        resolve_reaction_target(&reader, &human_target(current))
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Current
    );
    assert_eq!(
        resolve_reaction_target(&reader, &human_target(old))
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Earlier
    );
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(orphan)).await,
        Err(Error::TargetUnavailable)
    ));
    let deleted = reader.message(Some(current), None);
    reader.messages.insert((tid(), mid()), deleted);
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(old)).await,
        Err(Error::TargetUnavailable)
    ));
    reader.messages.insert((tid(), mid()), current);
    reader.thread(ThreadStatusV1::Tombstoned);
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(old)).await,
        Err(Error::TargetUnavailable)
    ));
    reader.thread(ThreadStatusV1::Archived);
    assert!(
        resolve_reaction_target(&reader, &human_target(old))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn agent_message_checks_selected_batch_and_exact_event_content_witnesses() {
    let mut reader = Reader::default();
    let content = reader.content("message");
    let other = reader.content("other");
    let event_id = a::AgentEventId::new([7; 16]);
    let message_id = a::MessageId::new([8; 16]);
    let batch = reader.agent(a::AgentObjectV1::EventBatch(a::AgentEventBatchV1 {
        schema_version: 1,
        normalizer_version: "v1".into(),
        evidence: aref(a::AgentObjectKind::ObservationBatch),
        events: vec![a::AgentEventV1::MessageCompleted {
            event_id,
            session_id: sid(),
            turn_id: None,
            message_id,
            role: a::MessageRole::Assistant,
            content,
        }],
    }));
    reader.session(vec![batch], BTreeMap::new());
    let target = ReactionTargetV1::AgentMessage {
        session_id: sid(),
        message_id,
        event_id,
        batch,
        content,
    };
    assert_eq!(
        resolve_reaction_target(&reader, &target)
            .await
            .unwrap()
            .exact_key,
        target.exact_key().unwrap()
    );
    let mut wrong = target.clone();
    if let ReactionTargetV1::AgentMessage { content, .. } = &mut wrong {
        *content = other;
    }
    assert!(matches!(
        resolve_reaction_target(&reader, &wrong).await,
        Err(Error::TargetUnavailable)
    ));
    let mut wrong = target.clone();
    if let ReactionTargetV1::AgentMessage { event_id, .. } = &mut wrong {
        *event_id = a::AgentEventId::new([9; 16]);
    }
    assert!(matches!(
        resolve_reaction_target(&reader, &wrong).await,
        Err(Error::TargetUnavailable)
    ));
    reader.session(vec![], BTreeMap::new());
    assert!(matches!(
        resolve_reaction_target(&reader, &target).await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn tool_output_is_exact_and_old_snapshot_requires_selected_tool_membership() {
    let mut reader = Reader::default();
    let old_content = reader.content("old");
    let current_content = reader.content("new");
    let tool_id = a::ToolCallId::new([9; 16]);
    let tool = |output| {
        a::AgentObjectV1::ToolSnapshot(a::ToolInvocationSnapshotV1 {
            schema_version: 1,
            tool_call_id: tool_id,
            session_id: sid(),
            turn_id: None,
            native_name: "test".into(),
            kind: a::ToolKind::Execute,
            state: a::ToolState::Succeeded,
            attempts: BTreeSet::new(),
            input: None,
            output: Some(output),
            evidence: vec![],
        })
    };
    let old = reader.agent(tool(old_content));
    let current = reader.agent(tool(current_content));
    reader.session(vec![], BTreeMap::from([(tool_id, current)]));
    let target = ReactionTargetV1::ToolOutput {
        session_id: sid(),
        tool_id,
        snapshot: old,
        content: old_content,
    };
    assert_eq!(
        resolve_reaction_target(&reader, &target)
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Earlier
    );
    let wrong = ReactionTargetV1::ToolOutput {
        session_id: sid(),
        tool_id,
        snapshot: old,
        content: current_content,
    };
    assert!(matches!(
        resolve_reaction_target(&reader, &wrong).await,
        Err(Error::TargetUnavailable)
    ));
    let invocation = ReactionTargetV1::ToolInvocation {
        session_id: sid(),
        tool_id,
        snapshot: current,
    };
    assert_eq!(
        resolve_reaction_target(&reader, &invocation)
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Current
    );
    reader.session(vec![], BTreeMap::new());
    assert!(matches!(
        resolve_reaction_target(&reader, &target).await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn review_current_hidden_tombstone_and_conflict_suppress_prior_reactions() {
    let mut reader = Reader::default();
    let body = reader.body("visible");
    let old = reader.comment(None, r::CommentRevisionContentV1::Visible { body });
    let new_body = reader.body("edited");
    let current = reader.comment(
        Some(old),
        r::CommentRevisionContentV1::Visible { body: new_body },
    );
    for merged in [false, true] {
        reader.select_review(current, merged);
        assert_eq!(
            resolve_reaction_target(&reader, &review_target(old))
                .await
                .unwrap()
                .revision,
            TargetRevisionV1::Earlier
        );
        assert_eq!(
            resolve_reaction_target(&reader, &review_target(current))
                .await
                .unwrap()
                .revision,
            TargetRevisionV1::Current
        );
        for content in [
            r::CommentRevisionContentV1::Hidden {
                reason: "moderated".into(),
            },
            r::CommentRevisionContentV1::Tombstoned { reason: None },
        ] {
            let hidden = reader.comment(Some(current), content);
            reader.select_review(hidden, merged);
            assert!(matches!(
                resolve_reaction_target(&reader, &review_target(old)).await,
                Err(Error::TargetUnavailable)
            ));
        }
        let conflict = reader.review(r::ReviewObjectV1::CommentConflict(Box::new(
            r::ReviewCommentConflictV1 {
                schema_version: 1,
                thread_id: rid(),
                comment_id: cid(),
                alternatives: BTreeSet::from([old, current]).into_iter().collect(),
            },
        )));
        reader.select_review(conflict, merged);
        assert!(matches!(
            resolve_reaction_target(&reader, &review_target(old)).await,
            Err(Error::TargetUnavailable)
        ));
        assert!(
            resolve_reaction_target(
                &reader,
                &ReactionTargetV1::ReviewThread { thread_id: rid() }
            )
            .await
            .is_ok()
        );
    }
}

#[tokio::test]
async fn resolved_review_conflict_retains_exact_alternatives_but_rejects_orphans() {
    let mut reader = Reader::default();
    let body = reader.body("original");
    let old = reader.comment(None, r::CommentRevisionContentV1::Visible { body });
    let body = reader.body("a");
    let a = reader.comment(Some(old), r::CommentRevisionContentV1::Visible { body });
    let body = reader.body("b");
    let b = reader.comment(Some(old), r::CommentRevisionContentV1::Visible { body });
    let body = reader.body("unrelated");
    let orphan = reader.comment(None, r::CommentRevisionContentV1::Visible { body });
    let body = reader.body("resolution");
    let resolved = reader.review(r::ReviewObjectV1::CommentRevision(Box::new(
        r::ReviewCommentRevisionV1 {
            schema_version: 1,
            thread_id: rid(),
            comment_id: cid(),
            author: actor(),
            actor: actor(),
            created_at_millis: 2,
            previous_revision: None,
            superseded_revisions: BTreeSet::from([a, b]).into_iter().collect(),
            content: r::CommentRevisionContentV1::Visible { body },
        },
    )));
    reader.select_review(resolved, true);
    for reference in [old, a, b] {
        assert_eq!(
            resolve_reaction_target(&reader, &review_target(reference))
                .await
                .unwrap()
                .revision,
            TargetRevisionV1::Earlier
        );
    }
    assert!(matches!(
        resolve_reaction_target(&reader, &review_target(orphan)).await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn selected_review_replies_use_the_same_exact_revision_policy() {
    let mut reader = Reader::default();
    let body = reader.body("initial");
    let first = reader.comment(None, r::CommentRevisionContentV1::Visible { body });
    reader.select_review(first, false);
    let reply_id = r::CommentId::new([10; 16]);
    let body = reader.body("reply");
    let reply = reader.review(r::ReviewObjectV1::CommentRevision(Box::new(
        r::ReviewCommentRevisionV1 {
            schema_version: 1,
            thread_id: rid(),
            comment_id: reply_id,
            author: actor(),
            actor: actor(),
            created_at_millis: 2,
            previous_revision: None,
            superseded_revisions: vec![],
            content: r::CommentRevisionContentV1::Visible { body },
        },
    )));
    let selected = reader.reviews[&rid()];
    let r::ReviewObjectV1::ThreadSnapshot(mut thread) = reader.review_objects[&selected].clone()
    else {
        panic!()
    };
    thread.comments.push(r::CommentSelectionV1 {
        comment_id: reply_id,
        revision: reply,
        parent_comment_id: Some(cid()),
    });
    let selected = reader.review(r::ReviewObjectV1::ThreadSnapshot(thread));
    reader.reviews.insert(rid(), selected);
    let target = ReactionTargetV1::ReviewComment {
        thread_id: rid(),
        comment_id: reply_id,
        revision: reply,
    };
    assert_eq!(
        resolve_reaction_target(&reader, &target)
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Current
    );
    let wrong = ReactionTargetV1::ReviewComment {
        thread_id: rid(),
        comment_id: cid(),
        revision: reply,
    };
    assert!(matches!(
        resolve_reaction_target(&reader, &wrong).await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn checkpoint_versions_require_exact_native_or_portable_supersession() {
    let mut reader = Reader::default();
    let session = reader.session(vec![], BTreeMap::new());
    let checkpoint_id = a::AgentCheckpointId::new([11; 16]);
    let facts = a::AgentCheckpointFactsV1 {
        checkpoint_id,
        session_id: sid(),
        turn_id: None,
        scope_id: None,
        session_snapshot: session,
        transcript_manifest: aref(a::AgentObjectKind::TranscriptManifest),
        event_batches: vec![],
        tools: BTreeMap::new(),
        associations: vec![],
        evidence_method: a::EvidenceMethod::ManualAssertion,
        source_trust: a::SourceTrust::UntrustedAssertion,
        exclusive_write_scope: false,
        coverage: coverage(),
        capture_status: a::CaptureStatusV1::Missing,
        execution_outcome: a::ExecutionOutcomeV1::Unknown,
        supersedes: None,
    };
    let checkpoint = a::AgentCheckpointVersionV1 {
        schema_version: 1,
        checkpoint_id,
        session_id: sid(),
        turn_id: None,
        scope_id: None,
        session_snapshot: session,
        transcript_manifest: facts.transcript_manifest,
        event_batches: vec![],
        tools: BTreeMap::new(),
        before: a::CodeAnchorV1::default(),
        after: a::CodeAnchorV1::default(),
        code_relation: a::CodeRelation::NotObserved,
        associations: vec![],
        evidence_method: facts.evidence_method,
        source_trust: facts.source_trust,
        exclusive_write_scope: false,
        coverage: coverage(),
        capture_status: facts.capture_status,
        execution_outcome: facts.execution_outcome,
        supersedes: None,
    };
    let native = reader.agent(a::AgentObjectV1::CheckpointVersion(checkpoint));
    let mut portable = a::AgentCheckpointVersionV2 {
        schema_version: 2,
        before: a::CodeAnchorV2::default(),
        after: a::CodeAnchorV2::default(),
        code_relation: a::CodeRelation::NotObserved,
        facts,
    };
    let orphan = reader.agent(a::AgentObjectV1::PortableCheckpointVersion(
        portable.clone(),
    ));
    portable.facts.supersedes = Some(native);
    let current = reader.agent(a::AgentObjectV1::PortableCheckpointVersion(portable));
    reader.checkpoints.insert(checkpoint_id, current);
    for (version, expected) in [
        (native, TargetRevisionV1::Earlier),
        (current, TargetRevisionV1::Current),
    ] {
        assert_eq!(
            resolve_reaction_target(
                &reader,
                &ReactionTargetV1::AgentCheckpoint {
                    session_id: sid(),
                    checkpoint_id,
                    version
                }
            )
            .await
            .unwrap()
            .revision,
            expected
        );
    }
    assert!(matches!(
        resolve_reaction_target(
            &reader,
            &ReactionTargetV1::AgentCheckpoint {
                session_id: sid(),
                checkpoint_id,
                version: orphan
            }
        )
        .await,
        Err(Error::TargetUnavailable)
    ));
    reader.sessions.clear();
    assert!(matches!(
        resolve_reaction_target(
            &reader,
            &ReactionTargetV1::AgentCheckpoint {
                session_id: sid(),
                checkpoint_id,
                version: native
            }
        )
        .await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn forged_hashes_and_cross_entity_selections_fail_closed() {
    let mut reader = Reader::default();
    reader.thread(ThreadStatusV1::Active);
    let reference = reader.threads[&tid()];
    let CollabObjectV1::ThreadVersion(thread) = reader.graph.objects.get_mut(&reference).unwrap()
    else {
        panic!()
    };
    thread.title = "forged".into();
    assert!(matches!(
        resolve_reaction_target(&reader, &ReactionTargetV1::Thread { thread_id: tid() }).await,
        Err(Error::Invalid(_))
    ));
    let selected = reader.session(vec![], BTreeMap::new());
    reader
        .sessions
        .insert(a::AgentSessionId::new([12; 16]), selected);
    let target = ReactionTargetV1::ToolInvocation {
        session_id: a::AgentSessionId::new([12; 16]),
        tool_id: a::ToolCallId::new([9; 16]),
        snapshot: aref(a::AgentObjectKind::ToolSnapshot),
    };
    assert!(matches!(
        resolve_reaction_target(&reader, &target).await,
        Err(Error::Invalid(_))
    ));
    let body = reader.body("test");
    let revision = reader.comment(None, r::CommentRevisionContentV1::Visible { body });
    reader.select_review(revision, false);
    let r::ReviewObjectV1::CommentRevision(comment) =
        reader.review_objects.get_mut(&revision).unwrap()
    else {
        panic!()
    };
    comment.created_at_millis = 99;
    assert!(matches!(
        resolve_reaction_target(&reader, &review_target(revision)).await,
        Err(Error::Invalid(_))
    ));
}

#[tokio::test]
async fn revision_traversal_is_bounded_and_absent_targets_are_unavailable() {
    let mut reader = Reader::default();
    reader.thread(ThreadStatusV1::Active);
    let oldest = reader.message(None, Some("first"));
    let mut current = oldest;
    for _ in 0..MAX_TARGET_RESOLUTION_OBJECTS {
        current = reader.message(Some(current), Some("edit"));
    }
    reader.messages.insert((tid(), mid()), current);
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(oldest)).await,
        Err(Error::Limit)
    ));
    reader.messages.clear();
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(oldest)).await,
        Err(Error::TargetUnavailable)
    ));
    reader.threads.clear();
    assert!(matches!(
        resolve_reaction_target(&reader, &human_target(oldest)).await,
        Err(Error::TargetUnavailable)
    ));
}

#[tokio::test]
async fn work_comment_requires_exact_visible_identity_task_and_version() {
    use jj_work::{TaskId, WorkActorV1, WorkCommentId, WorkCommentV1};
    let mut reader = Reader::default();
    let task_id = TaskId::new([7; 16]);
    let comment_id = WorkCommentId::new([8; 16]);
    let target = ReactionTargetV1::WorkComment {
        task_id,
        comment_id,
        version: 1,
    };
    assert!(matches!(
        resolve_reaction_target(&reader, &target).await,
        Err(Error::TargetUnavailable)
    ));
    let comment = WorkCommentV1 {
        schema_version: 1,
        id: comment_id,
        task: task_id,
        reply_to: None,
        body: Some("Visible".into()),
        version: 1,
        actor: WorkActorV1 {
            created_by: actor(),
            updated_by: actor(),
            created_at_millis: 0,
            updated_at_millis: 0,
        },
    };
    reader.work = Some(comment.clone());
    assert_eq!(
        resolve_reaction_target(&reader, &target)
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Current
    );
    assert!(
        ReactionTargetV1::WorkComment {
            task_id,
            comment_id,
            version: 0
        }
        .exact_key()
        .is_err()
    );
    assert_ne!(
        target.exact_key().unwrap(),
        ReactionTargetV1::WorkComment {
            task_id,
            comment_id,
            version: 2
        }
        .exact_key()
        .unwrap()
    );
    for value in [
        WorkCommentV1 {
            version: 2,
            ..comment.clone()
        },
        WorkCommentV1 {
            id: WorkCommentId::new([9; 16]),
            ..comment.clone()
        },
        WorkCommentV1 {
            task: TaskId::new([9; 16]),
            ..comment.clone()
        },
        WorkCommentV1 {
            body: None,
            ..comment.clone()
        },
    ] {
        reader.work = Some(value);
        assert!(matches!(
            resolve_reaction_target(&reader, &target).await,
            Err(Error::TargetUnavailable)
        ));
    }
    // Existing target JSON and exact-key inputs are unaffected by the additive variant.
    assert_eq!(
        serde_json::to_string(&ReactionTargetV1::Thread { thread_id: tid() }).unwrap(),
        r#"{"type":"thread","thread_id":"01010101010101010101010101010101"}"#
    );
}
