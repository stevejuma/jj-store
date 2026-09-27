//! Release resource qualification for admitted native metadata and streamed files.
use blake2::{Blake2b512, Digest as _};
use jj_lib::{backend as b, merge::Merge, repo_path::RepoPathComponentBuf};
use jj_store_jj::{
    codec::{self, NativeCodec},
    query::{self as q, content as c},
    roots,
};
use jj_store_model::{
    self as m,
    envelope::{HEADER_LEN, Header},
};
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
use tokio::io::AsyncReadExt as _;
const REPO: m::RepositoryId = m::RepositoryId::new([83; 16]);
struct Fixture {
    data: BTreeMap<Key, Arc<[u8]>>,
    spool: Option<(Key, StagedUpload)>,
    inventory: BTreeSet<m::ObjectKey>,
}
impl Fixture {
    fn empty() -> Self {
        Self {
            data: BTreeMap::new(),
            spool: None,
            inventory: BTreeSet::new(),
        }
    }
    fn insert<T: NativeCodec>(&mut self, value: &T) -> m::ObjectKey {
        let (key, data) = codec::encode(value, codec::MAX_METADATA_BYTES).unwrap();
        self.inventory.insert(key);
        self.data
            .insert(Key::new(key.storage_path(REPO)).unwrap(), data.into());
        key
    }
    fn tree(entries: usize, arity: usize) -> (Arc<Self>, m::CommitId) {
        let mut f = Self::empty();
        let mut ids = Vec::new();
        for side in 0..3 {
            let tree = b::Tree::from_sorted_entries(
                (0..entries)
                    .map(|i| {
                        (
                            RepoPathComponentBuf::new(format!("path{i:06}")).unwrap(),
                            b::TreeValue::GitSubmodule(b::CommitId::from_bytes(&[side + 1])),
                        )
                    })
                    .collect(),
            );
            ids.push(b::TreeId::from_bytes(f.insert(&tree).as_bytes()));
        }
        let mut commit = roots::commit();
        commit.root_tree = Merge::from_vec(
            (0..arity)
                .map(|i| {
                    ids[if i % 2 == 1 {
                        1
                    } else if i % 4 == 0 {
                        0
                    } else {
                        2
                    }]
                    .clone()
                })
                .collect::<Vec<_>>(),
        );
        commit.conflict_labels = Merge::from_vec(vec![String::new(); arity]);
        let id = m::CommitId::from_slice(f.insert(&commit).as_bytes()).unwrap();
        (Arc::new(f), id)
    }
    async fn file(len: u64) -> (Arc<Self>, m::FileId) {
        let mut f = Self::empty();
        let zeros = vec![0; 64 * 1024];
        let mut hash = Blake2b512::new();
        for _ in 0..len / zeros.len() as u64 {
            hash.update(&zeros);
        }
        assert_eq!(len % zeros.len() as u64, 0);
        let id = m::FileId::new(hash.finalize().into());
        let key = m::ObjectKey::File(id);
        let mut hash = Header::hasher(key, len);
        for _ in 0..len / zeros.len() as u64 {
            hash.update(&zeros);
        }
        let header = Header {
            key,
            payload_len: len,
            digest: m::EnvelopeDigest::new(hash.finalize().into()),
        };
        let reader = std::io::Cursor::new(header.encode()).chain(tokio::io::repeat(0).take(len));
        let framed = StagedUpload::from_reader(reader, len + HEADER_LEN as u64)
            .await
            .unwrap();
        f.spool = Some((Key::new(key.storage_path(REPO)).unwrap(), framed));
        f.inventory.insert(key);
        (Arc::new(f), id)
    }
}
#[async_trait::async_trait]
impl q::ReadAccess for Fixture {
    fn repository(&self) -> m::RepositoryId {
        REPO
    }
    fn epoch(&self) -> m::EpochId {
        m::EpochId::new([84; 16])
    }
    fn operation(&self) -> m::OperationId {
        roots::ROOT_OPERATION
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
        let len = if let Some(data) = self.data.get(key) {
            data.len() as u64
        } else if let Some((_, spool)) = self.spool.as_ref().filter(|(id, _)| id == key) {
            spool.len()
        } else {
            return Err(jj_store_object::Error::NotFound);
        };
        Ok(Metadata {
            len,
            version: VersionToken::new("immutable-content-fixture")?,
        })
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if range.is_some() {
            return Err(jj_store_object::Error::Range);
        }
        let metadata = self.head(key).await?;
        let reader: jj_store_object::Reader = if let Some(data) = self.data.get(key) {
            Box::pin(std::io::Cursor::new(data.clone()))
        } else {
            self.spool.as_ref().unwrap().1.reader()?
        };
        Ok(Download { metadata, reader })
    }
    async fn put(
        &self,
        _: &Key,
        _: Condition,
        _: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        panic!("read qualification attempted publication")
    }
}
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        return Err("qualification requires --release".into());
    }
    let limits = c::Limits::default();
    let reader = c::ContentReader::new(limits, 1)?;
    let mut runs = vec![];
    for (name, entries, arity, accepted) in [
        ("dense_tree_at_limit", 50_000, 1, true),
        ("dense_tree_over_limit", 50_001, 1, false),
        ("merge_at_limit", 5_000, 65, true),
        ("merge_over_limit", 5_000, 1025, false),
    ] {
        let (fixture, commit) = Fixture::tree(entries, arity);
        let start = Instant::now();
        let result = reader
            .tree(
                fixture.clone(),
                fixture,
                c::TreeRequest {
                    commit,
                    path: String::new(),
                    limit: 500,
                    after: None,
                },
            )
            .await;
        if accepted {
            let value = result?;
            assert_eq!(value.entries.len(), 500);
            assert_eq!(value.entries[0].terms.len(), arity);
            runs.push(json!({"name":name,"result":"pass","entries":entries,"arity":arity,"rows":value.entries.len(),"stats":value.stats,"millis":start.elapsed().as_millis()}));
        } else {
            assert_eq!(result.unwrap_err(), q::Error::Limit);
            runs.push(json!({"name":name,"result":"pass","entries":entries,"arity":arity,"error":"Limit","millis":start.elapsed().as_millis()}));
        }
    }
    let (fixture, commit) = Fixture::tree(100, 1);
    let bounded = c::ContentReader::new(
        c::Limits {
            metadata: q::admission::Limits {
                max_nodes: 10,
                ..limits.metadata
            },
            ..limits
        },
        1,
    )?;
    let start = Instant::now();
    assert_eq!(
        bounded
            .object(fixture.clone(), fixture, m::ObjectKey::Commit(commit))
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    runs.push(json!({"name":"cumulative_nodes_over_limit","result":"pass","max_nodes":10,"error":"Limit","millis":start.elapsed().as_millis()}));
    let len = limits.max_blob_bytes;
    let (fixture, id) = Fixture::file(len).await;
    for (name, range) in [
        ("stream_blob_at_limit", None),
        ("verified_tail_range", Some(ByteRange::new(len - 16, len)?)),
    ] {
        let start = Instant::now();
        let mut file = reader
            .file(fixture.clone(), fixture.clone(), id, range)
            .await?;
        assert_eq!(file.info().await?.full_length, len);
        let mut bytes = 0;
        let mut max_chunk = 0;
        while let Some(chunk) = file.next_chunk().await? {
            assert!(chunk.iter().all(|&v| v == 0));
            max_chunk = max_chunk.max(chunk.len());
            bytes += chunk.len() as u64;
        }
        assert_eq!(bytes, range.map_or(len, |r| r.len()));
        assert!(max_chunk <= 64 * 1024);
        runs.push(json!({"name":name,"result":"pass","verified_bytes":len,"returned_bytes":bytes,"max_chunk_bytes":max_chunk,"millis":start.elapsed().as_millis()}));
    }
    let bounded = c::ContentReader::new(
        c::Limits {
            max_blob_bytes: len - 1,
            ..limits
        },
        1,
    )?;
    let start = Instant::now();
    assert_eq!(
        bounded
            .file(fixture.clone(), fixture, id, None)
            .await
            .unwrap_err(),
        q::Error::Limit
    );
    runs.push(json!({"name":"blob_over_limit","result":"pass","error":"Limit","millis":start.elapsed().as_millis()}));
    println!(
        "{}",
        json!({"profile":jj_store_jj::factories::PROFILE,"revision":"7c41cdeb16b6b321c64e789a966b6adf723816a5","build":"release","limits":{"max_metadata_nodes":limits.metadata.max_nodes,"max_merge_terms":limits.metadata.max_merge_terms,"max_tree_entries":limits.metadata.max_tree_entries,"max_parent_edges":limits.metadata.max_parent_edges,"max_work":limits.max_work,"max_blob_bytes":limits.max_blob_bytes,"max_read_bytes":limits.max_read_bytes,"max_output_bytes":limits.max_output_bytes,"timeout_millis":limits.timeout.as_millis()},"runs":runs})
    );
    Ok(())
}
