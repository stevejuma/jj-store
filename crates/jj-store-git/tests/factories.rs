use jj_store_core::*;
use jj_store_git::{
    GitProvider, LocalRepository,
    metadata::{Limits, MetadataStore},
};
use jj_store_model as m;
use std::{
    collections::BTreeSet,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
struct Host {
    writes: AtomicUsize,
    denied: AtomicBool,
}
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, _: StoreKind, _: Access) -> Result<()> {
        if self.denied.load(Ordering::SeqCst) {
            Err(Error::Denied)
        } else {
            Ok(())
        }
    }
    fn actor(&self, writing: bool) -> Result<Option<m::PrincipalId>> {
        if writing {
            self.writes.fetch_add(1, Ordering::SeqCst);
        }
        Ok(Some(m::PrincipalId::new([11; 16])))
    }
    fn now_millis(&self) -> Result<i64> {
        Ok(12345)
    }
}
fn host() -> Arc<Host> {
    Arc::new(Host {
        writes: AtomicUsize::new(0),
        denied: AtomicBool::new(false),
    })
}
fn repo(path: &Path) -> LocalRepository {
    LocalRepository::new(path, MetadataStore::open(path, Limits::default()).unwrap())
}
fn open(
    repository: LocalRepository,
    host: Arc<Host>,
    configure: impl FnOnce(&GitProvider, &mut StoreFactories, &mut StoreConfig),
) -> RepositoryStores {
    let provider = GitProvider::new(repository);
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "git").unwrap();
    let mut config = StoreConfig::all("git");
    configure(&provider, &mut factories, &mut config);
    factories
        .loader()
        .open(&config, StoreContext::new(provider.scope(), host))
        .unwrap()
}
fn committed<T>(outcome: StoreOutcome<T>) -> StoreReceipt<T> {
    let StoreOutcome::Committed(r) = outcome else {
        panic!("expected committed receipt")
    };
    r
}
fn request<C>(id: u8, command: C) -> StoreRequest<C> {
    StoreRequest::new(m::RequestId::new([id; 16]), command)
}
fn batch() -> jj_agent::SanitizedObservationBatchV1 {
    use jj_agent::*;
    let values: Vec<serde_json::Value> = serde_json::from_slice(include_bytes!(
        "../../jj-agent/tests/fixtures/codex-0.156.1-hooks.json"
    ))
    .unwrap();
    SanitizedObservationBatchV1 {
        schema_version: 1,
        batch_id: CaptureBatchId::new([70; 16]),
        integration: IntegrationId::Codex,
        source_instance_id: SourceInstanceId::new([71; 16]),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        representation: SanitizedRepresentation::SanitizedNative,
        observations: values
            .into_iter()
            .enumerate()
            .map(|(i, v)| SanitizedObservationV1 {
                observation_id: ObservationId::new([i as u8 + 1; 16]),
                source_sequence: i as u64 + 1,
                occurred_at_millis: Some(1725000000000 + i as i64),
                source_kind: SourceKind::Hook,
                source_trust: SourceTrust::LocalUserHook,
                payload: SanitizedPayloadV1::new(
                    SanitizedMediaType::Json,
                    &serde_json::to_vec(&v).unwrap(),
                )
                .unwrap(),
            })
            .collect(),
    }
}
#[tokio::test]
async fn factories_are_read_only_on_load_and_all_five_domains_reopen_and_replay() {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let host = host();
    let stores = open(repo(dir.path()), host.clone(), |_, _, _| {});
    assert_eq!(host.writes.load(Ordering::SeqCst), 0);
    assert!(repo(dir.path()).repository().unwrap().is_none());
    assert!(!dir.path().join("lapis").exists());
    let agents = stores.agent().unwrap();
    let work = stores.work().unwrap();
    let reviews = stores.review().unwrap();
    let threads = stores.thread().unwrap();
    let reactions = stores.reaction().unwrap();
    assert_eq!(agents.snapshot().await.unwrap().version, 0);
    assert_eq!(work.snapshot().await.unwrap().version, 0);
    assert_eq!(reviews.snapshot().await.unwrap().version, 0);
    assert_eq!(threads.snapshot().await.unwrap().version, 0);
    assert_eq!(reactions.snapshot().await.unwrap().version, 0);
    assert_eq!(host.writes.load(Ordering::SeqCst), 0);
    let ar = request(
        1,
        AgentCommand::Capture {
            profile: jj_agent::NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch()),
            binding: None,
        },
    );
    let a = committed(stores.execute_agent(&ar).await.unwrap());
    let wr = request(
        2,
        jj_work::WorkCommandV1::CreateProject {
            id: jj_work::ProjectId::new([2; 16]),
            title: "Project".into(),
            body: String::new(),
        },
    );
    let w = committed(work.execute(&wr).await.unwrap());
    assert_eq!(w.actor, m::PrincipalId::new([11; 16]));
    assert_eq!(
        w.result.project.as_ref().unwrap().actor.created_at_millis,
        12345
    );
    let rr = request(
        3,
        ReviewCommand::RepositoryThread {
            body: jj_review::ReviewCommentBodyV1 {
                schema_version: 1,
                format: jj_review::ReviewBodyFormatV1::PlainText,
                text: "Review".into(),
                sanitizer_version: "plain@1".into(),
            },
        },
    );
    let r = committed(reviews.execute(&rr).await.unwrap());
    let tr = request(
        4,
        jj_collab::ThreadCommandV1::Create {
            thread_id: jj_collab::ThreadId::new([4; 16]),
            title: "Thread".into(),
            baseline: None,
        },
    );
    let t = committed(threads.execute(&tr).await.unwrap());
    let xr = request(
        5,
        ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
            emoji: None,
            target: jj_collab::ReactionTargetV1::Thread {
                thread_id: jj_collab::ThreadId::new([4; 16]),
            },
            value: jj_collab::ReactionValueV1::Like,
            observed: BTreeSet::new(),
        }),
    );
    let x = committed(reactions.execute(&xr).await.unwrap());
    assert_eq!(agents.snapshot().await.unwrap().version, 1);
    assert_eq!(work.snapshot().await.unwrap().version, 1);
    assert_eq!(reviews.snapshot().await.unwrap().version, 1);
    assert_eq!(threads.snapshot().await.unwrap().version, 1);
    assert_eq!(reactions.snapshot().await.unwrap().version, 1);
    drop(stores);
    let reopened = open(repo(dir.path()), host, |_, _, _| {});
    assert_eq!(
        committed(reopened.execute_agent(&ar).await.unwrap()).provider_receipt,
        a.provider_receipt
    );
    assert_eq!(
        committed(reopened.work().unwrap().execute(&wr).await.unwrap()).provider_receipt,
        w.provider_receipt
    );
    assert_eq!(
        committed(reopened.review().unwrap().execute(&rr).await.unwrap()).provider_receipt,
        r.provider_receipt
    );
    assert_eq!(
        committed(reopened.thread().unwrap().execute(&tr).await.unwrap()).provider_receipt,
        t.provider_receipt
    );
    assert_eq!(
        committed(reopened.reaction().unwrap().execute(&xr).await.unwrap()).provider_receipt,
        x.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .agent()
                .unwrap()
                .status(ar.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        a.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .work()
                .unwrap()
                .status(wr.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        w.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .review()
                .unwrap()
                .status(rr.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        r.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .thread()
                .unwrap()
                .status(tr.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        t.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .reaction()
                .unwrap()
                .status(xr.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        x.provider_receipt
    );
    assert!(matches!(
        reopened.thread().unwrap().status(xr.request_id).await,
        Err(Error::IntentConflict)
    ));
}
#[tokio::test]
async fn bound_capture_requires_selected_atomic_participant_before_any_write() {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let host = host();
    let repository = repo(dir.path());
    let other = GitProvider::new(repository.clone());
    let stores = open(repository.clone(), host.clone(), |_, factories, config| {
        other.add_thread_store(factories, "another").unwrap();
        config.enable(StoreKind::Thread, "another");
    });
    let create = request(
        1,
        jj_collab::ThreadCommandV1::Create {
            thread_id: jj_collab::ThreadId::new([1; 16]),
            title: "bound".into(),
            baseline: None,
        },
    );
    let t = committed(stores.thread().unwrap().execute(&create).await.unwrap());
    let jj_collab::CollabCommandResultV1::Thread { result: t } = t.result else {
        panic!("thread")
    };
    let writes = host.writes.load(Ordering::SeqCst);
    let before = repository
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    let capture = request(
        2,
        AgentCommand::Capture {
            profile: jj_agent::NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch()),
            binding: Some(jj_collab::ThreadBindingV1 {
                thread_id: t.thread_id,
                observed_thread: t.version,
                observed_assignment: None,
            }),
        },
    );
    assert!(matches!(
        stores.execute_agent(&capture).await,
        Err(Error::Unsupported(_))
    ));
    assert_eq!(host.writes.load(Ordering::SeqCst), writes);
    assert_eq!(
        repository
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        before
    );
    let stores = open(repository.clone(), host, |_, _, _| {});
    let accepted = committed(stores.execute_agent(&capture).await.unwrap());
    let selected = repository.metadata_store().read().unwrap().unwrap();
    assert_eq!(selected.sequence(), 2);
    assert_eq!(
        selected
            .selection(jj_store_git::metadata::Domain::Agent)
            .unwrap()
            .version,
        1
    );
    assert_eq!(
        selected
            .selection(jj_store_git::metadata::Domain::Thread)
            .unwrap()
            .version,
        2
    );
    let history = repository.metadata_store().history(&selected).unwrap();
    assert_eq!(history.len(), 2);
    assert!(
        history[1]
            .selection(jj_store_git::metadata::Domain::Agent)
            .is_none()
    );
    assert_eq!(
        committed(stores.execute_agent(&capture).await.unwrap()).provider_receipt,
        accepted.provider_receipt
    );
}
#[tokio::test]
async fn authorization_is_fresh_and_reading_or_rejection_never_initializes() {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let host = host();
    let stores = open(repo(dir.path()), host.clone(), |_, _, config| {
        config.disable(StoreKind::Agent);
    });
    assert!(stores.agent().is_none());
    host.denied.store(true, Ordering::SeqCst);
    assert!(matches!(
        stores.work().unwrap().snapshot().await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        stores
            .thread()
            .unwrap()
            .status(m::RequestId::new([1; 16]))
            .await,
        Err(Error::Denied)
    ));
    assert_eq!(host.writes.load(Ordering::SeqCst), 0);
    assert!(repo(dir.path()).repository().unwrap().is_none());
}
#[tokio::test]
async fn staged_metadata_is_not_a_selected_domain_read_capability() {
    use jj_store_git::metadata::*;
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let repository = repo(dir.path());
    let metadata = repository.metadata_store();
    let id = m::RepositoryId::new([8; 16]);
    let bytes = serde_json::to_vec(
        &jj_store_git::repository::migration::work::LocalWorkRootV1::from_state(
            id,
            jj_work::WorkStateV1::default(),
        ),
    )
    .unwrap();
    let update = Update {
        repository: id,
        actor: m::PrincipalId::new([1; 16]),
        request: m::RequestId::new([1; 16]),
        intent: m::IntentDigest::new([1; 32]),
        domain: Domain::Work,
        expected: None,
        bytes,
        result: Vec::new(),
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = metadata.prepare(None, &update).unwrap() else {
        panic!("new candidate")
    };
    let staged = metadata.read_at(candidate.commit()).unwrap();
    assert!(metadata.read().unwrap().is_none());
    assert!(matches!(
        repository.work_state(Some(&staged)),
        Err(jj_store_git::work_api::Error::Stale)
    ));
    assert!(matches!(
        repository.agent_store_at(Some(&staged), id).await,
        Err(jj_store_git::LocalError::Stale)
    ));
    assert!(matches!(
        metadata.publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert!(
        repository
            .work_state(Some(&staged))
            .unwrap()
            .projects
            .is_empty()
    );
}
#[test]
fn missing_qualified_evidence_validator_rejects_anchored_data() {
    use sha2::{Digest as _, Sha256};
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let repository = repo(dir.path());
    let bytes = b"untrusted source evidence";
    let reference = m::portable::CodeEvidenceRefV1 {
        profile: m::portable::CodeProfileV1::GitSha1Jj045,
        repository: m::RepositoryId::new([1; 16]),
        operation: m::OperationId::new([1; 64]),
        capsule: m::CodeCapsuleId::new(Sha256::digest(bytes).into()),
    };
    assert!(matches!(
        repository
            .metadata_store()
            .validate_evidence(&reference, bytes),
        Err(jj_store_git::metadata::Error::Invalid(
            "qualified evidence validator required"
        ))
    ));
    assert!(!dir.path().join("lapis").exists());
    assert!(repository.repository().unwrap().is_none());
}

async fn excluded_thread_source(replace: bool) {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let host = host();
    let original = open(repo(dir.path()), host.clone(), |_, _, _| {});
    original
        .thread()
        .unwrap()
        .execute(&request(
            81,
            jj_collab::ThreadCommandV1::Create {
                thread_id: jj_collab::ThreadId::new([81; 16]),
                title: "Existing source".into(),
                baseline: None,
            },
        ))
        .await
        .unwrap();
    let selected = repo(dir.path())
        .metadata_store()
        .read()
        .unwrap()
        .unwrap()
        .commit();
    let writes = host.writes.load(Ordering::SeqCst);
    let stores = open(repo(dir.path()), host.clone(), |_, factories, config| {
        if replace {
            GitProvider::new(repo(dir.path()))
                .add_thread_store(factories, "replacement")
                .unwrap();
            config.enable(StoreKind::Thread, "replacement");
        } else {
            config.disable(StoreKind::Thread);
        }
    });
    let outcome = stores
        .reaction()
        .unwrap()
        .execute(&request(
            82,
            ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
                emoji: None,
                target: jj_collab::ReactionTargetV1::Thread {
                    thread_id: jj_collab::ThreadId::new([81; 16]),
                },
                value: jj_collab::ReactionValueV1::Like,
                observed: BTreeSet::new(),
            }),
        ))
        .await;
    assert!(
        matches!(outcome, Err(Error::Unsupported(_))),
        "unselected built-in source was used"
    );
    assert_eq!(host.writes.load(Ordering::SeqCst), writes);
    assert_eq!(
        repo(dir.path())
            .metadata_store()
            .read()
            .unwrap()
            .unwrap()
            .commit(),
        selected
    );
}
#[tokio::test]
async fn reactions_reject_disabled_thread_sources_before_staging() {
    excluded_thread_source(false).await
}
#[tokio::test]
async fn reactions_reject_replaced_thread_sources_before_staging() {
    excluded_thread_source(true).await
}

#[tokio::test]
async fn committed_reaction_replays_without_the_current_source_store() {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let host = host();
    let stores = open(repo(dir.path()), host.clone(), |_, _, _| {});
    let thread_id = jj_collab::ThreadId::new([91; 16]);
    stores
        .thread()
        .unwrap()
        .execute(&request(
            91,
            jj_collab::ThreadCommandV1::Create {
                thread_id,
                title: "Receipt source".into(),
                baseline: None,
            },
        ))
        .await
        .unwrap();
    let command = request(
        92,
        ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
            emoji: None,
            target: jj_collab::ReactionTargetV1::Thread { thread_id },
            value: jj_collab::ReactionValueV1::Like,
            observed: BTreeSet::new(),
        }),
    );
    let original = committed(stores.reaction().unwrap().execute(&command).await.unwrap());
    let reopened = open(repo(dir.path()), host.clone(), |_, _, config| {
        config.disable(StoreKind::Thread)
    });
    let writes = host.writes.load(Ordering::SeqCst);
    let replay = committed(
        reopened
            .reaction()
            .unwrap()
            .execute(&command)
            .await
            .unwrap(),
    );
    assert_eq!(replay.provider_receipt, original.provider_receipt);
    assert_eq!(host.writes.load(Ordering::SeqCst), writes);
}

#[tokio::test]
async fn different_registration_names_can_share_the_selected_atomic_provider() {
    let dir = tempfile::tempdir().unwrap();
    gix::init(dir.path()).unwrap();
    let stores = open(repo(dir.path()), host(), |provider, factories, config| {
        provider.add_thread_store(factories, "other-name").unwrap();
        config.enable(StoreKind::Thread, "other-name");
    });
    stores
        .require_shared_publication(&[StoreKind::Agent, StoreKind::Thread])
        .unwrap();
    let thread_id = jj_collab::ThreadId::new([93; 16]);
    stores
        .thread()
        .unwrap()
        .execute(&request(
            93,
            jj_collab::ThreadCommandV1::Create {
                thread_id,
                title: "Selected thread".into(),
                baseline: None,
            },
        ))
        .await
        .unwrap();
    committed(
        stores
            .reaction()
            .unwrap()
            .execute(&request(
                94,
                ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
                    emoji: None,
                    target: jj_collab::ReactionTargetV1::Thread { thread_id },
                    value: jj_collab::ReactionValueV1::Like,
                    observed: BTreeSet::new(),
                }),
            ))
            .await
            .unwrap(),
    );
}
