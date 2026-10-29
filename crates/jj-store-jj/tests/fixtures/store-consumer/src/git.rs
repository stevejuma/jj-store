use super::*;
#[cfg(not(feature = "stock-jj"))]
use jj_store_git::metadata::MetadataStore;
use jj_store_git::{GitProvider, LocalRepository};
#[cfg(feature = "stock-jj")]
use std::{collections::BTreeMap, path::Path};
#[cfg(feature = "stock-jj")]
fn heads(path: &Path) -> BTreeMap<String, Vec<u8>> {
    std::fs::read_dir(path.join("op_heads"))
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().is_file())
        .map(|entry| {
            (
                entry.file_name().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).unwrap(),
            )
        })
        .collect()
}
fn open(
    repository: LocalRepository,
    host: Arc<Host>,
    calls: Arc<AtomicUsize>,
    disable_thread: bool,
) -> RepositoryStores {
    let provider = GitProvider::new(repository);
    let mut factories = StoreFactories::empty();
    provider.register(&mut factories, "git").unwrap();
    register_custom(&mut factories, calls);
    let mut config = StoreConfig::all("git");
    config.enable(StoreKind::Work, "example-work");
    if disable_thread {
        config.disable(StoreKind::Thread);
    }
    factories
        .loader()
        .open(&config, StoreContext::new(provider.scope(), host))
        .unwrap()
}
pub async fn run() {
    let dir = tempfile::tempdir().unwrap();
    #[cfg(feature = "stock-jj")]
    let (repo_path, metadata) = {
        use jj_lib::{config::StackedConfig, settings::UserSettings, workspace::Workspace};
        let settings = UserSettings::from_config(StackedConfig::with_defaults()).unwrap();
        let (workspace, _repo) =
            Workspace::init_internal_git(&settings, dir.path(), gix::hash::Kind::Sha1)
                .await
                .unwrap();
        let path = workspace.repo_path().to_owned();
        let metadata = jj_store_jj::local_repository::metadata_store(&settings, &path).unwrap();
        (path, metadata.carrier().clone())
    };
    #[cfg(not(feature = "stock-jj"))]
    let (repo_path, metadata) = {
        gix::init_bare(dir.path()).unwrap();
        (
            dir.path().join("private-provenance"),
            MetadataStore::open(dir.path(), jj_store_git::metadata::Limits::default()).unwrap(),
        )
    };
    #[cfg(feature = "stock-jj")]
    let original_heads = heads(&repo_path);
    let repository = LocalRepository::new(&repo_path, metadata.clone());
    let host = Arc::new(Host::default());
    let calls = Arc::new(AtomicUsize::new(0));
    let stores = open(repository.clone(), host.clone(), calls.clone(), false);
    assert!(metadata.read().unwrap().is_none());
    assert!(!repo_path.join("lapis").exists());
    let pinned = stores.agent().unwrap().snapshot().await.unwrap();
    assert!(
        stores
            .work()
            .unwrap()
            .snapshot()
            .await
            .unwrap()
            .data
            .projects
            .is_empty()
    );
    assert_eq!(
        stores.review().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(
        stores.thread().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(
        stores.reaction().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(host.writes.load(Ordering::SeqCst), 0);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        stores.require_shared_publication(&[StoreKind::Agent, StoreKind::Work]),
        Err(Error::Unsupported(_))
    ));
    let t = committed(stores.thread().unwrap().execute(&thread(51)).await.unwrap());
    let jj_collab::CollabCommandResultV1::Thread { result: tdata } = &t.result else {
        panic!("thread")
    };
    let capture = request(
        52,
        capture(Some(jj_collab::ThreadBindingV1 {
            thread_id: tdata.thread_id,
            observed_thread: tdata.version,
            observed_assignment: None,
        })),
    );
    let a = committed(stores.execute_agent(&capture).await.unwrap());
    let r = committed(stores.review().unwrap().execute(&review(53)).await.unwrap());
    let reaction = reaction(54, tdata.thread_id);
    let x = committed(stores.reaction().unwrap().execute(&reaction).await.unwrap());
    assert!(
        engine::AgentReader::new(pinned.data.as_ref())
            .list_sessions(None, 16)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    let current = stores.agent().unwrap().snapshot().await.unwrap();
    assert!(
        !engine::AgentReader::new(current.data.as_ref())
            .list_sessions(None, 16)
            .await
            .unwrap()
            .selections
            .is_empty()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let output = std::process::Command::new("git")
        .arg("--git-dir")
        .arg(metadata.git_dir())
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reopened = open(repository, host.clone(), calls.clone(), true);
    assert_eq!(
        committed(reopened.execute_agent(&capture).await.unwrap()).provider_receipt,
        a.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .review()
                .unwrap()
                .status(r.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        r.provider_receipt
    );
    assert_eq!(
        committed(
            reopened
                .reaction()
                .unwrap()
                .status(x.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        x.provider_receipt
    );
    let before = metadata.read().unwrap().unwrap().commit();
    assert!(matches!(
        reopened
            .reaction()
            .unwrap()
            .execute(&super::reaction(55, tdata.thread_id))
            .await,
        Err(Error::Unsupported(_))
    ));
    assert_eq!(metadata.read().unwrap().unwrap().commit(), before);
    #[cfg(feature = "stock-jj")]
    assert_eq!(heads(&repo_path), original_heads);
    host.denied.store(true, Ordering::SeqCst);
    assert!(matches!(
        reopened.agent().unwrap().snapshot().await,
        Err(Error::Denied)
    ));
}
