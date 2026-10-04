use super::*;
use jj_store_wal::publication::WorkStateV1;

fn fixture() -> WorkStateV1 {
    let project: ProjectV1 = serde_json::from_value(serde_json::json!({
        "schema_version": 1, "id": ProjectId::new([1; 16]), "title": "Project",
        "body": "", "version": 1, "repositories": {}, "actor": {
            "created_by": jj_store_model::PrincipalId::new([1; 16]),
            "updated_by": jj_store_model::PrincipalId::new([1; 16]),
            "created_at_millis": 1, "updated_at_millis": 1
        }
    }))
    .unwrap();
    let actor = project.actor;
    let mut state = WorkStateV1::default();
    for byte in 1..=100 {
        let project = ProjectId::new([byte; 16]);
        state.projects.insert(
            project,
            ProjectV1 {
                schema_version: 1,
                id: project,
                title: format!("Project {byte:03}"),
                body: "A searchable NEEDLE".into(),
                version: 1,
                repositories: Default::default(),
                actor: actor.clone(),
            },
        );
        let id = TaskId::new([byte; 16]);
        state.tasks.insert(
            id,
            TaskV1 {
                schema_version: 1,
                id,
                project,
                title: format!("Task {byte:03}"),
                body: "A searchable NEEDLE".into(),
                state: TaskStateV1::Open,
                repository: None,
                parent: None,
                version: 1,
                actor: actor.clone(),
            },
        );
        let id = WorkCommentId::new([byte; 16]);
        state.comments.insert(
            id,
            WorkCommentV1 {
                schema_version: 1,
                id,
                task: TaskId::new([byte; 16]),
                reply_to: None,
                body: (byte != 90).then(|| "Comment".into()),
                version: 1,
                actor: actor.clone(),
            },
        );
    }
    state.validate().unwrap();
    state
}

#[test]
fn work_browse_hydrates_only_the_returned_window() {
    let state = fixture();
    for kind in [WorkBrowseKind::Projects, WorkBrowseKind::Tasks] {
        let rows = browse_rows(
            &state,
            &WorkBrowseQuery {
                kind,
                q: "needle".into(),
                limit: 1,
                ..Default::default()
            },
            89,
        )
        .unwrap();
        assert_eq!(rows.total_matches, 100);
        assert_eq!(rows.count, 1);
        assert!(rows.more);
        assert!(
            rows.hydrated <= 2,
            "hydrated {} summaries for one deep row",
            rows.hydrated
        );
        if kind == WorkBrowseKind::Projects {
            assert_eq!(rows.projects[0].id, ProjectId::new([90; 16]));
            assert_eq!(rows.projects[0].task_counts.active, 1);
        } else {
            assert_eq!(rows.tasks[0].id, TaskId::new([90; 16]));
            assert_eq!(rows.tasks[0].comment_count, 0);
        }
    }
}

#[test]
fn task_activity_counts_the_complete_root_and_pages_only_compact_groups() {
    let mut state = fixture();
    let mut task = state.tasks[&TaskId::new([1; 16])].clone();
    // One aggregate remains complete even when it represents more than 256 tasks.
    for value in 1000u128..1300 {
        task.id = TaskId::new(value.to_be_bytes());
        task.actor.updated_at_millis = value as i64;
        state.tasks.insert(task.id, task.clone());
    }
    let repository = jj_store_model::RepositoryId::new([9; 16]);
    task.id = TaskId::new([0xff; 16]);
    task.repository = Some(repository);
    state
        .projects
        .get_mut(&task.project)
        .unwrap()
        .repositories
        .insert(repository, true);
    state.tasks.insert(task.id, task);
    state.validate().unwrap();
    let projection = super::super::projection::Projection::new(&state).unwrap();
    let query = WorkActivityQuery {
        limit: 1,
        cursor: None,
    };
    let (first, more) = activity_rows(&projection, &query, 0).unwrap();
    assert!(more);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].repository, None);
    assert_eq!(first[0].project, ProjectId::new([1; 16]));
    assert_eq!(first[0].tasks, 301);
    assert_eq!(first[0].updated_at_millis, 1299);
    assert_eq!(projection.activity.len(), 101);
    let (last, more) = activity_rows(&projection, &query, 100).unwrap();
    assert!(!more);
    assert_eq!(last[0].repository, Some(repository));
    assert_eq!(last[0].tasks, 1);
    assert_eq!(
        projection
            .activity
            .iter()
            .map(|group| group.tasks)
            .sum::<usize>(),
        state.tasks.len()
    );
    let encoded = serde_json::to_string(&first).unwrap();
    assert!(!encoded.contains("NEEDLE"));
    assert!(!encoded.contains("Task 001"));
    assert_eq!(
        activity_rows(&projection, &query, 102).unwrap_err(),
        Error::InvalidCursor
    );
    assert_eq!(
        activity_rows(
            &projection,
            &WorkActivityQuery {
                limit: 257,
                cursor: None
            },
            0
        )
        .unwrap_err(),
        Error::Invalid
    );
}
