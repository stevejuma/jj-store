use super::*;
use jj_store_wal::publication::*;

#[tokio::test]
async fn saved_agent_replies_and_reactions_survive_restart_and_keep_exact_targets() {
    let (root, repo) = fixture();
    let (_, _, rows) = capture(&repo).await;
    let feedback = repo.agent_feedback(&rows).await.unwrap();
    assert_eq!(feedback.messages.len(), 2);
    for (i, message) in feedback.messages.iter().enumerate() {
        let target = message.target.clone();
        let create = request(
            90 + i as u8,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::CreateAgentDiscussion {
                    thread_id: ThreadId::new([90 + i as u8; 16]),
                    title: "Reply".into(),
                    target: target.clone(),
                },
            },
        );
        let created = thread_result(receipt(repo.apply_collab_command(&create).await.unwrap()));
        let reply = request(
            100 + i as u8,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Message {
                    emojis: Vec::new(),
                    thread_id: created.thread_id,
                    expected_thread: created.version,
                    message_id: HumanMessageId::new([100 + i as u8; 16]),
                    expected_revision: None,
                    reply_to: None,
                    body: Some(format!("Feedback {i}")),
                },
            },
        );
        let accepted = repo.apply_collab_command(&reply).await.unwrap();
        assert_eq!(repo.apply_collab_command(&reply).await.unwrap(), accepted);
        repo.apply_collab_command(&react(
            110 + i as u8,
            target.clone(),
            ReactionValueV1::Like,
            BTreeSet::new(),
        ))
        .await
        .unwrap();
        assert_eq!(
            repo.reaction_query(&target, None, 20, None)
                .await
                .unwrap()
                .rows
                .len(),
            1
        );
    }
    // The sidebar catalog contains user threads; agent discussions are direct targets.
    for id in [160, 161] {
        repo.apply_collab_command(&request(
            id,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: ThreadId::new([id; 16]),
                    title: "User thread for pagination".into(),
                    baseline: None,
                },
            },
        ))
        .await
        .unwrap();
    }
    let reopened = reopen(&root.path().join("repo"));
    let selection = reopened.inbox_selection().await.unwrap();
    let pin = selection.snapshot.metadata_commit;
    let page = reopened
        .inbox_page(
            pin,
            &jj_store_service::collab_pages::Query::Threads { limit: 1 },
            None,
        )
        .await
        .unwrap();
    assert_eq!(page.snapshot.metadata_commit, pin);
    assert!(page.next_after.is_some());
    let more = reopened
        .inbox_page(
            pin,
            &jj_store_service::collab_pages::Query::Threads { limit: 1 },
            page.next_after,
        )
        .await
        .unwrap();
    assert!(more.next_after.is_none());
    let thread = ThreadId::new([90; 16]);
    let detail = reopened.inbox_thread(pin, thread).await.unwrap();
    let events = reopened
        .inbox_page(
            pin,
            &jj_store_service::collab_pages::Query::Events {
                thread_id: thread,
                limit: 1,
            },
            None,
        )
        .await
        .unwrap();
    assert!(events.next_after.is_some());
    let foreign = reopened
        .inbox_thread(pin, ThreadId::new([91; 16]))
        .await
        .unwrap();
    assert!(
        reopened
            .inbox_page(
                pin,
                &jj_store_service::collab_pages::Query::Events {
                    thread_id: thread,
                    limit: 1
                },
                Some(serde_json::to_string(&foreign.thread.value.event).unwrap())
            )
            .await
            .is_err()
    );
    assert!(reopened.inbox_thread(None, thread).await.is_err());
    assert_eq!(
        detail.thread.value.agent_message.as_ref(),
        Some(&feedback.messages[0].target)
    );
    let feedback = reopened.agent_feedback(&rows).await.unwrap();
    assert!(feedback.messages.iter().all(|m| m.replies.len() == 1));
    assert!(feedback.prompt_context().unwrap().contains("Feedback 0"));
    assert!(feedback.prompt_context().unwrap().contains("Feedback 1"));
    let ReactionTargetV1::AgentMessage {
        session_id,
        message_id: _,
        event_id,
        batch,
        content,
    } = feedback.messages[0].target.clone()
    else {
        panic!()
    };
    let forged = ReactionTargetV1::AgentMessage {
        session_id,
        message_id: MessageId::new([255; 16]),
        event_id,
        batch,
        content,
    };
    assert!(
        reopened
            .apply_collab_command(&request(
                120,
                LocalCollabCommand::Thread {
                    intent: ThreadCommandV1::CreateAgentDiscussion {
                        thread_id: ThreadId::new([120; 16]),
                        title: "Forged".into(),
                        target: forged
                    }
                }
            ))
            .await
            .is_err()
    );
    let reply = &feedback.messages[0].replies[0];
    let thread = reopened.thread_store().await.unwrap();
    reopened
        .apply_collab_command(&request(
            121,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Message {
                    emojis: Vec::new(),
                    thread_id: reply.thread_id,
                    expected_thread: thread.state.threads[&reply.thread_id],
                    message_id: reply.message.message_id,
                    expected_revision: Some(reply.revision),
                    reply_to: None,
                    body: None,
                },
            },
        ))
        .await
        .unwrap();
    assert_eq!(
        reopened.agent_feedback(&rows).await.unwrap().messages[0]
            .replies
            .len(),
        0
    );
}

pub(super) async fn capture(
    repo: &LocalRepository,
) -> (
    AgentSessionId,
    AgentObjectRef,
    Vec<(
        String,
        AgentSessionId,
        jj_store_service::agent_execution::FeedbackSubject,
    )>,
) {
    let batch = SanitizedObservationBatchV1 {
        schema_version:1, batch_id:CaptureBatchId::new([81;16]), integration:IntegrationId::OwnedController,
        source_instance_id:SourceInstanceId::new([82;16]), sanitizer_version:"test".into(),
        representation:SanitizedRepresentation::SanitizedNative,
        observations: [serde_json::json!({"type":"prompt","controllerSessionId":"live","requestId":"turn","text":"Question"}),
            serde_json::json!({"type":"record","controllerSessionId":"live","requestId":"turn","assistantText":"Answer"})]
            .into_iter().enumerate().map(|(i,payload)| SanitizedObservationV1 {
                observation_id:ObservationId::new([i as u8+1;16]),source_sequence:i as u64+1,
                occurred_at_millis:Some(10+i as i64),source_kind:SourceKind::Controller,
                source_trust:SourceTrust::AuthenticatedController,
                payload:SanitizedPayloadV1::new(SanitizedMediaType::Json,&serde_json::to_vec(&payload).unwrap()).unwrap()
            }).collect(),
    };
    let profile = NormalizerProfileV1::acp_controller_v2();
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([81; 16]),
        command: LocalAgentCommand::Capture {
            profile,
            batch: Box::new(batch),
        },
    })
    .await
    .unwrap();
    let agents = repo.agent_store().await.unwrap();
    let selected = agents
        .reads()
        .list_sessions(None, 20)
        .await
        .unwrap()
        .selections[0]
        .1;
    let AgentObjectV1::SessionSnapshot(session) = agents.object(selected).await.unwrap() else {
        panic!()
    };
    let mut rows = Vec::new();
    for batch in &session.event_batches {
        let AgentObjectV1::EventBatch(events) = agents.object(*batch).await.unwrap() else {
            panic!()
        };
        for event in events.events {
            if let AgentEventV1::MessageCompleted {
                session_id,
                message_id,
                ..
            } = event
            {
                rows.push((
                    message_id.to_string(),
                    session_id,
                    jj_store_service::agent_execution::FeedbackSubject::Message(message_id),
                ));
            }
        }
    }
    (session.session_id, selected, rows)
}

#[tokio::test]
async fn tool_feedback_targets_selected_output_and_retains_replies_and_reactions() {
    use jj_store_service::agent_execution::FeedbackSubject;
    let (root, repo) = fixture();
    let mut payload = serde_json::json!({"type":"activity","controllerSessionId":"tool-chat","requestId":"turn","toolSnapshot":{"toolCallId":"read","kind":"read","title":"Read file","status":"completed","rawOutput":{"formatted_output":"captured bytes"}}});
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([71; 16]),
        integration: IntegrationId::OwnedController,
        source_instance_id: SourceInstanceId::new([72; 16]),
        sanitizer_version: "test".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([73; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(123),
            source_kind: SourceKind::Controller,
            source_trust: SourceTrust::AuthenticatedController,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&payload).unwrap(),
            )
            .unwrap(),
        }],
    };
    repo.apply_agent_command(&LocalAgentRequest {
        version: 1,
        request_id: m::RequestId::new([71; 16]),
        command: LocalAgentCommand::Capture {
            profile: NormalizerProfileV1::acp_controller_v2(),
            batch: Box::new(batch),
        },
    })
    .await
    .unwrap();
    let agents = repo.agent_store().await.unwrap();
    let selected = agents
        .reads()
        .list_sessions(None, 20)
        .await
        .unwrap()
        .selections[0]
        .1;
    let AgentObjectV1::SessionSnapshot(session) = agents.object(selected).await.unwrap() else {
        panic!()
    };
    let tool_id = *session.tools.keys().next().unwrap();
    let rows = vec![(
        "turn/tool/read".into(),
        session.session_id,
        FeedbackSubject::Tool(tool_id),
    )];
    let target = repo.agent_feedback(&rows).await.unwrap().messages[0]
        .target
        .clone();
    assert!(matches!(target,ReactionTargetV1::ToolOutput {tool_id:id,..} if id==tool_id));
    let created = thread_result(receipt(
        repo.apply_collab_command(&request(
            75,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::CreateAgentDiscussion {
                    thread_id: ThreadId::new([75; 16]),
                    title: "Tool reply".into(),
                    target: target.clone(),
                },
            },
        ))
        .await
        .unwrap(),
    ));
    repo.apply_collab_command(&request(
        76,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Message {
                emojis: Vec::new(),
                thread_id: created.thread_id,
                expected_thread: created.version,
                message_id: HumanMessageId::new([76; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("Use the captured file".into()),
            },
        },
    ))
    .await
    .unwrap();
    repo.apply_collab_command(&react(
        77,
        target.clone(),
        ReactionValueV1::Like,
        BTreeSet::new(),
    ))
    .await
    .unwrap();
    let reopened = reopen(&root.path().join("repo"));
    let feedback = reopened.agent_feedback(&rows).await.unwrap();
    assert_eq!(feedback.messages[0].target, target);
    assert_eq!(feedback.messages[0].replies.len(), 1);
    assert_eq!(
        reopened
            .reaction_query(&target, None, 20, None)
            .await
            .unwrap()
            .rows
            .len(),
        1
    );
    let context = reopened
        .agent_prompt_context(
            &rows,
            &BTreeSet::from([session.session_id]),
            &BTreeSet::new(),
        )
        .await
        .unwrap();
    assert!(context.text.contains("Use the captured file"));
    assert!(context.text.contains("tool_output"));
    assert!(
        reopened
            .agent_prompt_context(
                &rows,
                &BTreeSet::from([session.session_id]),
                &context.revisions
            )
            .await
            .unwrap()
            .text
            .is_empty()
    );
    payload["toolSnapshot"]["toolCallId"] = serde_json::json!("forged");
    let forged = [(
        "turn/tool/forged".into(),
        session.session_id,
        FeedbackSubject::Tool(ToolCallId::new([255; 16])),
    )];
    assert!(
        reopened
            .agent_feedback(&forged)
            .await
            .unwrap()
            .messages
            .is_empty()
    );
}
