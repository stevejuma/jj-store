use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::conversion::*;
use jj_store_wal::{self as wal, inventory::*, publication::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
#[path = "../../jj-store-wal/tests/support/review.rs"]
mod review_fixture;
struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(
        &self,
        key: m::ObjectKey,
        _: u64,
    ) -> wal::inventory::Result<VerifiedNode> {
        if key != m::ObjectKey::Operation(m::OperationId::new([0; 64])) {
            return Err(InventoryError::Source("fixture only".into()));
        }
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}
struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
}
async fn fixture(id: u8) -> Coordinator {
    let repo = m::RepositoryId::new([id; 16]);
    let c = Coordinator::new(
        Arc::new(MemoryStore::default()),
        repo,
        Arc::new(Source(SourceScope::new(repo))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    c.initialize(m::EpochId::new([2; 16]), vec![])
        .await
        .unwrap();
    c
}
#[tokio::test]
async fn plain_agent_capture_translates_every_repository_bound_record_and_exact_payload() {
    let source = fixture(1).await;
    let destination = fixture(3).await;
    let selected = source.read().await.unwrap();
    let old = source
        .agent_store(selected.manifest().agent_root.unwrap())
        .await
        .unwrap();
    let payload = serde_json::json!({"capture_format":"codex-jsonl/v1","product_version":"0.159.2","session_id":"00000000-0000-0000-0000-000000000001","record":{"type":"response_item","payload":{"type":"message","id":"native-message","role":"assistant","content":[{"type":"output_text","text":"Recorded response"}]}}});
    let batch = SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([30; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([31; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: vec![SanitizedObservationV1 {
            observation_id: ObservationId::new([32; 16]),
            source_sequence: 1,
            occurred_at_millis: Some(10),
            source_kind: SourceKind::Transcript,
            source_trust: SourceTrust::LocalTranscript,
            payload: SanitizedPayloadV1::new(
                SanitizedMediaType::Json,
                &serde_json::to_vec(&payload).unwrap(),
            )
            .unwrap(),
        }],
    };
    let delta = source
        .stage_agent_capture(
            &old,
            &NormalizerProfileV1::codex_desktop_0_159_2(),
            &batch,
            123,
        )
        .await
        .unwrap();
    let root = source.stage_agent_delta(&old, &delta).await.unwrap();
    let pin = source.agent_store(root).await.unwrap();
    let archive = archive_agent(&pin).await.unwrap();
    let (root, map) = transfer_agent(&archive, &destination).await.unwrap();
    assert!(!map.is_empty());
    assert!(map.iter().all(|(a, b)| a != b));
    let reference = destination.stage_agent_root(&root).await.unwrap();
    let rebuilt = destination.agent_store(reference).await.unwrap();
    assert_eq!(rebuilt.object_count(), pin.object_count());
    for (old, value) in &archive.objects {
        if matches!(
            value,
            AgentObjectV1::ObservationBatch(_)
                | AgentObjectV1::Content(_)
                | AgentObjectV1::TranscriptChunk(_)
                | AgentObjectV1::Alias(_)
                | AgentObjectV1::SourceState(_)
        ) {
            assert_eq!(&rebuilt.read_object(map[old]).await.unwrap(), value);
        }
    }
    let mut corrupt = archive.clone();
    corrupt
        .objects
        .retain(|(_, v)| !matches!(v, AgentObjectV1::TranscriptChunk(_)));
    assert!(transfer_agent(&corrupt, &destination).await.is_err());
}
#[tokio::test]
async fn repository_reviews_preserve_comments_events_actors_and_rebuild_anchor_relation_keys() {
    let source = fixture(1).await;
    let destination = fixture(3).await;
    let selected = source.read().await.unwrap();
    let empty = source
        .review_store(selected.manifest().review_root.unwrap())
        .await
        .unwrap();
    let objects = review_fixture::stage_thread(&source, 15, None).await;
    let mut edits = review_fixture::edits(&objects);
    let subject = ReviewRelationSubjectV1::OriginalAnchor {
        anchor_id: objects.anchor_id,
    };
    let relation = source
        .stage_review_object(&ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
            schema_version: 1,
            subject: subject.clone(),
            entries: vec![ReviewThreadRelationV1 {
                thread_id: objects.thread_id,
                snapshot: objects.snapshot,
            }],
            next: None,
        }))
        .await
        .unwrap();
    edits.push(ReviewCatalogEditV1 {
        catalog: ReviewCatalogKind::AnchorRelations,
        key: subject.catalog_key().unwrap(),
        expected: None,
        replacement: Some(relation),
    });
    let root = source
        .stage_review_delta(
            &empty,
            &ReviewDeltaV1 {
                schema_version: 1,
                edits,
            },
        )
        .await
        .unwrap();
    let pin = source.review_store(root).await.unwrap();
    let archive = archive_review(&pin).await.unwrap();
    let (root, map) = transfer_review(&archive, &destination).await.unwrap();
    let reference = destination.stage_review_root(&root).await.unwrap();
    let rebuilt = destination.review_store(reference).await.unwrap();
    let ReviewObjectV1::Anchor(anchor) = rebuilt.read_object(map[&objects.anchor]).await.unwrap()
    else {
        panic!()
    };
    assert_eq!(anchor.repository_id, m::RepositoryId::new([3; 16]));
    assert_ne!(anchor.anchor_id().unwrap(), objects.anchor_id);
    let next_subject = ReviewRelationSubjectV1::OriginalAnchor {
        anchor_id: anchor.anchor_id().unwrap(),
    };
    assert_eq!(
        rebuilt
            .lookup(
                ReviewCatalogKind::AnchorRelations,
                next_subject.catalog_key().unwrap()
            )
            .await
            .unwrap(),
        Some(map[&relation])
    );
    assert!(
        rebuilt
            .lookup(
                ReviewCatalogKind::AnchorRelations,
                subject.catalog_key().unwrap()
            )
            .await
            .unwrap()
            .is_none()
    );
    let ReviewObjectV1::CommentRevision(old) = pin.read_object(objects.revision).await.unwrap()
    else {
        panic!()
    };
    let ReviewObjectV1::CommentRevision(new) =
        rebuilt.read_object(map[&objects.revision]).await.unwrap()
    else {
        panic!()
    };
    assert_eq!(
        (new.author, new.actor, new.created_at_millis, new.comment_id),
        (old.author, old.actor, old.created_at_millis, old.comment_id)
    );
}
#[test]
fn reaction_slots_and_exact_targets_are_rekeyed_with_observed_history_preserved() {
    use jj_collab::*;
    let mut graph = CollabGraphV1::default();
    let target = ReactionTargetV1::Thread {
        thread_id: ThreadId::new([6; 16]),
    };
    let actor = m::PrincipalId::new([7; 16]);
    let old_repo = m::RepositoryId::new([1; 16]);
    let new_repo = m::RepositoryId::new([3; 16]);
    let addition = graph
        .stage(CollabObjectV1::Reaction(ReactionV1 {
            schema_version: 1,
            id: ReactionId::new([8; 16]),
            repository: old_repo,
            actor,
            target: target.clone(),
            value: ReactionValueV1::Like,
            origin: ReactionOriginV1::Human,
            accepted_at_millis: 12,
        }))
        .unwrap();
    let mut slot = ReactionSlotV1::empty(old_repo, actor, target);
    slot.additions.insert(addition);
    let old_key = slot.key().unwrap();
    let reference = graph.stage(CollabObjectV1::ReactionSlot(slot)).unwrap();
    let slots = graph
        .catalog_update(None, BTreeMap::from([(old_key, reference)]))
        .unwrap();
    let archive = CollabGraphArchiveV1 {
        thread: None,
        reaction: Some(ReactionRootV1 {
            schema_version: 1,
            slots,
            emojis: None,
        }),
        objects: graph.objects.into_iter().collect(),
    };
    let (result, rebuilt, map) =
        transfer_collab(&archive, new_repo, &BTreeMap::new(), &BTreeMap::new()).unwrap();
    let CollabObjectV1::ReactionSlot(slot) = rebuilt.get(map[&reference]).unwrap() else {
        panic!()
    };
    assert_eq!(slot.repository, new_repo);
    assert_ne!(slot.key().unwrap(), old_key);
    assert_eq!(slot.additions, BTreeSet::from([map[&addition]]));
    assert_eq!(
        slot.resolve(&rebuilt.objects).unwrap().value,
        Some(ReactionValueV1::Like)
    );
    let catalog = rebuilt.catalog(result.reaction.unwrap().slots).unwrap();
    assert!(catalog.contains_key(&slot.key().unwrap()));
    assert!(!catalog.contains_key(&old_key));
}

#[tokio::test]
async fn local_history_archive_retains_every_selected_work_version_and_exact_git_evidence() {
    use jj_store_jj::local_metadata::{Limits as LocalLimits, MetadataStore};
    use jj_store_service::{local_repository::LocalRepository, work};
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("source");
    let output = std::process::Command::new("jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(&path)
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(output.status.success());
    let local = LocalRepository::new(
        &path.join(".jj/repo"),
        MetadataStore::open(&path.join(".git"), LocalLimits::default()).unwrap(),
    );
    let project = ProjectId::new([20; 16]);
    for (id, command) in [
        (
            20,
            WorkCommandV1::CreateProject {
                id: project,
                title: "Original".into(),
                body: "Original body".into(),
            },
        ),
        (
            21,
            WorkCommandV1::UpdateProject {
                id: project,
                expected_version: 1,
                title: "Renamed".into(),
                body: "Changed body".into(),
            },
        ),
    ] {
        local
            .apply_work_command(&work::Request {
                version: 1,
                request_id: m::RequestId::new([id; 16]),
                command,
            })
            .unwrap();
    }
    let output = std::process::Command::new("jj")
        .current_dir(&path)
        .args([
            "--no-pager",
            "op",
            "log",
            "--no-graph",
            "-n",
            "1",
            "-T",
            "id",
        ])
        .env("JJ_CONFIG", "/dev/null")
        .output()
        .unwrap();
    assert!(output.status.success());
    let operation = String::from_utf8(output.stdout).unwrap().parse().unwrap();
    std::fs::write(path.join("dirty.txt"), "uncommitted").unwrap();
    let selected = local.metadata_store().read().unwrap().unwrap();
    let actor = local.actor().unwrap();
    let archive = local.conversion_archive(operation).await.unwrap();
    assert_eq!(archive.collaboration.len(), 2);
    assert_eq!(
        archive.collaboration[0].work.as_ref().unwrap().projects[&project].title,
        "Original"
    );
    assert_eq!(
        archive.collaboration[1].work.as_ref().unwrap().projects[&project].title,
        "Renamed"
    );
    let jj_store_jj::conversion::CodeArchiveV1::Git { objects, .. } = &archive.code else {
        panic!()
    };
    assert!(
        objects
            .iter()
            .any(|o| o.oid.to_string() == selected.commit().to_string())
    );
    assert_eq!(
        local.metadata_store().read().unwrap().unwrap().commit(),
        selected.commit()
    );
    assert_eq!(local.actor().unwrap(), actor);
    assert_eq!(
        std::fs::read_to_string(path.join("dirty.txt")).unwrap(),
        "uncommitted"
    );
    let bytes = archive.encode().unwrap();
    let decoded = RepositoryArchiveV1::decode(&bytes).unwrap();
    assert_eq!(decoded.encode().unwrap(), bytes);
    let destination_id = m::RepositoryId::new([85; 16]);
    let host = jj_store_service::administration::RepositoryHost::new(
        Arc::new(MemoryStore::default()),
        Arc::new(Operators),
        Arc::new(jj_store_service::auth::SystemClock),
        Default::default(),
        jj_store_service::native::SessionLimits::default(),
        16 * 1024 * 1024,
    )
    .unwrap();
    host.create_local_trusted(
        destination_id,
        m::EpochId::new([86; 16]),
        jj_store_service::auth::Permission::Admin,
    )
    .await
    .unwrap();
    let session = host
        .endpoint(destination_id)
        .unwrap()
        .bind_local_trusted(
            m::PrincipalId::new([87; 16]),
            jj_store_service::auth::Permission::Admin,
        )
        .unwrap()
        .into_session();
    let mappings = session
        .import_conversion_history(&archive, m::RequestId::new([88; 16]))
        .await
        .unwrap();
    assert_eq!(mappings.selections.len(), 2);
    assert_eq!(mappings.selections[1].versions["work"], 2);
    let native_archive = session
        .conversion_archive(jj_store_jj::roots::ROOT_OPERATION)
        .await
        .unwrap();
    assert!(
        native_archive
            .collaboration
            .iter()
            .any(|s| s.work.as_ref().is_some_and(|w| w
                .projects
                .get(&project)
                .is_some_and(|p| p.title == "Original")))
    );
    assert_eq!(
        native_archive
            .collaboration
            .last()
            .unwrap()
            .work
            .as_ref()
            .unwrap()
            .projects[&project]
            .title,
        "Renamed"
    );
    let exported = tmp.path().join("exported");
    jj_store_jj::conversion::to_local(&native_archive.code, &tmp.path().join("scratch"), &exported)
        .await
        .unwrap();
    let exported = LocalRepository::new(
        &exported.join(".jj/repo"),
        MetadataStore::open(&exported.join(".git"), LocalLimits::default()).unwrap(),
    );
    let final_id = m::RepositoryId::new([89; 16]);
    let maps = exported
        .import_conversion_history(
            &native_archive,
            final_id,
            m::PrincipalId::new([90; 16]),
            m::RequestId::new([91; 16]),
        )
        .await
        .unwrap();
    assert_eq!(maps.selections.len(), native_archive.collaboration.len());
    let history = exported
        .metadata_store()
        .history(&exported.metadata_store().read().unwrap().unwrap())
        .unwrap();
    assert_eq!(history.len(), native_archive.collaboration.len());
    assert!(exported.actor().unwrap().is_none());
    let query = work::WorkQuery {
        project: None,
        task: None,
        limit: 256,
        cursor: None,
    };
    assert_eq!(
        exported.work_page(&query).unwrap().projects[0].title,
        "Renamed"
    );
    assert!(history.iter().any(|pin| {
        exported
            .work_page_at(pin.commit(), &query)
            .unwrap()
            .projects
            .iter()
            .any(|p| p.title == "Original")
    }));

    let mut padded = bytes.clone();
    padded.push(b' ');
    assert!(RepositoryArchiveV1::decode(&padded).is_err());
}
struct Operators;
#[async_trait::async_trait]
impl jj_store_service::administration::OperatorPolicies for Operators {
    async fn current(&self) -> jj_store_service::auth::Result<jj_store_service::auth::TokenPolicy> {
        Ok(jj_store_service::auth::TokenPolicy::new(
            jj_store_service::auth::Scope::Operator,
        ))
    }
}
#[tokio::test]
async fn native_history_archive_contains_selected_evidence_and_excludes_configuration() {
    use jj_store_service::{
        administration::RepositoryHost,
        auth::{Permission, SystemClock},
        native::SessionLimits,
    };
    let store = Arc::new(MemoryStore::default());
    let repository = m::RepositoryId::new([80; 16]);
    let host = RepositoryHost::new(
        store,
        Arc::new(Operators),
        Arc::new(SystemClock),
        Default::default(),
        SessionLimits::default(),
        16 * 1024 * 1024,
    )
    .unwrap();
    host.create_local_trusted(repository, m::EpochId::new([81; 16]), Permission::Admin)
        .await
        .unwrap();
    let session = host
        .endpoint(repository)
        .unwrap()
        .bind_local_trusted(m::PrincipalId::new([82; 16]), Permission::Admin)
        .unwrap()
        .into_session();
    let snapshot = session.snapshot().await.unwrap();
    let operation = *snapshot.manifest().heads.iter().next().unwrap();
    let archive = session.conversion_archive(operation).await.unwrap();
    assert_eq!(archive.repository, repository);
    assert!(!archive.collaboration.is_empty());
    assert!(
        archive
            .native_records
            .iter()
            .all(|(reference, _)| !matches!(
                reference.kind,
                jj_store_wal::records::RecordKind::Settings
                    | jj_store_wal::records::RecordKind::ReplicationRoot
                    | jj_store_wal::records::RecordKind::Wal
            ))
    );
    let bytes = archive.encode().unwrap();
    assert_eq!(
        RepositoryArchiveV1::decode(&bytes)
            .unwrap()
            .encode()
            .unwrap(),
        bytes
    );
    assert_eq!(
        session.snapshot().await.unwrap().digest(),
        snapshot.digest()
    );
}
