use super::*;

pub(super) async fn edit_submitted_comment(
    local: &jj_store_service::local_repository::LocalRepository,
    supervisor: &Supervisor,
    actor: PrincipalId,
    session: &str,
    feedback: &jj_store_service::agent_execution::Feedback,
    request: RequestId,
) {
    let original = feedback
        .messages
        .iter()
        .flat_map(|m| &m.replies)
        .next()
        .unwrap();
    assert_eq!(original.batches, vec![request]);
    assert!(!original.historical);
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let expected = local
        .inbox_thread(pin, original.thread_id)
        .await
        .unwrap()
        .thread
        .reference;
    local
        .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
            version: 1,
            request_id: RequestId::new([191; 16]),
            command: jj_collab::CollabCommandV1::Thread {
                intent: jj_collab::ThreadCommandV1::Message {
                    emojis: Vec::new(),
                    thread_id: original.thread_id,
                    expected_thread: expected,
                    message_id: original.message.message_id,
                    expected_revision: Some(original.revision),
                    reply_to: None,
                    body: Some("Edited question 🦀".into()),
                },
            },
        })
        .await
        .unwrap();
    let refreshed = supervisor.feedback(actor, session.into()).await.unwrap();
    let revisions: Vec<_> = refreshed
        .messages
        .iter()
        .flat_map(|m| &m.replies)
        .filter(|r| r.message.message_id == original.message.message_id)
        .collect();
    assert_eq!(revisions.len(), 2);
    let old = revisions.iter().find(|r| r.historical).unwrap();
    assert_eq!(old.message.body, original.message.body);
    assert_eq!(old.revision, original.revision);
    assert_eq!(old.batches, vec![request]);
    assert_eq!(old.answers.len(), 1);
    let current = revisions.iter().find(|r| !r.historical).unwrap();
    assert_eq!(current.message.body.as_deref(), Some("Edited question 🦀"));
    assert!(current.answers.is_empty());
    assert!(current.batches.is_empty());
    let captured_session = match &refreshed.messages[0].target {
        jj_collab::ReactionTargetV1::AgentMessage { session_id, .. } => *session_id,
        _ => panic!("expected captured message"),
    };
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let recorded = local
        .browser_agent_session(pin.unwrap(), captured_session)
        .await
        .unwrap();
    assert!(recorded.conversation_unavailable.is_none());
    let recorded = recorded.conversation_feedback.unwrap();
    let revisions: Vec<_> = recorded
        .messages
        .iter()
        .flat_map(|m| &m.replies)
        .filter(|r| r.message.message_id == original.message.message_id)
        .collect();
    assert_eq!(revisions.len(), 2);
    assert_eq!(
        revisions
            .iter()
            .find(|r| r.historical)
            .unwrap()
            .answers
            .len(),
        1
    );
    assert!(
        revisions
            .iter()
            .find(|r| !r.historical)
            .unwrap()
            .answers
            .is_empty()
    );
}

pub(super) async fn tombstone_submitted_comment(
    local: &jj_store_service::local_repository::LocalRepository,
    supervisor: &Supervisor,
    actor: PrincipalId,
    session: &str,
    feedback: &jj_store_service::agent_execution::Feedback,
) {
    let current = feedback
        .messages
        .iter()
        .flat_map(|m| &m.replies)
        .find(|r| !r.historical)
        .unwrap();
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let expected = local
        .inbox_thread(pin, current.thread_id)
        .await
        .unwrap()
        .thread
        .reference;
    local
        .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
            version: 1,
            request_id: RequestId::new([192; 16]),
            command: jj_collab::CollabCommandV1::Thread {
                intent: jj_collab::ThreadCommandV1::Message {
                    emojis: Vec::new(),
                    thread_id: current.thread_id,
                    expected_thread: expected,
                    message_id: current.message.message_id,
                    expected_revision: Some(current.revision),
                    reply_to: None,
                    body: None,
                },
            },
        })
        .await
        .unwrap();
    let refreshed = supervisor.feedback(actor, session.into()).await.unwrap();
    assert!(
        !refreshed
            .messages
            .iter()
            .flat_map(|m| &m.replies)
            .any(|r| r.message.message_id == current.message.message_id)
    );
    let captured = match &refreshed.messages[0].target {
        jj_collab::ReactionTargetV1::AgentMessage { session_id, .. } => *session_id,
        _ => panic!(),
    };
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit
        .unwrap();
    let recorded = local
        .browser_agent_session(pin, captured)
        .await
        .unwrap()
        .conversation_feedback
        .unwrap();
    assert!(
        !recorded
            .messages
            .iter()
            .flat_map(|m| &m.replies)
            .any(|r| r.message.message_id == current.message.message_id)
    );
}
