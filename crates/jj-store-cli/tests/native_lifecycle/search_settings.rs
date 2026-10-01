use super::*;
use jj_store_server::{config, search_management::Manager};
use jj_store_service::search_management::{
    SaveSearchSettings, SearchAction, SearchCommand, SearchManager, SearchOperation,
};
fn config_file(root: &std::path::Path) -> std::path::PathBuf {
    let path = root.join("server.toml");
    let text = config::EXAMPLE.replace(
        "mode = \"authenticated\"",
        "mode = \"local-trusted\"\nrpc_socket = \"./service.sock\"\npermission = \"admin\"",
    );
    let text = format!(
        "{text}\n[[search]]\nrepository = \"{REPO}\"\npoll_millis = 10\nsnapshot_every = 0\n[search.source.scopes.main]\nkind = \"bookmark\"\nname = \"main\"\n"
    );
    std::fs::write(&path, text).unwrap();
    path
}
fn compose(
    f: &Fixture,
    path: &std::path::Path,
) -> (PreparedSearch, Arc<Manager>, jj_store_http::HttpApi) {
    let config = config::Config::load(path).unwrap();
    let host = host(f);
    let (prepared, indexes) = PreparedSearch::new(
        &host,
        &config.state_dir,
        &config.search,
        &config.limits,
        SearchAccess::LocalTrusted(ACTOR),
        |_| unreachable!(),
    )
    .unwrap();
    let manager = Arc::new(
        Manager::new(
            path,
            &config,
            host.clone(),
            prepared.control(),
            SearchAccess::LocalTrusted(ACTOR),
            Arc::new(|_, _| {}),
        )
        .unwrap(),
    );
    let http = jj_store_http::HttpApi::with_search(
        host,
        config.state_dir.join("reads"),
        jj_store_http::Config {
            local_search: Some(manager.clone()),
            access: jj_store_http::Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            ..Default::default()
        },
        indexes,
    )
    .unwrap();
    manager.bind(http.search_registry());
    (prepared, manager, http)
}
async fn request(router: &Router, path: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1");
    let body = if let Some(body) = body {
        builder = builder
            .method("POST")
            .header("content-type", "application/json");
        Body::from(body.to_string())
    } else {
        Body::empty()
    };
    let response = router
        .clone()
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap();
    let status = response.status();
    (
        status,
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap(),
    )
}
async fn settle(manager: &Manager) -> jj_store_service::search_management::LocalSearchSettings {
    for _ in 0..500 {
        let status = manager.status(REPO).await.unwrap();
        if matches!(
            status.operation,
            SearchOperation::Idle | SearchOperation::Failed
        ) {
            return status;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("search operation did not settle");
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn local_search_save_apply_rebuild_disable_and_restart_preserve_authority_and_pins() {
    tokio::task::LocalSet::new().run_until(async {
        let f=Fixture::new().await;
        let (first,_)=publish_text_scope(&f,roots::ROOT_COMMIT,1,&[("code.rs",b"needle")]).await;
        publish_text_scope(&f,first,2,&[("code.rs",b"needle")]).await;
        let root=tempfile::tempdir().unwrap();let path=config_file(root.path());
        let original=serde_json::to_value(config::Config::load(&path).unwrap()).unwrap();
        let (prepared,manager,http)=compose(&f,&path);let router=http.router();let running=prepared.start().await;
        let (code,pinned)=request(&router,"search?q=metafixture&page_size=1&minimum_position=2&wait_millis=5000",None).await;
        assert_eq!(code,StatusCode::OK,"{pinned}");let cursor=pinned["next_cursor"].as_str().unwrap();
        let before=f.session.snapshot().await.unwrap().manifest().clone();
        let status=manager.status(REPO).await.unwrap();let mut desired=status.desired.clone();desired.exclude_prefixes.insert("code.rs".into());
        let save=SaveSearchSettings{version:1,expected_revision:status.revision.clone(),configuration:desired.clone()};
        let saved=manager.save(REPO,save.clone()).await.unwrap();assert_ne!(saved.desired,saved.effective);
        assert_eq!(manager.save(REPO,save).await.unwrap_err(),jj_store_service::search_management::Error::Conflict);
        let updated=serde_json::to_value(config::Config::load(&path).unwrap()).unwrap();
        for key in ["storage","access","limits","protection"] {assert_eq!(original[key],updated[key]);}
        let accepted=manager.control(REPO,SearchCommand{version:1,expected_revision:saved.revision.clone(),action:SearchAction::Apply}).await.unwrap();assert_eq!(accepted.operation,SearchOperation::Applying);
        let active=settle(&manager).await;assert_eq!(active.operation,SearchOperation::Idle,"{:?}",active.error);assert_eq!(active.desired,active.effective);
        let (_,page)=request(&router,"search?q=needle&scope=main",None).await;assert_eq!(page["results"],json!([]));
        let (code,page)=request(&router,&format!("search?q=metafixture&page_size=1&minimum_position=2&wait_millis=5000&cursor={cursor}"),None).await;assert_eq!(code,StatusCode::OK,"{page}");assert_eq!(page["index_generation"],pinned["index_generation"]);
        let (_,replacement_page)=request(&router,"search?q=metafixture&page_size=1&minimum_position=2&wait_millis=5000",None).await;
        let replacement_cursor=replacement_page["next_cursor"].as_str().unwrap();
        // Rebuild cannot restore the old cached projection, and retry reuses normal runner recovery.
        for action in [SearchAction::Rebuild,SearchAction::Retry] {manager.control(REPO,SearchCommand{version:1,expected_revision:active.revision.clone(),action}).await.unwrap();assert_eq!(settle(&manager).await.operation,SearchOperation::Idle);}
        // A replacement too small to hold the index fails without changing effective configuration.
        let mut too_small=desired.clone();too_small.index_bytes=1;
        let saved=manager.save(REPO,SaveSearchSettings{version:1,expected_revision:active.revision,configuration:too_small}).await.unwrap();
        manager.control(REPO,SearchCommand{version:1,expected_revision:saved.revision.clone(),action:SearchAction::Apply}).await.unwrap();
        let failed=settle(&manager).await;assert_eq!(failed.operation,SearchOperation::Failed);assert_eq!(failed.effective,desired);
        desired.enabled=false;
        let saved=manager.save(REPO,SaveSearchSettings{version:1,expected_revision:failed.revision,configuration:desired}).await.unwrap();
        manager.control(REPO,SearchCommand{version:1,expected_revision:saved.revision,action:SearchAction::Apply}).await.unwrap();assert!(!settle(&manager).await.effective.enabled);
        assert_eq!(request(&router,"capabilities",None).await.1["search"],false);
        let (code,page)=request(&router,&format!("search?q=metafixture&page_size=1&minimum_position=2&wait_millis=5000&cursor={cursor}"),None).await;assert_eq!(code,StatusCode::OK,"{page}");assert_eq!(page["index_generation"],pinned["index_generation"]);
        let (code,page)=request(&router,&format!("search?q=metafixture&page_size=1&minimum_position=2&wait_millis=5000&cursor={replacement_cursor}"),None).await;
        assert_eq!(code,StatusCode::OK,"{page}");assert_eq!(page["index_generation"],replacement_page["index_generation"]);
        assert_eq!(before,*f.session.snapshot().await.unwrap().manifest());
        running.shutdown().await.unwrap();drop(router);drop(http);drop(manager);
        assert_eq!(std::fs::read_dir(root.path().join("state/search-replacements")).unwrap().count(),0);
        let (prepared,manager,http)=compose(&f,&path);let running=prepared.start().await;assert!(!manager.status(REPO).await.unwrap().effective.enabled);
        assert_eq!(request(&http.router(),"capabilities",None).await.1["search"],false);
        let status=manager.status(REPO).await.unwrap();let mut desired=status.desired;desired.enabled=true;
        manager.save(REPO,SaveSearchSettings{version:1,expected_revision:status.revision,configuration:desired}).await.unwrap();
        running.shutdown().await.unwrap();drop(http);drop(manager);
        let (prepared,manager,http)=compose(&f,&path);let running=prepared.start().await;
        assert!(manager.status(REPO).await.unwrap().effective.enabled);
        let (code,page)=request(&http.router(),"search?q=needle&scope=main&minimum_position=2&wait_millis=5000",None).await;
        assert_eq!(code,StatusCode::OK,"{page}");assert_eq!(page["results"],json!([]));running.shutdown().await.unwrap();
    }).await;
}
