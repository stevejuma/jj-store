use jj_vault::{
    Checkpoint,
    search::{
        HistoryRevision,
        indexed::{Engine, Expression, IndexFile, IndexFiles, IndexRevision, Request, Source},
    },
};
use std::collections::BTreeMap;

struct Fixture {
    head: String,
    graph: BTreeMap<String, (Vec<String>, Vec<IndexFile>)>,
    fail: bool,
}
impl Fixture {
    fn new() -> Self {
        let file = |path: &str, id: &str, text: &str| IndexFile {
            path: path.into(),
            content_id: Some(id.into()),
            text: Some(text.into()),
            deleted: false,
            previous: None,
        };
        Self {
            head: "c".into(),
            fail: false,
            graph: BTreeMap::from([
                (
                    "a".into(),
                    (
                        vec![],
                        vec![file("Old.md", "body-one", "rare historical phrase")],
                    ),
                ),
                (
                    "b".into(),
                    (
                        vec!["a".into()],
                        vec![IndexFile {
                            path: "Old.md".into(),
                            content_id: None,
                            text: None,
                            deleted: true,
                            previous: Some("a".into()),
                        }],
                    ),
                ),
                (
                    "c".into(),
                    (
                        vec!["b".into()],
                        vec![
                            file("Old.md", "body-one", "rare historical phrase"),
                            file("New.md", "body-two", "new content"),
                        ],
                    ),
                ),
            ]),
        }
    }
}
impl Source for Fixture {
    fn head(&mut self) -> Result<Option<String>, String> {
        Ok(Some(self.head.clone()))
    }
    fn revision(&mut self, id: &str) -> Result<IndexRevision, String> {
        let (parents, _) = self.graph.get(id).ok_or("missing revision")?;
        Ok(IndexRevision {
            parents: parents.clone(),
            revision: HistoryRevision {
                checkpoint: Checkpoint {
                    commit: id.into(),
                    operation: "pinned-operation".into(),
                    change_id: "change".into(),
                    description: format!("checkpoint {id}"),
                    timestamp_millis: id.as_bytes()[0] as i64,
                },
                author: if id == "a" { "Ada" } else { "Grace" }.into(),
                email: "dev@lapis.md".into(),
            },
        })
    }
    fn files(&mut self, id: &str, _: Option<&str>) -> Result<IndexFiles, String> {
        if self.fail {
            return Err("interrupted".into());
        }
        Ok(IndexFiles {
            files: self.graph.get(id).unwrap().1.clone(),
            next: None,
        })
    }
    fn text(&mut self, _: &str, _: &str) -> Result<Option<String>, String> {
        panic!("query must not read current files")
    }
}
fn request(field: Option<&str>, value: &str) -> Request {
    Request {
        expression: Expression::Term {
            field: field.map(str::to_owned),
            value: value.into(),
        },
        cursor: None,
        limit: 100,
        case_sensitive: false,
    }
}
fn finish(engine: &mut Engine, source: &mut Fixture) {
    for _ in 0..20 {
        if engine.step(source).unwrap().complete {
            return;
        }
    }
    panic!("frontier did not drain");
}

#[test]
fn indexes_changed_versions_once_and_keeps_reintroduced_checkpoint_references() {
    let root = tempfile::tempdir().unwrap();
    let mut source = Fixture::new();
    let mut engine = Engine::open(root.path(), "vault/rules").unwrap();
    assert!(
        !engine
            .search(&mut source, &request(None, "rare"))
            .unwrap()
            .status
            .complete
    );
    finish(&mut engine, &mut source);
    let hits = engine
        .search(&mut source, &request(Some("content"), "rare"))
        .unwrap()
        .hits;
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].record.revision.checkpoint.commit, "c");
    assert_eq!(hits[1].record.revision.checkpoint.commit, "a");
    let mut author = request(Some("content"), "rare");
    author.expression = Expression::And {
        clauses: vec![
            author.expression,
            Expression::Term {
                field: Some("author".into()),
                value: "Ada".into(),
            },
        ],
    };
    assert_eq!(engine.search(&mut source, &author).unwrap().hits.len(), 1);
    let deleted = engine
        .search(&mut source, &request(Some("path"), "Old.md"))
        .unwrap()
        .hits;
    assert!(
        deleted
            .iter()
            .any(|hit| hit.record.deleted && hit.record.previous.as_deref() == Some("a"))
    );
    let metadata = engine
        .search(&mut source, &request(None, "checkpoint"))
        .unwrap();
    assert_eq!(metadata.hits.len(), 3);
    assert!(metadata.hits.iter().all(|hit| hit.record.path.is_none()));
    let mut dated = request(Some("content"), "rare");
    dated.expression = Expression::And {
        clauses: vec![
            dated.expression,
            Expression::Term {
                field: Some("after".into()),
                value: "98".into(),
            },
        ],
    };
    assert_eq!(engine.search(&mut source, &dated).unwrap().hits.len(), 1);
    let mut case = request(Some("content"), "RARE");
    case.case_sensitive = true;
    assert!(engine.search(&mut source, &case).unwrap().hits.is_empty());
    let index = tantivy::Index::open_in_dir(root.path().join("index")).unwrap();
    let reader: tantivy::IndexReader = index.reader().unwrap();
    let parser = tantivy::query::QueryParser::for_index(
        &index,
        vec![index.schema().get_field("kind").unwrap()],
    );
    let count = reader
        .searcher()
        .search(
            &parser.parse_query("body").unwrap(),
            &tantivy::collector::Count,
        )
        .unwrap();
    assert_eq!(
        count, 2,
        "one stored/indexed document per immutable content identity"
    );
}

#[test]
fn resumes_after_failure_and_fences_changed_generation_cursors() {
    let root = tempfile::tempdir().unwrap();
    let mut source = Fixture::new();
    let mut engine = Engine::open(root.path(), "rules").unwrap();
    source.fail = true;
    assert!(engine.step(&mut source).is_err());
    drop(engine);
    source.fail = false;
    let mut engine = Engine::open(root.path(), "rules").unwrap();
    engine.step(&mut source).unwrap();
    let mut query = request(Some("path"), "md");
    query.limit = 1;
    let first = engine.search(&mut source, &query).unwrap();
    assert!(first.next.is_some());
    query.cursor = first.next;
    finish(&mut engine, &mut source);
    assert!(engine.search(&mut source, &query).unwrap().reset);
    drop(engine);
    let engine = Engine::open(root.path(), "different-exclusions").unwrap();
    assert_eq!(
        engine
            .search(&mut source, &request(None, "rare"))
            .unwrap()
            .hits
            .len(),
        0
    );
}

#[test]
fn traverses_each_merge_parent_and_recovers_a_corrupt_index() {
    let root = tempfile::tempdir().unwrap();
    let mut source = Fixture::new();
    source.graph.insert(
        "side".into(),
        (
            vec!["a".into()],
            vec![IndexFile {
                path: "Side.md".into(),
                content_id: Some("side-body".into()),
                text: Some("branch-specific text".into()),
                deleted: false,
                previous: None,
            }],
        ),
    );
    source.graph.get_mut("c").unwrap().0.push("side".into());
    let mut engine = Engine::open(root.path(), "rules").unwrap();
    finish(&mut engine, &mut source);
    assert_eq!(
        engine
            .search(&mut source, &request(Some("content"), "branch-specific"))
            .unwrap()
            .hits
            .len(),
        1
    );
    drop(engine);
    std::fs::write(root.path().join("index/meta.json"), "broken").unwrap();
    let mut engine = Engine::open(root.path(), "rules").unwrap();
    finish(&mut engine, &mut source);
    assert_eq!(
        engine
            .search(&mut source, &request(Some("content"), "branch-specific"))
            .unwrap()
            .hits
            .len(),
        1
    );
}

#[test]
fn native_delta_projection_keeps_exact_deleted_versions_and_never_captures_disk() {
    use jj_vault::{Binding, NativeRepository, Repository, search::NativeHistory};
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "indexed-native".into(),
        root: root.join("vault"),
        state: root.join("private/history"),
    };
    std::fs::create_dir(&binding.root).unwrap();
    std::fs::write(
        binding.root.join("Old.md"),
        "first\nrare native phrase\nlast",
    )
    .unwrap();
    let mut repository = NativeRepository {
        binding: binding.clone(),
        exclusions: vec![],
    };
    let first = repository.capture().unwrap().checkpoint.unwrap();
    std::fs::remove_file(binding.root.join("Old.md")).unwrap();
    std::fs::write(binding.root.join("Current.md"), "saved now").unwrap();
    let second = repository.capture().unwrap().checkpoint.unwrap();
    std::fs::write(
        binding.root.join("Current.md"),
        "unsnapshotted saved editor change",
    )
    .unwrap();
    let history = NativeHistory {
        binding: binding.clone(),
        exclusions: vec![],
    };
    for _ in 0..10 {
        if history.index_step().unwrap().complete {
            break;
        }
    }
    let hits = history
        .indexed_search(&request(Some("content"), "rare native"))
        .unwrap()
        .hits;
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].record.revision.checkpoint.commit, first.commit);
    assert_eq!(
        history
            .indexed_text(&first.commit, "Old.md")
            .unwrap()
            .as_deref(),
        Some("first\nrare native phrase\nlast")
    );
    assert!(
        history
            .indexed_search(&request(Some("path"), "Old.md"))
            .unwrap()
            .hits
            .iter()
            .any(|hit| hit.record.deleted)
    );
    assert!(
        history
            .indexed_search(&request(Some("content"), "unsnapshotted"))
            .unwrap()
            .hits
            .is_empty()
    );
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let operation = runtime.block_on(async {
        jj_store_jj::vault_workspace::HistoryReader::open(binding.clone())
            .await
            .unwrap()
            .history_revision(None, 0)
            .await
            .unwrap()
            .revision
            .unwrap()
            .checkpoint
            .operation
    });
    assert_eq!(operation, second.operation);
    assert_eq!(
        std::fs::read_to_string(binding.root.join("Current.md")).unwrap(),
        "unsnapshotted saved editor change"
    );
}
