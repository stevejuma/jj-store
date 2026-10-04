use jj_store_model::PrincipalId;
use jj_work::*;

#[test]
fn rejects_cross_project_hierarchy_cross_task_replies_and_invalid_admission() {
    let actor = PrincipalId::new([1; 16]);
    let mut state = WorkStateV1::default();
    let mut apply = |command: WorkCommandV1| {
        state = state.apply(&command, actor, 1).unwrap().0;
    };
    let a = ProjectId::new([2; 16]);
    let b = ProjectId::new([3; 16]);
    for id in [a, b] {
        apply(WorkCommandV1::CreateProject {
            id,
            title: "Project".into(),
            body: "".into(),
        });
    }
    let x = TaskId::new([4; 16]);
    let y = TaskId::new([5; 16]);
    for (id, project) in [(x, a), (y, b)] {
        apply(WorkCommandV1::CreateTask {
            id,
            project,
            expected_project_version: 1,
            title: "Task".into(),
            body: "".into(),
            repository: None,
            parent: None,
        });
    }
    let comment = WorkCommentId::new([6; 16]);
    apply(WorkCommandV1::AddComment {
        id: comment,
        task: x,
        expected_task_version: 1,
        reply_to: None,
        body: "Comment".into(),
    });
    assert_eq!(
        state
            .apply(
                &WorkCommandV1::UpdateTask {
                    id: y,
                    expected_version: 1,
                    expected_project_version: 1,
                    title: "Task".into(),
                    body: "".into(),
                    state: TaskStateV1::Open,
                    repository: None,
                    parent: Some(x)
                },
                actor,
                2
            )
            .unwrap_err(),
        Error::Invalid("cross-project parent")
    );
    assert_eq!(
        state
            .apply(
                &WorkCommandV1::AddComment {
                    id: WorkCommentId::new([7; 16]),
                    task: y,
                    expected_task_version: 1,
                    reply_to: Some(comment),
                    body: "Reply".into()
                },
                actor,
                2
            )
            .unwrap_err(),
        Error::Invalid("cross-task reply")
    );
    assert!(
        WorkCommandV1::CreateProject {
            id: a,
            title: " ".into(),
            body: "".into()
        }
        .validate()
        .is_err()
    );
    assert!(
        WorkCommandV1::CreateProject {
            id: a,
            title: "Long".into(),
            body: "x".repeat(MAX_BODY_BYTES + 1)
        }
        .validate()
        .is_err()
    );
    assert!(
        WorkCommandV1::AddComment {
            id: comment,
            task: x,
            expected_task_version: 2,
            reply_to: None,
            body: " ".into()
        }
        .validate()
        .is_err()
    );
    assert_eq!(state.tasks[&y].version, 1);
    let edited = state
        .apply(
            &WorkCommandV1::EditComment {
                id: comment,
                expected_version: 1,
                expected_task_version: 2,
                body: "New body".into(),
            },
            PrincipalId::new([8; 16]),
            2,
        )
        .unwrap()
        .0;
    assert_eq!(edited.comments[&comment].actor.created_by, actor);
    assert_eq!(
        edited.comments[&comment].actor.updated_by,
        PrincipalId::new([8; 16])
    );
}
