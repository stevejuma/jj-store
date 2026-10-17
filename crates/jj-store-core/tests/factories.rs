use jj_store_core::*;
use jj_store_model::{PrincipalId, RequestId};
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct Host;
#[async_trait::async_trait]
impl StoreHost for Host {
    async fn authorize(&self, _: StoreKind, _: Access) -> Result<()> {
        Ok(())
    }
    fn actor(&self, _: bool) -> Result<Option<PrincipalId>> {
        Ok(None)
    }
    fn now_millis(&self) -> Result<i64> {
        Ok(0)
    }
}
fn context() -> StoreContext {
    StoreContext {
        scope: PublicationScope {
            repository: None,
            authority: "test-repository".into(),
        },
        host: Arc::new(Host),
        enabled: BTreeSet::new(),
    }
}
struct ExternalWork {
    context: StoreContext,
    name: String,
    calls: Arc<AtomicUsize>,
}
impl StoreBinding for ExternalWork {
    fn name(&self) -> &str {
        &self.name
    }
    fn scope(&self) -> &PublicationScope {
        &self.context.scope
    }
}
#[async_trait::async_trait]
impl WorkStore for ExternalWork {
    async fn snapshot(&self) -> Result<Pinned<Arc<jj_work::WorkStateV1>>> {
        self.context.admit(StoreKind::Work, Access::Read).await?;
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Pinned {
            scope: self.scope().clone(),
            version: 0,
            selection: Vec::new(),
            data: Arc::new(jj_work::WorkStateV1::default()),
        })
    }
    async fn execute(
        &self,
        _: &StoreRequest<jj_work::WorkCommandV1>,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        Err(Error::Unsupported("test read-only provider"))
    }
    async fn status(
        &self,
        request_id: RequestId,
    ) -> Result<StoreOutcome<jj_work::WorkCommandResultV1>> {
        Ok(StoreOutcome::NotKnown { request_id })
    }
}
fn register(
    factories: &mut StoreFactories,
    name: &str,
    calls: Arc<AtomicUsize>,
    wrong_binding: bool,
) -> Result<()> {
    let implementation = name.to_owned();
    factories.add_work_store(name, move |context| {
        let mut context = context.clone();
        if wrong_binding {
            context.scope.authority = "another-repository".into();
        }
        Ok(Arc::new(ExternalWork {
            context,
            name: implementation.clone(),
            calls: calls.clone(),
        }))
    })
}
#[tokio::test]
async fn external_factory_is_independent_and_loading_never_reads_or_initializes() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut factories = StoreFactories::empty();
    register(&mut factories, "external", calls.clone(), false).unwrap();
    let mut config = StoreConfig::default();
    config.enable(StoreKind::Work, "external");
    let stores = factories.loader().open(&config, context()).unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(
        stores.agent.is_none()
            && stores.review.is_none()
            && stores.thread.is_none()
            && stores.reaction.is_none()
    );
    let work = stores.work.as_ref().unwrap();
    assert!(work.snapshot().await.unwrap().data.projects.is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        stores.require_shared_publication(&[StoreKind::Work, StoreKind::Agent]),
        Err(Error::Unsupported(_))
    ));
}
#[test]
fn invalid_configuration_is_rejected_before_factory_construction() {
    let constructed = Arc::new(AtomicUsize::new(0));
    let mut factories = StoreFactories::empty();
    let counter = constructed.clone();
    factories
        .add_work_store("external", move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
            Err(Error::Denied)
        })
        .unwrap();
    assert!(
        factories
            .add_work_store("external", |_| Err(Error::Denied))
            .is_err()
    );
    let mut config = StoreConfig::all("missing");
    config.enable(StoreKind::Work, "external");
    assert!(matches!(
        factories.loader().open(&config, context()),
        Err(Error::Unsupported(_))
    ));
    assert_eq!(constructed.load(Ordering::SeqCst), 0);
    let mut config = StoreConfig::default();
    config.enable(StoreKind::Work, "external");
    assert!(matches!(
        factories.loader().open(&config, context()),
        Err(Error::Denied)
    ));
}
#[test]
fn provider_binding_mismatch_and_disabled_stores_fail_closed() {
    let mut factories = StoreFactories::empty();
    register(
        &mut factories,
        "external",
        Arc::new(AtomicUsize::new(0)),
        true,
    )
    .unwrap();
    let mut config = StoreConfig::default();
    config.enable(StoreKind::Work, "external");
    assert!(matches!(
        factories.loader().open(&config, context()),
        Err(Error::Invalid(_))
    ));
    config.disable(StoreKind::Work);
    let stores = factories.loader().open(&config, context()).unwrap();
    assert!(stores.work.is_none());
    assert!(
        PublicationScope {
            repository: None,
            authority: "a".into()
        }
        .require_same(&PublicationScope {
            repository: None,
            authority: "b".into()
        })
        .is_err()
    );
}
