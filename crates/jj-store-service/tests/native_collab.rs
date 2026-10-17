use jj_collab::*;
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, Scope, SystemClock, TokenPolicy},
    collab_mutations::{CollabCommandService, Error, Outcome, Request, Status},
    collab_reads::*,
    native::{NativeSession, SessionLimits, create_repository, create_repository_local_trusted},
};
use jj_store_wal::publication::Limits;
use std::{collections::BTreeSet, sync::Arc};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const EPOCH: m::EpochId = m::EpochId::new([2; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([3; 16]);
struct Fixture {
    store: Arc<MemoryStore>,
    objects: Arc<NativeObjects>,
}
impl Fixture {
    async fn code() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
        create_repository_local_trusted(
            store.clone(),
            objects.clone(),
            Permission::Admin,
            EPOCH,
            Limits::default(),
        )
        .await
        .unwrap();
        Self { store, objects }
    }
    async fn home() -> Self {
        let fixture = Self::code().await;
        work_home(fixture.session(ACTOR, Permission::Write)).await;
        fixture
    }
    fn session(&self, actor: m::PrincipalId, permission: Permission) -> Arc<NativeSession> {
        self.session_limits(actor, permission, SessionLimits::default())
    }
    fn session_limits(
        &self,
        actor: m::PrincipalId,
        permission: Permission,
        limits: SessionLimits,
    ) -> Arc<NativeSession> {
        Arc::new(
            NativeSession::new_local_trusted(
                self.store.clone(),
                self.objects.clone(),
                actor,
                permission,
                Arc::new(SystemClock),
                Protection::default(),
                limits,
            )
            .unwrap(),
        )
    }
    async fn read(&self) -> PinnedCollabRead {
        PinnedCollabRead::open(
            self.session(ACTOR, Permission::Read),
            Arc::new(RepositoryReactionActors(REPO)),
        )
        .await
        .unwrap()
    }
}
async fn work_home(session: Arc<NativeSession>) {
    use jj_store_service::work::*;
    WorkCommandService
        .apply(
            session,
            &Request {
                version: 1,
                request_id: m::RequestId::new([99; 16]),
                command: WorkCommandV1::CreateProject {
                    id: ProjectId::new([99; 16]),
                    title: "Collaboration home".into(),
                    body: "Native local collaboration".into(),
                },
            },
        )
        .await
        .unwrap();
}
fn request(n: u8, command: CollabCommandV1) -> Request {
    Request {
        version: 1,
        request_id: m::RequestId::new([n; 16]),
        command,
    }
}
fn create(n: u8) -> Request {
    request(
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
async fn apply(
    session: Arc<NativeSession>,
    request: &Request,
) -> jj_store_service::collab_mutations::Receipt {
    let Outcome::Committed { receipt } =
        CollabCommandService.apply(session, request).await.unwrap()
    else {
        panic!()
    };
    *receipt
}
fn thread(receipt: jj_store_service::collab_mutations::Receipt) -> ThreadCommandResultV1 {
    let CollabCommandResultV1::Thread { result } = receipt.result else {
        panic!()
    };
    result
}
fn react(n: u8, target: ReactionTargetV1, value: ReactionValueV1) -> Request {
    request(
        n,
        CollabCommandV1::Reaction {
            intent: ReactionCommandV1::Set {
                emoji: None,
                target,
                observed: BTreeSet::new(),
                value,
            },
        },
    )
}
fn message(
    n: u8,
    version: &ThreadCommandResultV1,
    id: HumanMessageId,
    prior: Option<CollabObjectRef>,
    text: Option<&str>,
) -> Request {
    request(
        n,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Message {
                emojis: Vec::new(),
                reply_to: None,
                thread_id: version.thread_id,
                expected_thread: version.version,
                message_id: id,
                expected_revision: prior,
                body: text.map(str::to_owned),
            },
        },
    )
}

#[tokio::test]
async fn absent_reads_do_not_select_domains_or_change_code_and_limits_fail_closed() {
    let f = Fixture::code().await;
    let session = f.session(ACTOR, Permission::Read);
    let before = session.snapshot().await.unwrap();
    let read = f.read().await;
    assert!(
        read.threads(session.clone(), None, 10)
            .await
            .unwrap()
            .threads
            .is_empty()
    );
    assert!(
        read.emojis(session.clone(), None, 10)
            .await
            .unwrap()
            .definitions
            .is_empty()
    );
    assert!(read.snapshot().thread_root.is_none());
    assert!(read.snapshot().reaction_root.is_none());
    assert_eq!(session.snapshot().await.unwrap().digest(), before.digest());
    for limit in [0, 257] {
        assert_eq!(
            read.threads(session.clone(), None, limit)
                .await
                .unwrap_err(),
            Error::Invalid
        );
    }
    assert!(matches!(
        read.reactions(
            session,
            &ReactionTargetV1::Thread {
                thread_id: ThreadId::new([9; 16])
            },
            None,
            10
        )
        .await,
        Err(Error::NotFound)
    ));
}

#[tokio::test]
async fn commands_derive_actor_replay_receipts_and_keep_status_scoped_to_current_identity() {
    let f = Fixture::home().await;
    let write = f.session(ACTOR, Permission::Write);
    let read = f.session(ACTOR, Permission::Read);
    let command = create(10);
    assert_eq!(
        CollabCommandService
            .apply(read.clone(), &command)
            .await
            .unwrap_err(),
        Error::Denied
    );
    let first = apply(write.clone(), &command).await;
    assert_eq!(first.actor, ACTOR);
    assert_eq!(apply(write.clone(), &command).await, first);
    let Status::Committed { receipt } = CollabCommandService
        .status(write.clone(), command.request_id)
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(*receipt, first);
    assert_eq!(
        CollabCommandService
            .status(read, command.request_id)
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert!(matches!(
        CollabCommandService
            .status(
                f.session(m::PrincipalId::new([4; 16]), Permission::Write),
                command.request_id
            )
            .await
            .unwrap(),
        Status::NotKnown { .. }
    ));
    let mut encoded = serde_json::to_value(&command).unwrap();
    encoded["actor"] = serde_json::to_value(m::PrincipalId::new([4; 16])).unwrap();
    assert!(serde_json::from_value::<Request>(encoded).is_err());
    let mut changed = command.clone();
    changed.command = CollabCommandV1::Thread {
        intent: ThreadCommandV1::Create {
            thread_id: ThreadId::new([10; 16]),
            title: "different".into(),
            baseline: None,
        },
    };
    assert_eq!(
        CollabCommandService
            .apply(write, &changed)
            .await
            .unwrap_err(),
        Error::IntentConflict
    );
}

struct HiddenActor(m::PrincipalId);
impl ReactionActorVisibility for HiddenActor {
    fn can_disclose(
        &self,
        repo: m::RepositoryId,
        _: m::PrincipalId,
        actor: m::PrincipalId,
    ) -> bool {
        repo == REPO && actor != self.0
    }
}
#[tokio::test]
async fn counts_filter_actors_before_pagination_and_pins_keep_exact_source_roots() {
    let f = Fixture::home().await;
    let write = f.session(ACTOR, Permission::Write);
    let created = thread(apply(write.clone(), &create(20)).await);
    let target = ReactionTargetV1::Thread {
        thread_id: created.thread_id,
    };
    let second = m::PrincipalId::new([4; 16]);
    let third = m::PrincipalId::new([5; 16]);
    let first = apply(
        write.clone(),
        &react(21, target.clone(), ReactionValueV1::Like),
    )
    .await;
    apply(
        f.session(second, Permission::Write),
        &react(22, target.clone(), ReactionValueV1::Like),
    )
    .await;
    apply(
        f.session(third, Permission::Write),
        &react(23, target.clone(), ReactionValueV1::Dislike),
    )
    .await;
    let read_session = f.session(ACTOR, Permission::Read);
    let pin = PinnedCollabRead::open(read_session.clone(), Arc::new(HiddenActor(second)))
        .await
        .unwrap();
    let page = pin
        .reactions(read_session.clone(), &target, None, 1)
        .await
        .unwrap();
    assert_eq!(page.actors.len(), 1);
    assert_eq!(page.actors[0].actor, ACTOR);
    assert_eq!(page.next_actor, Some(ACTOR));
    assert_eq!(
        page.reactions
            .iter()
            .find(|r| r.value == ReactionValueV1::Like)
            .unwrap()
            .count,
        1
    );
    assert!(
        page.reactions
            .iter()
            .find(|r| r.value == ReactionValueV1::Like)
            .unwrap()
            .viewer_reacted
    );
    let next = pin
        .reactions(read_session.clone(), &target, page.next_actor, 1)
        .await
        .unwrap();
    assert_eq!(next.reactions, page.reactions);
    assert_eq!(next.actors[0].actor, third);
    assert!(next.next_actor.is_none());
    let CollabCommandResultV1::Reaction { result } = first.result else {
        panic!()
    };
    apply(
        write,
        &request(
            24,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Clear {
                    target: target.clone(),
                    observed: result.active,
                },
            },
        ),
    )
    .await;
    assert_eq!(
        pin.reactions(read_session.clone(), &target, None, 10)
            .await
            .unwrap()
            .reactions,
        page.reactions
    );
    let fresh = f
        .read()
        .await
        .reactions(read_session.clone(), &target, None, 10)
        .await
        .unwrap();
    assert_eq!(fresh.reactions.len(), 2);
    assert!(!fresh.reactions.iter().any(|r| r.viewer_reacted));
    assert_eq!(
        pin.reactions(f.session(second, Permission::Read), &target, None, 10)
            .await
            .unwrap_err(),
        Error::Denied
    );
    let deny = PinnedCollabRead::open(
        read_session.clone(),
        Arc::new(RepositoryReactionActors(m::RepositoryId::new([99; 16]))),
    )
    .await
    .unwrap();
    let denied = deny
        .reactions(read_session, &target, None, 10)
        .await
        .unwrap();
    assert!(denied.actors.is_empty());
    assert!(denied.reactions.is_empty());
    assert_eq!(denied.conflict_count, 0);
}

#[tokio::test]
async fn history_continuations_are_thread_members_and_revision_feedback_never_moves() {
    let f = Fixture::home().await;
    let session = f.session(ACTOR, Permission::Write);
    let first = thread(apply(session.clone(), &create(30)).await);
    let id = HumanMessageId::new([31; 16]);
    let human = thread(
        apply(
            session.clone(),
            &message(31, &first, id, None, Some("original")),
        )
        .await,
    );
    let target = ReactionTargetV1::HumanMessage {
        thread_id: first.thread_id,
        message_id: id,
        revision: human.message.unwrap(),
    };
    apply(
        session.clone(),
        &react(32, target.clone(), ReactionValueV1::Like),
    )
    .await;
    let old = f.read().await;
    let edited = thread(
        apply(
            session.clone(),
            &message(33, &human, id, human.message, Some("edited")),
        )
        .await,
    );
    let current = f.read().await;
    let earlier = current
        .reactions(session.clone(), &target, None, 10)
        .await
        .unwrap();
    assert_eq!(earlier.revision, TargetRevisionV1::Earlier);
    assert_eq!(earlier.reactions[0].count, 1);
    let new_target = ReactionTargetV1::HumanMessage {
        revision: edited.message.unwrap(),
        thread_id: first.thread_id,
        message_id: id,
    };
    assert!(
        current
            .reactions(session.clone(), &new_target, None, 10)
            .await
            .unwrap()
            .reactions
            .is_empty()
    );
    assert_eq!(
        old.message(session.clone(), &target)
            .await
            .unwrap()
            .body
            .as_deref(),
        Some("original")
    );
    let page = current
        .events(session.clone(), first.thread_id, None, 1)
        .await
        .unwrap();
    assert_eq!(page.events.len(), 1);
    assert_eq!(page.events[0].message.as_ref().unwrap().message_id, id);
    assert_eq!(
        page.events[0].message.as_ref().unwrap().body.as_deref(),
        Some("edited")
    );
    let next = current
        .events(session.clone(), first.thread_id, page.next_event, 1)
        .await
        .unwrap();
    assert_eq!(next.events[0].reference, human.event);
    let other = thread(apply(session.clone(), &create(34)).await);
    let refreshed = f.read().await;
    assert_eq!(
        refreshed
            .events(session.clone(), first.thread_id, Some(other.event), 1)
            .await
            .unwrap_err(),
        Error::NotFound
    );
    apply(
        session.clone(),
        &message(35, &edited, id, edited.message, None),
    )
    .await;
    let tombstoned = f.read().await;
    assert_eq!(
        tombstoned
            .reactions(session.clone(), &target, None, 10)
            .await
            .unwrap_err(),
        Error::NotFound
    );
    assert_eq!(
        tombstoned
            .message(session.clone(), &target)
            .await
            .unwrap_err(),
        Error::NotFound
    );
    assert_eq!(
        old.message(session, &target).await.unwrap().body.as_deref(),
        Some("original")
    );
}

#[tokio::test]
async fn library_and_asset_reads_retain_original_versions_and_reject_unselected_assets() {
    let f = Fixture::home().await;
    let session = f.session(ACTOR, Permission::Write);
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::new_rgba8(2, 2)
        .write_to(&mut bytes, image::ImageFormat::Png)
        .unwrap();
    let bytes = bytes.into_inner();
    let id = CustomEmojiId::new([40; 16]);
    let defined = apply(
        session.clone(),
        &request(
            40,
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
    let CollabCommandResultV1::Emoji { version } = defined.result else {
        panic!()
    };
    let old = f.read().await;
    let page = old.emojis(session.clone(), None, 1).await.unwrap();
    assert_eq!(page.versions[&id], version);
    let emoji = &page.definitions[0];
    assert_eq!(
        old.image(session.clone(), emoji.asset).await.unwrap().bytes,
        bytes
    );
    assert!(old.image(session.clone(), emoji.preview).await.is_ok());
    assert_eq!(
        old.image(session.clone(), version).await.unwrap_err(),
        Error::Invalid
    );
    let missing = CollabObjectRef {
        id: CollabObjectId::new([99; 32]),
        kind: CollabObjectKind::ImageAsset,
    };
    assert_eq!(
        old.image(session.clone(), missing).await.unwrap_err(),
        Error::NotFound
    );
    apply(
        session.clone(),
        &request(
            41,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Retire {
                    emoji_id: id,
                    expected: version,
                },
            },
        ),
    )
    .await;
    assert!(
        !old.emojis(session.clone(), None, 1)
            .await
            .unwrap()
            .definitions[0]
            .retired
    );
    assert!(
        f.read()
            .await
            .emojis(session, None, 1)
            .await
            .unwrap()
            .definitions[0]
            .retired
    );
}

#[tokio::test]
async fn bounded_pins_and_current_token_revocation_apply_to_reads_and_receipt_status() {
    let f = Fixture::home().await;
    let session = f.session(ACTOR, Permission::Write);
    apply(session.clone(), &create(50)).await;
    let limits = SessionLimits {
        inventory_nodes: 1,
        ..SessionLimits::default()
    };
    assert!(matches!(
        PinnedCollabRead::open(
            f.session_limits(ACTOR, Permission::Read, limits),
            Arc::new(RepositoryReactionActors(REPO))
        )
        .await,
        Err(Error::Limit)
    ));
    let tiny = SessionLimits {
        timeout: std::time::Duration::from_nanos(1),
        ..SessionLimits::default()
    };
    assert!(matches!(
        PinnedCollabRead::open(
            f.session_limits(ACTOR, Permission::Read, tiny),
            Arc::new(RepositoryReactionActors(REPO))
        )
        .await,
        Err(Error::Limit)
    ));
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 8 * 1024 * 1024).unwrap());
    let mut operator = TokenPolicy::new(Scope::Operator);
    let operator_token = operator
        .issue(ACTOR, Permission::CreateRepository, None)
        .unwrap();
    let mut policy = TokenPolicy::new(Scope::Repository(REPO));
    let admin = policy.issue(ACTOR, Permission::Admin, None).unwrap();
    let writer = policy
        .issue(m::PrincipalId::new([4; 16]), Permission::Write, None)
        .unwrap();
    create_repository(
        store.clone(),
        objects.clone(),
        &operator,
        &operator_token,
        &policy,
        EPOCH,
        &SystemClock,
        Limits::default(),
    )
    .await
    .unwrap();
    let make = |token| {
        Arc::new(
            NativeSession::new(
                store.clone(),
                objects.clone(),
                token,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        )
    };
    let admin_session = make(admin);
    let writer_session = make(writer.clone());
    work_home(admin_session.clone()).await;
    let command = create(51);
    apply(writer_session.clone(), &command).await;
    let pin = PinnedCollabRead::open(
        writer_session.clone(),
        Arc::new(RepositoryReactionActors(REPO)),
    )
    .await
    .unwrap();
    let expected = admin_session
        .snapshot()
        .await
        .unwrap()
        .manifest()
        .snapshot();
    admin_session
        .change_token(&jj_store_service::administration::TokenRequest {
            request_id: m::RequestId::new([52; 16]),
            expected,
            change: jj_store_service::administration::TokenChange::Revoke { token: writer.id() },
        })
        .await
        .unwrap();
    assert_eq!(
        pin.threads(writer_session.clone(), None, 10)
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert_eq!(
        CollabCommandService
            .status(writer_session, command.request_id)
            .await
            .unwrap_err(),
        Error::Denied
    );
}

#[tokio::test]
async fn replicated_code_commands_remain_gated_without_changing_authority() {
    let f = Fixture::code().await;
    let session = f.session(ACTOR, Permission::Write);
    let before = session.snapshot().await.unwrap();
    assert!(before.manifest().replication_root.is_some());
    assert!(before.manifest().work_root.is_none());
    assert_eq!(
        CollabCommandService
            .apply(session.clone(), &create(60))
            .await
            .unwrap_err(),
        Error::Unsupported
    );
    assert_eq!(session.snapshot().await.unwrap().digest(), before.digest());
    work_home(session.clone()).await;
    let ledger = session
        .snapshot()
        .await
        .unwrap()
        .manifest()
        .replication_root;
    apply(session.clone(), &create(60)).await;
    assert_eq!(
        session
            .snapshot()
            .await
            .unwrap()
            .manifest()
            .replication_root,
        ledger
    );
}

struct PermitImportedFixture;
#[async_trait::async_trait]
impl jj_store_wal::publication::PublicationGuard for PermitImportedFixture {
    async fn authorize(
        &self,
        _: &jj_store_wal::publication::ManifestSnapshot,
        _: &jj_store_wal::publication::Request,
    ) -> jj_store_wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &jj_store_wal::publication::ManifestSnapshot,
        _: &jj_store_wal::publication::Request,
        _: &jj_store_wal::publication::Snapshot,
    ) -> jj_store_wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(
        &self,
        _: &jj_store_wal::publication::ManifestSnapshot,
        _: &jj_store_wal::publication::DomainRequest,
    ) -> jj_store_wal::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn imported_conflicted_actor_slots_have_no_ordinary_value_or_hidden_count() {
    use jj_store_wal::publication::{Coordinator, Domain, DomainRequest, PublishOutcome};
    let f = Fixture::home().await;
    let session = f.session(ACTOR, Permission::Write);
    let created = thread(apply(session.clone(), &create(70)).await);
    let target = ReactionTargetV1::Thread {
        thread_id: created.thread_id,
    };
    apply(
        session.clone(),
        &react(71, target.clone(), ReactionValueV1::Like),
    )
    .await;
    let c = Coordinator::new(
        f.store.clone(),
        REPO,
        f.objects.clone(),
        Arc::new(PermitImportedFixture),
        Limits::default(),
    )
    .unwrap();
    let pin = c.read().await.unwrap();
    let expected = pin.manifest().domain_state(Domain::Reaction).unwrap();
    let mut store = c.native_reaction_store(expected.root).await.unwrap();
    let state = ReactionStateV1::read(REPO, store.root.clone(), &store.graph).unwrap();
    let key = slot_key(REPO, ACTOR, &target).unwrap();
    let selected = state.slots[&key];
    let CollabObjectV1::ReactionSlot(mut slot) = store.graph.get(selected).unwrap().clone() else {
        panic!()
    };
    let incoming = store
        .graph
        .stage(CollabObjectV1::Reaction(ReactionV1 {
            schema_version: 1,
            id: ReactionId::new([72; 16]),
            repository: REPO,
            actor: ACTOR,
            target: target.clone(),
            value: ReactionValueV1::Dislike,
            origin: ReactionOriginV1::Imported {
                source: "fixture".into(),
                external_id: "conflict".into(),
                original_value: "-".into(),
            },
            accepted_at_millis: 1,
        }))
        .unwrap();
    slot.additions.insert(incoming);
    let reference = store
        .graph
        .stage(CollabObjectV1::ReactionSlot(slot))
        .unwrap();
    store.root.slots = store
        .graph
        .catalog_update(
            store.root.slots,
            std::collections::BTreeMap::from([(key, reference)]),
        )
        .unwrap();
    let root = c
        .stage_native_reaction_root(&store.root, &store.graph)
        .await
        .unwrap();
    assert!(matches!(
        c.publish_domain(
            &DomainRequest::new(
                ACTOR,
                m::RequestId::new([72; 16]),
                m::PublicationId::new([72; 16]),
                Domain::Reaction,
                expected,
                root,
                vec![]
            )
            .unwrap()
        )
        .await
        .unwrap(),
        PublishOutcome::Committed(_)
    ));
    let page = f
        .read()
        .await
        .reactions(session.clone(), &target, None, 10)
        .await
        .unwrap();
    assert_eq!(page.conflict_count, 1);
    assert!(page.reactions.is_empty());
    assert!(page.actors[0].conflict);
    assert!(page.actors[0].value.is_none());
    let hidden = PinnedCollabRead::open(session.clone(), Arc::new(HiddenActor(ACTOR)))
        .await
        .unwrap()
        .reactions(session, &target, None, 10)
        .await
        .unwrap();
    assert_eq!(hidden.conflict_count, 0);
    assert!(hidden.reactions.is_empty());
    assert!(hidden.actors.is_empty());
}

#[tokio::test]
async fn service_deadline_keeps_unknown_request_identity_for_exact_retry() {
    let f = Fixture::home().await;
    let request = create(80);
    let tiny = SessionLimits {
        timeout: std::time::Duration::from_nanos(1),
        ..SessionLimits::default()
    };
    assert!(
        matches!(CollabCommandService.apply(f.session_limits(ACTOR,Permission::Write,tiny),&request).await.unwrap(),Outcome::OutcomeUnknown { request_id } if request_id==request.request_id)
    );
    let normal = f.session(ACTOR, Permission::Write);
    let first = apply(normal.clone(), &request).await;
    assert_eq!(apply(normal.clone(), &request).await, first);
    assert_eq!(
        normal
            .snapshot()
            .await
            .unwrap()
            .manifest()
            .thread_state_version,
        1
    );
}

#[tokio::test]
async fn native_work_comment_reactions_pin_versions_authorize_and_survive_cold_sessions() {
    use jj_store_service::work::{
        ProjectId, TaskId, WorkCommandService, WorkCommandV1, WorkCommentId,
    };
    let f = Fixture::home().await;
    let writer = f.session(ACTOR, Permission::Write);
    let reader = f.session(ACTOR, Permission::Read);
    let task = TaskId::new([90; 16]);
    let comment = WorkCommentId::new([91; 16]);
    for (n, command) in [
        (
            90,
            WorkCommandV1::CreateTask {
                id: task,
                project: ProjectId::new([99; 16]),
                expected_project_version: 1,
                title: "Task".into(),
                body: String::new(),
                repository: None,
                parent: None,
            },
        ),
        (
            91,
            WorkCommandV1::AddComment {
                id: comment,
                task,
                expected_task_version: 1,
                reply_to: None,
                body: "Visible".into(),
            },
        ),
    ] {
        WorkCommandService
            .apply(
                writer.clone(),
                &jj_store_service::work::Request {
                    version: 1,
                    request_id: m::RequestId::new([n; 16]),
                    command,
                },
            )
            .await
            .unwrap();
    }
    let target = ReactionTargetV1::WorkComment {
        task_id: task,
        comment_id: comment,
        version: 1,
    };
    let set = react(92, target.clone(), ReactionValueV1::Like);
    let before = writer.snapshot().await.unwrap();
    assert_eq!(
        CollabCommandService
            .apply(reader.clone(), &set)
            .await
            .unwrap_err(),
        Error::Denied
    );
    let accepted = apply(writer.clone(), &set).await;
    assert_eq!(accepted.actor, ACTOR);
    let after = writer.snapshot().await.unwrap();
    assert_eq!(before.manifest().work_root, after.manifest().work_root);
    assert_eq!(
        before.manifest().work_state_version,
        after.manifest().work_state_version
    );
    let pinned = f.read().await;
    let page = pinned
        .reactions(reader.clone(), &target, None, 20)
        .await
        .unwrap();
    assert_eq!(page.reactions[0].count, 1);
    assert!(page.reactions[0].viewer_reacted);
    assert_eq!(
        page.snapshot.work_root,
        after.manifest().work_root.map(|root| root.digest)
    );
    let cold = f.session(ACTOR, Permission::Write);
    assert_eq!(apply(cold.clone(), &set).await, accepted);
    assert!(matches!(
        CollabCommandService
            .status(cold.clone(), set.request_id)
            .await
            .unwrap(),
        Status::Committed { .. }
    ));
    let wrong = ReactionTargetV1::WorkComment {
        task_id: TaskId::new([89; 16]),
        comment_id: comment,
        version: 1,
    };
    assert_eq!(
        CollabCommandService
            .apply(cold.clone(), &react(93, wrong, ReactionValueV1::Like))
            .await
            .unwrap_err(),
        Error::NotFound
    );
    WorkCommandService
        .apply(
            writer.clone(),
            &jj_store_service::work::Request {
                version: 1,
                request_id: m::RequestId::new([94; 16]),
                command: WorkCommandV1::EditComment {
                    id: comment,
                    expected_version: 1,
                    expected_task_version: 2,
                    body: "Edited".into(),
                },
            },
        )
        .await
        .unwrap();
    // Captured reads still disclose the captured visible version, fresh reads do not.
    assert_eq!(
        pinned
            .reactions(reader.clone(), &target, None, 20)
            .await
            .unwrap()
            .reactions[0]
            .count,
        1
    );
    assert!(matches!(
        f.read()
            .await
            .reactions(reader.clone(), &target, None, 20)
            .await,
        Err(Error::NotFound)
    ));
    let current = ReactionTargetV1::WorkComment {
        task_id: task,
        comment_id: comment,
        version: 2,
    };
    assert!(
        f.read()
            .await
            .reactions(reader.clone(), &current, None, 20)
            .await
            .unwrap()
            .reactions
            .is_empty()
    );
    assert_eq!(
        CollabCommandService
            .apply(writer.clone(), &react(95, target, ReactionValueV1::Like))
            .await
            .unwrap_err(),
        Error::NotFound
    );
    let second = react(
        96,
        current.clone(),
        ReactionValueV1::Emoji {
            sequence: "🎉".into(),
        },
    );
    let result = apply(writer.clone(), &second).await;
    let CollabCommandResultV1::Reaction { result } = result.result else {
        panic!()
    };
    apply(
        writer.clone(),
        &request(
            97,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Clear {
                    target: current.clone(),
                    observed: result.active,
                },
            },
        ),
    )
    .await;
    assert!(
        f.read()
            .await
            .reactions(reader.clone(), &current, None, 20)
            .await
            .unwrap()
            .reactions
            .is_empty()
    );
    WorkCommandService
        .apply(
            writer.clone(),
            &jj_store_service::work::Request {
                version: 1,
                request_id: m::RequestId::new([98; 16]),
                command: WorkCommandV1::TombstoneComment {
                    id: comment,
                    expected_version: 2,
                    expected_task_version: 3,
                },
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        f.read().await.reactions(reader, &current, None, 20).await,
        Err(Error::NotFound)
    ));
}

#[tokio::test]
async fn retained_library_handles_keep_the_session_permission_boundary() {
    let fixture = Fixture::code().await;
    let session = fixture.session(ACTOR, Permission::Read);
    let before = session.snapshot().await.unwrap();
    let stores = session.collaboration_stores().await.unwrap();
    assert_eq!(
        stores.thread().unwrap().snapshot().await.unwrap().version,
        0
    );
    let request = jj_store_core::StoreRequest::new(
        m::RequestId::new([96; 16]),
        ThreadCommandV1::Create {
            thread_id: ThreadId::new([96; 16]),
            title: "Denied library write".into(),
            baseline: None,
        },
    );
    assert!(matches!(
        stores.thread().unwrap().execute(&request).await,
        Err(jj_store_core::Error::Denied)
    ));
    assert!(matches!(
        stores.thread().unwrap().status(request.request_id).await,
        Err(jj_store_core::Error::Denied)
    ));
    assert_eq!(
        session.snapshot().await.unwrap().manifest(),
        before.manifest()
    );
}
