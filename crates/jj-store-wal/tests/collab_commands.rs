use jj_collab::*;
use jj_store_model::{
    EpochId, ObjectKey, OperationId, PrincipalId, PublicationId, RepositoryId, RequestId,
};
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
    memory::MemoryStore,
};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, Ordering},
    },
    time::Duration,
};
use tokio::sync::Notify;

const REPO: RepositoryId = RepositoryId::new([1; 16]);
const ACTOR: PrincipalId = PrincipalId::new([2; 16]);
struct Source;
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPO)
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}
#[derive(Default)]
struct Guard(AtomicBool);
#[async_trait::async_trait]
impl PublicationGuard for Guard {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Err(wal::Error::Denied)
        } else {
            Ok(())
        }
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}
/// Inject faults only at manifest CAS, after all immutable preparation.
#[derive(Default)]
struct Store {
    memory: MemoryStore,
    mode: AtomicU8,
    blocked: Notify,
    release: Notify,
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.memory.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        self.memory.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        let mode = if key.as_str().ends_with("/manifest.json") {
            self.mode.swap(0, Ordering::SeqCst)
        } else {
            0
        };
        if mode == 1 {
            self.blocked.notify_one();
            self.release.notified().await;
        }
        if mode == 2 {
            return Ok(WriteOutcome::Unknown);
        }
        let result = self.memory.put(key, condition, upload).await?;
        if mode == 3 && matches!(result, WriteOutcome::Applied(_)) {
            Ok(WriteOutcome::Unknown)
        } else {
            Ok(result)
        }
    }
}
fn coordinator(storage: Arc<Store>, guard: Arc<Guard>) -> Arc<Coordinator> {
    Arc::new(Coordinator::new(storage, REPO, Arc::new(Source), guard, Limits::default()).unwrap())
}
async fn fixture() -> (Arc<Store>, Arc<Guard>, Arc<Coordinator>) {
    let storage = Arc::new(Store::default());
    let guard = Arc::new(Guard::default());
    let c = coordinator(storage.clone(), guard.clone());
    c.initialize(EpochId::new([3; 16]), vec![]).await.unwrap();
    (storage, guard, c)
}
fn command_request(n: u8, command: CollabCommandV1) -> CollabCommandRequestV1 {
    CollabCommandRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: RequestId::new([n; 16]),
        publication_id: PublicationId::new([n; 16]),
        command,
    }
}
fn thread(n: u8) -> CollabCommandRequestV1 {
    command_request(
        n,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Create {
                thread_id: ThreadId::new([n; 16]),
                title: format!("Thread {n}"),
                baseline: None,
            },
        },
    )
}
async fn publish(c: &Coordinator, request: &CollabCommandRequestV1) -> Receipt {
    let PublishOutcome::Committed(value) = c.publish_collab_command(request).await.unwrap() else {
        panic!("expected receipt")
    };
    CollabCommandReceiptV1::decode(&value.result).unwrap();
    value
}
fn thread_result(receipt: &Receipt) -> ThreadCommandResultV1 {
    let CollabCommandResultV1::Thread { result } = CollabCommandReceiptV1::decode(&receipt.result)
        .unwrap()
        .result
    else {
        panic!()
    };
    result
}
fn reaction_result(receipt: &Receipt) -> ReactionCommandResultV1 {
    let CollabCommandResultV1::Reaction { result } =
        CollabCommandReceiptV1::decode(&receipt.result)
            .unwrap()
            .result
    else {
        panic!()
    };
    result
}
fn react(
    n: u8,
    target: ReactionTargetV1,
    observed: BTreeSet<CollabObjectRef>,
    value: ReactionValueV1,
) -> CollabCommandRequestV1 {
    command_request(
        n,
        CollabCommandV1::Reaction {
            intent: ReactionCommandV1::Set {
                target,
                observed,
                value,
            },
        },
    )
}
fn message(
    n: u8,
    created: &ThreadCommandResultV1,
    id: HumanMessageId,
    prior: Option<CollabObjectRef>,
    body: Option<&str>,
) -> CollabCommandRequestV1 {
    command_request(
        n,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Message {
                reply_to: None,
                thread_id: created.thread_id,
                expected_thread: created.version,
                message_id: id,
                expected_revision: prior,
                body: body.map(str::to_owned),
            },
        },
    )
}

#[tokio::test]
async fn native_commands_pin_history_preserve_siblings_and_replay_before_target_checks() {
    let (storage, guard, c) = fixture().await;
    let before = c.read().await.unwrap().manifest().clone();
    let created = thread_result(&publish(&c, &thread(10)).await);
    let id = HumanMessageId::new([11; 16]);
    let human = thread_result(&publish(&c, &message(11, &created, id, None, Some("first"))).await);
    let target = ReactionTargetV1::HumanMessage {
        thread_id: created.thread_id,
        message_id: id,
        revision: human.message.unwrap(),
    };
    let set = react(
        12,
        target.clone(),
        BTreeSet::new(),
        ReactionValueV1::Emoji {
            sequence: "👩🏽‍💻".into(),
        },
    );
    let first = publish(&c, &set).await;
    let active = reaction_result(&first).active;
    assert!(matches!(
        c.publish_collab_command(&react(
            13,
            target.clone(),
            BTreeSet::new(),
            ReactionValueV1::Dislike
        ))
        .await,
        Err(CollabCommandError::Model(jj_collab::Error::Stale))
    ));
    let replacement = publish(
        &c,
        &react(14, target.clone(), active, ReactionValueV1::Like),
    )
    .await;
    assert_eq!(
        reaction_result(&replacement).value,
        Some(ReactionValueV1::Like)
    );
    let edit =
        thread_result(&publish(&c, &message(15, &human, id, human.message, Some("edited"))).await);
    let clear = command_request(
        16,
        CollabCommandV1::Reaction {
            intent: ReactionCommandV1::Clear {
                target: target.clone(),
                observed: reaction_result(&replacement).active,
            },
        },
    );
    assert!(
        reaction_result(&publish(&c, &clear).await)
            .active
            .is_empty()
    );
    let after = c.read().await.unwrap().manifest().clone();
    assert_eq!(after.code_state_version, before.code_state_version);
    assert_eq!(after.heads, before.heads);
    assert_eq!(after.agent_root, before.agent_root);
    assert_eq!(after.review_root, before.review_root);
    assert_eq!(after.format, 5);
    let pin = c
        .native_thread_store(after.thread_root.unwrap())
        .await
        .unwrap();
    assert!(pin.graph.get(human.message.unwrap()).is_ok());
    publish(&c, &message(17, &edit, id, edit.message, None)).await;
    assert!(matches!(
        c.publish_collab_command(&react(18, target, BTreeSet::new(), ReactionValueV1::Like))
            .await,
        Err(CollabCommandError::Model(
            jj_collab::Error::TargetUnavailable
        ))
    ));
    c.checkpoint().await.unwrap();
    let cold = coordinator(storage, guard.clone());
    cold.recover(100).await.unwrap();
    assert_eq!(publish(&cold, &set).await, first);
    guard.0.store(true, Ordering::SeqCst);
    assert!(matches!(
        cold.publish_collab_command(&set).await,
        Err(CollabCommandError::Publication(wal::Error::Denied))
    ));
}

#[tokio::test]
async fn source_thread_cas_race_revalidates_visibility_before_reaction_publication() {
    let (storage, _, c) = fixture().await;
    let created = thread_result(&publish(&c, &thread(20)).await);
    let id = HumanMessageId::new([21; 16]);
    let human =
        thread_result(&publish(&c, &message(21, &created, id, None, Some("visible"))).await);
    let target = ReactionTargetV1::HumanMessage {
        thread_id: created.thread_id,
        message_id: id,
        revision: human.message.unwrap(),
    };
    storage.mode.store(1, Ordering::SeqCst);
    let task = tokio::spawn({
        let c = c.clone();
        async move {
            c.publish_collab_command(&react(22, target, BTreeSet::new(), ReactionValueV1::Like))
                .await
        }
    });
    tokio::time::timeout(Duration::from_secs(5), storage.blocked.notified())
        .await
        .unwrap();
    publish(&c, &message(23, &human, id, human.message, None)).await;
    let hidden = c.read().await.unwrap();
    storage.release.notify_one();
    assert!(matches!(
        task.await.unwrap(),
        Err(CollabCommandError::Model(
            jj_collab::Error::TargetUnavailable
        ))
    ));
    assert_eq!(c.read().await.unwrap().digest(), hidden.digest());
    assert!(hidden.manifest().reaction_root.is_none());
}

async fn create_review(c: &Coordinator, n: u8) -> jj_review::ReviewCommandResultV1 {
    use jj_review as r;
    let request = ReviewCommandRequest::new(
        ACTOR,
        RequestId::new([n; 16]),
        PublicationId::new([n; 16]),
        r::ReviewCommandV1::CreateThread {
            anchor: Box::new(r::ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPO,
                target: r::ArtifactTargetV1::Repository,
                created_context: r::CreatedContextV1 {
                    publication_position: Some(0),
                    code_state_version: Some(0),
                    agent_root: None,
                },
                sensitivity: r::SensitivityV1::Repository,
            }),
            body: r::ReviewCommentBodyV1 {
                schema_version: 1,
                format: r::ReviewBodyFormatV1::PlainText,
                text: "visible".into(),
                sanitizer_version: "v1".into(),
            },
        },
    )
    .unwrap();
    let PublishOutcome::Committed(value) = c.publish_review_command(&request).await.unwrap() else {
        panic!()
    };
    r::decode_command_result(&value.result).unwrap()
}
#[tokio::test]
async fn source_review_cas_race_revalidates_moderation_and_keeps_staged_reactions_invisible() {
    let (storage, _, c) = fixture().await;
    let review = create_review(&c, 30).await;
    let target = ReactionTargetV1::ReviewComment {
        thread_id: review.thread_id,
        comment_id: review.comment_id.unwrap(),
        revision: review.comment_revision.unwrap(),
    };
    storage.mode.store(1, Ordering::SeqCst);
    let task = tokio::spawn({
        let c = c.clone();
        async move {
            c.publish_collab_command(&react(31, target, BTreeSet::new(), ReactionValueV1::Like))
                .await
        }
    });
    tokio::time::timeout(Duration::from_secs(5), storage.blocked.notified())
        .await
        .unwrap();
    let hide = ReviewCommandRequest::new(
        ACTOR,
        RequestId::new([32; 16]),
        PublicationId::new([32; 16]),
        jj_review::ReviewCommandV1::HideComment {
            comment_id: review.comment_id.unwrap(),
            expected_revision_id: jj_review::CommentRevisionId::new(
                *review.comment_revision.unwrap().id.as_bytes(),
            ),
            moderation_reason: "hidden".into(),
        },
    )
    .unwrap();
    c.publish_review_command(&hide).await.unwrap();
    let hidden = c.read().await.unwrap();
    storage.release.notify_one();
    assert!(matches!(
        task.await.unwrap(),
        Err(CollabCommandError::Model(
            jj_collab::Error::TargetUnavailable
        ))
    ));
    assert_eq!(c.read().await.unwrap().digest(), hidden.digest());
    c.recover(100).await.unwrap();
}

#[tokio::test]
async fn manifest_unknown_outcomes_keep_identity_and_replay_after_reopen() {
    for mode in [2, 3] {
        let (storage, guard, c) = fixture().await;
        let request = thread(40);
        storage.mode.store(mode, Ordering::SeqCst);
        let outcome = c.publish_collab_command(&request).await.unwrap();
        if mode == 2 {
            assert!(
                matches!(outcome, PublishOutcome::OutcomeUnknown { request_id, .. } if request_id == request.request_id)
            );
        } else {
            assert!(matches!(outcome, PublishOutcome::Committed(_)));
        }
        let cold = coordinator(storage, guard);
        let first = publish(&cold, &request).await;
        let pin = cold.read().await.unwrap().digest();
        assert_eq!(publish(&cold, &request).await, first);
        assert_eq!(cold.read().await.unwrap().digest(), pin);
        let altered = command_request(
            40,
            CollabCommandV1::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: ThreadId::new([40; 16]),
                    title: "different intent".into(),
                    baseline: None,
                },
            },
        );
        assert!(matches!(
            cold.publish_collab_command(&altered).await,
            Err(CollabCommandError::Publication(wal::Error::IntentConflict))
        ));
        cold.recover(100).await.unwrap();
    }
}

#[tokio::test]
async fn native_custom_images_retire_without_rewriting_historical_values_or_assets() {
    let (storage, guard, c) = fixture().await;
    let created = thread_result(&publish(&c, &thread(50)).await);
    let target = ReactionTargetV1::Thread {
        thread_id: created.thread_id,
    };
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::new_rgba8(2, 2)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let bytes = bytes.into_inner();
    let id = CustomEmojiId::new([51; 16]);
    let definition = publish(
        &c,
        &command_request(
            51,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Define {
                    emoji_id: id,
                    expected: None,
                    shortcode: "custom".into(),
                    label: "Custom".into(),
                    bytes: bytes.clone(),
                },
            },
        ),
    )
    .await;
    let CollabCommandResultV1::Emoji { version } =
        CollabCommandReceiptV1::decode(&definition.result)
            .unwrap()
            .result
    else {
        panic!()
    };
    let value = ReactionValueV1::Custom {
        emoji_id: id,
        version,
    };
    let set = react(52, target.clone(), BTreeSet::new(), value.clone());
    let first = publish(&c, &set).await;
    publish(
        &c,
        &command_request(
            53,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Retire {
                    emoji_id: id,
                    expected: version,
                },
            },
        ),
    )
    .await;
    let pin = c.read().await.unwrap();
    assert!(matches!(
        c.publish_collab_command(&react(54, target, reaction_result(&first).active, value))
            .await,
        Err(CollabCommandError::Model(jj_collab::Error::Stale))
    ));
    let cold = coordinator(storage, guard);
    cold.recover(100).await.unwrap();
    let reactions = cold
        .native_reaction_store(pin.manifest().reaction_root.unwrap())
        .await
        .unwrap();
    let CollabObjectV1::CustomEmojiVersion(emoji) = reactions.graph.get(version).unwrap() else {
        panic!()
    };
    let CollabObjectV1::ImageAsset(asset) = reactions.graph.get(emoji.asset).unwrap() else {
        panic!()
    };
    assert_eq!(asset.bytes, bytes);
    assert!(reactions.graph.get(emoji.preview).is_ok());
    assert_eq!(publish(&cold, &set).await, first);
    let target_storage = Arc::new(Store::default());
    let target_c = coordinator(target_storage.clone(), Arc::new(Guard::default()));
    cold.copy_native_to(target_storage, Arc::new(Source), 10000)
        .await
        .unwrap();
    target_c.recover(100).await.unwrap();
    assert!(
        target_c
            .native_reaction_store(pin.manifest().reaction_root.unwrap())
            .await
            .unwrap()
            .graph
            .get(emoji.asset)
            .is_ok()
    );
}

#[tokio::test]
async fn native_code_links_require_selected_accepted_identities_and_preserve_absent_domains() {
    let (_, _, c) = fixture().await;
    let before = c.read().await.unwrap();
    let mut invalid = thread(60);
    if let CollabCommandV1::Thread {
        intent: ThreadCommandV1::Create { baseline, .. },
    } = &mut invalid.command
    {
        *baseline = Some(ThreadCodeAnchorV1::Native(jj_agent::CodeAnchorV1 {
            operations: BTreeSet::from([OperationId::new([77; 64])]),
            commits: BTreeSet::new(),
        }));
    }
    assert!(matches!(
        c.publish_collab_command(&invalid).await,
        Err(CollabCommandError::Publication(wal::Error::Stale))
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    if let CollabCommandV1::Thread {
        intent: ThreadCommandV1::Create { baseline, .. },
    } = &mut invalid.command
    {
        *baseline = Some(ThreadCodeAnchorV1::Native(jj_agent::CodeAnchorV1 {
            operations: BTreeSet::new(),
            commits: BTreeSet::from([jj_store_model::CommitId::new([77; 64])]),
        }));
    }
    assert!(matches!(
        c.publish_collab_command(&invalid).await,
        Err(CollabCommandError::Publication(wal::Error::Stale))
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    if let CollabCommandV1::Thread {
        intent: ThreadCommandV1::Create { baseline, .. },
    } = &mut invalid.command
    {
        *baseline = Some(ThreadCodeAnchorV1::Native(jj_agent::CodeAnchorV1 {
            operations: before.manifest().heads.clone(),
            commits: BTreeSet::new(),
        }));
    }
    publish(&c, &invalid).await;
    let pin = c.read().await.unwrap();
    assert!(pin.manifest().reaction_root.is_none());
    assert_eq!(pin.manifest().agent_state_version, 0);
    assert_eq!(pin.manifest().review_state_version, 0);
    c.recover(100).await.unwrap();
}

#[tokio::test]
async fn native_sessions_remain_optional_and_links_check_selected_membership_and_assignment() {
    use jj_agent as a;
    let (_, _, c) = fixture().await;
    let session_id = a::AgentSessionId::new([70; 16]);
    let session = a::AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id,
        integration: a::IntegrationId::Codex,
        state: a::SessionState::Ended,
        parent_session_id: None,
        evidence: vec![],
        event_batches: vec![],
        transcripts: vec![],
        tools: BTreeMap::new(),
        coverage: a::CoverageV1 {
            completeness: a::CompletenessV1::Unknown,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    };
    let snapshot = c
        .stage_agent_object(&a::AgentObjectV1::SessionSnapshot(session.clone()))
        .await
        .unwrap();
    let first = thread_result(&publish(&c, &thread(71)).await);
    let link_command =
        |thread: &ThreadCommandResultV1, primary, observed_primary| CollabCommandV1::Thread {
            intent: ThreadCommandV1::Link {
                thread_id: thread.thread_id,
                expected_thread: thread.version,
                link_id: ThreadLinkId::new(
                    [if thread.thread_id == first.thread_id {
                        72
                    } else {
                        75
                    }; 16],
                ),
                target: ThreadLinkTargetV1::Session {
                    session_id,
                    snapshot,
                    primary,
                },
                observed_primary,
            },
        };
    assert!(matches!(
        c.publish_collab_command(&command_request(72, link_command(&first, true, None)))
            .await,
        Err(CollabCommandError::Model(jj_collab::Error::Stale))
    ));
    let page = c
        .stage_agent_object(&a::AgentObjectV1::IndexPage(a::AgentIndexPageV1::Leaf {
            prefix: String::new(),
            entries: BTreeMap::from([(session.catalog_key().unwrap(), snapshot)]),
        }))
        .await
        .unwrap();
    let mut root = a::AgentRootV1::empty();
    root.sessions = Some(page);
    let reference = c.stage_agent_root(&root).await.unwrap();
    let expected = c
        .read()
        .await
        .unwrap()
        .manifest()
        .domain_state(Domain::Agent)
        .unwrap();
    c.publish_domain(
        &DomainRequest::new(
            ACTOR,
            RequestId::new([73; 16]),
            PublicationId::new([73; 16]),
            Domain::Agent,
            expected,
            reference,
            vec![],
        )
        .unwrap(),
    )
    .await
    .unwrap();
    let pin = c.read().await.unwrap();
    assert!(
        c.native_thread_store(pin.manifest().thread_root.unwrap())
            .await
            .unwrap()
            .state
            .assignments
            .is_empty()
    );
    let linked =
        thread_result(&publish(&c, &command_request(72, link_command(&first, true, None))).await);
    let second = thread_result(&publish(&c, &thread(74)).await);
    assert!(matches!(
        c.publish_collab_command(&command_request(75, link_command(&second, true, None)))
            .await,
        Err(CollabCommandError::Model(jj_collab::Error::Stale))
    ));
    let moved = thread_result(
        &publish(
            &c,
            &command_request(75, link_command(&second, true, linked.link)),
        )
        .await,
    );
    assert_eq!(moved.reassigned_from, Some(first.thread_id));
    let pin = c.read().await.unwrap();
    let threads = c
        .native_thread_store(pin.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert_eq!(
        threads.state.assignments.get(&session_id),
        moved.link.as_ref()
    );
    let CollabObjectV1::ThreadLink(old) = threads.graph.get(linked.link.unwrap()).unwrap() else {
        panic!()
    };
    assert!(old.active); // immutable historical assignment
    assert_eq!(threads.state.assignments.len(), 1);
    c.recover(100).await.unwrap();
}

#[tokio::test]
async fn disjoint_actor_cas_race_reapplies_original_intent_and_retains_both_slots() {
    let (storage, _, c) = fixture().await;
    let created = thread_result(&publish(&c, &thread(80)).await);
    let target = ReactionTargetV1::Thread {
        thread_id: created.thread_id,
    };
    let first = react(81, target.clone(), BTreeSet::new(), ReactionValueV1::Like);
    storage.mode.store(1, Ordering::SeqCst);
    let task = tokio::spawn({
        let c = c.clone();
        let first = first.clone();
        async move { c.publish_collab_command(&first).await }
    });
    tokio::time::timeout(Duration::from_secs(5), storage.blocked.notified())
        .await
        .unwrap();
    let mut other = react(
        82,
        target.clone(),
        BTreeSet::new(),
        ReactionValueV1::Dislike,
    );
    other.principal = PrincipalId::new([8; 16]);
    publish(&c, &other).await;
    storage.release.notify_one();
    assert!(matches!(
        task.await.unwrap().unwrap(),
        PublishOutcome::Committed(_)
    ));
    let pin = c.read().await.unwrap();
    let reactions = c
        .native_reaction_store(pin.manifest().reaction_root.unwrap())
        .await
        .unwrap();
    let state = ReactionStateV1::read(REPO, reactions.root, &reactions.graph).unwrap();
    assert_eq!(state.slots.len(), 2);
    for (actor, value) in [
        (ACTOR, ReactionValueV1::Like),
        (other.principal, ReactionValueV1::Dislike),
    ] {
        let key = slot_key(REPO, actor, &target).unwrap();
        let CollabObjectV1::ReactionSlot(slot) = reactions.graph.get(state.slots[&key]).unwrap()
        else {
            panic!()
        };
        assert_eq!(
            slot.resolve(&reactions.graph.objects).unwrap().value,
            Some(value)
        );
    }
    c.recover(100).await.unwrap();
}

#[tokio::test]
async fn conversation_submission_is_atomic_receipt_backed_and_rejects_forged_ranges() {
    let (storage, guard, c) = fixture().await;
    let parent = thread_result(&publish(&c, &thread(80)).await);
    let original = thread_result(
        &publish(
            &c,
            &message(
                81,
                &parent,
                HumanMessageId::new([81; 16]),
                None,
                Some("hello 🦀"),
            ),
        )
        .await,
    );
    let source = ConversationSourceV1 {
        target: ReactionTargetV1::HumanMessage {
            thread_id: parent.thread_id,
            message_id: HumanMessageId::new([81; 16]),
            revision: original.message.unwrap(),
        },
        part: None,
        selection: Some(jj_review::TextSelectionV1 {
            encoding: jj_review::TextEncoding::Utf8,
            start_byte: jj_review::ByteOffset::new(6),
            end_byte: jj_review::ByteOffset::new(10),
            selected_sha256: jj_review::ReviewDigest::sha256("🦀".as_bytes()),
        }),
        quote: "🦀".into(),
    };
    let mut submission = ConversationSubmissionV1 {
        workspace_comments: vec![],
        reply_to: None,
        thread_id: parent.thread_id,
        expected_thread: original.version,
        message_id: None,
        text: None,
        comments: vec![ConversationCommentDraftV1 {
            thread_id: ThreadId::new([82; 16]),
            expected_thread: None,
            source,
            message_id: HumanMessageId::new([82; 16]),
            expected_revision: None,
            reply_to: None,
            body: "Explain this".into(),
        }],
        references: vec![],
    };
    let mut forged = submission.clone();
    forged.comments[0].source.quote = "false".into();
    assert!(
        c.publish_collab_command(&command_request(
            82,
            CollabCommandV1::Thread {
                intent: ThreadCommandV1::Submit {
                    submission: Box::new(forged)
                }
            }
        ))
        .await
        .is_err()
    );
    let request = command_request(
        83,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Submit {
                submission: Box::new(submission.clone()),
            },
        },
    );
    let committed = publish(&c, &request).await;
    let cold = coordinator(storage, guard);
    let replay = publish(&cold, &request).await;
    assert_eq!(replay, committed);
    let pin = cold.read().await.unwrap();
    let store = cold
        .native_thread_store(pin.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert_eq!(store.state.threads.len(), 2);
    assert_eq!(store.state.messages.len(), 2);
    let batch = thread_result(&committed).link.unwrap();
    assert!(
        matches!(&store.graph.get(batch).unwrap(),CollabObjectV1::ThreadLink(ThreadLinkV1 {target:ThreadLinkTargetV1::CommentBatch {comments,..},..}) if comments.len()==1)
    );
    submission.comments[0].body = "changed request".into();
    assert!(
        cold.publish_collab_command(&command_request(
            83,
            CollabCommandV1::Thread {
                intent: ThreadCommandV1::Submit {
                    submission: Box::new(submission)
                }
            }
        ))
        .await
        .is_err()
    );
}

#[tokio::test]
async fn qualified_reviews_require_an_explicit_reader_without_foreign_dependencies() {
    use jj_review as r;
    struct Authorized(AtomicBool);
    #[async_trait::async_trait]
    impl QualifiedReviewRead for Authorized {
        async fn comment(
            &self,
            target: &ReactionTargetV1,
        ) -> jj_collab::Result<QualifiedReviewComment> {
            if !self.0.load(Ordering::SeqCst) {
                return Err(jj_collab::Error::TargetUnavailable);
            }
            let ReactionTargetV1::RepositoryReviewComment { repository, .. } = target else {
                panic!()
            };
            Ok(QualifiedReviewComment {
                target: target.clone(),
                revision: TargetRevisionV1::Earlier,
                body: "Review".into(),
                file: b"hello".to_vec(),
                anchor: r::ArtifactAnchorV1 {
                    schema_version: 1,
                    repository_id: *repository,
                    target: r::ArtifactTargetV1::CodeArtifact(Box::new(r::CodeArtifactAnchorV1 {
                        operation_id: OperationId::new([1; 64]),
                        commit_id: jj_store_model::CommitId::new([2; 64]),
                        change_id: None,
                        path: "hello.txt".into(),
                        object: ObjectKey::File(jj_store_model::FileId::new([3; 64])),
                        value_kind: r::CodeValueKind::File,
                    })),
                    created_context: r::CreatedContextV1 {
                        publication_position: None,
                        code_state_version: None,
                        agent_root: None,
                    },
                    sensitivity: r::SensitivityV1::Repository,
                },
            })
        }
    }
    let (storage, guard, c) = fixture().await;
    let parent = thread_result(&publish(&c, &thread(90)).await);
    let target = ReactionTargetV1::RepositoryReviewComment {
        repository: RepositoryId::new([99; 16]),
        thread_id: r::ReviewThreadId::new([91; 16]),
        comment_id: r::CommentId::new([92; 16]),
        revision: r::ReviewObjectRef::new(
            r::ReviewObjectKind::CommentRevision,
            r::ReviewObjectId::new([93; 32]),
        ),
    };
    let request = command_request(
        91,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Submit {
                submission: Box::new(ConversationSubmissionV1 {
                    workspace_comments: vec![],
                    thread_id: parent.thread_id,
                    expected_thread: parent.version,
                    message_id: None,
                    text: None,
                    reply_to: None,
                    comments: vec![],
                    references: vec![target.clone()],
                }),
            },
        },
    );
    let before = c.read().await.unwrap().manifest().clone();
    assert!(c.publish_collab_command(&request).await.is_err());
    let authority = Authorized(AtomicBool::new(false));
    assert!(
        c.publish_collab_command_with_reviews(&request, Some(&authority))
            .await
            .is_err()
    );
    assert_eq!(c.read().await.unwrap().manifest(), &before);
    authority.0.store(true, Ordering::SeqCst);
    let PublishOutcome::Committed(receipt) = c
        .publish_collab_command_with_reviews(&request, Some(&authority))
        .await
        .unwrap()
    else {
        panic!()
    };
    let cold = coordinator(storage, guard);
    // A known receipt replays independently of the foreign source's later availability.
    authority.0.store(false, Ordering::SeqCst);
    let PublishOutcome::Committed(replay) = cold
        .publish_collab_command_with_reviews(&request, Some(&authority))
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(receipt, replay);
    let pin = cold.read().await.unwrap();
    let store = cold
        .native_thread_store(pin.manifest().thread_root.unwrap())
        .await
        .unwrap();
    assert_eq!(store.state.threads.len(), 1);
    assert!(store.state.messages.is_empty());
    let batch = thread_result(&receipt).link.unwrap();
    assert!(
        matches!(store.graph.get(batch).unwrap(), CollabObjectV1::ThreadLink(ThreadLinkV1 { target: ThreadLinkTargetV1::CommentBatch { comments, .. }, .. }) if comments == &vec![target])
    );
    assert_eq!(pin.manifest().review_root, before.review_root);
}
