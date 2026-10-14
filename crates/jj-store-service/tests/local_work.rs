use jj_store_jj::local_metadata::{Domain, Limits, MetadataStore};
use jj_store_model as m;
use jj_store_service::{
    local_repository::{LocalRepository, LocalWorkOutcome},
    work::*,
};
use std::{fs, process::Command};

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let output = Command::new("jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(root.path().join("repo"))
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let path = root.path().join("repo");
    let store = MetadataStore::open(&path.join(".git"), Limits::default()).unwrap();
    (root, LocalRepository::new(&path.join(".jj/repo"), store))
}
fn request(id: u8, command: WorkCommandV1) -> Request {
    Request {
        version: 1,
        request_id: m::RequestId::new([id; 16]),
        command,
    }
}
fn project(repository: &LocalRepository, id: u8) -> ProjectId {
    let project = ProjectId::new([id; 16]);
    assert!(matches!(
        repository
            .apply_work_command(&request(
                id,
                WorkCommandV1::CreateProject {
                    id: project,
                    title: format!("Project {id}"),
                    body: "Project body".into()
                }
            ))
            .unwrap(),
        LocalWorkOutcome::Committed { .. }
    ));
    project
}
fn task(
    repository: &LocalRepository,
    id: u8,
    project: ProjectId,
    parent: Option<TaskId>,
) -> TaskId {
    let task = TaskId::new([id; 16]);
    repository
        .apply_work_command(&request(
            id,
            WorkCommandV1::CreateTask {
                id: task,
                project,
                expected_project_version: 1,
                title: format!("Task {id}"),
                body: "Task body".into(),
                repository: None,
                parent,
            },
        ))
        .unwrap();
    task
}

#[test]
fn local_reads_do_not_initialize_metadata_or_actor_state() {
    let (root, repository) = fixture();
    let page = repository.work_page(&WorkQuery::default()).unwrap();
    assert_eq!(page.version, 2);
    assert!(page.snapshot.repo_id.is_none());
    assert!(page.projects.is_empty());
    assert_eq!(
        repository
            .work_command_status(m::RequestId::new([1; 16]))
            .unwrap(),
        LocalWorkOutcome::NotKnown {
            request_id: m::RequestId::new([1; 16])
        }
    );
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
    assert!(repository.metadata_store().read().unwrap().is_none());
}

#[test]
fn local_commands_replay_original_results_after_reconstruction_and_git_gc() {
    let (root, repository) = fixture();
    let command = request(
        1,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([1; 16]),
            title: "Home".into(),
            body: "Body".into(),
        },
    );
    let original = repository.apply_work_command(&command).unwrap();
    let selected = repository.metadata_store().read().unwrap().unwrap();
    let actor = repository.actor().unwrap().unwrap();
    project(&repository, 2);
    let path = root.path().join("repo");
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(path.join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(output.status.success());
    let restarted = LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), Limits::default()).unwrap(),
    );
    assert_eq!(restarted.actor().unwrap(), Some(actor));
    assert_eq!(restarted.apply_work_command(&command).unwrap(), original);
    assert_eq!(
        restarted.work_command_status(command.request_id).unwrap(),
        original
    );
    assert_eq!(
        restarted
            .metadata_store()
            .history(&restarted.metadata_store().read().unwrap().unwrap())
            .unwrap()
            .last()
            .unwrap()
            .commit(),
        selected.commit()
    );
    let changed = request(
        1,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([1; 16]),
            title: "Different".into(),
            body: "Body".into(),
        },
    );
    assert_eq!(
        restarted.apply_work_command(&changed).unwrap_err(),
        Error::IntentConflict
    );
}

#[test]
fn local_pagination_remains_pinned_after_later_publications() {
    let (_root, repository) = fixture();
    project(&repository, 3);
    project(&repository, 4);
    let query = WorkQuery {
        limit: 1,
        ..Default::default()
    };
    let first = repository.work_page(&query).unwrap();
    let cursor = first.next_cursor.clone().unwrap();
    project(&repository, 5);
    let second = repository
        .work_page(&WorkQuery {
            cursor: Some(cursor.clone()),
            ..query.clone()
        })
        .unwrap();
    assert_eq!(second.snapshot, first.snapshot);
    assert_eq!(second.projects[0].id, ProjectId::new([4; 16]));
    assert!(second.next_cursor.is_none());
    assert_eq!(
        repository
            .work_page(&WorkQuery {
                cursor: Some(cursor),
                limit: 2,
                ..Default::default()
            })
            .unwrap_err(),
        Error::InvalidCursor
    );
    assert_eq!(
        repository
            .work_page(&WorkQuery::default())
            .unwrap()
            .projects
            .len(),
        3
    );
}

#[test]
fn local_task_hierarchy_and_stale_edits_use_shared_domain_rules() {
    let (_root, repository) = fixture();
    let first = project(&repository, 6);
    let second = project(&repository, 7);
    let parent = task(&repository, 8, first, None);
    let child = task(&repository, 9, first, Some(parent));
    let cross = request(
        10,
        WorkCommandV1::CreateTask {
            id: TaskId::new([10; 16]),
            project: second,
            expected_project_version: 1,
            title: "Cross".into(),
            body: "".into(),
            repository: None,
            parent: Some(parent),
        },
    );
    assert_eq!(
        repository.apply_work_command(&cross).unwrap_err(),
        Error::Invalid
    );
    let cycle = request(
        11,
        WorkCommandV1::UpdateTask {
            id: parent,
            expected_version: 1,
            expected_project_version: 1,
            title: "Parent".into(),
            body: "".into(),
            state: TaskStateV1::Open,
            repository: None,
            parent: Some(child),
        },
    );
    assert_eq!(
        repository.apply_work_command(&cycle).unwrap_err(),
        Error::Invalid
    );
    let stale = request(
        12,
        WorkCommandV1::UpdateProject {
            id: first,
            expected_version: 99,
            title: "Stale".into(),
            body: "".into(),
        },
    );
    assert_eq!(
        repository.apply_work_command(&stale).unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        repository
            .work_page(&WorkQuery {
                project: Some(first),
                ..Default::default()
            })
            .unwrap()
            .tasks
            .len(),
        2
    );
}

#[test]
fn local_threaded_comments_keep_edits_and_tombstones_in_immutable_history() {
    let (_root, repository) = fixture();
    let home = project(&repository, 13);
    let task = task(&repository, 14, home, None);
    let comment = WorkCommentId::new([15; 16]);
    repository
        .apply_work_command(&request(
            15,
            WorkCommandV1::AddComment {
                id: comment,
                task,
                expected_task_version: 1,
                reply_to: None,
                body: "Original".into(),
            },
        ))
        .unwrap();
    let original = repository.metadata_store().read().unwrap().unwrap();
    repository
        .apply_work_command(&request(
            16,
            WorkCommandV1::AddComment {
                id: WorkCommentId::new([16; 16]),
                task,
                expected_task_version: 2,
                reply_to: Some(comment),
                body: "Reply".into(),
            },
        ))
        .unwrap();
    repository
        .apply_work_command(&request(
            17,
            WorkCommandV1::EditComment {
                id: comment,
                expected_version: 1,
                expected_task_version: 3,
                body: "Edited".into(),
            },
        ))
        .unwrap();
    repository
        .apply_work_command(&request(
            18,
            WorkCommandV1::TombstoneComment {
                id: comment,
                expected_version: 2,
                expected_task_version: 4,
            },
        ))
        .unwrap();
    let page = repository
        .work_page(&WorkQuery {
            task: Some(task),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page.comments[0].body, None);
    assert_eq!(page.comments[0].version, 3);
    assert_eq!(page.comments[1].reply_to, Some(comment));
    assert_eq!(page.tasks[0].version, 5);
    let bytes = repository
        .metadata_store()
        .read_domain(&original, Domain::Work)
        .unwrap()
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["comments"][comment.to_string()]["body"], "Original");
}

#[test]
fn invalid_first_commands_leave_the_repository_uninitialized() {
    let (root, repository) = fixture();
    let invalid = request(
        19,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([19; 16]),
            title: "Too much".into(),
            body: "x".repeat(64 * 1024 + 1),
        },
    );
    assert_eq!(
        repository.apply_work_command(&invalid).unwrap_err(),
        Error::Limit
    );
    assert!(repository.metadata_store().read().unwrap().is_none());
    assert!(!root.path().join("repo/.jj/repo/lapis").exists());
    fs::write(
        root.path().join("repo/unsnapshotted.txt"),
        b"preserve user files",
    )
    .unwrap();
    project(&repository, 20);
    assert_eq!(
        fs::read(root.path().join("repo/unsnapshotted.txt")).unwrap(),
        b"preserve user files"
    );
}

#[test]
fn forged_continuations_cannot_read_staged_metadata_candidates() {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    use jj_store_jj::local_metadata::{Preparation, Update};
    let (_root, repository) = fixture();
    project(&repository, 21);
    project(&repository, 22);
    let query = WorkQuery {
        limit: 1,
        ..Default::default()
    };
    let first = repository.work_page(&query).unwrap();
    let selected = repository.metadata_store().read().unwrap().unwrap();
    let update = Update {
        repository: selected.repository(),
        actor: repository.actor().unwrap().unwrap(),
        request: m::RequestId::new([23; 16]),
        intent: m::IntentDigest::new([23; 32]),
        domain: Domain::Work,
        expected: selected.selection(Domain::Work).cloned(),
        bytes: repository
            .metadata_store()
            .read_domain(&selected, Domain::Work)
            .unwrap()
            .unwrap(),
        result: vec![],
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = repository
        .metadata_store()
        .prepare(Some(&selected), &update)
        .unwrap()
    else {
        panic!("expected staged candidate");
    };
    assert_eq!(
        repository
            .work_page_at(candidate.commit(), &WorkQuery::default())
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        repository
            .work_browse_at(candidate.commit(), &WorkBrowseQuery::default())
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        repository
            .work_activity_at(candidate.commit(), &WorkActivityQuery::default())
            .unwrap_err(),
        Error::Stale
    );
    let mut cursor: serde_json::Value =
        serde_json::from_slice(&URL_SAFE_NO_PAD.decode(first.next_cursor.unwrap()).unwrap())
            .unwrap();
    cursor["commit"] = serde_json::to_value(candidate.commit()).unwrap();
    let cursor = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&cursor).unwrap());
    assert_eq!(
        repository
            .work_page(&WorkQuery {
                cursor: Some(cursor),
                ..query
            })
            .unwrap_err(),
        Error::InvalidCursor
    );
    assert_eq!(
        repository
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected.commit()
    );
}

#[test]
fn concurrent_local_writers_share_actor_and_preserve_disjoint_projects() {
    let (_root, repository) = fixture();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
    std::thread::scope(|scope| {
        for id in 24..28 {
            let repository = repository.clone();
            let barrier = barrier.clone();
            scope.spawn(move || {
                let request = request(
                    id,
                    WorkCommandV1::CreateProject {
                        id: ProjectId::new([id; 16]),
                        title: format!("Concurrent {id}"),
                        body: "".into(),
                    },
                );
                barrier.wait();
                for _ in 0..16 {
                    match repository.apply_work_command(&request) {
                        Ok(LocalWorkOutcome::Committed { .. }) => return,
                        Ok(LocalWorkOutcome::OutcomeUnknown { .. }) | Err(Error::Busy) => {
                            std::thread::sleep(std::time::Duration::from_millis(5))
                        }
                        other => panic!("unexpected concurrent outcome: {other:?}"),
                    }
                }
                panic!("concurrent request did not complete");
            });
        }
    });
    let page = repository.work_page(&WorkQuery::default()).unwrap();
    assert_eq!(page.projects.len(), 4);
    let actor = repository.actor().unwrap().unwrap();
    assert!(page.projects.iter().all(|p| p.actor.created_by == actor));
}

#[test]
fn local_browser_browse_uses_complete_root_and_explicit_committed_pin() {
    let (_root, repository) = fixture();
    let first = project(&repository, 30);
    project(&repository, 31);
    let parent = task(&repository, 32, first, None);
    task(&repository, 33, first, Some(parent));
    let selected = repository.metadata_store().read().unwrap().unwrap();
    let query = WorkBrowseQuery {
        limit: 1,
        ..Default::default()
    };
    let page = repository
        .work_browse_at(selected.commit(), &query)
        .unwrap();
    assert_eq!(page.total_matches, 2);
    assert_eq!(page.projects[0].task_counts.total, 2);
    assert_eq!(page.projects[0].task_counts.active, 2);
    project(&repository, 34);
    let resumed = repository
        .work_browse_at(
            selected.commit(),
            &WorkBrowseQuery {
                cursor: page.next_cursor.clone(),
                ..query.clone()
            },
        )
        .unwrap();
    assert_eq!(resumed.snapshot, page.snapshot);
    assert_eq!(resumed.total_matches, 2);
    assert_eq!(resumed.projects[0].id, ProjectId::new([31; 16]));
    assert!(resumed.next_cursor.is_none());
    assert_eq!(
        repository
            .work_browse_at(
                selected.commit(),
                &WorkBrowseQuery {
                    cursor: page.next_cursor,
                    q: "changed".into(),
                    ..query
                }
            )
            .unwrap_err(),
        Error::InvalidCursor
    );
    let tasks = repository
        .work_browse_at(
            selected.commit(),
            &WorkBrowseQuery {
                kind: WorkBrowseKind::Tasks,
                parent: Some(parent),
                q: "Task body".into(),
                state: Some(WorkStateFilter::Active),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(tasks.total_matches, 1);
    assert_eq!(tasks.tasks[0].parent_title.as_deref(), Some("Task 32"));
    assert_eq!(
        repository
            .work_page_at(selected.commit(), &WorkQuery::default())
            .unwrap()
            .projects
            .len(),
        2
    );
    let (_other, other) = fixture();
    project(&other, 35);
    assert_eq!(
        other
            .work_page_at(selected.commit(), &WorkQuery::default())
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        other
            .work_browse_at(selected.commit(), &WorkBrowseQuery::default())
            .unwrap_err(),
        Error::Stale
    );
}

#[test]
fn local_task_activity_pages_pin_complete_counts_and_reject_foreign_or_changed_queries() {
    let (_root, repository) = fixture();
    let first = project(&repository, 70);
    let second = project(&repository, 71);
    task(&repository, 72, first, None);
    task(&repository, 73, first, None);
    task(&repository, 74, second, None);
    let selected = repository.metadata_store().read().unwrap().unwrap();
    let query = WorkActivityQuery {
        limit: 1,
        cursor: None,
    };
    let page = repository
        .work_activity_at(selected.commit(), &query)
        .unwrap();
    assert_eq!(page.groups.len(), 1);
    assert_eq!(page.groups[0].tasks, 2);
    assert_eq!(page.groups[0].project, first);
    assert_eq!(page.total_groups, 2);
    let third = project(&repository, 75);
    task(&repository, 76, third, None);
    let resumed_query = WorkActivityQuery {
        cursor: page.next_cursor.clone(),
        ..query
    };
    let resumed = repository
        .work_activity_at(selected.commit(), &resumed_query)
        .unwrap();
    assert_eq!(resumed.snapshot, page.snapshot);
    assert_eq!(resumed.groups[0].project, second);
    assert_eq!(resumed.groups[0].tasks, 1);
    assert_eq!(resumed.total_groups, 2);
    assert!(resumed.next_cursor.is_none());
    assert_eq!(
        repository
            .work_activity_at(
                selected.commit(),
                &WorkActivityQuery {
                    limit: 2,
                    ..resumed_query.clone()
                }
            )
            .unwrap_err(),
        Error::InvalidCursor
    );
    let current = repository.metadata_store().read().unwrap().unwrap();
    assert_eq!(
        repository
            .work_activity_at(current.commit(), &resumed_query)
            .unwrap_err(),
        Error::InvalidCursor
    );
    let fresh = repository
        .work_activity_at(current.commit(), &WorkActivityQuery::default())
        .unwrap();
    assert_eq!(fresh.total_groups, 3);
    assert_ne!(fresh.snapshot, page.snapshot);
    let (_other, other) = fixture();
    project(&other, 77);
    assert_eq!(
        other
            .work_activity_at(selected.commit(), &WorkActivityQuery::default())
            .unwrap_err(),
        Error::Stale
    );
}

#[test]
fn bound_local_browser_commands_cannot_follow_replaced_repository_identity() {
    let (_root, repository) = fixture();
    project(&repository, 36);
    let selected = repository.metadata_store().read().unwrap().unwrap();
    let bound = repository.clone().bound_to(m::RepositoryId::new([99; 16]));
    assert_eq!(
        bound
            .apply_work_command(&request(
                37,
                WorkCommandV1::CreateProject {
                    id: ProjectId::new([37; 16]),
                    title: "Replacement".into(),
                    body: "".into()
                }
            ))
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        bound
            .work_command_status(m::RequestId::new([36; 16]))
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        bound
            .work_browse_at(selected.commit(), &WorkBrowseQuery::default())
            .unwrap_err(),
        Error::Stale
    );
    assert_eq!(
        repository
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected.commit()
    );
}
