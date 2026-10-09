use super::*;
use jj_store_service::agent_execution::workspace_inspection as inspect;
pub(super) async fn exercise(supervisor: &Supervisor, actor: PrincipalId, session: &Session) {
    let binding = session
        .repository_bindings
        .iter()
        .find(|b| Path::new(&b.workspace).join("source.txt").exists())
        .unwrap();
    let root = Path::new(&binding.workspace);
    let checkout = fs::read(root.join(".jj/working_copy/checkout")).unwrap();
    let mut query = inspect::Request {
        version: 1,
        session: session.id.clone(),
        repository: binding.repository,
        query: inspect::InspectionView::Files,
        path: Some("agent-edit.txt".into()),
        cursor: None,
    };
    let first = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert_eq!(
        binding.workspace_name.as_deref(),
        Some(first.workspace.as_str())
    );
    assert_eq!(
        first.file.unwrap().contents.as_deref(),
        Some("review this\n")
    );
    fs::write(root.join("agent-edit.txt"), "later unsnapshotted 🌍\n").unwrap();
    fs::write(root.join("untracked.txt"), "new\n").unwrap();
    let current = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert_eq!(
        current.file.unwrap().contents.as_deref(),
        Some("later unsnapshotted 🌍\n")
    );
    assert!(current.files.iter().any(|f| f.path == "untracked.txt"));
    query.query = inspect::InspectionView::Changes {
        range: inspect::ComparisonRange::LastTurn,
        base: None,
    };
    let last_turn = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert!(last_turn.comparison.as_ref().unwrap().unavailable.is_none());
    assert_eq!(
        last_turn.file.unwrap().contents.as_deref(),
        Some("review this\n")
    );
    assert!(!last_turn.files.iter().any(|f| f.path == "untracked.txt"));
    query.query = inspect::InspectionView::Changes {
        range: inspect::ComparisonRange::SinceLastCommit,
        base: None,
    };
    let changed = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert!(
        changed
            .files
            .iter()
            .any(|f| f.path == "untracked.txt" && f.status.as_deref() == Some("added"))
    );
    assert_eq!(
        changed.file.unwrap().contents.as_deref(),
        Some("later unsnapshotted 🌍\n")
    );
    query.path = Some("source.txt".into());
    fs::remove_file(root.join("source.txt")).unwrap();
    let deleted = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert!(
        deleted
            .files
            .iter()
            .any(|f| f.path == "source.txt" && f.status.as_deref() == Some("deleted"))
    );
    let file = deleted.file.unwrap();
    assert!(file.unavailable.is_none(), "{file:?}");
    assert!(file.contents.is_none());
    assert_eq!(file.before.as_deref(), Some("base\n"));
    fs::write(root.join("source.txt"), "base\n").unwrap();
    assert_eq!(
        fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
        checkout
    );
    query.path = None;
    query.query = inspect::InspectionView::Changes {
        range: inspect::ComparisonRange::Branch,
        base: None,
    };
    let branch = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert!(branch.comparison.unwrap().unavailable.is_some());
    query.query = inspect::InspectionView::Files;
    for i in 0..55 {
        fs::write(root.join(format!("page-{i:02}.txt")), "page").unwrap();
    }
    let first_page = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert_eq!(first_page.files.len(), 50);
    query.cursor = first_page.next;
    assert!(query.cursor.is_some());
    let next_page = supervisor
        .inspect_workspace(actor, query.clone())
        .await
        .unwrap();
    assert!(next_page.next.is_none());
    assert!(!next_page.files.is_empty());
    fs::write(root.join("page-00.txt"), "changed").unwrap();
    assert!(matches!(
        supervisor.inspect_workspace(actor, query.clone()).await,
        Err(Error::Stale)
    ));
    for i in 0..55 {
        fs::remove_file(root.join(format!("page-{i:02}.txt"))).unwrap();
    }
    query.cursor = None;
    assert_eq!(
        fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
        checkout
    );
    query.repository = jj_store_model::RepositoryId::new([255; 16]);
    assert!(matches!(
        supervisor.inspect_workspace(actor, query).await,
        Err(Error::Denied)
    ));
}
