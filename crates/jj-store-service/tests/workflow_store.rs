use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    work, workflow as w,
};
use std::{collections::BTreeSet, sync::Arc};
const REPO: m::RepositoryId = m::RepositoryId::new([1; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([2; 16]);
fn session(
    store: Arc<dyn jj_store_object::ObjectStore>,
    actor: m::PrincipalId,
    permission: Permission,
) -> Arc<NativeSession> {
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    Arc::new(
        NativeSession::new_local_trusted(
            store,
            objects,
            actor,
            permission,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    )
}

#[path = "workflow_store/index.rs"]
mod index;
fn request(byte: u8, command: w::Command) -> w::Request {
    w::Request {
        version: 1,
        request_id: m::RequestId::new([byte; 16]),
        command,
    }
}
fn definition() -> w::Definition {
    w::Definition {
        name: "Build".into(),
        description: String::new(),
        enabled: true,
        repository: REPO,
        trigger: w::Trigger::Manual,
        labels: Default::default(),
        conditions: Default::default(),
        actions: vec![w::Action {
            id: "build".into(),
            executor: w::Executor::Command {
                name: "build".into(),
            },
            inputs: Default::default(),
            timeout_seconds: 30,
        }],
    }
}
#[tokio::test]
async fn workflow_grants_receipts_and_pinned_history_share_one_service() {
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = session(store.clone(), ACTOR, Permission::Write);
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([10; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([10; 16]),
                    title: "Automation".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
    let service = w::WorkflowService {
        hosts: BTreeSet::from([ACTOR]).into(),
    };
    let workflow = w::WorkflowId::new([11; 16]);
    let put = request(
        11,
        w::Command::Put {
            workflow,
            expected: None,
            definition: definition(),
        },
    );
    let before = writer.snapshot().await.unwrap().digest();
    let reader = session(store.clone(), ACTOR, Permission::Read);
    assert!(matches!(
        service.apply(reader.clone(), &put).await,
        Err(w::Error::Denied)
    ));
    assert_eq!(writer.snapshot().await.unwrap().digest(), before);
    let accepted = service.apply(writer.clone(), &put).await.unwrap();
    assert_eq!(service.apply(writer.clone(), &put).await.unwrap(), accepted);
    let other = session(
        store.clone(),
        m::PrincipalId::new([4; 16]),
        Permission::Write,
    );
    assert!(matches!(
        service.status(other.clone(), put.request_id).await.unwrap(),
        w::Status::NotKnown { .. }
    ));
    let first = service
        .page(reader.clone(), w::Query::default())
        .await
        .unwrap();
    let mut edited = definition();
    edited.name = "Changed".into();
    let edit = request(
        12,
        w::Command::Put {
            workflow,
            expected: Some(first.workflows[0].version),
            definition: edited,
        },
    );
    assert!(matches!(
        service.apply(other.clone(), &edit).await,
        Err(w::Error::Denied)
    ));
    service.apply(writer.clone(), &edit).await.unwrap();
    let pinned = service
        .page(
            reader.clone(),
            w::Query {
                at: Some(first.position),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(pinned.position, first.position);
    assert_eq!(pinned.root, first.root);
    assert_eq!(pinned.workflows, first.workflows);
    let latest = service
        .page(reader.clone(), w::Query::default())
        .await
        .unwrap();
    let run = request(
        13,
        w::Command::Run {
            workflow,
            expected: latest.workflows[0].version,
            source: w::Source {
                repository: REPO,
                operation: Some(m::OperationId::new([5; 64])),
                commit: Some(m::CommitId::new([6; 64])),
                bookmark: None,
                conversation: None,
            },
        },
    );
    let w::Outcome::Committed { receipt } = service.apply(writer.clone(), &run).await.unwrap()
    else {
        panic!("committed")
    };
    let run = receipt.result.runs[0].clone();
    let claim = request(
        14,
        w::Command::Claim {
            run: run.id,
            attempt: run.current().id,
            host: "host-a".into(),
        },
    );
    let unchanged = writer.snapshot().await.unwrap().digest();
    assert!(matches!(
        w::WorkflowService::default()
            .apply(writer.clone(), &claim)
            .await,
        Err(w::Error::Denied)
    ));
    assert_eq!(writer.snapshot().await.unwrap().digest(), unchanged);
    service.apply(writer.clone(), &claim).await.unwrap();
    for sequence in 0..2 {
        service
            .apply(
                writer.clone(),
                &request(
                    15 + sequence as u8,
                    w::Command::Report {
                        run: run.id,
                        attempt: run.current().id,
                        sequence,
                        data: w::EvidenceData::Log {
                            step: None,
                            stream: "stdout".into(),
                            text: format!("line {sequence}\n"),
                        },
                    },
                ),
            )
            .await
            .unwrap();
    }
    let detail = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(detail.evidence.len(), 1);
    let more = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                at: Some(detail.position),
                after: detail.next.clone(),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(more.position, detail.position);
    assert_eq!(more.evidence[0].sequence, 1);
    let unauthorized = w::WorkflowService {
        hosts: BTreeSet::from([m::PrincipalId::new([4; 16])]).into(),
    };
    assert!(matches!(
        unauthorized
            .apply(
                other,
                &request(
                    17,
                    w::Command::Report {
                        run: run.id,
                        attempt: run.current().id,
                        sequence: 2,
                        data: w::EvidenceData::Log {
                            step: None,
                            stream: "stdout".into(),
                            text: "forged".into()
                        }
                    }
                )
            )
            .await,
        Err(w::Error::Denied)
    ));
    let interrupt = request(
        18,
        w::Command::Interrupt {
            run: run.id,
            attempt: run.current().id,
        },
    );
    // Row limits alone must not make a run with several artifacts unreadable.
    for sequence in 2..10 {
        service
            .apply(
                writer.clone(),
                &request(
                    20 + sequence as u8,
                    w::Command::Report {
                        run: run.id,
                        attempt: run.current().id,
                        sequence,
                        data: w::EvidenceData::Artifact {
                            name: format!("result-{sequence}.bin"),
                            media_type: "application/octet-stream".into(),
                            bytes: vec![255; 128 * 1024],
                        },
                    },
                ),
            )
            .await
            .unwrap();
    }
    let artifacts = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                limit: Some(256),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(artifacts.next.is_some());
    assert!(serde_json::to_vec(&artifacts).unwrap().len() < 8 * 1024 * 1024);
    let remainder = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                at: Some(artifacts.position),
                after: artifacts.next,
                limit: Some(256),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(artifacts.evidence.len() + remainder.evidence.len(), 10);
    service
        .apply(
            writer.clone(),
            &request(
                40,
                w::Command::Report {
                    run: run.id,
                    attempt: run.current().id,
                    sequence: 10,
                    data: w::EvidenceData::Step {
                        step: "yaml-test".into(),
                        status: w::RunStatus::Completed,
                        output: Default::default(),
                    },
                },
            ),
        )
        .await
        .unwrap();
    let summary = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(summary.evidence.len(), 1);
    assert_eq!(summary.evidence[0].sequence, 0);
    assert_eq!(summary.step_reports.len(), 1);
    assert_eq!(summary.step_reports[0].sequence, 10);
    let previous = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                at: Some(detail.position),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(previous.step_reports.is_empty());
    let other_attempt = service
        .page(
            reader.clone(),
            w::Query {
                run: Some(run.id),
                attempt: Some(w::ExecutionAttemptId::new([99; 16])),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(other_attempt.step_reports.is_empty());
    service.apply(writer.clone(), &interrupt).await.unwrap();
    assert!(matches!(
        service
            .apply(
                writer.clone(),
                &request(
                    19,
                    w::Command::Retry {
                        run: run.id,
                        expected: run.current().id
                    }
                )
            )
            .await,
        Err(w::Error::Stale)
    ));
    let cold = session(store, ACTOR, Permission::Write);
    assert!(matches!(
        service
            .status(cold.clone(), interrupt.request_id)
            .await
            .unwrap(),
        w::Status::Committed { .. }
    ));
    let unknown = service
        .page(
            cold,
            w::Query {
                run: Some(run.id),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(unknown.runs[0].current().status, w::RunStatus::Unknown);
    assert_eq!(unknown.runs[0].source, run.source);
    // Seek continuation still works after another host claims the preceding row.
    for byte in [30, 31] {
        service
            .apply(
                writer.clone(),
                &request(
                    byte,
                    w::Command::Run {
                        workflow,
                        expected: latest.workflows[0].version,
                        source: run.source.clone(),
                    },
                ),
            )
            .await
            .unwrap();
    }
    let queue = service
        .page(
            writer.clone(),
            w::Query {
                pending_only: Some(true),
                limit: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(queue.runs.len(), 1);
    assert!(queue.workflows.is_empty());
    let first = &queue.runs[0];
    service
        .apply(
            writer.clone(),
            &request(
                32,
                w::Command::Claim {
                    run: first.id,
                    attempt: first.current().id,
                    host: "host-b".into(),
                },
            ),
        )
        .await
        .unwrap();
    let following = service
        .page(
            writer,
            w::Query {
                pending_only: Some(true),
                after: queue.next,
                limit: Some(1),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(following.runs.len(), 1);
    assert_ne!(following.runs[0].id, first.id);
}

#[tokio::test]
async fn message_and_reaction_events_are_scoped_pinned_and_exclude_edits_and_removals() {
    use jj_collab as c;
    use jj_store_service::collab_mutations as cm;
    let store = Arc::new(MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 2 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects,
        Permission::Admin,
        m::EpochId::new([3; 16]),
        Default::default(),
    )
    .await
    .unwrap();
    let writer = session(store.clone(), ACTOR, Permission::Write);
    let service = w::WorkflowService {
        hosts: [ACTOR].into(),
    };
    let before = writer.snapshot().await.unwrap();
    assert!(matches!(
        service
            .apply(
                writer.clone(),
                &request(
                    90,
                    w::Command::Put {
                        workflow: w::WorkflowId::new([90; 16]),
                        expected: None,
                        definition: definition()
                    }
                )
            )
            .await,
        Err(w::Error::Invalid)
    ));
    assert_eq!(
        writer.snapshot().await.unwrap().manifest().published,
        before.manifest().published
    );
    work::WorkCommandService
        .apply(
            writer.clone(),
            &work::Request {
                version: 1,
                request_id: m::RequestId::new([91; 16]),
                command: work::WorkCommandV1::CreateProject {
                    id: work::ProjectId::new([91; 16]),
                    title: "Triggers".into(),
                    body: String::new(),
                },
            },
        )
        .await
        .unwrap();
    let mut definition = definition();
    definition.trigger = w::Trigger::MessagePosted;
    let thread = c::ThreadId::new([92; 16]);
    definition.conditions = w::Conditions {
        authors: [ACTOR.to_string()].into(),
        thread: Some(thread.to_string()),
        text: Some(w::TextCondition {
            operator: w::TextOperator::Contains,
            value: "deploy".into(),
        }),
        ..Default::default()
    };
    let capply = |id, command| cm::Request {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    };
    let cm::Outcome::Committed { receipt: created } = cm::CollabCommandService
        .apply(
            writer.clone(),
            &capply(
                92,
                c::CollabCommandV1::Thread {
                    intent: c::ThreadCommandV1::Create {
                        thread_id: thread,
                        title: "Deploy".into(),
                        baseline: None,
                    },
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let c::CollabCommandResultV1::Thread { result: created } = created.result.clone() else {
        panic!()
    };
    let workflow = w::WorkflowId::new([93; 16]);
    let w::Outcome::Committed { receipt: put } = service
        .apply(
            writer.clone(),
            &request(
                93,
                w::Command::Put {
                    workflow,
                    expected: None,
                    definition: definition.clone(),
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let cursor = put.accepted.position;
    let message = c::HumanMessageId::new([94; 16]);
    let msg = |expected, prior, body| c::CollabCommandV1::Thread {
        intent: c::ThreadCommandV1::Message {
            emojis: vec![],
            reply_to: None,
            thread_id: thread,
            expected_thread: expected,
            message_id: message,
            expected_revision: prior,
            body: Some(body),
        },
    };
    let cm::Outcome::Committed { receipt: posted } = cm::CollabCommandService
        .apply(
            writer.clone(),
            &capply(94, msg(created.version, None, "please deploy".into())),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let c::CollabCommandResultV1::Thread { result: posted } = posted.result.clone() else {
        panic!()
    };
    let query = |after| w::EventQuery {
        after: Some(after),
        filters: Some(serde_json::to_string(&[definition.conditions.clone()]).unwrap()),
        ..Default::default()
    };
    let page = service
        .source_events(writer.clone(), query(cursor))
        .await
        .unwrap();
    assert_eq!(page.events.len(), 1);
    let event = &page.events[0];
    assert_eq!(event.kind, w::EventKind::MessagePosted);
    assert_eq!(event.source.commit, None);
    assert_eq!(
        event.source.conversation.as_ref().unwrap().message,
        Some(message)
    );
    assert_eq!(
        event
            .filter_results
            .get(&definition.conditions.id().unwrap()),
        Some(&true)
    );
    let w::Outcome::Committed { receipt: evaluated } = service
        .apply(
            writer.clone(),
            &request(
                95,
                w::Command::Evaluate {
                    event: event.clone(),
                    definitions: Some(
                        [(workflow, put.result.definition.as_ref().unwrap().version)].into(),
                    ),
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(evaluated.result.runs.len(), 1);
    let mut other = event.clone();
    other.filter_results.clear();
    let w::Outcome::Committed { receipt: duplicate } = service
        .apply(
            writer.clone(),
            &request(
                96,
                w::Command::Evaluate {
                    event: other,
                    definitions: None,
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(duplicate.result.runs[0].id, evaluated.result.runs[0].id);
    let after = duplicate.accepted.position;
    cm::CollabCommandService
        .apply(
            writer.clone(),
            &capply(97, msg(posted.version, posted.message, "edited".into())),
        )
        .await
        .unwrap();
    assert!(
        service
            .source_events(writer.clone(), query(after))
            .await
            .unwrap()
            .events
            .is_empty()
    );
    let target = c::ReactionTargetV1::HumanMessage {
        thread_id: thread,
        message_id: message,
        revision: posted.message.unwrap(),
    };
    let before_reaction = writer.snapshot().await.unwrap().manifest().published;
    let cm::Outcome::Committed { receipt: reaction } = cm::CollabCommandService
        .apply(
            writer.clone(),
            &capply(
                98,
                c::CollabCommandV1::Reaction {
                    intent: c::ReactionCommandV1::Set {
                        emoji: None,
                        target: target.clone(),
                        observed: Default::default(),
                        value: c::ReactionValueV1::Like,
                    },
                },
            ),
        )
        .await
        .unwrap()
    else {
        panic!()
    };
    let conditions = w::Conditions {
        emoji: Some("+".into()),
        authors: [ACTOR.to_string()].into(),
        thread: Some(thread.to_string()),
        message: Some(message.to_string()),
        ..Default::default()
    };
    let page = service
        .source_events(
            writer.clone(),
            w::EventQuery {
                after: Some(before_reaction),
                filters: Some(serde_json::to_string(&[conditions.clone()]).unwrap()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(page.events.len(), 1);
    assert_eq!(page.events[0].kind, w::EventKind::ReactionAdded);
    assert_eq!(
        page.events[0].filter_results.get(&conditions.id().unwrap()),
        Some(&true)
    );
    let c::CollabCommandResultV1::Reaction { result: reaction } = reaction.result.clone() else {
        panic!()
    };
    cm::CollabCommandService
        .apply(
            writer.clone(),
            &capply(
                99,
                c::CollabCommandV1::Reaction {
                    intent: c::ReactionCommandV1::Clear {
                        target,
                        observed: reaction.active,
                    },
                },
            ),
        )
        .await
        .unwrap();
    assert!(
        service
            .source_events(
                writer.clone(),
                w::EventQuery {
                    after: Some(page.after),
                    ..Default::default()
                }
            )
            .await
            .unwrap()
            .events
            .is_empty()
    );
}
