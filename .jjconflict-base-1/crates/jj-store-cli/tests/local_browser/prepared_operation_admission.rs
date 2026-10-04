use super::{ReadBinding, jj};
use jj_store_jj::{local_browser as q, query::Cancellation};
use std::{fs, sync::Arc};

struct Binding {
    source: ReadBinding,
    denied: bool,
    generation: u64,
}
#[async_trait::async_trait]
impl q::Access for Binding {
    fn repository(&self) -> jj_store_model::RepositoryId {
        q::Access::repository(&self.source)
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        q::Access::metadata_path(&self.source)
    }
    async fn authorize(&self) -> q::Result<()> {
        if self.denied {
            Err(q::Error::Denied)
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn prepared_operation_pages_admit_fresh_metadata_without_any_view_or_git_reads() {
    let source = tempfile::tempdir().unwrap();
    let output = tempfile::tempdir().unwrap();
    jj(source.path(), &["git", "init", "--colocate"]);
    let metadata = source.path().join(".jj/repo");
    let access: Arc<dyn q::Access> = Arc::new(ReadBinding(metadata.clone()));
    let executor = q::Executor::new(1).unwrap();
    let mut operations = vec![];
    for n in 0..20 {
        jj(source.path(), &["new", "-m", &format!("Admission {n}")]);
        operations.push(jj(
            source.path(),
            &["op", "log", "--no-graph", "-n", "1", "-T", "id"],
        ));
    }
    let operation = operations.last().unwrap().parse().unwrap();
    let graph = |query: Option<String>| {
        q::Request::Graph(
            query.map_or(q::GraphRequest::Operations, |query| {
                q::GraphRequest::OperationsFiltered { query }
            }),
            q::GraphWindow {
                limit: 1,
                after: None,
            },
        )
    };
    let requests = [
        graph(Some(format!("from:{}", &operations[4][..12]))),
        graph(None),
        graph(Some("snapshot:false".into())),
        graph(Some("from:@-+".into())),
    ];
    let mut expected = vec![];
    for request in &requests {
        let q::Response::Graph(page) = executor
            .read(access.clone(), operation, request.clone())
            .await
            .unwrap()
        else {
            panic!()
        };
        expected.push(page);
    }
    let prepared = Arc::new(
        executor
            .prepare_index(
                access.clone(),
                operation,
                output.path().into(),
                16 * 1024 * 1024,
                q::preparation::Limits::default(),
                Cancellation::default(),
            )
            .await
            .unwrap(),
    );
    // A metadata-only ready path cannot use either source or prepared commit
    // indexes, Git bodies, or any selected/historical source view.
    fs::rename(
        metadata.join("op_store/views"),
        metadata.join("withheld-views"),
    )
    .unwrap();
    fs::rename(
        source.path().join(".git/objects"),
        source.path().join("withheld-git-objects"),
    )
    .unwrap();
    // Keep loader descriptors and the empty Git object directory: opening the
    // read-only backend still requires a structurally valid repository.
    fs::create_dir(source.path().join(".git/objects")).unwrap();
    fs::rename(
        metadata.join("index/segments"),
        metadata.join("withheld-index-segments"),
    )
    .unwrap();
    fs::rename(
        prepared.path().join("segments"),
        prepared.path().join("withheld-segments"),
    )
    .unwrap();
    let small = q::Executor::with_workers(
        jj_store_jj::query::workers::Workers::new(1).unwrap(),
        q::Limits {
            max_history_visits: 4,
            ..Default::default()
        },
    );
    for (request, expected) in requests.iter().zip(&expected) {
        let q::Response::Graph(actual) = small
            .read_prepared(access.clone(), prepared.clone(), request.clone())
            .await
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(actual.operation, operation);
        assert_eq!(actual.data, expected.data);
        assert_eq!(actual.next, expected.next);
        assert!(actual.stats.metadata_nodes <= 7, "{:?}", actual.stats);
        eprintln!("metadata-only page: {:?}", actual.stats);
    }
    let request = requests[0].clone();
    let binding = |denied, generation| {
        Arc::new(Binding {
            source: ReadBinding(metadata.clone()),
            denied,
            generation,
        }) as Arc<dyn q::Access>
    };
    assert!(matches!(
        small
            .read_prepared(binding(true, 1), prepared.clone(), request.clone())
            .await,
        Err(q::Error::Denied)
    ));
    assert!(matches!(
        small
            .read_prepared(binding(false, 2), prepared.clone(), request.clone())
            .await,
        Err(q::Error::EpochChanged)
    ));

    // Source filenames do not establish identity: validate the freshly loaded
    // selected operation as well as a historical operation hydrated for a row.
    let operations_path = metadata.join("op_store/operations");
    let selected = operations_path.join(operation.to_string());
    let selected_bytes = fs::read(&selected).unwrap();
    let historical = operations_path.join(&operations[4]);
    let historical_bytes = fs::read(&historical).unwrap();
    fs::write(&selected, &historical_bytes).unwrap();
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request.clone())
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(&selected, &selected_bytes).unwrap();
    fs::write(&historical, &selected_bytes).unwrap();
    assert!(matches!(
        small
            .read_prepared(access.clone(), prepared.clone(), request.clone())
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(&historical, historical_bytes).unwrap();
    assert!(
        small
            .read_prepared(access.clone(), prepared.clone(), request)
            .await
            .is_ok()
    );

    // Code readers retain their separate operation/view seal and fail closed.
    assert!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::Operation { operation }
            )
            .await
            .is_err()
    );
    assert!(!metadata.join("op_store/views").exists());
    assert!(!metadata.join("index/segments").exists());
    assert!(
        fs::read_dir(source.path().join(".git/objects"))
            .unwrap()
            .next()
            .is_none()
    );
    fs::rename(
        metadata.join("withheld-views"),
        metadata.join("op_store/views"),
    )
    .unwrap();
    let view_id = |index: usize| {
        let q::Nodes::Operations(nodes) = &expected[index].data else {
            panic!()
        };
        nodes[0].view
    };
    let selected_view = metadata.join("op_store/views").join(view_id(1).to_string());
    let original = fs::read(&selected_view).unwrap();
    let historical_view =
        fs::read(metadata.join("op_store/views").join(view_id(0).to_string())).unwrap();
    assert_ne!(original, historical_view);
    fs::write(&selected_view, historical_view).unwrap();
    assert!(
        small
            .read_prepared(access.clone(), prepared.clone(), requests[1].clone())
            .await
            .is_ok()
    );
    assert!(matches!(
        small
            .read_prepared(
                access.clone(),
                prepared.clone(),
                q::Request::Operation { operation }
            )
            .await,
        Err(q::Error::Corrupt)
    ));
    fs::write(selected_view, original).unwrap();
    assert!(
        small
            .read_prepared(access, prepared, q::Request::Operation { operation })
            .await
            .is_ok()
    );
}
