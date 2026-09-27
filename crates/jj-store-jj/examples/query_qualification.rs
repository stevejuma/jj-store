//! Release-only query qualification over encoded native fixtures, not deployment evidence.
use jj_lib::{backend as b, op_store as o};
use jj_store_jj::{
    codec::{self, NativeCodec},
    query as q, roots,
};
use jj_store_model as m;
use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, VersionToken,
    WriteOutcome,
};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Instant,
};

const REPO: m::RepositoryId = m::RepositoryId::new([81; 16]);
struct Fixture {
    data: BTreeMap<Key, Arc<[u8]>>,
    inventory: BTreeSet<m::ObjectKey>,
    operation: m::OperationId,
    bytes: u64,
}
impl Fixture {
    fn insert<T: NativeCodec>(&mut self, value: &T) -> m::ObjectKey {
        let (key, data) = codec::encode(value, codec::MAX_METADATA_BYTES).unwrap();
        self.bytes += data.len() as u64;
        self.inventory.insert(key);
        self.data
            .insert(Key::new(key.storage_path(REPO)).unwrap(), data.into());
        key
    }
    fn generate(commits: usize, wide: bool) -> Self {
        let mut fixture = Self {
            data: BTreeMap::new(),
            inventory: BTreeSet::from([
                m::ObjectKey::Commit(roots::ROOT_COMMIT),
                m::ObjectKey::View(roots::ROOT_VIEW),
                m::ObjectKey::Operation(roots::ROOT_OPERATION),
                b::Tree::default().key(),
            ]),
            operation: roots::ROOT_OPERATION,
            bytes: 0,
        };
        let mut parent = roots::ROOT_COMMIT;
        let mut heads = BTreeSet::new();
        for i in 1..commits {
            // count includes the native virtual root
            let mut commit = roots::commit();
            commit.change_id = b::ChangeId::from_bytes(&(i as u128).to_le_bytes());
            commit.parents = vec![b::CommitId::from_bytes(if wide {
                roots::ROOT_COMMIT.as_bytes()
            } else {
                parent.as_bytes()
            })];
            commit.description = format!("qualification commit {i}");
            commit.committer.timestamp.timestamp.0 = i as i64;
            let key = fixture.insert(&commit);
            parent = m::CommitId::from_slice(key.as_bytes()).unwrap();
            if wide {
                heads.insert(b::CommitId::from_bytes(parent.as_bytes()));
            }
        }
        if !wide {
            heads.insert(b::CommitId::from_bytes(parent.as_bytes()));
        }
        let mut view = roots::view();
        view.head_ids = heads.into_iter().collect();
        let key = fixture.insert(&view);
        let mut operation = o::Operation::make_root(o::ViewId::from_bytes(key.as_bytes()));
        operation.parents = vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())];
        let key = fixture.insert(&operation);
        fixture.operation = m::OperationId::from_slice(key.as_bytes()).unwrap();
        fixture
    }
}
#[async_trait::async_trait]
impl q::ReadAccess for Fixture {
    fn repository(&self) -> m::RepositoryId {
        REPO
    }
    fn epoch(&self) -> m::EpochId {
        m::EpochId::new([82; 16])
    }
    fn operation(&self) -> m::OperationId {
        self.operation
    }
    fn inventory(&self) -> &BTreeSet<m::ObjectKey> {
        &self.inventory
    }
    async fn authorize(&self) -> q::Result<()> {
        Ok(())
    }
}
#[async_trait::async_trait]
impl ObjectStore for Fixture {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        let data = self.data.get(key).ok_or(jj_store_object::Error::NotFound)?;
        Ok(Metadata {
            len: data.len() as u64,
            version: VersionToken::new("immutable-fixture")?,
        })
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if range.is_some() {
            return Err(jj_store_object::Error::Range);
        }
        let data = self
            .data
            .get(key)
            .ok_or(jj_store_object::Error::NotFound)?
            .clone();
        Ok(Download {
            metadata: self.head(key).await?,
            reader: Box::pin(std::io::Cursor::new(data)),
        })
    }
    async fn put(
        &self,
        _: &Key,
        _: Condition,
        _: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        panic!("read qualification attempted a write")
    }
}
fn log(expression: &str) -> q::Request {
    q::Request::Log {
        revset: expression.to_owned(),
        workspace: None,
        user_email: "qualified@example.invalid".into(),
        at_time_millis: 1_790_000_000_000,
    }
}
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        return Err("qualification requires --release".into());
    }
    let commits: usize = std::env::args()
        .nth(1)
        .ok_or("commit count required")?
        .parse()?;
    if !(100..=50_000).contains(&commits) {
        return Err("count must be 100..=50000".into());
    }
    let mut scenarios = vec![];
    for wide in [false, true] {
        let started = Instant::now();
        let fixture = Arc::new(Fixture::generate(commits, wide));
        let seed_millis = started.elapsed().as_millis();
        let cache = tempfile::tempdir()?;
        let limits = q::Limits::default();
        let executor = q::GraphExecutor::new(cache.path().to_owned(), limits, 1)?;
        let mut runs = vec![];
        for (name, request) in [
            ("cold_log", log("all()")),
            ("warm_log", log("all()")),
            ("warm_filter", log("subject(z)")),
            ("operations", q::Request::Operations),
        ] {
            let query = serde_json::to_value(&request)?;
            let start = Instant::now();
            let result = executor
                .execute(
                    fixture.clone(),
                    fixture.clone(),
                    request,
                    q::Page {
                        limit: 500,
                        after: None,
                    },
                )
                .await;
            let millis = start.elapsed().as_millis();
            let entry = match result {
                Ok(result) => {
                    let rows = match &result.data {
                        q::Nodes::Log(nodes) => nodes.len(),
                        q::Nodes::Operations(nodes) => nodes.len(),
                        q::Nodes::Evolution(nodes) => nodes.len(),
                    };
                    assert_eq!(
                        rows,
                        match name {
                            "warm_filter" => 0,
                            "operations" => 2,
                            _ => 500.min(commits),
                        }
                    );
                    json!({"name":name,"query":query,"result":"pass","millis":millis,"rows":rows,"response_bytes":serde_json::to_vec(&result)?.len(),"stats":result.stats,"cache":result.cache})
                }
                Err(error) => {
                    json!({"name":name,"query":query,"result":"limit_or_error","millis":millis,"error":error.to_string()})
                }
            };
            runs.push(entry);
        }
        let over_index = q::GraphExecutor::new(
            cache.path().to_owned(),
            q::Limits {
                max_index_commits: commits - 1,
                ..limits
            },
            1,
        )?;
        assert_eq!(
            over_index
                .execute(
                    fixture.clone(),
                    fixture.clone(),
                    log("all()"),
                    q::Page {
                        limit: 1,
                        after: None
                    }
                )
                .await
                .unwrap_err(),
            q::Error::IndexBudget
        );
        assert_eq!(
            executor
                .execute(
                    fixture.clone(),
                    fixture.clone(),
                    log(&"~".repeat(100)),
                    q::Page {
                        limit: 1,
                        after: None
                    }
                )
                .await
                .unwrap_err(),
            q::Error::Limit
        );
        scenarios.push(json!({"shape":if wide {"wide_heads"} else {"linear"},"commits":commits,"objects":fixture.inventory.len(),"encoded_bytes":fixture.bytes,"fixture_millis":seed_millis,"runs":runs,"negative_admission_cases":2}));
    }
    println!(
        "{}",
        serde_json::to_string(
            &json!({"profile":jj_store_jj::factories::PROFILE,"revision":"7c41cdeb16b6b321c64e789a966b6adf723816a5","build":"release","limits":{"max_metadata_nodes":q::Limits::default().metadata.max_nodes,"max_merge_terms":q::Limits::default().metadata.max_merge_terms,"max_tree_entries":q::Limits::default().metadata.max_tree_entries,"max_parent_edges":q::Limits::default().metadata.max_parent_edges,"max_index_commits":q::Limits::default().max_index_commits,"max_work":q::Limits::default().max_work,"max_object_bytes":q::Limits::default().max_object_bytes,"max_output_bytes":q::Limits::default().max_output_bytes,"timeout_millis":q::Limits::default().timeout.as_millis()},"scenarios":scenarios})
        )?
    );
    Ok(())
}
