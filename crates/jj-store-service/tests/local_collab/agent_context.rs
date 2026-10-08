use super::*;
use jj_store_service::agent_execution::context::ReplyRevision;
use jj_store_wal::publication::*;

async fn write(
    repo: &LocalRepository,
    request_id: u8,
    thread: ThreadId,
    id: u8,
    previous: Option<CollabObjectRef>,
    parent: Option<CollabObjectRef>,
    body: Option<&str>,
) -> CollabObjectRef {
    let state = repo.thread_store().await.unwrap().state;
    repo.apply_collab_command(&request(
        request_id,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Message {
                thread_id: thread,
                expected_thread: state.threads[&thread],
                message_id: HumanMessageId::new([id; 16]),
                expected_revision: previous,
                reply_to: parent,
                body: body.map(str::to_owned),
            },
        },
    ))
    .await
    .unwrap();
    repo.thread_store().await.unwrap().state.messages[&HumanMessageId::new([id; 16])]
}
async fn link(
    repo: &LocalRepository,
    req: u8,
    thread: ThreadId,
    session: AgentSessionId,
    snapshot: AgentObjectRef,
    primary: bool,
) -> CollabObjectRef {
    let state = repo.thread_store().await.unwrap().state;
    thread_result(receipt(
        repo.apply_collab_command(&request(
            req,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::Link {
                    thread_id: thread,
                    expected_thread: state.threads[&thread],
                    link_id: ThreadLinkId::new([req; 16]),
                    target: ThreadLinkTargetV1::Session {
                        session_id: session,
                        snapshot,
                        primary,
                    },
                    observed_primary: if primary {
                        state.assignments.get(&session).copied()
                    } else {
                        None
                    },
                },
            },
        ))
        .await
        .unwrap(),
    ))
    .link
    .unwrap()
}
#[tokio::test]
async fn incremental_context_tracks_primary_thread_edits_nested_replies_and_snapshot_races() {
    let (root, repo) = fixture();
    let (session, snapshot, rows) = agent_discussion::capture(&repo).await;
    let sessions = BTreeSet::from([session]);
    let primary = thread_result(receipt(
        repo.apply_collab_command(&thread(1)).await.unwrap(),
    ))
    .thread_id;
    let secondary = thread_result(receipt(
        repo.apply_collab_command(&thread(2)).await.unwrap(),
    ))
    .thread_id;
    let assignment = link(&repo, 3, primary, session, snapshot, true).await;
    link(&repo, 4, secondary, session, snapshot, false).await;
    write(
        &repo,
        5,
        secondary,
        5,
        None,
        None,
        Some("Unrelated secondary discussion"),
    )
    .await;
    let revision = write(
        &repo,
        6,
        primary,
        6,
        None,
        None,
        Some("From repository thread screen"),
    )
    .await;
    let first = repo
        .agent_prompt_context(&rows, &sessions, &BTreeSet::new())
        .await
        .unwrap();
    assert_eq!(first.revisions.len(), 1);
    assert_eq!(first.replies.len(), 1);
    assert!(first.revisions.contains(&first.replies[0].reply));
    assert!(first.text.contains("From repository thread screen"));
    assert!(!first.text.contains("Unrelated"));
    assert!(
        repo.agent_prompt_context(&rows, &sessions, &first.revisions)
            .await
            .unwrap()
            .text
            .is_empty()
    );
    let edited = write(
        &repo,
        7,
        primary,
        6,
        Some(revision),
        None,
        Some("Edited reply"),
    )
    .await;
    write(
        &repo,
        8,
        primary,
        8,
        None,
        Some(edited),
        Some("Nested after snapshot"),
    )
    .await;
    assert!(!first.text.contains("Nested")); // frozen captured batch is unchanged by later commits
    let next = reopen(&root.path().join("repo"))
        .agent_prompt_context(&rows, &sessions, &first.revisions)
        .await
        .unwrap();
    assert_eq!(next.revisions.len(), 2);
    assert!(next.text.contains(&edited.id.to_string()));
    assert!(next.text.contains("reply_to"));
    assert!(!next.text.contains("From repository thread screen"));
    assert!(
        repo.agent_prompt_context(
            &[],
            &BTreeSet::from([AgentSessionId::new([255; 16])]),
            &BTreeSet::new()
        )
        .await
        .unwrap()
        .text
        .is_empty()
    );
    let foreign: BTreeSet<ReplyRevision> = next
        .revisions
        .iter()
        .cloned()
        .map(|mut r| {
            r.repository = m::RepositoryId::new([255; 16]);
            r
        })
        .collect();
    assert_eq!(
        repo.agent_prompt_context(&rows, &sessions, &foreign)
            .await
            .unwrap()
            .revisions
            .len(),
        2
    );
    write(&repo, 9, primary, 6, Some(edited), None, None).await;
    assert_eq!(
        repo.agent_prompt_context(&rows, &sessions, &BTreeSet::new())
            .await
            .unwrap()
            .revisions
            .len(),
        1
    );
    let state = repo.thread_store().await.unwrap().state;
    repo.apply_collab_command(&request(
        10,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Detach {
                thread_id: primary,
                expected_thread: state.threads[&primary],
                expected_link: assignment,
            },
        },
    ))
    .await
    .unwrap();
    assert!(
        repo.agent_prompt_context(&rows, &sessions, &BTreeSet::new())
            .await
            .unwrap()
            .text
            .is_empty()
    );
}
#[tokio::test]
async fn overlapping_discussion_paths_deduplicate_and_limits_apply_only_to_new_revisions() {
    let (_root, repo) = fixture();
    let (session, snapshot, rows) = agent_discussion::capture(&repo).await;
    let target = repo.agent_feedback(&rows).await.unwrap().messages[0]
        .target
        .clone();
    let discussion = thread_result(receipt(
        repo.apply_collab_command(&request(
            1,
            LocalCollabCommand::Thread {
                intent: ThreadCommandV1::CreateAgentDiscussion {
                    thread_id: ThreadId::new([1; 16]),
                    title: "Both paths".into(),
                    target,
                },
            },
        ))
        .await
        .unwrap(),
    ))
    .thread_id;
    link(&repo, 2, discussion, session, snapshot, true).await;
    write(&repo, 3, discussion, 3, None, None, Some("Once only")).await;
    let sessions = BTreeSet::from([session]);
    let first = repo
        .agent_prompt_context(&rows, &sessions, &BTreeSet::new())
        .await
        .unwrap();
    assert_eq!(first.revisions.len(), 1);
    assert_eq!(first.text.matches("Once only").count(), 1);
    write(
        &repo,
        4,
        discussion,
        4,
        None,
        None,
        Some(&"x".repeat(32700)),
    )
    .await;
    assert!(matches!(
        repo.agent_prompt_context(&rows, &sessions, &first.revisions)
            .await,
        Err(jj_store_service::agent_execution::Error::Limit)
    ));
    let state = repo.thread_store().await.unwrap().state;
    let mut excluded = first.revisions;
    let old = excluded.first().unwrap().clone();
    excluded.insert(ReplyRevision {
        repository: old.repository,
        thread: discussion,
        message: HumanMessageId::new([4; 16]),
        revision: state.messages[&HumanMessageId::new([4; 16])],
    });
    assert!(
        repo.agent_prompt_context(&rows, &sessions, &excluded)
            .await
            .unwrap()
            .text
            .is_empty()
    );
    repo.apply_collab_command(&request(
        5,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Lifecycle {
                thread_id: discussion,
                expected: state.threads[&discussion],
                action: ThreadActionV1::Activate,
            },
        },
    ))
    .await
    .unwrap();
    let state = repo.thread_store().await.unwrap().state;
    repo.apply_collab_command(&request(
        6,
        LocalCollabCommand::Thread {
            intent: ThreadCommandV1::Lifecycle {
                thread_id: discussion,
                expected: state.threads[&discussion],
                action: ThreadActionV1::Tombstone,
            },
        },
    ))
    .await
    .unwrap();
    assert!(
        repo.agent_prompt_context(&rows, &sessions, &BTreeSet::new())
            .await
            .unwrap()
            .text
            .is_empty()
    );
}
