use jj_collab::*;
use jj_store_model::{EpochId, ObjectKey, PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{
        Coordinator, Domain, DomainRequest, DomainState, Limits, ManifestSnapshot,
        PublicationGuard, PublishOutcome, Request, Snapshot,
    },
};
use std::{collections::BTreeMap, sync::Arc};
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
            dependencies: Default::default(),
        })
    }
}
struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize_replication_seed(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::SeedReplicationRequestV1,
    ) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
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
fn coordinator(store: Arc<MemoryStore>) -> Coordinator {
    Coordinator::new(
        store,
        REPO,
        Arc::new(Source),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap()
}
async fn fixture() -> (Arc<MemoryStore>, Coordinator) {
    let store = Arc::new(MemoryStore::default());
    let c = coordinator(store.clone());
    c.initialize(EpochId::new([3; 16]), Vec::new())
        .await
        .unwrap();
    (store, c)
}
async fn publish(
    c: &Coordinator,
    n: u8,
    domain: Domain,
    expected: DomainState,
    root: wal::records::RecordRef,
) -> wal::publication::Receipt {
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([n; 16]),
        PublicationId::new([n; 16]),
        domain,
        expected,
        root,
        vec![n],
    )
    .unwrap();
    let PublishOutcome::Committed(receipt) = c.publish_domain(&request).await.unwrap() else {
        panic!()
    };
    receipt
}
async fn thread(
    c: &Coordinator,
) -> (
    DomainState,
    wal::records::RecordRef,
    CollabGraphV1,
    ThreadRootV1,
    ThreadCommandResultV1,
) {
    let mut graph = CollabGraphV1::default();
    let empty = c
        .stage_native_thread_root(&ThreadRootV1::empty(), &graph)
        .await
        .unwrap();
    let (root, result) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: ThreadId::new([4; 16]),
                title: "Persistent work".into(),
                baseline: None,
            },
            ACTOR,
            RequestId::new([4; 16]),
            10,
            &mut graph,
        )
        .unwrap();
    let reference = c.stage_native_thread_root(&root, &graph).await.unwrap();
    (
        DomainState {
            root: empty,
            version: 0,
        },
        reference,
        graph,
        root,
        result,
    )
}
#[tokio::test]
async fn explicit_feature_use_publishes_format_five_without_backfill_or_code_changes() {
    let (_, c) = fixture().await;
    let before = c.read().await.unwrap();
    let (expected, root, _, _, _) = thread(&c).await;
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
    assert!(before.manifest().thread_root.is_none());
    assert!(before.manifest().reaction_root.is_none());
    publish(&c, 4, Domain::Thread, expected, root).await;
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().format, 5);
    assert_eq!(after.manifest().thread_state_version, 1);
    assert_eq!(
        after.manifest().code_state_version,
        before.manifest().code_state_version
    );
    assert_eq!(after.manifest().agent_root, before.manifest().agent_root);
    assert_eq!(after.manifest().review_root, before.manifest().review_root);
    assert!(after.manifest().reaction_root.is_none());
    assert!(
        c.native_thread_store(root)
            .await
            .unwrap()
            .state
            .assignments
            .is_empty()
    );
    c.recover(100).await.unwrap();
}
#[tokio::test]
async fn cold_receipts_checkpoint_and_manifest_last_copy_retain_human_revision_reactions() {
    let (store, c) = fixture().await;
    let (expected, reference, mut graph, thread_root, created) = thread(&c).await;
    publish(&c, 4, Domain::Thread, expected, reference).await;
    let state = ThreadStateV1::read(thread_root, &graph).unwrap();
    let (thread_root, message) = state
        .apply(
            &ThreadCommandV1::Message {
                thread_id: created.thread_id,
                expected_thread: created.version,
                message_id: HumanMessageId::new([5; 16]),
                expected_revision: None,
                body: Some("Exact retained revision".into()),
            },
            ACTOR,
            RequestId::new([5; 16]),
            11,
            &mut graph,
        )
        .unwrap();
    let thread_reference = c
        .stage_native_thread_root(&thread_root, &graph)
        .await
        .unwrap();
    publish(
        &c,
        5,
        Domain::Thread,
        DomainState {
            root: reference,
            version: 1,
        },
        thread_reference,
    )
    .await;
    let target = ReactionTargetV1::HumanMessage {
        thread_id: created.thread_id,
        message_id: HumanMessageId::new([5; 16]),
        revision: message.message.unwrap(),
    };
    let slot = ReactionSlotV1::empty(REPO, ACTOR, target);
    let update = slot
        .apply(
            &Default::default(),
            Some(ReactionValueV1::Emoji {
                sequence: "👀".into(),
            }),
            false,
            (ReactionId::new([6; 16]), RetractionId::new([6; 16])),
            12,
            &graph.objects,
        )
        .unwrap();
    graph.objects.extend(update.staged);
    let slot = graph
        .stage(CollabObjectV1::ReactionSlot(update.slot.clone()))
        .unwrap();
    let mut reaction_root = ReactionRootV1::empty();
    reaction_root.slots = graph
        .catalog_update(None, BTreeMap::from([(update.slot.key().unwrap(), slot)]))
        .unwrap();
    let empty = c
        .stage_native_reaction_root(&ReactionRootV1::empty(), &CollabGraphV1::default())
        .await
        .unwrap();
    let root = c
        .stage_native_reaction_root(&reaction_root, &graph)
        .await
        .unwrap();
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([6; 16]),
        PublicationId::new([6; 16]),
        Domain::Reaction,
        DomainState {
            root: empty,
            version: 0,
        },
        root,
        vec![6],
    )
    .unwrap();
    let outcome = c.publish_domain(&request).await.unwrap();
    c.checkpoint().await.unwrap();
    let cold = coordinator(store);
    cold.recover(100).await.unwrap();
    assert_eq!(cold.publish_domain(&request).await.unwrap(), outcome);
    let destination = Arc::new(MemoryStore::default());
    let copied = cold
        .copy_native_to(destination.clone(), Arc::new(Source), 100)
        .await
        .unwrap();
    assert_eq!(
        copied.snapshot.manifest().thread_root,
        Some(thread_reference)
    );
    assert_eq!(copied.snapshot.manifest().reaction_root, Some(root));
    let restored = coordinator(destination);
    let pinned = restored.native_reaction_store(root).await.unwrap();
    assert!(pinned.graph.objects.contains_key(&message.message.unwrap()));
    let CollabObjectV1::ReactionSlot(slot) = pinned.graph.get(slot).unwrap() else {
        panic!()
    };
    assert_eq!(
        slot.resolve(&pinned.graph.objects).unwrap().value,
        Some(ReactionValueV1::Emoji {
            sequence: "👀".into()
        })
    );
}
#[tokio::test]
async fn custom_emoji_assets_and_static_previews_survive_native_copy() {
    let (_, c) = fixture().await;
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        2,
        2,
        image::Rgba([1, 2, 3, 255]),
    ))
    .write_to(&mut bytes, image::ImageFormat::Png)
    .unwrap();
    let (asset, preview) = ImageAssetV1::ingest(&bytes.into_inner()).unwrap();
    let mut graph = CollabGraphV1::default();
    let asset = graph.stage(CollabObjectV1::ImageAsset(asset)).unwrap();
    let preview = graph.stage(CollabObjectV1::ImageAsset(preview)).unwrap();
    let emoji_id = CustomEmojiId::new([9; 16]);
    let definition = graph
        .stage(CollabObjectV1::CustomEmojiVersion(CustomEmojiVersionV1 {
            schema_version: 1,
            emoji_id,
            shortcode: "ship_it".into(),
            label: "Ship it".into(),
            actor: ACTOR,
            accepted_at_millis: 1,
            previous: None,
            asset,
            preview,
            retired: false,
        }))
        .unwrap();
    let mut root = ReactionRootV1::empty();
    root.emojis = graph
        .catalog_update(
            None,
            BTreeMap::from([(
                entity_key("emoji", emoji_id.as_bytes()).unwrap(),
                definition,
            )]),
        )
        .unwrap();
    let empty = c
        .stage_native_reaction_root(&ReactionRootV1::empty(), &CollabGraphV1::default())
        .await
        .unwrap();
    let reference = c.stage_native_reaction_root(&root, &graph).await.unwrap();
    publish(
        &c,
        9,
        Domain::Reaction,
        DomainState {
            root: empty,
            version: 0,
        },
        reference,
    )
    .await;
    let destination = Arc::new(MemoryStore::default());
    c.copy_native_to(destination.clone(), Arc::new(Source), 100)
        .await
        .unwrap();
    let cold = coordinator(destination);
    let restored = cold.native_reaction_store(reference).await.unwrap();
    assert_eq!(
        restored.graph.get(asset).unwrap(),
        graph.get(asset).unwrap()
    );
    assert_eq!(
        restored.graph.get(preview).unwrap(),
        graph.get(preview).unwrap()
    );
}
#[tokio::test]
async fn atomic_domain_selection_replays_one_receipt_and_never_partially_accepts_stale_roots() {
    let (_, c) = fixture().await;
    let (expected, thread_root, _, _, _) = thread(&c).await;
    let reaction_empty = c
        .stage_native_reaction_root(&ReactionRootV1::empty(), &CollabGraphV1::default())
        .await
        .unwrap();
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([30; 16]),
        PublicationId::new([30; 16]),
        Domain::Thread,
        expected,
        thread_root,
        vec![30],
    )
    .unwrap()
    .with_additional(wal::publication::DomainSelectionV1 {
        domain: Domain::Reaction,
        expected: DomainState {
            root: reaction_empty,
            version: 0,
        },
        root: reaction_empty,
    })
    .unwrap();
    let outcome = c.publish_domain(&request).await.unwrap();
    assert!(matches!(outcome, PublishOutcome::Committed(_)));
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().published.0, 1);
    assert_eq!(after.manifest().thread_state_version, 1);
    assert_eq!(after.manifest().reaction_state_version, 1);
    assert_eq!(c.publish_domain(&request).await.unwrap(), outcome);
    let mut stale = request;
    stale.request_id = RequestId::new([31; 16]);
    stale.publication_id = PublicationId::new([31; 16]);
    assert!(matches!(
        c.publish_domain(&stale).await,
        Err(wal::Error::Stale)
    ));
    assert_eq!(c.read().await.unwrap().digest(), after.digest());
    c.checkpoint().await.unwrap();
    c.recover(100).await.unwrap();
}
struct DenyReaction;
#[async_trait::async_trait]
impl PublicationGuard for DenyReaction {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(
        &self,
        _: &ManifestSnapshot,
        request: &DomainRequest,
    ) -> wal::Result<()> {
        if request.domain == Domain::Reaction {
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
#[tokio::test]
async fn each_atomic_domain_requires_authorization_before_any_root_is_selected() {
    let store = Arc::new(MemoryStore::default());
    let c = Coordinator::new(
        store,
        REPO,
        Arc::new(Source),
        Arc::new(DenyReaction),
        Limits::default(),
    )
    .unwrap();
    c.initialize(EpochId::new([3; 16]), Vec::new())
        .await
        .unwrap();
    let before = c.read().await.unwrap();
    let (expected, thread_root, _, _, _) = thread(&c).await;
    let reaction = c
        .stage_native_reaction_root(&ReactionRootV1::empty(), &CollabGraphV1::default())
        .await
        .unwrap();
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([40; 16]),
        PublicationId::new([40; 16]),
        Domain::Thread,
        expected,
        thread_root,
        vec![40],
    )
    .unwrap()
    .with_additional(wal::publication::DomainSelectionV1 {
        domain: Domain::Reaction,
        expected: DomainState {
            root: reaction,
            version: 0,
        },
        root: reaction,
    })
    .unwrap();
    assert!(matches!(
        c.publish_domain(&request).await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
}
#[tokio::test]
async fn legacy_manifest_and_domain_request_encoding_omit_all_additive_fields() {
    let (_, c) = fixture().await;
    let before = c.read().await.unwrap();
    let bytes = serde_json::to_vec(before.manifest()).unwrap();
    let encoded = std::str::from_utf8(&bytes).unwrap();
    for field in [
        "thread_root",
        "reaction_root",
        "thread_state_version",
        "reaction_state_version",
    ] {
        assert!(!encoded.contains(field));
    }
    let decoded: wal::publication::Manifest = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
    let state = before.manifest().domain_state(Domain::Agent).unwrap();
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([41; 16]),
        PublicationId::new([41; 16]),
        Domain::Agent,
        state,
        state.root,
        Vec::new(),
    )
    .unwrap();
    let bytes = serde_json::to_vec(&request).unwrap();
    assert!(!std::str::from_utf8(&bytes).unwrap().contains("additional"));
    assert_eq!(
        serde_json::to_vec(&serde_json::from_slice::<DomainRequest>(&bytes).unwrap()).unwrap(),
        bytes
    );
}
#[tokio::test]
async fn work_homes_keep_origin_ledger_frozen_while_accepting_local_threads() {
    let (_, c) = fixture().await;
    c.seed_replication(&wal::publication::SeedReplicationRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: RequestId::new([50; 16]),
        publication_id: PublicationId::new([50; 16]),
        replica: jj_store_model::ReplicaId::new([50; 16]),
        writer_epoch: jj_store_model::WriterEpoch::new([50; 16]),
    })
    .await
    .unwrap();
    let origin = c.read().await.unwrap().manifest().replication_root;
    let request = wal::publication::WorkCommandRequest::new(
        ACTOR,
        RequestId::new([51; 16]),
        PublicationId::new([51; 16]),
        wal::publication::WorkCommandV1::CreateProject {
            id: wal::publication::ProjectId::new([51; 16]),
            title: "Home".into(),
            body: "Local work home".into(),
        },
    )
    .unwrap();
    assert!(matches!(
        c.publish_work_command(&request).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    let (expected, root, _, _, _) = thread(&c).await;
    publish(&c, 52, Domain::Thread, expected, root).await;
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().replication_root, origin);
    assert_eq!(after.manifest().format, 5);
    c.recover(100).await.unwrap();
}
#[tokio::test]
async fn replicated_code_repositories_reject_unqualified_collaboration_without_changing_authority()
{
    let (_, c) = fixture().await;
    c.seed_replication(&wal::publication::SeedReplicationRequestV1 {
        schema_version: 1,
        principal: ACTOR,
        request_id: RequestId::new([60; 16]),
        publication_id: PublicationId::new([60; 16]),
        replica: jj_store_model::ReplicaId::new([60; 16]),
        writer_epoch: jj_store_model::WriterEpoch::new([60; 16]),
    })
    .await
    .unwrap();
    let before = c.read().await.unwrap();
    let (expected, root, _, _, _) = thread(&c).await;
    let request = DomainRequest::new(
        ACTOR,
        RequestId::new([61; 16]),
        PublicationId::new([61; 16]),
        Domain::Thread,
        expected,
        root,
        Vec::new(),
    )
    .unwrap();
    assert!(matches!(
        c.publish_domain(&request).await,
        Err(wal::Error::Unsupported(_))
    ));
    assert_eq!(c.read().await.unwrap().digest(), before.digest());
}
#[tokio::test]
async fn existing_domain_publication_keeps_format_five_and_thread_roots_pinned() {
    let (_, c) = fixture().await;
    let (expected, root, _, _, _) = thread(&c).await;
    publish(&c, 70, Domain::Thread, expected, root).await;
    let before = c.read().await.unwrap();
    let agent = before.manifest().domain_state(Domain::Agent).unwrap();
    publish(&c, 71, Domain::Agent, agent, agent.root).await;
    let after = c.read().await.unwrap();
    assert_eq!(after.manifest().format, 5);
    assert_eq!(after.manifest().thread_root, Some(root));
    assert_eq!(after.manifest().thread_state_version, 1);
    assert_eq!(after.manifest().agent_state_version, 1);
    assert_eq!(
        after.manifest().code_state_version,
        before.manifest().code_state_version
    );
    c.recover(100).await.unwrap();
}
