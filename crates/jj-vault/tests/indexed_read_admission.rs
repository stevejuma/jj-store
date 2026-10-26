use jj_vault::{
    Binding, NativeRepository, Repository,
    search::{
        NativeHistory,
        indexed::{Expression, Request},
    },
};
use std::{
    fs::{self, File, OpenOptions},
    sync::mpsc,
    time::{Duration, Instant},
};

struct Fixture {
    _temp: tempfile::TempDir,
    binding: Binding,
    index: std::path::PathBuf,
    commit: String,
}
impl Fixture {
    fn new(index: bool) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        let binding = Binding {
            vault_id: "read-admission".into(),
            root: root.join("vault"),
            state: root.join("history"),
        };
        fs::create_dir(&binding.root).unwrap();
        fs::write(binding.root.join("Note.md"), "retained original version\n").unwrap();
        let commit = NativeRepository {
            binding: binding.clone(),
            exclusions: vec![],
        }
        .capture()
        .unwrap()
        .checkpoint
        .unwrap()
        .commit;
        let history = NativeHistory {
            binding: binding.clone(),
            exclusions: vec![],
        };
        if index {
            for _ in 0..10 {
                if history.index_step().unwrap().complete {
                    break;
                }
            }
        }
        fs::write(
            binding.root.join("Note.md"),
            "unsnapshotted saved version\n",
        )
        .unwrap();
        let directory = binding.state.join("history-search-v2");
        let path = if index {
            fs::read_dir(directory)
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .path()
        } else {
            directory
        };
        Self {
            _temp: temp,
            binding,
            index: path,
            commit,
        }
    }
    fn owner(&self) -> File {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.index.join("owner.lock"))
            .unwrap();
        file.lock().unwrap();
        file
    }
    fn query(
        &self,
    ) -> impl FnOnce() -> Result<jj_vault::search::indexed::Page, String> + Send + 'static {
        let binding = self.binding.clone();
        move || {
            NativeHistory {
                binding,
                exclusions: vec![],
            }
            .indexed_search(&Request {
                expression: Expression::Term {
                    field: Some("content".into()),
                    value: "retained original".into(),
                },
                case_sensitive: false,
                cursor: None,
                limit: 100,
            })
        }
    }
}

#[test]
fn native_query_waits_for_the_publisher_then_reads_the_retained_generation() {
    let fixture = Fixture::new(true);
    let owner = fixture.owner();
    let (send, receive) = mpsc::channel();
    let query = fixture.query();
    let worker = std::thread::spawn(move || send.send(query()).unwrap());
    assert!(
        matches!(
            receive.recv_timeout(Duration::from_millis(150)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ),
        "ordinary native reads must wait for a publishing batch instead of rejecting Busy"
    );
    drop(owner);
    let page = receive
        .recv_timeout(Duration::from_secs(5))
        .unwrap()
        .unwrap();
    worker.join().unwrap();
    assert_eq!(page.hits.len(), 1);
    assert_eq!(
        page.hits[0].record.revision.checkpoint.commit,
        fixture.commit
    );
    assert_eq!(
        fs::read_to_string(fixture.binding.root.join("Note.md")).unwrap(),
        "unsnapshotted saved version\n"
    );
}

#[test]
fn sustained_publisher_contention_remains_a_bounded_busy_error() {
    let fixture = Fixture::new(true);
    let _owner = fixture.owner();
    let started = Instant::now();
    let error = fixture.query()().unwrap_err();
    assert_eq!(error, "History index is busy");
    assert!(
        started.elapsed() >= Duration::from_secs(4),
        "transient publishers must have time to finish"
    );
    assert!(
        started.elapsed() < Duration::from_secs(8),
        "reader admission must remain bounded"
    );
}

#[test]
fn native_query_never_creates_a_missing_projection() {
    let fixture = Fixture::new(false);
    assert!(fixture.query()().is_err());
    assert!(
        !fixture.index.exists(),
        "only the native owner may initialize derived storage"
    );
}

#[test]
fn native_query_never_repairs_corrupt_projection_bytes() {
    let fixture = Fixture::new(true);
    let meta = fixture.index.join("index/meta.json");
    fs::write(&meta, "broken projection").unwrap();
    assert!(fixture.query()().is_err());
    assert_eq!(fs::read_to_string(meta).unwrap(), "broken projection");
}
