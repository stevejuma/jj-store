use jj_collab::*;
use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore};
use jj_store_model as m;
use jj_store_service::local_repository::*;
use std::{collections::BTreeSet, path::Path, process::Command};
#[path = "local_collab/agent_context.rs"]
mod agent_context;
#[path = "local_collab/agent_discussion.rs"]
mod agent_discussion;

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("repo");
    let result = Command::new("/opt/homebrew/bin/jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(&path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    (root, reopen(&path))
}
fn reopen(path: &Path) -> LocalRepository {
    LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    )
}
fn request(id: u8, command: LocalCollabCommand) -> LocalCollabRequest {
    LocalCollabRequest {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    }
}
fn thread(id: u8) -> LocalCollabRequest {
    request(
        id,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Create {
                thread_id: ThreadId::new([id; 16]),
                title: format!("Thread {id}"),
                baseline: None,
            },
        },
    )
}
fn receipt(outcome: LocalCollabOutcome) -> LocalCollabReceipt {
    let LocalCollabOutcome::Committed { receipt } = outcome else {
        panic!("expected receipt")
    };
    *receipt
}
fn thread_result(receipt: LocalCollabReceipt) -> ThreadCommandResultV1 {
    let LocalCollabResultV1::Thread { result } = receipt.result else {
        panic!("expected Thread")
    };
    result
}
fn reaction_result(receipt: LocalCollabReceipt) -> ReactionCommandResultV1 {
    let LocalCollabResultV1::Reaction { result } = receipt.result else {
        panic!("expected Reaction")
    };
    result
}
fn react(
    id: u8,
    target: ReactionTargetV1,
    value: ReactionValueV1,
    observed: BTreeSet<CollabObjectRef>,
) -> LocalCollabRequest {
    request(
        id,
        LocalCollabCommand::Reaction {
            intent: ReactionCommandV1::Set {
                target,
                value,
                observed,
            },
        },
    )
}
fn gc(path: &Path) {
    let result = Command::new("git")
        .arg("--git-dir")
        .arg(path.join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}
fn png(color: u8) -> Vec<u8> {
    let mut output = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        2,
        2,
        image::Rgba([color, 0, 0, 255]),
    ))
    .write_to(&mut output, image::ImageFormat::Png)
    .unwrap();
    output.into_inner()
}

#[tokio::test]
async fn local_captured_sessions_remain_optional_and_tool_targets_keep_native_identity() {
    use jj_store_wal::publication::*;
    let (root, repo) = fixture();
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(i, value)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1_725_000_000_000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&value).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    };
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([70; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch),
        },
    })
    .await
    .unwrap();
    assert!(repo.thread_store().await.unwrap().state.threads.is_empty());
    let agents = repo.agent_store().await.unwrap();
    let sessions = agents.reads().list_sessions(None, 20).await.unwrap();
    let selected = sessions.selections[0].1;
    let AgentObjectV1::SessionSnapshot(session) = agents.object(selected).await.unwrap() else {
        panic!()
    };
    let created = thread_result(receipt(
        repo.apply_collab_command(&thread(1)).await.unwrap(),
    ));
    assert!(
        repo.thread_store()
            .await
            .unwrap()
            .state
            .assignments
            .is_empty()
    );
    let link = request(
        2,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Link {
                thread_id: created.thread_id,
                expected_thread: created.version,
                link_id: ThreadLinkId::new([2; 16]),
                target: ThreadLinkTargetV1::Session {
                    session_id: session.session_id,
                    snapshot: selected,
                    primary: true,
                },
                observed_primary: None,
            },
        },
    );
    let linked = thread_result(receipt(repo.apply_collab_command(&link).await.unwrap()));
    assert_eq!(
        repo.thread_store()
            .await
            .unwrap()
            .state
            .assignments
            .get(&session.session_id),
        linked.link.as_ref()
    );
    let (tool_id, snapshot) = session.tools.first_key_value().unwrap();
    let AgentObjectV1::ToolSnapshot(tool) = agents.object(*snapshot).await.unwrap() else {
        panic!()
    };
    let content = tool.output.unwrap();
    let target = ReactionTargetV1::ToolOutput {
        session_id: session.session_id,
        tool_id: *tool_id,
        snapshot: *snapshot,
        content,
    };
    repo.apply_collab_command(&react(
        3,
        target.clone(),
        ReactionValueV1::Emoji {
            sequence: "👀".into(),
        },
        BTreeSet::new(),
    ))
    .await
    .unwrap();
    let path = root.path().join("repo");
    gc(&path);
    let restarted = reopen(&path);
    assert_eq!(
        restarted
            .reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows
            .len(),
        1
    );
    assert!(matches!(
        restarted
            .thread_store()
            .await
            .unwrap()
            .object(linked.link.unwrap())
            .unwrap(),
        CollabObjectV1::ThreadLink(_)
    ));
    let bad = ReactionTargetV1::ToolOutput {
        session_id: session.session_id,
        tool_id: *tool_id,
        snapshot: *snapshot,
        content: tool.input.unwrap(),
    };
    assert_eq!(
        restarted
            .reaction_query(&bad, None, 20, None)
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
}

#[tokio::test]
async fn local_thread_reaction_reads_do_not_initialize_metadata_or_actor() {
    let (root, repo) = fixture();
    let threads = repo.thread_store().await.unwrap();
    let reactions = repo.reaction_store().await.unwrap();
    assert!(threads.state.threads.is_empty());
    assert!(threads.state.assignments.is_empty());
    assert!(threads.snapshot.repo_id.is_none());
    assert!(reactions.snapshot.repo_id.is_none());
    assert!(matches!(
        repo.collab_command_status(m::RequestId::new([1; 16]))
            .unwrap(),
        LocalCollabOutcome::NotKnown { .. }
    ));
    assert_eq!(
        repo.reaction_query(
            &ReactionTargetV1::Thread {
                thread_id: ThreadId::new([1; 16])
            },
            None,
            20,
            None
        )
        .await
        .unwrap_err(),
        LocalError::NotFound
    );
    assert!(repo.metadata_store().read().unwrap().is_none());
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
}

#[tokio::test]
async fn selected_descriptor_cannot_read_unretained_staged_blobs() {
    use jj_store_jj::local_metadata::{Preparation, PublishOutcome, Update};
    let (_root, repo) = fixture();
    let created = receipt(repo.apply_collab_command(&thread(1)).await.unwrap());
    let pin = repo.metadata_store().read().unwrap().unwrap();
    let bytes = repo
        .metadata_store()
        .read_domain(&pin, Domain::Thread)
        .unwrap()
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let original = value["objects"][0]["blob"].as_str().unwrap();
    let staged = repo
        .metadata_store()
        .stage_blob(b"unselected fixture bytes")
        .unwrap();
    let bytes = String::from_utf8(bytes)
        .unwrap()
        .replace(original, &staged.to_string())
        .into_bytes();
    let update = Update {
        repository: pin.repository(),
        actor: created.actor,
        request: m::RequestId::new([99; 16]),
        intent: m::IntentDigest::new([99; 32]),
        domain: Domain::Thread,
        expected: pin.selection(Domain::Thread).cloned(),
        bytes,
        result: b"corrupt fixture".to_vec(),
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = repo.metadata_store().prepare(Some(&pin), &update).unwrap()
    else {
        panic!()
    };
    assert!(matches!(
        repo.metadata_store().publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert!(matches!(
        repo.thread_store().await,
        Err(LocalError::Corrupt)
    ));
}

#[tokio::test]
async fn local_commands_keep_exact_history_preserve_siblings_and_replay_after_gc() {
    use jj_store_service::work::*;
    let (root, repo) = fixture();
    repo.apply_work_command(&Request {
        version: 1,
        request_id: m::RequestId::new([70; 16]),
        command: WorkCommandV1::CreateProject {
            id: ProjectId::new([70; 16]),
            title: "Project".into(),
            body: String::new(),
        },
    })
    .unwrap();
    let legacy = repo.metadata_store().read().unwrap().unwrap();
    assert_eq!(legacy.format_version(), 1);
    let create = thread(1);
    let created = receipt(repo.apply_collab_command(&create).await.unwrap());
    let created_result = thread_result(created.clone());
    let pin = repo.metadata_store().read().unwrap().unwrap();
    assert_eq!(pin.format_version(), 2);
    assert_eq!(pin.selection(Domain::Work), legacy.selection(Domain::Work));
    assert!(pin.selection(Domain::Reaction).is_none());
    let message = request(
        2,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Message {
                reply_to: None,
                thread_id: created_result.thread_id,
                expected_thread: created_result.version,
                message_id: HumanMessageId::new([2; 16]),
                expected_revision: None,
                body: Some("before".into()),
            },
        },
    );
    let first = thread_result(receipt(repo.apply_collab_command(&message).await.unwrap()));
    let first_store = repo.thread_store().await.unwrap();
    let target = ReactionTargetV1::HumanMessage {
        thread_id: first.thread_id,
        message_id: HumanMessageId::new([2; 16]),
        revision: first.message.unwrap(),
    };
    let set = react(
        3,
        target.clone(),
        ReactionValueV1::Emoji {
            sequence: "👩🏽‍💻".into(),
        },
        BTreeSet::new(),
    );
    let set_receipt = receipt(repo.apply_collab_command(&set).await.unwrap());
    let reaction = reaction_result(set_receipt.clone());
    let page = repo.reaction_query(&target, None, 20, None).await.unwrap();
    assert_eq!(page.rows.len(), 1);
    assert_eq!(page.rows[0].actor, set_receipt.actor);
    let edited = request(
        4,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Message {
                reply_to: None,
                thread_id: first.thread_id,
                expected_thread: first.version,
                message_id: HumanMessageId::new([2; 16]),
                expected_revision: first.message,
                body: Some("after".into()),
            },
        },
    );
    let edit = thread_result(receipt(repo.apply_collab_command(&edited).await.unwrap()));
    assert_eq!(
        first_store.object(edit.message.unwrap()).unwrap_err(),
        LocalError::Stale
    );
    let current = ReactionTargetV1::HumanMessage {
        thread_id: edit.thread_id,
        message_id: HumanMessageId::new([2; 16]),
        revision: edit.message.unwrap(),
    };
    assert!(
        repo.reaction_query(&current, None, 20, None)
            .await
            .unwrap()
            .rows
            .is_empty()
    );
    assert_eq!(
        repo.reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .revision,
        TargetRevisionV1::Earlier
    );
    assert_eq!(
        repo.reaction_query(&target, None, 20, page.snapshot.metadata_commit)
            .await
            .unwrap_err(),
        LocalError::Stale
    );
    let replace = react(
        5,
        target.clone(),
        ReactionValueV1::Like,
        reaction.active.clone(),
    );
    let replaced = reaction_result(receipt(repo.apply_collab_command(&replace).await.unwrap()));
    assert_ne!(replaced.active, reaction.active);
    let stale = request(
        6,
        LocalCollabCommand::Reaction {
            intent: ReactionCommandV1::Clear {
                target: target.clone(),
                observed: reaction.active,
            },
        },
    );
    assert_eq!(
        repo.apply_collab_command(&stale).await.unwrap_err(),
        LocalError::Stale
    );
    let path = root.path().join("repo");
    gc(&path);
    let restarted = reopen(&path);
    assert_eq!(
        receipt(restarted.apply_collab_command(&create).await.unwrap()),
        created
    );
    assert_eq!(
        receipt(restarted.apply_collab_command(&set).await.unwrap()),
        set_receipt
    );
    assert_eq!(
        receipt(restarted.collab_command_status(set.request_id).unwrap()),
        set_receipt
    );
    assert_eq!(
        restarted
            .reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows[0]
            .value,
        Some(ReactionValueV1::Like)
    );
    assert_eq!(
        restarted
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .selection(Domain::Work),
        legacy.selection(Domain::Work)
    );
    assert!(
        restarted
            .thread_store()
            .await
            .unwrap()
            .state
            .assignments
            .is_empty()
    );
    let mut altered = create;
    altered.command = LocalCollabCommand::Thread {
        intent: ThreadCommandV1::Create {
            thread_id: ThreadId::new([1; 16]),
            title: "altered".into(),
            baseline: None,
        },
    };
    assert_eq!(
        restarted.apply_collab_command(&altered).await.unwrap_err(),
        LocalError::IntentConflict
    );
}

#[tokio::test]
async fn local_custom_library_pins_assets_retains_old_versions_and_rejects_stale_sets() {
    let (root, repo) = fixture();
    let created = thread_result(receipt(
        repo.apply_collab_command(&thread(1)).await.unwrap(),
    ));
    let id = CustomEmojiId::new([9; 16]);
    let source = png(20);
    let define = request(
        2,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Define {
                emoji_id: id,
                expected: None,
                shortcode: "ship_it".into(),
                label: "Ship it".into(),
                bytes: source.clone(),
            },
        },
    );
    let LocalCollabResultV1::Emoji { version } =
        receipt(repo.apply_collab_command(&define).await.unwrap()).result
    else {
        panic!()
    };
    let target = ReactionTargetV1::Thread {
        thread_id: created.thread_id,
    };
    let set = react(
        3,
        target.clone(),
        ReactionValueV1::Custom {
            emoji_id: id,
            version,
        },
        BTreeSet::new(),
    );
    let original = repo.apply_collab_command(&set).await.unwrap();
    let duplicate = request(
        4,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Define {
                emoji_id: CustomEmojiId::new([10; 16]),
                expected: None,
                shortcode: "ship_it".into(),
                label: "duplicate".into(),
                bytes: png(30),
            },
        },
    );
    let before = repo.metadata_store().read().unwrap().unwrap().commit();
    assert_eq!(
        repo.apply_collab_command(&duplicate).await.unwrap_err(),
        LocalError::Invalid
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before
    );
    let update = request(
        5,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Define {
                emoji_id: id,
                expected: Some(version),
                shortcode: "ship_it".into(),
                label: "Updated".into(),
                bytes: png(40),
            },
        },
    );
    let LocalCollabResultV1::Emoji { version: updated } =
        receipt(repo.apply_collab_command(&update).await.unwrap()).result
    else {
        panic!()
    };
    assert_eq!(
        repo.reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows[0]
            .value,
        Some(ReactionValueV1::Custom {
            emoji_id: id,
            version
        })
    );
    let page = repo.reaction_query(&target, None, 20, None).await.unwrap();
    let stale = react(
        6,
        target.clone(),
        ReactionValueV1::Custom {
            emoji_id: id,
            version,
        },
        page.rows[0].active.clone(),
    );
    assert_eq!(
        repo.apply_collab_command(&stale).await.unwrap_err(),
        LocalError::Stale
    );
    let retire = request(
        7,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Retire {
                emoji_id: id,
                expected: updated,
            },
        },
    );
    let LocalCollabResultV1::Emoji { version: retired } =
        receipt(repo.apply_collab_command(&retire).await.unwrap()).result
    else {
        panic!()
    };
    let observed = repo
        .reaction_query(&target, None, 20, None)
        .await
        .unwrap()
        .rows[0]
        .active
        .clone();
    let new = react(
        9,
        target.clone(),
        ReactionValueV1::Custom {
            emoji_id: id,
            version: retired,
        },
        observed,
    );
    assert_eq!(
        repo.apply_collab_command(&new).await.unwrap_err(),
        LocalError::Invalid
    );
    let path = root.path().join("repo");
    gc(&path);
    let restarted = reopen(&path);
    let store = restarted.reaction_store().await.unwrap();
    assert!(store.emojis(None, 20).unwrap().definitions[0].retired);
    let CollabObjectV1::CustomEmojiVersion(definition) = store.object(version).unwrap() else {
        panic!()
    };
    let CollabObjectV1::ImageAsset(asset) = store.object(definition.asset).unwrap() else {
        panic!()
    };
    assert_eq!(asset.bytes, source);
    let CollabObjectV1::ImageAsset(preview) = store.object(definition.preview).unwrap() else {
        panic!()
    };
    assert_eq!(preview.frames, 1);
    assert_eq!(preview.media_type, ImageMediaTypeV1::Png);
    assert_eq!(
        restarted.apply_collab_command(&set).await.unwrap(),
        original
    );
    let bad = request(
        8,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Define {
                emoji_id: CustomEmojiId::new([8; 16]),
                expected: None,
                shortcode: "svg".into(),
                label: "Unsafe".into(),
                bytes: b"<svg/>".to_vec(),
            },
        },
    );
    assert_eq!(
        restarted.apply_collab_command(&bad).await.unwrap_err(),
        LocalError::Invalid
    );
}

#[tokio::test]
async fn local_emoji_library_pages_are_bounded_and_keep_the_captured_store_pin() {
    let (_root, repo) = fixture();
    for id in [1u8, 2] {
        repo.apply_collab_command(&request(
            id,
            LocalCollabCommand::Emoji {
                intent: CustomEmojiCommandV1::Define {
                    emoji_id: CustomEmojiId::new([id; 16]),
                    expected: None,
                    shortcode: format!("emoji_{id}"),
                    label: format!("Emoji {id}"),
                    bytes: png(id),
                },
            },
        ))
        .await
        .unwrap();
    }
    let store = repo.reaction_store().await.unwrap();
    let first = store.emojis(None, 1).unwrap();
    assert_eq!(first.definitions.len(), 1);
    assert!(first.next_id.is_some());
    repo.apply_collab_command(&request(
        3,
        LocalCollabCommand::Emoji {
            intent: CustomEmojiCommandV1::Define {
                emoji_id: CustomEmojiId::new([3; 16]),
                expected: None,
                shortcode: "emoji_3".into(),
                label: "Emoji 3".into(),
                bytes: png(3),
            },
        },
    ))
    .await
    .unwrap();
    let second = store.emojis(first.next_id, 1).unwrap();
    assert_eq!(second.definitions.len(), 1);
    assert!(second.next_id.is_none());
    assert_eq!(first.snapshot, second.snapshot);
    assert!(store.emojis(None, 0).is_err());
    assert!(store.emojis(None, 257).is_err());
    assert_eq!(
        repo.reaction_store()
            .await
            .unwrap()
            .emojis(None, 20)
            .unwrap()
            .definitions
            .len(),
        3
    );
    assert!(repo.thread_store().await.unwrap().state.threads.is_empty());
}

#[tokio::test]
async fn local_review_reactions_follow_current_visibility_and_keep_independent_thread_links() {
    use jj_review as r;
    let (root, repo) = fixture();
    let body = r::ReviewCommentBodyV1 {
        schema_version: 1,
        format: r::ReviewBodyFormatV1::PlainText,
        text: "Review".into(),
        sanitizer_version: "v1".into(),
    };
    let LocalReviewOutcome::Committed { receipt: review } = repo
        .apply_review_command(&LocalReviewRequest {
            version: 1,
            request_id: m::RequestId::new([80; 16]),
            command: LocalReviewCommand::RepositoryThread { body },
        })
        .await
        .unwrap()
    else {
        panic!()
    };
    let review = review.command.unwrap();
    let created = thread_result(receipt(
        repo.apply_collab_command(&thread(1)).await.unwrap(),
    ));
    let link = request(
        2,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Link {
                thread_id: created.thread_id,
                expected_thread: created.version,
                link_id: ThreadLinkId::new([2; 16]),
                target: ThreadLinkTargetV1::Review {
                    review_id: review.thread_id,
                    snapshot: review.thread_snapshot,
                },
                observed_primary: None,
            },
        },
    );
    let linked = thread_result(receipt(repo.apply_collab_command(&link).await.unwrap()));
    let target = ReactionTargetV1::ReviewComment {
        thread_id: review.thread_id,
        comment_id: review.comment_id.unwrap(),
        revision: review.comment_revision.unwrap(),
    };
    let set = react(3, target.clone(), ReactionValueV1::Like, BTreeSet::new());
    let original = repo.apply_collab_command(&set).await.unwrap();
    assert_eq!(
        repo.reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows
            .len(),
        1
    );
    repo.apply_review_command(&LocalReviewRequest {
        version: 1,
        request_id: m::RequestId::new([81; 16]),
        command: LocalReviewCommand::Apply {
            intent: Box::new(r::ReviewCommandV1::HideComment {
                comment_id: review.comment_id.unwrap(),
                expected_revision_id: r::CommentRevisionId::new(
                    *review.comment_revision.unwrap().id.as_bytes(),
                ),
                moderation_reason: "hidden".into(),
            }),
        },
    })
    .await
    .unwrap();
    assert_eq!(
        repo.reaction_query(&target, None, 20, None)
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
    assert_eq!(repo.apply_collab_command(&set).await.unwrap(), original);
    let path = root.path().join("repo");
    gc(&path);
    let restarted = reopen(&path);
    let threads = restarted.thread_store().await.unwrap();
    assert!(matches!(
        threads.object(linked.link.unwrap()).unwrap(),
        CollabObjectV1::ThreadLink(_)
    ));
    assert_eq!(
        restarted
            .reaction_query(&target, None, 20, None)
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
    let new = react(4, target, ReactionValueV1::Dislike, BTreeSet::new());
    assert_eq!(
        restarted.apply_collab_command(&new).await.unwrap_err(),
        LocalError::NotFound
    );
}

#[tokio::test]
async fn local_unknown_outcome_retains_request_and_replays_after_restart() {
    let (root, repo) = fixture();
    repo.apply_collab_command(&thread(1)).await.unwrap();
    let before = repo.metadata_store().read().unwrap().unwrap();
    let path = root.path().join("repo");
    let heads = std::fs::read_dir(path.join(".jj/repo/op_heads"))
        .unwrap()
        .map(|v| v.unwrap().file_name())
        .collect::<Vec<_>>();
    let lock = path.join(".git/refs/lapis/state/v1.lock");
    std::fs::write(&lock, b"fixture lock").unwrap();
    let command = thread(2);
    assert!(
        matches!(repo.apply_collab_command(&command).await.unwrap(),LocalCollabOutcome::OutcomeUnknown {request_id} if request_id==command.request_id)
    );
    assert_eq!(
        repo.metadata_store().read().unwrap().unwrap().commit(),
        before.commit()
    );
    std::fs::remove_file(lock).unwrap();
    let restarted = reopen(&path);
    let original = restarted.apply_collab_command(&command).await.unwrap();
    assert_eq!(
        restarted.apply_collab_command(&command).await.unwrap(),
        original
    );
    assert_eq!(
        restarted.thread_store().await.unwrap().state.threads.len(),
        2
    );
    assert_eq!(
        std::fs::read_dir(path.join(".jj/repo/op_heads"))
            .unwrap()
            .map(|v| v.unwrap().file_name())
            .collect::<Vec<_>>(),
        heads
    );
}

#[tokio::test]
async fn local_reply_parent_and_receipt_survive_gc_and_cold_reopen() {
    let (root, repo) = fixture();
    let created = thread_result(receipt(
        repo.apply_collab_command(&thread(91)).await.unwrap(),
    ));
    let parent = thread_result(receipt(
        repo.apply_collab_command(&request(
            92,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Message {
                    thread_id: created.thread_id,
                    expected_thread: created.version,
                    message_id: HumanMessageId::new([92; 16]),
                    expected_revision: None,
                    reply_to: None,
                    body: Some("Parent".into()),
                },
            },
        ))
        .await
        .unwrap(),
    ));
    let command = request(
        93,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Message {
                thread_id: parent.thread_id,
                expected_thread: parent.version,
                message_id: HumanMessageId::new([93; 16]),
                expected_revision: None,
                reply_to: parent.message,
                body: Some("Persisted reply".into()),
            },
        },
    );
    let accepted = receipt(repo.apply_collab_command(&command).await.unwrap());
    let reply = thread_result(accepted.clone());
    let path = root.path().join("repo");
    drop(repo);
    gc(&path);
    let cold = reopen(&path);
    assert_eq!(
        receipt(cold.apply_collab_command(&command).await.unwrap()),
        accepted
    );
    let store = cold.thread_store().await.unwrap();
    let CollabObjectV1::HumanMessageRevision(message) =
        store.object(reply.message.unwrap()).unwrap()
    else {
        panic!()
    };
    assert_eq!(message.reply_to, parent.message);
    let CollabObjectV1::HumanMessageRevision(parent) =
        store.object(parent.message.unwrap()).unwrap()
    else {
        panic!()
    };
    assert_eq!(parent.body.as_deref(), Some("Parent"));
}

#[tokio::test]
async fn local_work_comment_reactions_share_companion_pin_and_survive_gc_and_reopen() {
    use jj_store_service::work::{ProjectId, TaskId, WorkCommandV1, WorkCommentId};
    let (root, repo) = fixture();
    let project = ProjectId::new([90; 16]);
    let task = TaskId::new([91; 16]);
    let comment = WorkCommentId::new([92; 16]);
    let work = |id, command| jj_store_service::work::Request {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    };
    for (id, command) in [
        (
            90,
            WorkCommandV1::CreateProject {
                id: project,
                title: "Home".into(),
                body: String::new(),
            },
        ),
        (
            91,
            WorkCommandV1::CreateTask {
                id: task,
                project,
                expected_project_version: 1,
                title: "Task".into(),
                body: String::new(),
                repository: None,
                parent: None,
            },
        ),
        (
            92,
            WorkCommandV1::AddComment {
                id: comment,
                task,
                expected_task_version: 1,
                reply_to: None,
                body: "Visible".into(),
            },
        ),
    ] {
        repo.apply_work_command(&work(id, command)).unwrap();
    }
    let target = ReactionTargetV1::WorkComment {
        task_id: task,
        comment_id: comment,
        version: 1,
    };
    let before = repo.metadata_store().read().unwrap().unwrap();
    let set = react(93, target.clone(), ReactionValueV1::Like, BTreeSet::new());
    let accepted = receipt(repo.apply_collab_command(&set).await.unwrap());
    let active = reaction_result(accepted.clone()).active;
    let page = repo.reaction_query(&target, None, 20, None).await.unwrap();
    assert_eq!(page.rows.len(), 1);
    assert_eq!(
        repo.metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .selection(Domain::Work),
        before.selection(Domain::Work)
    );
    let path = root.path().join("repo");
    drop(repo);
    gc(&path);
    let cold = reopen(&path);
    assert_eq!(
        receipt(cold.apply_collab_command(&set).await.unwrap()),
        accepted
    );
    assert_eq!(
        receipt(cold.collab_command_status(set.request_id).unwrap()),
        accepted
    );
    assert_eq!(
        cold.reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows[0]
            .value,
        Some(ReactionValueV1::Like)
    );
    let clear = request(
        94,
        LocalCollabCommand::Reaction {
            intent: ReactionCommandV1::Clear {
                target: target.clone(),
                observed: active,
            },
        },
    );
    cold.apply_collab_command(&clear).await.unwrap();
    assert!(
        cold.reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows
            .iter()
            .all(|row| row.value.is_none())
    );
    cold.apply_work_command(&work(
        95,
        WorkCommandV1::EditComment {
            id: comment,
            expected_version: 1,
            expected_task_version: 2,
            body: "Edited".into(),
        },
    ))
    .unwrap();
    assert_eq!(
        cold.reaction_query(&target, None, 20, None)
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
    assert_eq!(
        cold.reaction_query(&target, None, 20, page.snapshot.metadata_commit)
            .await
            .unwrap_err(),
        LocalError::Stale
    );
    assert_eq!(
        cold.apply_collab_command(&react(96, target, ReactionValueV1::Like, BTreeSet::new()))
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
    let current = ReactionTargetV1::WorkComment {
        task_id: task,
        comment_id: comment,
        version: 2,
    };
    assert!(
        cold.reaction_query(&current, None, 20, None)
            .await
            .unwrap()
            .rows
            .is_empty()
    );
    cold.apply_work_command(&work(
        97,
        WorkCommandV1::TombstoneComment {
            id: comment,
            expected_version: 2,
            expected_task_version: 3,
        },
    ))
    .unwrap();
    assert_eq!(
        cold.reaction_query(&current, None, 20, None)
            .await
            .unwrap_err(),
        LocalError::NotFound
    );
}

#[tokio::test]
async fn local_conversation_batch_survives_reopen_with_exact_ranges_and_receipt() {
    let (root, repo) = fixture();
    let parent = thread_result(receipt(
        repo.apply_collab_command(&thread(80)).await.unwrap(),
    ));
    let original = thread_result(receipt(
        repo.apply_collab_command(&request(
            81,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Message {
                    thread_id: parent.thread_id,
                    expected_thread: parent.version,
                    message_id: HumanMessageId::new([81; 16]),
                    expected_revision: None,
                    reply_to: None,
                    body: Some("hello 🦀".into()),
                },
            },
        ))
        .await
        .unwrap(),
    ));
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
    let submission = ConversationSubmissionV1 {
        reply_to: None,
        thread_id: parent.thread_id,
        expected_thread: original.version,
        message_id: Some(HumanMessageId::new([83; 16])),
        text: Some("Please review".into()),
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
    let command = request(
        82,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Submit {
                submission: Box::new(submission),
            },
        },
    );
    let committed = receipt(repo.apply_collab_command(&command).await.unwrap());
    let cold = reopen(&root.path().join("repo"));
    let replay = receipt(cold.apply_collab_command(&command).await.unwrap());
    assert_eq!(
        serde_json::to_value(&replay).unwrap(),
        serde_json::to_value(&committed).unwrap()
    );
    let before = cold.metadata_store().read().unwrap().unwrap().commit();
    let mut forged = command.clone();
    forged.request_id = m::RequestId::new([84; 16]);
    let LocalCollabCommand::Thread {
        intent: ThreadCommandV1::Submit { submission },
    } = &mut forged.command
    else {
        panic!()
    };
    submission.comments[0].source.quote = "not the source".into();
    assert!(cold.apply_collab_command(&forged).await.is_err());
    assert_eq!(
        cold.metadata_store().read().unwrap().unwrap().commit(),
        before
    );
}
