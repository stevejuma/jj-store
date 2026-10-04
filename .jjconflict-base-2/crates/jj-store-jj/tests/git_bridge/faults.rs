//! Every durable Git stage tolerates unknown manifest outcomes and cold replay.
use super::*;
use jj_store_object::{ObjectStore, memory::Fault};
use std::sync::atomic::{AtomicU8, Ordering};
#[derive(Default)]
struct FaultStore {
    inner: MemoryStore,
    mode: AtomicU8,
}
#[async_trait::async_trait]
impl ObjectStore for FaultStore {
    async fn head(
        &self,
        key: &jj_store_object::Key,
    ) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &jj_store_object::Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &jj_store_object::Key,
        condition: jj_store_object::Condition,
        upload: &jj_store_object::StagedUpload,
    ) -> jj_store_object::Result<jj_store_object::WriteOutcome> {
        if key.as_str().ends_with("/manifest.json") {
            match self.mode.swap(0, Ordering::SeqCst) {
                1 => self.inner.inject(Fault::BeforeWrite),
                2 => self.inner.inject(Fault::AfterWrite),
                _ => {}
            }
        }
        self.inner.put(key, condition, upload).await
    }
}
#[tokio::test]
async fn git_stage_manifest_unknown_outcomes_cold_replay_without_duplicate_effects() {
    for mode in [1, 2] {
        let store = Arc::new(FaultStore::default());
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 32 * 1024 * 1024).unwrap());
        let make = || {
            Coordinator::new(
                store.clone(),
                REPO,
                objects.clone(),
                Arc::new(Permit),
                Limits::default(),
            )
            .unwrap()
        };
        let c = make();
        c.initialize(m::EpochId::new([160; 16]), Vec::new())
            .await
            .unwrap();
        let operation = *c.read().await.unwrap().manifest().heads.first().unwrap();
        let remote = m::RequestId::new([161; 16]);
        let binding = m::RecordDigest::new([162; 32]);
        let observation = m::RequestId::new([163; 16]);
        let attempt = m::RequestId::new([164; 16]);
        let started = m::RequestId::new([165; 16]);
        let result = m::RequestId::new([166; 16]);
        let applied = m::RequestId::new([167; 16]);
        let observed = m::RequestId::new([168; 16]);
        let desired = GitRefV1 {
            oid: None,
            commit_oid: None,
            native: None,
        };
        let refs =
            std::collections::BTreeMap::from([("refs/heads/deleted".into(), desired.clone())]);
        let stages = vec![
            vec![
                GitRecordV1::Remote(GitRemoteV1 {
                    remote,
                    event: remote,
                    predecessors: BTreeSet::new(),
                    name: "origin".into(),
                    active: true,
                }),
                GitRecordV1::Observation(GitObservationV1 {
                    id: observation,
                    remote,
                    binding,
                    predecessors: BTreeSet::new(),
                    operation,
                    complete_heads: true,
                    refs: std::collections::BTreeMap::new(),
                }),
            ],
            vec![GitRecordV1::PushIntent(GitPushIntentV1 {
                id: attempt,
                executor: ACTOR,
                remote,
                binding,
                operation,
                observations: BTreeSet::from([observation]),
                refs: std::collections::BTreeMap::from([(
                    "refs/heads/deleted".into(),
                    GitPushRefV1 {
                        expected: None,
                        desired,
                    },
                )]),
            })],
            vec![GitRecordV1::PushEvent(GitPushEventV1 {
                id: started,
                attempt,
                predecessors: BTreeSet::new(),
                kind: GitPushEventKindV1::TransportStarted,
            })],
            vec![GitRecordV1::PushEvent(GitPushEventV1 {
                id: result,
                attempt,
                predecessors: BTreeSet::from([started]),
                kind: GitPushEventKindV1::ResultRecorded {
                    refs: std::collections::BTreeMap::from([(
                        "refs/heads/deleted".into(),
                        GitPushRefOutcomeV1::Acknowledged,
                    )]),
                },
            })],
            vec![
                GitRecordV1::Observation(GitObservationV1 {
                    id: observed,
                    remote,
                    binding,
                    predecessors: BTreeSet::from([observation]),
                    operation,
                    complete_heads: false,
                    refs,
                }),
                GitRecordV1::PushEvent(GitPushEventV1 {
                    id: applied,
                    attempt,
                    predecessors: BTreeSet::from([result]),
                    kind: GitPushEventKindV1::Applied {
                        observation: observed,
                    },
                }),
            ],
        ];
        for (index, records) in stages.into_iter().enumerate() {
            let request = Request {
                principal: ACTOR,
                request_id: m::RequestId::new([180 + index as u8; 16]),
                publication_id: m::PublicationId::new([180 + index as u8; 16]),
                transition: None,
                settings: None,
                result: Vec::new(),
                kind: SemanticKind::GitStateChanged,
            };
            let before = c.read().await.unwrap();
            store.mode.store(mode, Ordering::SeqCst);
            let _unknown = c.publish_git(&request, &records).await;
            assert_eq!(
                store.mode.load(Ordering::SeqCst),
                0,
                "fault boundary was exercised"
            );
            let cold = make();
            let recovered = cold.recover(4096).await.unwrap();
            assert_eq!(recovered.manifest().heads, before.manifest().heads);
            assert_eq!(
                recovered.manifest().code_state_version,
                before.manifest().code_state_version
            );
            assert!(
                recovered.manifest().git_state_version == before.manifest().git_state_version
                    || recovered.manifest().git_state_version
                        == before.manifest().git_state_version + 1
            );
            let receipt = cold.publish_git(&request, &records).await.unwrap();
            assert!(matches!(receipt, PublishOutcome::Committed(_)));
            let selected = cold.read().await.unwrap();
            assert_eq!(
                selected.manifest().git_state_version,
                before.manifest().git_state_version + 1
            );
            assert_eq!(cold.publish_git(&request, &records).await.unwrap(), receipt);
            assert_eq!(cold.read().await.unwrap().digest(), selected.digest());
        }
        c.checkpoint().await.unwrap();
        let cold = make();
        let recovered = cold.recover(4096).await.unwrap();
        assert_eq!(
            cold.git_store(recovered.manifest().git_root.unwrap())
                .await
                .unwrap()
                .push_status(attempt),
            "applied"
        );
    }
}
