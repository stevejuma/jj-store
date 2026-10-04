use super::*;
use jj_store_adapters as a;
use jj_store_service::{
    administration::{TokenChange, TokenRequest},
    commands::{Command, Executor},
    mutations::{CommandService, Outcome, Request as MutationRequest},
};
struct Inspect {
    checkpoint: Option<a::Checkpoint>,
    staged: m::ObjectKey,
    seen: Arc<std::sync::Mutex<Vec<m::ObjectKey>>>,
}
#[async_trait::async_trait]
impl a::Projection for Inspect {
    async fn recover(&mut self, context: a::Context) -> a::Result<Option<a::Checkpoint>> {
        context.check()?;
        Ok(self.checkpoint.clone())
    }
    async fn bootstrap(
        &mut self,
        snapshot: a::Snapshot,
        identity: &a::Identity,
        context: a::Context,
    ) -> a::Result<a::Checkpoint> {
        for key in snapshot.objects.entries().keys().copied() {
            context.check()?;
            if let Some(bytes) = snapshot.objects.read(key).await? {
                match key {
                    m::ObjectKey::Operation(_) => {
                        jj_store_jj::codec::decode::<o::Operation>(key, &bytes, 4 * 1024 * 1024)
                            .unwrap();
                    }
                    m::ObjectKey::View(_) => {
                        jj_store_jj::codec::decode::<o::View>(key, &bytes, 4 * 1024 * 1024)
                            .unwrap();
                    }
                    m::ObjectKey::Commit(_) => {
                        jj_store_jj::codec::decode::<b::Commit>(key, &bytes, 4 * 1024 * 1024)
                            .unwrap();
                    }
                    _ => {}
                }
                self.seen.lock().unwrap().push(key);
            }
        }
        assert_eq!(
            snapshot.objects.read(self.staged).await.unwrap_err(),
            a::Error::Denied
        );
        let checkpoint = a::Checkpoint {
            version: 1,
            identity: identity.clone(),
            source: snapshot.position,
            artifact: m::RecordDigest::new([1; 32]),
        };
        self.checkpoint = Some(checkpoint.clone());
        Ok(checkpoint)
    }
    async fn apply(
        &mut self,
        batch: a::Batch,
        identity: &a::Identity,
        context: a::Context,
    ) -> a::Result<a::Checkpoint> {
        self.bootstrap(batch.snapshot, identity, context).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_adapter_source_verifies_committed_objects_and_rechecks_revocation() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let f = Fixture::new().await;
            let commands = CommandService::new(Executor::new(Default::default(), 1).unwrap());
            let request = MutationRequest {
                version: 1,
                request_id: m::RequestId::new([191; 16]),
                expected: f.session.snapshot().await.unwrap().manifest().snapshot(),
                command: Command::New {
                    parents: vec![roots::ROOT_COMMIT],
                    description: "adapter metadata".into(),
                    workspace: None,
                },
            };
            let Outcome::Committed { receipt } =
                commands.apply(f.session.clone(), &request).await.unwrap()
            else {
                panic!("receipt")
            };
            let (file, _) = f
                .objects
                .put_file(b"unpublished bytes".as_slice())
                .await
                .unwrap();
            let read = f.session(f.reader.clone(), Protection::default());
            let root = tempfile::tempdir().unwrap();
            let seen = Arc::new(std::sync::Mutex::new(vec![]));
            let identity = a::Identity {
                name: "native-test".into(),
                implementation: "1".into(),
                schema: 1,
                config: m::RecordDigest::new([2; 32]),
            };
            let mut runner = a::Runner::new(
                read.adapter_source(),
                identity,
                Inspect {
                    checkpoint: None,
                    staged: m::ObjectKey::File(file),
                    seen: seen.clone(),
                },
                root.path(),
                Default::default(),
            )
            .unwrap();
            let health = runner.step(a::CancellationToken::new()).await.unwrap();
            assert_eq!(
                health.applied.as_ref().unwrap().source.position,
                receipt.accepted.position
            );
            assert_eq!(health.lag, Some(0));
            assert!(
                seen.lock()
                    .unwrap()
                    .contains(&m::ObjectKey::Operation(receipt.result.operation))
            );
            assert!(
                seen.lock()
                    .unwrap()
                    .iter()
                    .any(|k| matches!(k, m::ObjectKey::Commit(_)))
            );
            assert_eq!(
                f.session.snapshot().await.unwrap().manifest().snapshot(),
                receipt.accepted
            );
            f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([192; 16]),
                    expected: receipt.accepted.clone(),
                    change: TokenChange::Revoke {
                        token: f.reader.id(),
                    },
                })
                .await
                .unwrap();
            let before = seen.lock().unwrap().len();
            assert_eq!(
                runner.step(a::CancellationToken::new()).await.unwrap_err(),
                a::Error::Denied
            );
            assert_eq!(seen.lock().unwrap().len(), before);
            assert_eq!(runner.health().applied, health.applied);
        })
        .await;
}
