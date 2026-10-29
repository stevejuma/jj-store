use jj_vault::{
    Binding, NativeRepository, Repository,
    search::{Index, NativeHistory, Query, Request},
};
use std::fs;
#[test]
fn search_finds_lost_text_metadata_and_paths_after_cache_loss_and_revalidates_retained_anchors() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "search".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    fs::create_dir(&binding.root).unwrap();
    fs::write(
        binding.root.join("Old.md"),
        "before\nrare historical phrase\nafter",
    )
    .unwrap();
    let mut repo = NativeRepository {
        binding: binding.clone(),
        exclusions: vec![],
    };
    let first = repo.capture().unwrap().checkpoint.unwrap();
    fs::remove_file(binding.root.join("Old.md")).unwrap();
    fs::write(binding.root.join("Current.md"), "current text").unwrap();
    repo.capture().unwrap();
    fs::write(
        binding.root.join("Current.md"),
        "unsnapshotted buffer on disk",
    )
    .unwrap();
    let mut source = NativeHistory {
        binding: binding.clone(),
        exclusions: vec![],
    };
    let index = Index::new(binding.state.join("search"), "rules-v1".into());
    let mut request = Request {
        query: Query {
            text: "rare historical".into(),
            ..Default::default()
        },
        cursor: None,
    };
    let mut hits = vec![];
    loop {
        let page = index.search(&mut source, &request).unwrap();
        hits.extend(page.hits);
        request.cursor = page.next;
        if request.cursor.is_none() {
            break;
        }
    }
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].revision.checkpoint.commit, first.commit);
    assert_eq!(hits[0].path.as_deref(), Some("Old.md"));
    assert_eq!(hits[0].line, Some(2));
    fs::remove_dir_all(binding.state.join("search")).unwrap();
    let page = index
        .search(
            &mut source,
            &Request {
                query: Query {
                    text: "Automatic vault".into(),
                    ..Default::default()
                },
                cursor: None,
            },
        )
        .unwrap();
    assert!(page.hits.iter().any(|h| h.path.is_none()));
    let mut cursor = page.next.unwrap();
    cursor.anchor = "f".repeat(40);
    assert!(
        index
            .search(
                &mut source,
                &Request {
                    query: Query::default(),
                    cursor: Some(cursor)
                }
            )
            .is_err()
    );
    // Exclusions are a native namespace boundary, including already indexed bytes.
    let mut hidden = NativeHistory {
        binding: binding.clone(),
        exclusions: vec!["Old.md".into()],
    };
    let index = Index::new(binding.state.join("search"), "rules-v2".into());
    let mut request = Request {
        query: Query {
            text: "rare historical".into(),
            ..Default::default()
        },
        cursor: None,
    };
    loop {
        let page = index.search(&mut hidden, &request).unwrap();
        assert!(page.hits.is_empty());
        request.cursor = page.next;
        if request.cursor.is_none() {
            break;
        }
    }
    assert_eq!(
        fs::read_to_string(binding.root.join("Current.md")).unwrap(),
        "unsnapshotted buffer on disk"
    );
}

#[test]
fn projection_recovers_corrupt_cache_and_applies_author_date_and_path_filters() {
    use jj_vault::{
        Checkpoint,
        search::{HistoryFile, HistoryFiles, HistoryPage, HistoryRevision, Source},
    };
    struct Fake {
        reads: usize,
    }
    impl Source for Fake {
        fn revision(&mut self, _: Option<&str>, offset: u32) -> Result<HistoryPage, String> {
            Ok(HistoryPage {
                anchor: Some("a".repeat(40)),
                more: false,
                revision: (offset == 0).then(|| HistoryRevision {
                    checkpoint: Checkpoint {
                        operation: "b".repeat(128),
                        commit: "a".repeat(40),
                        change_id: "c".repeat(32),
                        description: "Launch decision".into(),
                        timestamp_millis: 123,
                    },
                    author: "Ada".into(),
                    email: "dev@lapis.md".into(),
                }),
            })
        }
        fn files(&mut self, commit: &str, after: Option<&str>) -> Result<HistoryFiles, String> {
            self.reads += 1;
            Ok(HistoryFiles {
                commit: commit.into(),
                after: after.map(str::to_owned),
                files: vec![HistoryFile {
                    path: "Plans/Launch.md".into(),
                    text: Some("αβ\nLaunch decision retained".into()),
                    unindexed: false,
                }],
                next: None,
            })
        }
    }
    let temp = tempfile::tempdir().unwrap();
    let cache = temp.path().canonicalize().unwrap().join("cache");
    let index = Index::new(cache.clone(), "rules".into());
    let mut source = Fake { reads: 0 };
    let query = Query {
        text: "launch".into(),
        author: Some("ADA".into()),
        path: Some("plans/".into()),
        from_millis: Some(123),
        to_millis: Some(123),
    };
    let request = Request {
        query: query.clone(),
        cursor: None,
    };
    assert_eq!(index.search(&mut source, &request).unwrap().hits.len(), 1);
    assert_eq!(source.reads, 1);
    index.search(&mut source, &request).unwrap();
    assert_eq!(source.reads, 1, "immutable projection reused");
    for file in fs::read_dir(&cache).unwrap() {
        fs::write(file.unwrap().path(), "not valid JSON").unwrap();
    }
    assert_eq!(
        index.search(&mut source, &request).unwrap().hits[0].line,
        Some(2)
    );
    assert_eq!(source.reads, 2);
    let request = Request {
        query: Query {
            from_millis: Some(124),
            to_millis: Some(125),
            ..query.clone()
        },
        cursor: None,
    };
    assert!(index.search(&mut source, &request).unwrap().hits.is_empty());
    assert_eq!(source.reads, 2);
    assert!(
        index
            .search(
                &mut source,
                &Request {
                    query: Query {
                        from_millis: Some(125),
                        to_millis: Some(124),
                        ..query
                    },
                    cursor: None
                }
            )
            .is_err()
    );
}

#[test]
fn history_search_indexes_transcripts_but_not_private_runtime_files() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let binding = Binding {
        vault_id: "chat-search".into(),
        root: base.join("notes"),
        state: base.join("private/history"),
    };
    let path = ".lapis/agents/sessions/7297795f-c00b-475e-b430-5b0e2e11a2e3/transcript.jsonl";
    fs::create_dir_all(binding.root.join(path).parent().unwrap()).unwrap();
    fs::write(binding.root.join(path), "rare transcript phrase").unwrap();
    fs::create_dir_all(binding.root.join(".lapis/cache")).unwrap();
    fs::write(
        binding.root.join(".lapis/cache/private"),
        "rare transcript phrase",
    )
    .unwrap();
    let mut repo = NativeRepository {
        binding: binding.clone(),
        exclusions: vec![],
    };
    let first = repo.capture().unwrap().checkpoint.unwrap();
    let mut source = NativeHistory {
        binding: binding.clone(),
        exclusions: vec![],
    };
    let page = source
        .search(&Request {
            query: Query {
                text: "rare transcript".into(),
                ..Default::default()
            },
            cursor: None,
        })
        .unwrap();
    assert_eq!(page.hits.len(), 1);
    assert_eq!(page.hits[0].path.as_deref(), Some(path));
    assert_eq!(page.hits[0].revision.checkpoint.commit, first.commit);
    use jj_vault::search::indexed::{Expression, Request as IndexedRequest};
    let indexed = IndexedRequest {
        expression: Expression::Term {
            field: Some("content".into()),
            value: "rare transcript".into(),
        },
        cursor: None,
        limit: 100,
        case_sensitive: false,
    };
    for _ in 0..20 {
        if source.index_step().unwrap().complete {
            break;
        }
    }
    let hits = source.indexed_search(&indexed).unwrap();
    assert!(hits.status.complete);
    assert_eq!(hits.hits.len(), 1);
    assert_eq!(hits.hits[0].record.path.as_deref(), Some(path));
    source.exclusions = vec![".lapis/agents".into()];
    let page = source
        .search(&Request {
            query: Query {
                text: "rare transcript".into(),
                ..Default::default()
            },
            cursor: None,
        })
        .unwrap();
    assert!(page.hits.is_empty());
    for _ in 0..20 {
        if source.index_step().unwrap().complete {
            break;
        }
    }
    let hidden = source.indexed_search(&indexed).unwrap();
    assert!(hidden.status.complete);
    assert!(hidden.hits.is_empty());
}
