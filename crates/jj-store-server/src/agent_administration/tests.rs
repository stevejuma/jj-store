use super::*;
use jj_store_model::{PrincipalId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    agent_administration::Administration, auth::Permission, directory::Directory,
};

#[tokio::test]
async fn conditional_agent_saves_preserve_other_sections_and_require_independent_permission() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("server.toml");
    std::fs::write(&path, crate::config::EXAMPLE).unwrap();
    let config = Config::load(&path).unwrap();
    let principal = PrincipalId::new([1; 16]);
    let allowed = BearerToken::generate().unwrap();
    let denied = BearerToken::generate().unwrap();
    let policy = policy::PolicyFile::new(config.operator_policy.clone()).unwrap();
    policy
        .initialize_permission(principal, &allowed, Permission::ManageAgents)
        .unwrap();
    policy
        .grant_permission(principal, &denied, Permission::ManageIdentities)
        .unwrap();
    let directory = Arc::new(
        crate::directory::Manager::new(Arc::new(MemoryStore::default()), "11".repeat(16), None)
            .unwrap(),
    );
    directory
        .enroll(
            &Caller::Operator(principal),
            RequestId::new([1; 16]),
            principal,
            &allowed,
            None,
        )
        .await
        .unwrap();
    let manager = Manager::new(&path, &config, directory).unwrap();
    assert!(matches!(
        manager.status(denied.clone()).await,
        Err(Error::Denied)
    ));
    let before = manager.status(allowed.clone()).await.unwrap();
    assert_eq!(before.principals, vec![principal]);
    let value = a::Configuration {
        acp_executable: "/usr/bin/true".into(),
        jj_executable: "/usr/bin/true".into(),
        agents: vec![a::Definition {
            principals: [principal].into_iter().collect(),
            setup: jj_store_service::agent_execution::Setup {
                id: "builder".into(),
                name: "Builder".into(),
                avatar: "🤖".into(),
                description: "Builds".into(),
                instructions: "Build carefully".into(),
                runner: "codex".into(),
                preferences: jj_store_service::agent_execution::Preferences {
                    model: None,
                    effort: None,
                    mode: Some("ask".into()),
                },
                options: Default::default(),
            },
        }],
    };
    let request = a::Save {
        expected_revision: before.revision.clone(),
        configuration: Some(value.clone()),
    };
    assert!(matches!(
        manager.save(denied, request.clone()).await,
        Err(Error::Denied)
    ));
    let saved = manager
        .save(allowed.clone(), request.clone())
        .await
        .unwrap();
    assert!(saved.restart_required);
    assert_eq!(saved.desired, Some(value));
    assert!(saved.effective.is_none());
    assert_eq!(Config::load(&path).unwrap().public_url, config.public_url);
    assert_eq!(
        Config::load(&path).unwrap().operator_policy,
        config.operator_policy
    );
    assert!(matches!(
        manager.save(allowed.clone(), request).await,
        Err(Error::Stale)
    ));
    policy.revoke(allowed.id()).unwrap();
    assert!(matches!(manager.status(allowed).await, Err(Error::Denied)));
}
