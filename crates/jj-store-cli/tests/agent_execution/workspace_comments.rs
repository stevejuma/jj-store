use super::*;
use jj_collab::*;
use jj_store_service::agent_execution::workspace_inspection as inspect;

#[tokio::test]
async fn workspace_comments_freeze_group_publish_and_recover_without_duplicate_reviews() {
    let fixture = Fixture::new().await;
    let control = fs::read_to_string(&fixture.resources.executable).unwrap();
    let marker = "        emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=f'Fixture in {workspace}')))";
    assert!(control.contains(marker));
    fs::write(&fixture.resources.executable, control.replace(marker, r#"        import re
        refs = re.findall(r'- Discussion: ([a-z0-9-]+); comment: ([a-z0-9-]+);', frame.get('context', ''))
        groups = {}
        for d, c in refs:
            groups.setdefault(d, []).append(c)
        answer = '\n'.join('<comment-reply discussion="' + d + '" comments="' + ' '.join(cs) + '">Retained file answer 🦀.</comment-reply>' for d, cs in groups.items())
        emit('activity', request, update=dict(sessionUpdate='agent_message_chunk', content=dict(type='text', text=answer or 'Ready')))"#)).unwrap();
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
    let target = supervisor.targets(actor, None).await.unwrap().remove(0);
    let launched = supervisor
        .launch(
            actor,
            Launch {
                version: 1,
                request_id: RequestId::new([201; 16]),
                target_id: target.id,
                repository: target.repository,
                operation: target.operation,
                commit: target.commit,
                runner: "codex".into(),
                setup_id: None,
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
    let session = supervisor
        .status(actor)
        .await
        .unwrap()
        .sessions
        .into_iter()
        .find(|s| s.id == id)
        .unwrap();
    let file = Path::new(&session.workspace).join("unchanged.txt");
    fs::write(&file, "café 🦀\nsecond line\n").unwrap();
    let query = inspect::Request {
        version: 1,
        session: id.clone(),
        repository: session.repository,
        query: inspect::InspectionView::Files,
        path: Some("unchanged.txt".into()),
        cursor: None,
    };
    let observed = supervisor.inspect_workspace(actor, query).await.unwrap();
    let observed_file = observed.file.unwrap();
    let local = resources
        .folders
        .as_ref()
        .unwrap()
        .open(session.repository)
        .unwrap();
    let thread = ThreadId::new([202; 16]);
    let created = local
        .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
            version: 1,
            request_id: RequestId::new([202; 16]),
            command: CollabCommandV1::Thread {
                intent: ThreadCommandV1::Create {
                    thread_id: thread,
                    title: "File reviews".into(),
                    baseline: None,
                },
            },
        })
        .await
        .unwrap();
    let jj_store_service::local_repository::LocalCollabOutcome::Committed { receipt } = created
    else {
        panic!()
    };
    let CollabCommandResultV1::Thread { result } = receipt.result else {
        panic!()
    };
    let source = WorkspaceFileSourceV1 {
        session: id.parse().unwrap(),
        repository: session.repository,
        workspace: observed.workspace,
        path: "unchanged.txt".into(),
        operation: observed.operation,
        commit: observed.commit,
        content_sha256: jj_review::ReviewDigest::sha256(
            observed_file.contents.as_ref().unwrap().as_bytes(),
        ),
        current_fingerprint: Some(observed_file.fingerprint),
        selection: Some(jj_review::TextSelectionV1 {
            encoding: jj_review::TextEncoding::Utf8,
            start_byte: jj_review::ByteOffset::new(0),
            end_byte: jj_review::ByteOffset::new("café 🦀".len() as u64),
            selected_sha256: jj_review::ReviewDigest::sha256("café 🦀".as_bytes()),
        }),
        diff: None,
    };
    let drafts = (0..2)
        .map(|i| WorkspaceCommentDraftV1 {
            id: RequestId::new([203 + i; 16]),
            source: source.clone(),
            body: format!("Review question {i}"),
            previous: None,
            edit: false,
        })
        .collect();
    let command = Command {
        version: 1,
        request_id: RequestId::new([205; 16]),
        session_id: id.clone(),
        action: Action::Prompt {
            text: String::new(),
            max_duration_ms: 5000,
            comments: Some(Box::new(ConversationSubmissionV1 {
                emojis: Vec::new(),
                thread_id: thread,
                expected_thread: result.version,
                text: None,
                message_id: None,
                reply_to: None,
                comments: vec![],
                references: vec![],
                workspace_comments: drafts,
            })),
        },
    };
    // Current bytes changed after selection: refuse without snapshotting or publication.
    let mut stale = command.clone();
    stale.request_id = RequestId::new([211; 16]);
    fs::write(&file, "changed before send\n").unwrap();
    let checkout =
        fs::read(Path::new(&session.workspace).join(".jj/working_copy/checkout")).unwrap();
    supervisor.command(actor, stale.clone()).await.unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if supervisor
                .outcome(actor, stale.request_id)
                .await
                .unwrap()
                .state
                == "failed"
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        supervisor
            .outcome(actor, stale.request_id)
            .await
            .unwrap()
            .comment_publication
            .unwrap()
            .state,
        "refused"
    );
    assert_eq!(
        checkout,
        fs::read(Path::new(&session.workspace).join(".jj/working_copy/checkout")).unwrap()
    );
    assert!(
        local
            .review_threads_at(None, 50, None)
            .await
            .unwrap()
            .threads
            .is_empty()
    );
    fs::write(&file, "café 🦀\nsecond line\n").unwrap();
    let current = supervisor
        .inspect_workspace(
            actor,
            inspect::Request {
                version: 1,
                session: id.clone(),
                repository: session.repository,
                query: inspect::InspectionView::Files,
                path: Some("unchanged.txt".into()),
                cursor: None,
            },
        )
        .await
        .unwrap();
    let mut command = command;
    if let Action::Prompt {
        comments: Some(submission),
        ..
    } = &mut command.action
    {
        for draft in &mut submission.workspace_comments {
            draft.source.current_fingerprint =
                Some(current.file.as_ref().unwrap().fingerprint.clone());
        }
    }
    supervisor.command(actor, command.clone()).await.unwrap();
    let outcome = tokio::time::timeout(Duration::from_secs(45), async {
        loop {
            let out = supervisor.outcome(actor, command.request_id).await.unwrap();
            if matches!(
                out.state.as_str(),
                "completed" | "failed" | "outcome_unknown"
            ) {
                break out;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(outcome.state, "completed", "{outcome:?}");
    assert_eq!(
        outcome.comment_publication.as_ref().unwrap().state,
        "published"
    );
    let threads = local.review_threads_at(None, 50, None).await.unwrap();
    assert_eq!(
        threads.threads.len(),
        1,
        "same exact source shares one discussion"
    );
    let events = supervisor.events(actor, id.clone(), 0).await.unwrap();
    let prompt = events
        .events
        .iter()
        .find(|e| {
            e.payload["type"] == "prompt"
                && e.payload["requestId"] == command.request_id.to_string()
        })
        .unwrap();
    assert_eq!(prompt.payload["userText"], "");
    assert_eq!(
        prompt.payload["sentContext"]["commentReplyProtocol"]["references"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(!prompt.payload.to_string().contains("deferredFiles"));
    tokio::time::timeout(Duration::from_secs(45), async {
        loop {
            let feedback = supervisor.feedback(actor, id.clone()).await.unwrap();
            if feedback
                .messages
                .iter()
                .map(|m| m.answers.len())
                .sum::<usize>()
                == 1
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    fs::remove_file(&file).unwrap();
    let before = local.metadata_store().read().unwrap().unwrap().commit();
    supervisor.command(actor, command.clone()).await.unwrap();
    assert_eq!(
        local.metadata_store().read().unwrap().unwrap().commit(),
        before
    );
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(resources).unwrap();
    let duplicate = reopened.command(actor, command.clone()).await.unwrap();
    assert_eq!(duplicate.state, "completed");
    assert_eq!(
        reopened
            .outcome(actor, command.request_id)
            .await
            .unwrap()
            .comment_publication
            .unwrap()
            .state,
        "published"
    );
    assert_eq!(
        local
            .review_threads_at(None, 50, None)
            .await
            .unwrap()
            .threads
            .len(),
        1
    );
    reopened.shutdown().await.unwrap();
}
