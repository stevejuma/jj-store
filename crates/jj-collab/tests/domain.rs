use jj_collab::*;
use jj_store_model::{PrincipalId, RepositoryId};
use std::collections::{BTreeMap, BTreeSet};

fn slot() -> ReactionSlotV1 {
    ReactionSlotV1::empty(
        RepositoryId::new([1; 16]),
        PrincipalId::new([2; 16]),
        ReactionTargetV1::Thread {
            thread_id: ThreadId::new([3; 16]),
        },
    )
}
fn add(
    base: &ReactionSlotV1,
    value: ReactionValueV1,
    id: u8,
    objects: &mut BTreeMap<CollabObjectRef, CollabObjectV1>,
) -> ReactionSlotV1 {
    let observed = base.resolve(objects).unwrap().active;
    let update = base
        .apply(
            &observed,
            Some(value),
            false,
            (ReactionId::new([id; 16]), RetractionId::new([id; 16])),
            10,
            objects,
        )
        .unwrap();
    objects.extend(update.staged);
    update.slot
}
#[test]
fn exact_observed_replacement_retains_auditable_history_and_rejects_stale_clear() {
    let mut objects = BTreeMap::new();
    let empty = slot();
    let first = add(&empty, ReactionValueV1::Like, 1, &mut objects);
    let observed = first.resolve(&objects).unwrap().active;
    let second = add(
        &first,
        ReactionValueV1::Emoji {
            sequence: "👩🏽‍💻".into(),
        },
        2,
        &mut objects,
    );
    assert_eq!(second.additions.len(), 2);
    assert_eq!(second.retractions.len(), 1);
    assert!(matches!(
        second.apply(
            &observed,
            None,
            false,
            (ReactionId::new([9; 16]), RetractionId::new([9; 16])),
            11,
            &objects
        ),
        Err(Error::Stale)
    ));
    let selected = second.resolve(&objects).unwrap();
    let cleared = second
        .apply(
            &selected.active,
            None,
            false,
            (ReactionId::new([9; 16]), RetractionId::new([9; 16])),
            11,
            &objects,
        )
        .unwrap();
    objects.extend(cleared.staged);
    assert!(cleared.slot.resolve(&objects).unwrap().active.is_empty());
    assert_eq!(cleared.slot.additions.len(), 2);
    assert_eq!(cleared.slot.retractions.len(), 2);
}
#[test]
fn delivery_permutations_preserve_conflicts_and_unseen_concurrent_additions() {
    let mut objects = BTreeMap::new();
    let empty = slot();
    let a = add(&empty, ReactionValueV1::Like, 1, &mut objects);
    let b = add(&empty, ReactionValueV1::Dislike, 2, &mut objects);
    let union = a.merge(&b).unwrap();
    assert_eq!(union, b.merge(&a).unwrap());
    let selected = union.resolve(&objects).unwrap();
    assert!(selected.is_conflict());
    assert_eq!(selected.value, None);
    assert!(
        union
            .apply(
                &selected.active,
                Some(ReactionValueV1::Like),
                false,
                (ReactionId::new([9; 16]), RetractionId::new([9; 16])),
                11,
                &objects
            )
            .is_err()
    );
    let clear = a
        .apply(
            &a.resolve(&objects).unwrap().active,
            None,
            false,
            (ReactionId::new([9; 16]), RetractionId::new([9; 16])),
            11,
            &objects,
        )
        .unwrap();
    objects.extend(clear.staged);
    let merged = clear.slot.merge(&b).unwrap();
    assert_eq!(
        merged.resolve(&objects).unwrap().value,
        Some(ReactionValueV1::Dislike)
    );
    let resolved = union
        .apply(
            &selected.active,
            Some(ReactionValueV1::Emoji {
                sequence: "🚀".into(),
            }),
            true,
            (ReactionId::new([10; 16]), RetractionId::new([10; 16])),
            12,
            &objects,
        )
        .unwrap();
    objects.extend(resolved.staged);
    assert!(!resolved.slot.resolve(&objects).unwrap().is_conflict());
}
#[test]
fn same_value_duplicates_count_once_and_same_value_set_is_a_noop() {
    let mut objects = BTreeMap::new();
    let empty = slot();
    let a = add(&empty, ReactionValueV1::Like, 1, &mut objects);
    let b = add(&empty, ReactionValueV1::Like, 2, &mut objects);
    let union = a.merge(&b).unwrap();
    let selected = union.resolve(&objects).unwrap();
    assert_eq!(selected.active.len(), 2);
    assert_eq!(selected.alternatives.len(), 1);
    assert_eq!(selected.value, Some(ReactionValueV1::Like));
    let same = union
        .apply(
            &selected.active,
            Some(ReactionValueV1::Like),
            false,
            (ReactionId::new([9; 16]), RetractionId::new([9; 16])),
            11,
            &objects,
        )
        .unwrap();
    assert!(same.staged.is_empty());
}
#[test]
fn rich_values_preserve_literal_emoji_and_reject_non_emoji_or_multiple_values() {
    assert_eq!(
        ReactionValueV1::imported("+").unwrap(),
        ReactionValueV1::Like
    );
    assert_eq!(
        ReactionValueV1::imported("-").unwrap(),
        ReactionValueV1::Dislike
    );
    assert_ne!(
        ReactionValueV1::imported("👍").unwrap(),
        ReactionValueV1::Like
    );
    for value in ["❤️", "🏳️‍🌈", "🇰🇪", "1️⃣", "👨‍👩‍👧‍👦"] {
        ReactionValueV1::imported(value).unwrap();
    }
    for value in ["hello", "👍🚀", "", "\u{202e}", ":custom:"] {
        assert!(ReactionValueV1::imported(value).is_err(), "{value}");
    }
}
#[test]
fn message_revision_identity_and_agent_content_are_exact_and_witnesses_are_separate() {
    let target = ReactionTargetV1::HumanMessage {
        thread_id: ThreadId::new([1; 16]),
        message_id: HumanMessageId::new([2; 16]),
        revision: CollabObjectRef {
            kind: CollabObjectKind::HumanMessageRevision,
            id: CollabObjectId::new([3; 32]),
        },
    };
    let mut edited = target.clone();
    if let ReactionTargetV1::HumanMessage { revision, .. } = &mut edited {
        revision.id = CollabObjectId::new([4; 32]);
    }
    assert_ne!(target.exact_key().unwrap(), edited.exact_key().unwrap());
    let agent = ReactionTargetV1::AgentMessage {
        session_id: jj_agent::AgentSessionId::new([1; 16]),
        message_id: jj_agent::MessageId::new([2; 16]),
        event_id: jj_agent::AgentEventId::new([3; 16]),
        batch: jj_agent::AgentObjectRef::new(
            jj_agent::AgentObjectKind::EventBatch,
            jj_agent::AgentObjectId::new([4; 32]),
        ),
        content: jj_agent::AgentObjectRef::new(
            jj_agent::AgentObjectKind::Content,
            jj_agent::AgentObjectId::new([5; 32]),
        ),
    };
    let mut alternate_witness = agent.clone();
    if let ReactionTargetV1::AgentMessage { event_id, .. } = &mut alternate_witness {
        *event_id = jj_agent::AgentEventId::new([6; 16]);
    }
    assert_eq!(
        agent.exact_key().unwrap(),
        alternate_witness.exact_key().unwrap()
    );
}
#[test]
fn thread_lifecycle_is_independent_and_restore_returns_idle() {
    use ThreadActionV1 as A;
    use ThreadStatusV1 as S;
    let mut status = S::Created;
    for action in [
        A::Activate,
        A::Idle,
        A::Activate,
        A::Archive,
        A::Restore,
        A::Tombstone,
        A::Restore,
    ] {
        status = status.transition(action).unwrap();
    }
    assert_eq!(status, S::Idle);
    assert!(S::Created.transition(A::Archive).is_err());
    assert!(S::Archived.transition(A::Activate).is_err());
    assert!(S::Tombstoned.transition(A::Activate).is_err());
}
#[test]
fn canonical_codec_rejects_unknown_fields_whitespace_and_wrong_kinds() {
    let object = CollabObjectV1::ReactionSlot(slot());
    let bytes = encode_object(&object).unwrap();
    assert_eq!(decode_object(&bytes).unwrap(), object);
    let mut trailing = bytes.clone();
    trailing.push(b' ');
    assert!(decode_object(&trailing).is_err());
    let altered = String::from_utf8(bytes)
        .unwrap()
        .replace("\"schema_version\":1", "\"unknown\":0,\"schema_version\":1");
    assert!(decode_object(altered.as_bytes()).is_err());
    let mut invalid = slot();
    invalid.additions = BTreeSet::from([CollabObjectRef {
        kind: CollabObjectKind::ThreadVersion,
        id: CollabObjectId::new([0; 32]),
    }]);
    assert!(invalid.validate().is_err());
    assert!(
        "AB000000000000000000000000000000"
            .parse::<ThreadId>()
            .is_err()
    );
    assert!(
        "00000000000000000000000000000000"
            .parse::<ThreadId>()
            .is_ok()
    );
}
#[test]
fn custom_assets_validate_retained_bytes_and_static_preview() {
    let image = image::RgbaImage::from_pixel(3, 2, image::Rgba([255, 0, 10, 255]));
    let mut png = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image)
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    let (asset, preview) = ImageAssetV1::ingest(&png.into_inner()).unwrap();
    asset.validate().unwrap();
    preview.validate().unwrap();
    assert_eq!((asset.width, asset.height, preview.frames), (3, 2, 1));
    let object = CollabObjectV1::ImageAsset(asset.clone());
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
    let mut wrong = asset;
    wrong.width = 4;
    assert!(wrong.validate().is_err());
    assert!(ImageAssetV1::ingest(b"<svg xmlns='http://www.w3.org/2000/svg'/>").is_err());
    assert!(ImageAssetV1::ingest(&vec![0; MAX_IMAGE_BYTES + 1]).is_err());
}
#[test]
fn animation_bounds_are_enforced_and_preview_is_first_frame() {
    let mut bytes = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut bytes);
        for _ in 0..2 {
            encoder
                .encode_frame(image::Frame::new(image::RgbaImage::from_pixel(
                    2,
                    2,
                    image::Rgba([1, 2, 3, 255]),
                )))
                .unwrap();
        }
    }
    let (source, preview) = ImageAssetV1::ingest(&bytes).unwrap();
    assert_eq!(source.frames, 2);
    assert_eq!(preview.frames, 1);
    assert_eq!(preview.media_type, ImageMediaTypeV1::Png);
    let mut many = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut many);
        for _ in 0..101 {
            encoder
                .encode_frame(image::Frame::new(image::RgbaImage::new(1, 1)))
                .unwrap();
        }
    }
    assert!(ImageAssetV1::ingest(&many).is_err());
    let mut large = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::new(513, 1))
        .write_to(&mut large, image::ImageFormat::Png)
        .unwrap();
    assert!(ImageAssetV1::ingest(&large.into_inner()).is_err());
}
fn run_thread(
    state: &ThreadStateV1,
    command: ThreadCommandV1,
    n: u8,
    graph: &mut CollabGraphV1,
) -> (ThreadStateV1, ThreadCommandResultV1) {
    let (root, result) = state
        .apply(
            &command,
            PrincipalId::new([2; 16]),
            jj_store_model::RequestId::new([n; 16]),
            10 + i64::from(n),
            graph,
        )
        .unwrap();
    (ThreadStateV1::read(root, graph).unwrap(), result)
}
#[test]
fn optional_sessions_reference_many_threads_and_primary_reassignment_keeps_history() {
    let mut graph = CollabGraphV1::default();
    let t1 = ThreadId::new([1; 16]);
    let t2 = ThreadId::new([2; 16]);
    let (state, one) = run_thread(
        &ThreadStateV1::empty(),
        ThreadCommandV1::Create {
            thread_id: t1,
            title: "First".into(),
            baseline: None,
        },
        1,
        &mut graph,
    );
    let (state, two) = run_thread(
        &state,
        ThreadCommandV1::Create {
            thread_id: t2,
            title: "Second".into(),
            baseline: None,
        },
        2,
        &mut graph,
    );
    assert!(state.assignments.is_empty());
    let session_id = jj_agent::AgentSessionId::new([5; 16]);
    let snapshot = jj_agent::AgentObjectRef::new(
        jj_agent::AgentObjectKind::SessionSnapshot,
        jj_agent::AgentObjectId::new([6; 32]),
    );
    let (state, linked) = run_thread(
        &state,
        ThreadCommandV1::Link {
            thread_id: t1,
            expected_thread: one.version,
            link_id: ThreadLinkId::new([3; 16]),
            target: ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                primary: true,
            },
            observed_primary: None,
        },
        3,
        &mut graph,
    );
    let (state, referenced) = run_thread(
        &state,
        ThreadCommandV1::Link {
            thread_id: t2,
            expected_thread: two.version,
            link_id: ThreadLinkId::new([4; 16]),
            target: ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                primary: false,
            },
            observed_primary: None,
        },
        4,
        &mut graph,
    );
    assert_eq!(state.assignments.get(&session_id), linked.link.as_ref());
    let stale = ThreadCommandV1::Link {
        thread_id: t2,
        expected_thread: referenced.version,
        link_id: ThreadLinkId::new([7; 16]),
        target: ThreadLinkTargetV1::Session {
            session_id,
            snapshot,
            primary: true,
        },
        observed_primary: None,
    };
    assert!(matches!(
        state.apply(
            &stale,
            PrincipalId::new([2; 16]),
            jj_store_model::RequestId::new([5; 16]),
            20,
            &mut graph
        ),
        Err(Error::Stale)
    ));
    let (state, reassigned) = run_thread(
        &state,
        ThreadCommandV1::Link {
            thread_id: t2,
            expected_thread: referenced.version,
            link_id: ThreadLinkId::new([7; 16]),
            target: ThreadLinkTargetV1::Session {
                session_id,
                snapshot,
                primary: true,
            },
            observed_primary: linked.link,
        },
        5,
        &mut graph,
    );
    assert_eq!(reassigned.reassigned_from, Some(t1));
    assert_eq!(state.assignments.get(&session_id), reassigned.link.as_ref());
    let CollabObjectV1::ThreadLink(inactive) = graph
        .get(*state.links.get(&ThreadLinkId::new([3; 16])).unwrap())
        .unwrap()
    else {
        panic!()
    };
    assert!(!inactive.active);
    assert_eq!(inactive.previous, linked.link);
    assert!(graph.objects.contains_key(&linked.link.unwrap()));
    let (state, _) = run_thread(
        &state,
        ThreadCommandV1::Detach {
            thread_id: t2,
            expected_thread: reassigned.version,
            expected_link: reassigned.link.unwrap(),
        },
        6,
        &mut graph,
    );
    assert!(state.assignments.is_empty());
}
#[test]
fn human_message_edits_keep_old_revisions_and_archiving_preserves_assignments() {
    let mut graph = CollabGraphV1::default();
    let thread_id = ThreadId::new([1; 16]);
    let message_id = HumanMessageId::new([2; 16]);
    let (state, created) = run_thread(
        &ThreadStateV1::empty(),
        ThreadCommandV1::Create {
            thread_id,
            title: "Conversation".into(),
            baseline: None,
        },
        1,
        &mut graph,
    );
    let (state, first) = run_thread(
        &state,
        ThreadCommandV1::Message {
            thread_id,
            expected_thread: created.version,
            message_id,
            expected_revision: None,
            body: Some("Before an agent starts".into()),
        },
        2,
        &mut graph,
    );
    assert!(state.assignments.is_empty());
    let (state, edited) = run_thread(
        &state,
        ThreadCommandV1::Message {
            thread_id,
            expected_thread: first.version,
            message_id,
            expected_revision: first.message,
            body: Some("Edited human message".into()),
        },
        3,
        &mut graph,
    );
    assert_ne!(first.message, edited.message);
    assert!(
        graph
            .closure(state.root.dependencies())
            .unwrap()
            .contains(&first.message.unwrap())
    );
    let (state, active) = run_thread(
        &state,
        ThreadCommandV1::Lifecycle {
            thread_id,
            expected: edited.version,
            action: ThreadActionV1::Activate,
        },
        4,
        &mut graph,
    );
    let (state, archived) = run_thread(
        &state,
        ThreadCommandV1::Lifecycle {
            thread_id,
            expected: active.version,
            action: ThreadActionV1::Archive,
        },
        5,
        &mut graph,
    );
    let next = ThreadCommandV1::Message {
        thread_id,
        expected_thread: archived.version,
        message_id: HumanMessageId::new([3; 16]),
        expected_revision: None,
        body: Some("Late human edit".into()),
    };
    assert!(
        state
            .apply(
                &next,
                PrincipalId::new([2; 16]),
                jj_store_model::RequestId::new([6; 16]),
                30,
                &mut graph
            )
            .is_err()
    );
    let (state, restored) = run_thread(
        &state,
        ThreadCommandV1::Lifecycle {
            thread_id,
            expected: archived.version,
            action: ThreadActionV1::Restore,
        },
        6,
        &mut graph,
    );
    let CollabObjectV1::ThreadVersion(v) = graph.get(restored.version).unwrap() else {
        panic!()
    };
    assert_eq!(v.status, ThreadStatusV1::Idle);
    assert_eq!(state.messages.get(&message_id), edited.message.as_ref());
}
#[test]
fn bounded_radix_updates_preserve_prior_snapshots_and_validate_keys() {
    let mut graph = CollabGraphV1::default();
    let mut root = None;
    let mut expected = BTreeMap::new();
    for n in 0u16..130 {
        let mut id = [0; 16];
        id[..2].copy_from_slice(&n.to_be_bytes());
        let object = CollabObjectV1::HumanMessageRevision(HumanMessageRevisionV1 {
            schema_version: 1,
            message_id: HumanMessageId::new(id),
            thread_id: ThreadId::new([1; 16]),
            actor: PrincipalId::new([2; 16]),
            accepted_at_millis: 1,
            previous: None,
            body: Some(format!("{n}")),
        });
        let reference = graph.stage(object).unwrap();
        let key = entity_key("message", &id).unwrap();
        let before = root;
        let old = graph.catalog(before).unwrap();
        root = graph
            .catalog_update(root, BTreeMap::from([(key, reference)]))
            .unwrap();
        expected.insert(key, reference);
        assert_eq!(graph.catalog(before).unwrap(), old);
    }
    assert_eq!(graph.catalog(root).unwrap(), expected);
    graph.closure(root).unwrap();
    let forged = CollabObjectV1::IndexPage(CollabIndexPageV1::Leaf {
        prefix: "a".into(),
        entries: BTreeMap::new(),
    });
    assert!(graph.stage(forged).is_err());
}
#[test]
fn conflicting_reuse_of_stable_reaction_identity_is_rejected() {
    let mut objects = BTreeMap::new();
    let empty = slot();
    let a = add(&empty, ReactionValueV1::Like, 1, &mut objects);
    let b = add(&empty, ReactionValueV1::Dislike, 1, &mut objects);
    assert!(a.merge(&b).unwrap().resolve(&objects).is_err());
}
