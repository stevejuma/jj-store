//! Independent library consumer; the example host trusts only synthetic fixture data.
use jj_store_core::*;
#[cfg(feature = "native")]
use jj_store_model as m;
use jj_store_model::{PrincipalId, RequestId};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
mod git;
#[cfg(feature = "native")]
mod native;
const ACTOR: PrincipalId = PrincipalId::new([45; 16]);
#[derive(Default)]
struct Host {
    denied: AtomicBool,
    writes: AtomicUsize,
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
    fn actor(&self, writing: bool) -> Result<Option<PrincipalId>> {
        if writing {
            self.writes.fetch_add(1, Ordering::SeqCst);
        }
        Ok(Some(ACTOR))
    }
    fn now_millis(&self) -> Result<i64> {
        Ok(12345)
    }
}
struct CustomWork {
    context: StoreContext,
    calls: Arc<AtomicUsize>,
}
impl StoreBinding for CustomWork {
    fn name(&self) -> &str {
        "example-work"
    }
    fn scope(&self) -> &PublicationScope {
        &self.context.scope
    }
}
#[async_trait::async_trait]
impl WorkStore for CustomWork {
    async fn snapshot(&self) -> Result<Pinned<Arc<jj_work::WorkStateV1>>> {
        self.context.admit(StoreKind::Work, Access::Read).await?;
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Pinned {
            scope: self.scope().clone(),
            version: 0,
            selection: b"example-work".to_vec(),
            data: Arc::new(jj_work::WorkStateV1::default()),
        })
    }
    async fn execute(
        &self,
        _: &StoreRequest<jj_work::WorkCommandV1>,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        self.context.admit(StoreKind::Work, Access::Write).await?;
        Err(Error::Unsupported("read-only example WorkStore"))
    }
    async fn status(
        &self,
        request_id: RequestId,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        self.context.admit(StoreKind::Work, Access::Write).await?;
        Ok(StoreOutcome::NotKnown { request_id })
    }
}
fn register_custom(factories: &mut StoreFactories, calls: Arc<AtomicUsize>) {
    factories
        .add_work_store("example-work", move |context| {
            Ok(Arc::new(CustomWork {
                context: context.clone(),
                calls: calls.clone(),
            }))
        })
        .unwrap();
}
fn request<C>(id: u8, command: C) -> StoreRequest<C> {
    StoreRequest::new(RequestId::new([id; 16]), command)
}
fn committed<T>(outcome: StoreOutcome<T>) -> StoreReceipt<T> {
    let StoreOutcome::Committed(value) = outcome else {
        panic!("committed receipt expected")
    };
    value
}
fn capture(binding: Option<jj_collab::ThreadBindingV1>) -> AgentCommand {
    let values: Vec<serde_json::Value> =
        serde_json::from_slice(include_bytes!("../fixtures/codex-hooks.json")).unwrap();
    AgentCommand::Capture {
        profile: jj_agent::NormalizerProfileV1::codex_0_156_1(),
        binding,
        batch: Box::new(jj_agent::SanitizedObservationBatchV1 {
            schema_version: 1,
            batch_id: jj_agent::CaptureBatchId::new([46; 16]),
            integration: jj_agent::IntegrationId::Codex,
            source_instance_id: jj_agent::SourceInstanceId::new([47; 16]),
            sanitizer_version: "agent-redaction@0.1.0".into(),
            representation: jj_agent::SanitizedRepresentation::SanitizedNative,
            observations: values
                .into_iter()
                .enumerate()
                .map(|(i, value)| jj_agent::SanitizedObservationV1 {
                    observation_id: jj_agent::ObservationId::new([i as u8 + 1; 16]),
                    source_sequence: i as u64 + 1,
                    occurred_at_millis: Some(1725000000000 + i as i64),
                    source_kind: jj_agent::SourceKind::Hook,
                    source_trust: jj_agent::SourceTrust::LocalUserHook,
                    payload: jj_agent::SanitizedPayloadV1::new(
                        jj_agent::SanitizedMediaType::Json,
                        &serde_json::to_vec(&value).unwrap(),
                    )
                    .unwrap(),
                })
                .collect(),
        }),
    }
}
fn thread(id: u8) -> StoreRequest<jj_collab::ThreadCommandV1> {
    request(
        id,
        jj_collab::ThreadCommandV1::Create {
            thread_id: jj_collab::ThreadId::new([id; 16]),
            title: "Embedding example".into(),
            baseline: None,
        },
    )
}
fn reaction(id: u8, thread: jj_collab::ThreadId) -> StoreRequest<ReactionCommand> {
    request(
        id,
        ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
            emoji: None,
            target: jj_collab::ReactionTargetV1::Thread { thread_id: thread },
            value: jj_collab::ReactionValueV1::Like,
            observed: Default::default(),
        }),
    )
}
fn review(id: u8) -> StoreRequest<ReviewCommand> {
    request(
        id,
        ReviewCommand::RepositoryThread {
            body: jj_review::ReviewCommentBodyV1 {
                schema_version: 1,
                format: jj_review::ReviewBodyFormatV1::PlainText,
                text: "Example review".into(),
                sanitizer_version: "plain@1".into(),
            },
        },
    )
}
#[tokio::main]
async fn main() {
    git::run().await;
    #[cfg(feature = "native")]
    native::run().await;
    println!("Independent store composition and original receipt replay passed.");
}
#[cfg(test)]
mod tests {
    #[tokio::test(flavor = "multi_thread")]
    async fn git_and_independent_custom_store() {
        super::git::run().await;
    }
    #[cfg(feature = "native")]
    #[tokio::test(flavor = "multi_thread")]
    async fn native_without_application_services() {
        super::native::run().await;
    }
}
