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
