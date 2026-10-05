#![cfg(unix)]
//! Real owner-admitted Unix gRPC over disposable native MemoryStore authority.
use jj_store_jj::policy::Protection;
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_rpc::{
    client::{NativeClient, Options},
    server::{Limits, NativeRpc},
    wire::{self, native_store_client::NativeStoreClient},
};
use jj_store_service::{
    administration::{OperatorPolicies, RepositoryHost},
    auth::{self, Permission, Scope, SystemClock, TokenPolicy},
    collab_model::*,
    collab_mutations::{Outcome, Request, Status},
    collab_pages::{Items, Query},
    native::SessionLimits,
};
use std::{
    collections::BTreeSet, os::unix::fs::MetadataExt as _, path::Path, sync::Arc, time::Duration,
};
use tonic::{
    Code,
    transport::{Endpoint, Server},
};
const REPO: m::RepositoryId = m::RepositoryId::new([71; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([73; 16]);
struct NoOperators;
#[async_trait::async_trait]
impl OperatorPolicies for NoOperators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(TokenPolicy::new(Scope::Operator))
    }
}
fn host() -> RepositoryHost {
    RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        8 * 1024 * 1024,
    )
    .unwrap()
}
async fn serve(
    host: RepositoryHost,
    root: &Path,
    actor: m::PrincipalId,
) -> (
    NativeClient,
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<()>,
) {
    let socket = root.join("native.sock");
    let _ = std::fs::remove_file(&socket);
    let rpc = NativeRpc::hosted_local_trusted(
        host,
        Limits::default(),
        actor,
        Permission::Admin,
        std::fs::metadata(root).unwrap().uid(),
    )
    .unwrap();
    let listener = tokio::net::UnixListener::bind(&socket).unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        Server::builder()
            .add_service(rpc.service())
            .serve_with_incoming_shutdown(
                tokio_stream::wrappers::UnixListenerStream::new(listener),
                async {
                    let _ = stopped.await;
                },
            )
            .await
            .unwrap();
    });
    let client = NativeClient::connect_unix(
        &socket,
        REPO,
        Options {
            timeout: Duration::from_secs(5),
            ..Options::default()
        },
    )
    .await
    .unwrap();
    (client, stop, task)
}
async fn stop(stop: tokio::sync::oneshot::Sender<()>, task: tokio::task::JoinHandle<()>) {
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
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
async fn home(client: &NativeClient) {
    use jj_store_service::work::*;
    client
        .apply_work_command(&Request {
            version: 1,
            request_id: m::RequestId::new([99; 16]),
            command: WorkCommandV1::CreateProject {
                id: ProjectId::new([99; 16]),
                title: "Home".into(),
                body: String::new(),
            },
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn explicit_capture_binding_survives_preparation_publication_and_transport_reconstruction() {
    capture_binding_acceptance(false).await;
}
#[tokio::test]
async fn diagnostic_gap_binding_survives_preparation_publication_and_transport_reconstruction() {
    capture_binding_acceptance(true).await;
}
async fn capture_binding_acceptance(diagnostic: bool) {
    use jj_agent::{
        CaptureBatchId, IntegrationId, NormalizerProfileV1, ObservationId, SanitizedMediaType,
        SanitizedObservationBatchV1, SanitizedObservationV1, SanitizedPayloadV1,
        SanitizedRepresentation, SourceInstanceId, SourceKind, SourceTrust,
    };
    let root = tempfile::tempdir().unwrap();
    let host = host();
    let (client, shutdown, task) = serve(host.clone(), root.path(), ACTOR).await;
    client
        .create_repository_local(m::EpochId::new([72; 16]))
        .await
        .unwrap();
    home(&client).await;
    let first = thread(client.apply_collab_command(&create(20)).await.unwrap());
    let binding = ThreadBindingV1 {
        thread_id: first.thread_id,
        observed_thread: first.version,
        observed_assignment: None,
    };
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([21; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([22; 16]),
        sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([23; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(1_725_000_000_000),
            source_kind: SourceKind::Hook,
            source_trust: SourceTrust::LocalUserHook,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                if diagnostic {
                    br#"{"future_record":true}"#.as_slice()
                } else {
                    br#"{"hook_event_name":"SessionStart","session_id":"bound-rpc"}"#.as_slice()
                },
            )
            .unwrap(),
        }],
    };
    let before = client.collab_thread(first.thread_id).await.unwrap();
    let prepared = client
        .prepare_agent_capture_with_binding(
            m::RequestId::new([24; 16]),
            m::PublicationId::new([25; 16]),
            &NormalizerProfileV1::codex_0_156_1(),
            &batch,
            b"bound capture".to_vec(),
            Some(&binding),
        )
        .await
        .unwrap();
    assert_eq!(prepared.principal, ACTOR);
    assert_eq!(prepared.thread_binding.as_ref().unwrap().binding, binding);
    assert!(prepared.thread_binding.as_ref().unwrap().accepted_at_millis > 0);
    assert_eq!(
        prepared
            .delta
            .edits
            .iter()
            .filter(|e| e.catalog == jj_agent::AgentCatalogKind::Sessions)
            .count(),
        usize::from(!diagnostic)
    );
    assert_eq!(client.collab_thread(first.thread_id).await.unwrap(), before);
    let outcome = client.publish_agent_delta(&prepared).await.unwrap();
    assert!(matches!(
        outcome,
        jj_store_wal::publication::PublishOutcome::Committed(_)
    ));
    let selected = client.collab_thread(first.thread_id).await.unwrap();
    assert_ne!(selected.snapshot.agent_root, before.snapshot.agent_root);
    assert_ne!(selected.snapshot.thread_root, before.snapshot.thread_root);
    assert_eq!(selected.thread.value.status, before.thread.value.status);
    let events = client
        .collab_page(
            &Query::Events {
                thread_id: first.thread_id,
                limit: 10,
            },
            None,
        )
        .await
        .unwrap();
    let Items::Events { events, .. } = events.items else {
        panic!()
    };
    assert!(matches!(
        events[0].value.event,
        ThreadEventKindV1::Link { .. }
    ));
    assert_eq!(
        matches!(
            events[0].link.as_ref().unwrap().target,
            ThreadLinkTargetV1::CaptureGap { .. }
        ),
        diagnostic
    );
    let renamed = request(
        26,
        CollabCommandV1::Thread {
            intent: ThreadCommandV1::Rename {
                thread_id: first.thread_id,
                expected: selected.thread.reference,
                title: "After capture".into(),
            },
        },
    );
    client.apply_collab_command(&renamed).await.unwrap();
    stop(shutdown, task).await;
    let (client, shutdown, task) = serve(host, root.path(), ACTOR).await;
    assert_eq!(
        client.publish_agent_delta(&prepared).await.unwrap(),
        outcome
    );
    let mut changed = prepared;
    changed
        .thread_binding
        .as_mut()
        .unwrap()
        .binding
        .observed_thread = first.version;
    changed.thread_binding.as_mut().unwrap().accepted_at_millis += 1;
    assert_eq!(
        client
            .publish_agent_delta(&changed)
            .await
            .unwrap_err()
            .code(),
        Code::Aborted
    );
    stop(shutdown, task).await;
}
fn thread(outcome: Outcome) -> ThreadCommandResultV1 {
    let Outcome::Committed { receipt } = outcome else {
        panic!()
    };
    let CollabCommandResultV1::Thread { result } = receipt.result else {
        panic!()
    };
    result
}
fn png() -> Vec<u8> {
    "89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c4890000000d49444154789c63f8cfc0f01f00050001ff89993d1d0000000049454e44ae426082".as_bytes().as_chunks::<2>().0.iter().map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect()
}

#[tokio::test]
async fn unix_rpc_keeps_exact_feedback_assets_pages_and_receipts_across_transport_reconstruction() {
    let root = tempfile::tempdir().unwrap();
    let host = host();
    let (client, shutdown, task) = serve(host.clone(), root.path(), ACTOR).await;
    client
        .create_repository_local(m::EpochId::new([72; 16]))
        .await
        .unwrap();
    let before = client.snapshot().await.unwrap();
    let empty = client
        .collab_page(&Query::Threads { limit: 10 }, None)
        .await
        .unwrap();
    assert!(empty.snapshot.thread_root.is_none());
    assert_eq!(client.snapshot().await.unwrap(), before);
    assert_eq!(
        client
            .apply_collab_command(&create(1))
            .await
            .unwrap_err()
            .code(),
        Code::FailedPrecondition
    );
    home(&client).await;
    let original = client.apply_collab_command(&create(1)).await.unwrap();
    let first = thread(original.clone());
    assert_eq!(
        client.apply_collab_command(&create(1)).await.unwrap(),
        original
    );
    client.apply_collab_command(&create(2)).await.unwrap();
    let page = client
        .collab_page(&Query::Threads { limit: 1 }, None)
        .await
        .unwrap();
    let cursor = page.next_cursor.as_deref().unwrap();
    client.apply_collab_command(&create(3)).await.unwrap();
    let second = client
        .collab_page(&Query::Threads { limit: 1 }, Some(cursor))
        .await
        .unwrap();
    assert_eq!(second.snapshot, page.snapshot);
    assert!(second.next_cursor.is_none());
    assert_eq!(
        client
            .collab_page(&Query::Emojis { limit: 1 }, Some(cursor))
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    assert_eq!(
        client
            .collab_thread(first.thread_id)
            .await
            .unwrap()
            .thread
            .reference,
        first.version
    );
    let message = thread(
        client
            .apply_collab_command(&request(
                4,
                CollabCommandV1::Thread {
                    intent: ThreadCommandV1::Message {
                        reply_to: None,
                        thread_id: first.thread_id,
                        expected_thread: first.version,
                        message_id: HumanMessageId::new([4; 16]),
                        expected_revision: None,
                        body: Some("Human note".into()),
                    },
                },
            ))
            .await
            .unwrap(),
    );
    let target = ReactionTargetV1::HumanMessage {
        thread_id: first.thread_id,
        message_id: HumanMessageId::new([4; 16]),
        revision: message.message.unwrap(),
    };
    assert_eq!(
        client
            .collab_message(&target)
            .await
            .unwrap()
            .message
            .body
            .as_deref(),
        Some("Human note")
    );
    let reacted = client
        .apply_collab_command(&request(
            5,
            CollabCommandV1::Reaction {
                intent: ReactionCommandV1::Set {
                    target: target.clone(),
                    value: ReactionValueV1::Emoji {
                        sequence: "👩🏽‍💻".into(),
                    },
                    observed: BTreeSet::new(),
                },
            },
        ))
        .await
        .unwrap();
    let query = Query::Reactions {
        target: target.clone(),
        limit: 10,
    };
    let feedback = client.collab_page(&query, None).await.unwrap();
    let Items::Reactions {
        actors, reactions, ..
    } = feedback.items
    else {
        panic!()
    };
    assert_eq!(actors[0].actor, ACTOR);
    assert_eq!(reactions[0].count, 1);
    let edited = thread(
        client
            .apply_collab_command(&request(
                6,
                CollabCommandV1::Thread {
                    intent: ThreadCommandV1::Message {
                        reply_to: None,
                        thread_id: first.thread_id,
                        expected_thread: message.version,
                        message_id: HumanMessageId::new([4; 16]),
                        expected_revision: message.message,
                        body: Some("Edited".into()),
                    },
                },
            ))
            .await
            .unwrap(),
    );
    let current = ReactionTargetV1::HumanMessage {
        thread_id: first.thread_id,
        message_id: HumanMessageId::new([4; 16]),
        revision: edited.message.unwrap(),
    };
    let Items::Reactions { reactions, .. } = client
        .collab_page(
            &Query::Reactions {
                target: current,
                limit: 10,
            },
            None,
        )
        .await
        .unwrap()
        .items
    else {
        panic!()
    };
    assert!(reactions.is_empty());
    let Items::Reactions { revision, .. } = client.collab_page(&query, None).await.unwrap().items
    else {
        panic!()
    };
    assert_eq!(revision, TargetRevisionV1::Earlier);
    let emoji_id = CustomEmojiId::new([7; 16]);
    let defined = client
        .apply_collab_command(&request(
            7,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Define {
                    emoji_id,
                    expected: None,
                    shortcode: "ship_it".into(),
                    label: "Ship it".into(),
                    bytes: png(),
                },
            },
        ))
        .await
        .unwrap();
    let Outcome::Committed { receipt } = defined else {
        panic!()
    };
    let CollabCommandResultV1::Emoji { version } = receipt.result else {
        panic!()
    };
    let Items::Emojis {
        definitions,
        versions,
    } = client
        .collab_page(&Query::Emojis { limit: 10 }, None)
        .await
        .unwrap()
        .items
    else {
        panic!()
    };
    assert_eq!(versions[&emoji_id], version);
    let definition = &definitions[0];
    assert_eq!(
        client
            .collab_image(definition.asset)
            .await
            .unwrap()
            .asset
            .bytes,
        png()
    );
    assert_eq!(
        client
            .collab_image(definition.preview)
            .await
            .unwrap()
            .asset
            .frames,
        1
    );
    assert_eq!(
        client.collab_image(version).await.unwrap_err().code(),
        Code::InvalidArgument
    );
    let absent = CollabObjectRef {
        kind: CollabObjectKind::ImageAsset,
        id: CollabObjectId::new([80; 32]),
    };
    assert_eq!(
        client.collab_image(absent).await.unwrap_err().code(),
        Code::NotFound
    );
    client
        .apply_collab_command(&request(
            8,
            CollabCommandV1::Emoji {
                intent: CustomEmojiCommandV1::Retire {
                    emoji_id,
                    expected: version,
                },
            },
        ))
        .await
        .unwrap();
    assert_eq!(
        client
            .collab_image(definition.asset)
            .await
            .unwrap()
            .asset
            .bytes,
        png()
    );
    stop(shutdown, task).await;
    let (restarted, shutdown, task) = serve(host.clone(), root.path(), ACTOR).await;
    assert_eq!(
        restarted.apply_collab_command(&create(1)).await.unwrap(),
        original
    );
    let Outcome::Committed { receipt } = reacted else {
        panic!()
    };
    assert_eq!(
        restarted
            .collab_command_status(m::RequestId::new([5; 16]))
            .await
            .unwrap(),
        Status::Committed { receipt }
    );
    assert_eq!(
        restarted
            .collab_page(&Query::Threads { limit: 1 }, Some(cursor))
            .await
            .unwrap_err()
            .code(),
        Code::FailedPrecondition
    );
    assert_eq!(
        restarted
            .collab_image(definition.asset)
            .await
            .unwrap()
            .asset
            .bytes,
        png()
    );
    stop(shutdown, task).await;
    let (other, shutdown, task) = serve(host, root.path(), m::PrincipalId::new([74; 16])).await;
    assert!(matches!(
        other
            .collab_command_status(m::RequestId::new([1; 16]))
            .await
            .unwrap(),
        Status::NotKnown { .. }
    ));
    stop(shutdown, task).await;
}

#[tokio::test]
async fn raw_rpc_refuses_actor_injection_ambiguous_rasters_and_limits_without_publication() {
    let root = tempfile::tempdir().unwrap();
    let (client, shutdown, task) = serve(host(), root.path(), ACTOR).await;
    client
        .create_repository_local(m::EpochId::new([72; 16]))
        .await
        .unwrap();
    home(&client).await;
    let socket = root.path().join("native.sock");
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(tower::service_fn(move |_| {
            let socket = socket.clone();
            async move {
                tokio::net::UnixStream::connect(socket)
                    .await
                    .map(hyper_util::rt::TokioIo::new)
            }
        }))
        .await
        .unwrap();
    let mut raw = NativeStoreClient::new(channel).max_encoding_message_size(8 * 1024 * 1024);
    let before = client.snapshot().await.unwrap();
    let mut injected = serde_json::to_value(create(1)).unwrap();
    injected["actor"] = serde_json::json!(ACTOR);
    for (request_json, image, code) in [
        (
            serde_json::to_vec(&injected).unwrap(),
            Vec::new(),
            Code::InvalidArgument,
        ),
        (
            serde_json::to_vec(&create(1)).unwrap(),
            vec![1],
            Code::InvalidArgument,
        ),
        (
            vec![b' '; 4 * 1024 * 1024 + 1],
            Vec::new(),
            Code::InvalidArgument,
        ),
        (
            serde_json::to_vec(&create(1)).unwrap(),
            vec![0; MAX_IMAGE_BYTES + 1],
            Code::ResourceExhausted,
        ),
        (
            serde_json::to_vec(&request(
                2,
                CollabCommandV1::Emoji {
                    intent: CustomEmojiCommandV1::Define {
                        emoji_id: CustomEmojiId::new([2; 16]),
                        expected: None,
                        shortcode: "unsafe".into(),
                        label: "Unsafe".into(),
                        bytes: png(),
                    },
                },
            ))
            .unwrap(),
            png(),
            Code::InvalidArgument,
        ),
    ] {
        assert_eq!(
            raw.apply_collab_command(wire::CollabCommandRequest {
                repository: REPO.as_bytes().to_vec(),
                request_json,
                image
            })
            .await
            .unwrap_err()
            .code(),
            code
        );
    }
    assert_eq!(
        raw.collab_thread(wire::CollabThreadRequest {
            repository: REPO.as_bytes().to_vec(),
            thread_id: vec![0; 15]
        })
        .await
        .unwrap_err()
        .code(),
        Code::InvalidArgument
    );
    assert_eq!(client.snapshot().await.unwrap(), before);
    stop(shutdown, task).await;
}
