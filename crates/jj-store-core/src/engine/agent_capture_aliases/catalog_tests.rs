use super::*;
use jj_agent::{AgentObjectId, AgentRootV1, SourceInstanceId};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

struct Snapshot {
    root: AgentRootV1,
    objects: BTreeMap<AgentObjectRef, AgentObjectV1>,
    reads: AtomicUsize,
}
#[async_trait::async_trait]
impl AgentSnapshotRead for Snapshot {
    fn repository(&self) -> RepositoryId {
        RepositoryId::new([1; 16])
    }
    fn root(&self) -> &AgentRootV1 {
        &self.root
    }
    async fn lookup(
        &self,
        _: AgentCatalogKind,
        _: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        unreachable!()
    }
    async fn read_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        self.reads.fetch_add(1, Relaxed);
        self.objects.get(&reference).cloned().ok_or(Error::Stale)
    }
}
fn snapshot() -> Snapshot {
    let mut objects = BTreeMap::new();
    let mut entries = BTreeMap::new();
    for id in 1..=3 {
        let alias = AgentAliasV1 {
            schema_version: 1,
            integration: IntegrationId::Codex,
            source_instance_id: SourceInstanceId::new([id; 16]),
            native_kind: NativeEntityKind::Session,
            native_id: "00000000-0000-0000-0000-000000000001".into(),
            target: AgentEntityRefV1::Session {
                session_id: AgentSessionId::new([id; 16]),
            },
        };
        let reference = AgentObjectRef::new(
            jj_agent::AgentObjectKind::Alias,
            AgentObjectId::new([id; 32]),
        );
        entries.insert(alias.catalog_key().unwrap(), reference);
        objects.insert(reference, AgentObjectV1::Alias(alias));
    }
    let page = AgentObjectV1::IndexPage(AgentIndexPageV1::Leaf {
        prefix: String::new(),
        entries,
    });
    page.validate().unwrap();
    let reference = AgentObjectRef::new(page.kind(), AgentObjectId::new([4; 32]));
    objects.insert(reference, page);
    let mut root = AgentRootV1::empty();
    root.aliases = Some(reference);
    Snapshot {
        root,
        objects,
        reads: AtomicUsize::new(0),
    }
}

#[tokio::test]
async fn incomplete_alias_or_page_scan_never_returns_a_partial_identity_proof() {
    let selected = snapshot();
    assert!(scan(&selected, 2, 1).await.unwrap().is_none());
    assert_eq!(selected.reads.load(Relaxed), 4);
    selected.reads.store(0, Relaxed);
    assert!(scan(&selected, 3, 0).await.unwrap().is_none());
    assert_eq!(selected.reads.load(Relaxed), 0);
    assert_eq!(scan(&selected, 3, 1).await.unwrap().unwrap().len(), 3);
}
