use super::*;
use jj_collab::{
    ConversationCommentDraftV1, ConversationSourceV1, ConversationSubmissionV1, HumanMessageId,
    ThreadId,
};
use jj_store_service::agent_execution::conversations as chat;
use serde_json::json;
#[path = "comment_answers/history.rs"]
mod history;

#[tokio::test]
async fn comments_only_batches_link_multiple_answers_in_one_turn_and_survive_restart() {
    exercise_comments(true).await;
}

#[tokio::test]
async fn standalone_comments_only_prompt_publishes_and_recovers_answer_links() {
    exercise_comments(false).await;
}

async fn exercise_comments(managed: bool) {
    let fixture = Fixture::new().await;
    let original = fs::read_to_string(&fixture.resources.executable).unwrap();
    let marker = "        emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=f'Fixture in {workspace}')))";
    assert!(original.contains(marker));
    let replacement = r#"        import re
        refs = re.findall(r'- Discussion: ([a-z0-9-]+); comment: ([a-z0-9-]+);', frame.get('context', ''))
        groups = {}
        for discussion, comment in refs:
            groups.setdefault(discussion, []).append(comment)
        answer = f'Fixture in {workspace}'
        if groups:
            answer = 'Ordinary introduction.\n' + '\nBetween answers.\n'.join('<comment-reply discussion="' + d + '" comments="' + ' '.join(cs) + '">\nAnswered 🦀.\n</comment-reply>' for d, cs in groups.items())
        middle = len(answer) // 2
        for chunk in [answer[:middle], answer[middle:]]:
            emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=chunk)))"#;
    fs::write(
        &fixture.resources.executable,
        original.replace(marker, replacement),
    )
    .unwrap();
    let actor = fixture.actor;
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(resources.clone()).unwrap();
    let _publication = supervisor.start_publication();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    supervisor
        .save_setup(
            actor,
            SetupCommand {
                version: 1,
                request_id: RequestId::new([181; 16]),
                expected_revision: supervisor.status(actor).await.unwrap().revision,
                setup: Some(Setup {
                    options: Default::default(),
                    id: "comments".into(),
                    name: "Comment agent".into(),
                    avatar: "🤖".into(),
                    description: String::new(),
                    instructions: "Test fixture".into(),
                    runner: "codex".into(),
                    preferences: Preferences {
                        model: None,
                        effort: None,
                        mode: Some("ask".into()),
                    },
                }),
                remove: None,
            },
        )
        .await
        .unwrap();
    let mut send = chat::Send {
        thread_codebase: None,
        version: 1,
        request_id: RequestId::new([182; 16]),
        conversation_id: "b1".repeat(16),
        origin: chat::Origin {
            scope: "personal".into(),
            home: None,
            thread: None,
            agent_target: None,
            context: "".into(),
        },
        agent_ids: vec!["comments".into()],
        attachments: vec![],
        text: "Start".into(),
        comments: None,
        max_duration_ms: 5000,
    };
    let standalone = if managed {
        None
    } else {
        let target = supervisor.targets(actor, None).await.unwrap().remove(0);
        let launched = supervisor
            .launch(
                actor,
                Launch {
                    version: 1,
                    request_id: RequestId::new([180; 16]),
                    target_id: target.id,
                    repository: target.repository,
                    operation: target.operation,
                    commit: target.commit,
                    runner: "codex".into(),
                    setup_id: Some("comments".into()),
                    context: None,
                },
            )
            .await
            .unwrap();
        let id = launched.session_id.unwrap();
        wait(&supervisor, actor, |s| {
            s.sessions.iter().any(|s| s.id == id && s.state == "idle")
        })
        .await;
        Some(id)
    };
    let first = send_batch(&supervisor, actor, &send, standalone.as_deref()).await;
    assert!(first.agents[0].error.is_none(), "{first:?}");
    let session = first.agents[0].session_id.clone().unwrap();
    let mut observed = String::new();
    let feedback = tokio::time::timeout(Duration::from_secs(45), async {
        loop {
            let feedback = supervisor.feedback(actor, session.clone()).await.unwrap();
            observed = format!(
                "feedback={feedback:?}; status={:?}; events={:?}",
                supervisor.status(actor).await.unwrap().sessions,
                supervisor.events(actor, session.clone(), 0).await.unwrap()
            );
            if feedback
                .messages
                .iter()
                .any(|m| m.row_id.ends_with("/assistant"))
            {
                break feedback;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap_or_else(|e| panic!("{e}: {observed}"));
    let source = feedback
        .messages
        .iter()
        .find(|m| m.row_id.ends_with("/assistant"))
        .unwrap();
    assert!(source.text_parts[0].text.starts_with("Fixture"));
    let status = supervisor.status(actor).await.unwrap();
    let home = status
        .sessions
        .iter()
        .find(|s| s.id == session)
        .unwrap()
        .repository;
    let thread = if managed {
        status.conversations[0].discussion_thread.unwrap()
    } else {
        ThreadId::new([190; 16])
    };
    let local = resources.folders.as_ref().unwrap().open(home).unwrap();
    if !managed {
        local
            .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
                version: 1,
                request_id: RequestId::new([190; 16]),
                command: jj_collab::CollabCommandV1::Thread {
                    intent: jj_collab::ThreadCommandV1::Create {
                        thread_id: thread,
                        title: "Standalone conversation".into(),
                        baseline: None,
                    },
                },
            })
            .await
            .unwrap();
    }
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let expected = local
        .inbox_thread(pin, thread)
        .await
        .unwrap()
        .thread
        .reference;
    let selector = ConversationSourceV1 {
        target: source.target.clone(),
        part: None,
        quote: "Fixture".into(),
        selection: Some(jj_review::TextSelectionV1 {
            encoding: jj_review::TextEncoding::Utf8,
            start_byte: jj_review::ByteOffset::new(0),
            end_byte: jj_review::ByteOffset::new(7),
            selected_sha256: jj_review::ReviewDigest::sha256(b"Fixture"),
        }),
    };
    let comments = (0..3)
        .map(|i| ConversationCommentDraftV1 {
            thread_id: ThreadId::new([if i < 2 { 184 } else { 185 }; 16]),
            expected_thread: None,
            source: if i < 2 {
                selector.clone()
            } else {
                ConversationSourceV1 {
                    target: source.target.clone(),
                    part: None,
                    selection: None,
                    quote: String::new(),
                }
            },
            message_id: HumanMessageId::new([186 + i; 16]),
            expected_revision: None,
            reply_to: None,
            body: format!("Question {i}"),
        })
        .collect();
    send.request_id = RequestId::new([183; 16]);
    send.text.clear();
    send.comments = Some(Box::new(ConversationSubmissionV1 {
        emojis: Vec::new(),
        workspace_comments: vec![],
        reply_to: None,
        thread_id: thread,
        expected_thread: expected,
        message_id: None,
        text: None,
        comments,
        references: vec![],
    }));
    let result = send_batch(&supervisor, actor, &send, standalone.as_deref()).await;
    assert_eq!(
        result.comment_publication.as_ref().unwrap().state,
        "published"
    );
    assert!(result.agents[0].error.is_none(), "{result:?}");
    let replied = tokio::time::timeout(Duration::from_secs(45), async {
        loop {
            let feedback = supervisor.feedback(actor, session.clone()).await.unwrap();
            let replies: Vec<_> = feedback.messages.iter().flat_map(|m| &m.replies).collect();
            if replies.len() == 3 && replies.iter().all(|r| r.answers.len() == 1) {
                break feedback;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        replied
            .messages
            .iter()
            .map(|m| m.answers.len())
            .sum::<usize>(),
        2
    );
    let page = supervisor.events(actor, session.clone(), 0).await.unwrap();
    let prompt = page
        .events
        .iter()
        .find(|e| {
            e.payload["type"] == "prompt"
                && e.payload["requestId"] == result.agents[0].request_id.to_string()
        })
        .unwrap();
    assert_eq!(prompt.payload["userText"], json!(""));
    assert_eq!(
        prompt.payload["sentContext"]["commentReplyProtocol"]["references"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        prompt.payload["sentContext"]["commentSubmission"]["request_id"],
        json!(result.comment_publication.as_ref().unwrap().request_id)
    );
    assert_eq!(
        prompt.payload["sentContext"]["commentSubmission"]["thread_id"],
        json!(thread)
    );
    history::edit_submitted_comment(
        &local,
        &supervisor,
        actor,
        &session,
        &replied,
        result.comment_publication.as_ref().unwrap().request_id,
    )
    .await;
    let last = page.events.last().unwrap().sequence;
    let duplicate = send_batch(&supervisor, actor, &send, standalone.as_deref()).await;
    assert_eq!(
        duplicate.comment_publication.unwrap().batch,
        result.comment_publication.unwrap().batch
    );
    assert_eq!(
        supervisor
            .events(actor, session.clone(), 0)
            .await
            .unwrap()
            .events
            .last()
            .unwrap()
            .sequence,
        last
    );
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(resources).unwrap();
    let feedback = reopened.feedback(actor, session.clone()).await.unwrap();
    assert_eq!(
        feedback
            .messages
            .iter()
            .flat_map(|m| &m.replies)
            .filter(|r| r.answers.len() == 1)
            .count(),
        3
    );
    let duplicate = send_batch(&reopened, actor, &send, standalone.as_deref()).await;
    assert_eq!(duplicate.agents[0].state, "completed");
    assert_eq!(duplicate.comment_publication.unwrap().state, "published");
    history::tombstone_submitted_comment(&local, &reopened, actor, &session, &feedback).await;
    reopened.shutdown().await.unwrap();
}

async fn send_batch(
    supervisor: &Supervisor,
    actor: PrincipalId,
    send: &chat::Send,
    standalone: Option<&str>,
) -> chat::SendOutcome {
    if let Some(session) = standalone {
        supervisor
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: send.request_id,
                    session_id: session.into(),
                    action: Action::Prompt {
                        text: send.text.clone(),
                        comments: send.comments.clone(),
                        max_duration_ms: send.max_duration_ms,
                    },
                },
            )
            .await
            .unwrap();
        let outcome = supervisor.outcome(actor, send.request_id).await.unwrap();
        chat::SendOutcome {
            request_id: send.request_id,
            conversation_id: send.conversation_id.clone(),
            comment_publication: outcome.comment_publication.clone(),
            agents: vec![outcome],
        }
    } else {
        supervisor
            .send_conversation(actor, send.clone())
            .await
            .unwrap()
    }
}
