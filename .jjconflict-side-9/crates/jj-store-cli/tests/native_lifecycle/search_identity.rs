use super::*;
use jj_store_tantivy::{
    native::{Config, NativeExtractor, Scope},
    *,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_rejects_mislabeled_extractor_configuration_before_recovery() {
    let f = Fixture::new().await;
    let config = Config::default();
    for field in ["config", "implementation", "schema", "name"] {
        let root = tempfile::tempdir().unwrap();
        let mut identity = config.identity().unwrap();
        match field {
            "config" => identity.config = m::RecordDigest::new([99; 32]),
            "implementation" => identity.implementation = "other".into(),
            "schema" => identity.schema += 1,
            _ => identity.name = "other".into(),
        }
        let (projection, _search) = TantivyProjection::open(
            root.path().join("index"),
            NativeExtractor::new(config.clone()).unwrap(),
            Default::default(),
        )
        .unwrap();
        let runner = a::Runner::new(
            f.session.adapter_source(),
            identity,
            projection,
            root.path(),
            Default::default(),
        );
        assert!(
            matches!(runner, Err(a::Error::Invalid)),
            "mislabeled {field}"
        );
        assert!(
            !root.path().join("adapters").exists(),
            "invalid config acquired writer ownership"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn native_search_config_change_hides_incompatible_recovery_until_rebuild() {
    tokio::task::LocalSet::new().run_until(async {
        let f=Fixture::new().await;
        publish_text_scope(&f,roots::ROOT_COMMIT,1,&[("private.rs",b"priorconfigsecret")]).await;
        let config=Config { scopes: [("main".into(),Scope::Bookmark{name:"main".into()})].into(), ..Default::default() };
        let root=tempfile::tempdir().unwrap();
        let source=f.session.adapter_source();
        let (projection,search)=TantivyProjection::open(root.path().join("index"),NativeExtractor::new(config.clone()).unwrap(),Default::default()).unwrap();
        let mut runner=a::Runner::new(source.clone(),config.identity().unwrap(),projection,root.path(),Default::default()).unwrap();
        runner.step(a::CancellationToken::new()).await.unwrap();
        let old=search.status(&source).await.unwrap().visible.unwrap();
        assert_eq!(search.pin(&source).await.unwrap().query(&source,SearchRequest::historical("priorconfigsecret",0,20),Default::default()).await.unwrap().hits.len(),1);
        drop(runner);drop(search);
        let config=Config{exclude_prefixes:BTreeSet::from(["private.rs".into()]),..config};
        let (projection,search)=TantivyProjection::open(root.path().join("index"),NativeExtractor::new(config.clone()).unwrap(),Default::default()).unwrap();
        let faults=projection.faults();
        let mut runner=a::Runner::new(source.clone(),config.identity().unwrap(),projection,root.path(),Default::default()).unwrap();
        faults.once(Fault::BeforeCommit);
        assert!(matches!(runner.step(a::CancellationToken::new()).await,Err(a::Error::Projection)));
        let status=search.status(&source).await.unwrap();
        assert!(status.visible.is_none(),"incompatible recovered source config must not become searchable even if rebuilding fails");
        assert!(status.durable.is_none());
        assert!(status.degraded);
        assert!(search.pin(&source).await.is_err());
        let rebuilt=runner.step(a::CancellationToken::new()).await.unwrap().applied.unwrap();
        assert_eq!(rebuilt.identity,config.identity().unwrap());
        assert_eq!(rebuilt.source,old.source);
        assert_ne!(rebuilt.artifact,old.artifact);
        assert!(search.pin(&source).await.unwrap().query(&source,SearchRequest::historical("priorconfigsecret",0,20),Default::default()).await.unwrap().hits.is_empty());
        assert!(!search.status(&source).await.unwrap().degraded);
    }).await;
}
