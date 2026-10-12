use super::*;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    administration::OperatorPolicies,
    auth::{self, Scope as TokenScope, SystemClock, TokenPolicy},
    directory::{Caller as DirectoryCaller, Directory},
    remote_agents::RemoteAgents,
};
struct Operators(TokenPolicy);
#[async_trait::async_trait]
impl OperatorPolicies for Operators {
    async fn current(&self) -> auth::Result<TokenPolicy> {
        Ok(self.0.clone())
    }
}
#[tokio::test]
async fn remote_admission_filters_definitions_and_denies_ungranted_home_and_revoked_identity() {
    let root = tempfile::tempdir().unwrap();
    let installed = crate::installation::initialize_registry(&root.path().join("state")).unwrap();
    let identity = crate::installation::Identity::load(&installed.installation_file).unwrap();
    let store = Arc::new(MemoryStore::default());
    let operator = PrincipalId::new([9; 16]);
    let principal = PrincipalId::new([1; 16]);
    let token = BearerToken::generate().unwrap();
    let outsider = BearerToken::generate().unwrap();
    let directory = Arc::new(
        crate::directory::Manager::new(
            store.clone(),
            identity.installation.clone(),
            Some(operator),
        )
        .unwrap(),
    );
    directory
        .enroll(
            &DirectoryCaller::Operator(operator),
            jj_store_model::RequestId::new([1; 16]),
            principal,
            &token,
            None,
        )
        .await
        .unwrap();
    directory
        .enroll(
            &DirectoryCaller::Operator(operator),
            jj_store_model::RequestId::new([2; 16]),
            PrincipalId::new([2; 16]),
            &outsider,
            None,
        )
        .await
        .unwrap();
    let host = RepositoryHost::new(
        store,
        Arc::new(Operators(TokenPolicy::new(TokenScope::Operator))),
        Arc::new(SystemClock),
        Default::default(),
        Default::default(),
        16 * 1024 * 1024,
    )
    .unwrap();
    let registry = Arc::new(crate::registry::Registry::new(installed.catalog, identity).unwrap());
    let setup = Setup {
        id: "build-agent".into(),
        name: "Builder".into(),
        avatar: "🤖".into(),
        description: "Builds code".into(),
        instructions: "private instructions".into(),
        runner: "codex".into(),
        preferences: Preferences {
            model: None,
            effort: None,
            mode: Some("ask".into()),
        },
        options: settings::RuntimeOptions {
            environment: BTreeMap::from([(
                "SECRET_REFERENCE".into(),
                settings::EnvironmentValue::Environment("PRIVATE_ENV".into()),
            )]),
            ..Default::default()
        },
    };
    let config = Configuration {
        acp_executable: PathBuf::from("/usr/bin/true"),
        jj_executable: PathBuf::from("/usr/bin/true"),
        agents: vec![Agent {
            setup,
            principals: BTreeSet::from([principal]),
        }],
    };
    let broker = Broker::open(
        root.path().join("remote"),
        config,
        host,
        directory.clone(),
        registry,
    )
    .unwrap();
    assert!(matches!(broker.admit(outsider).await, Err(Error::Denied)));
    let (actor, caller) = broker.admit(token.clone()).await.unwrap();
    assert_eq!(actor, principal);
    assert!(matches!(caller.status(operator).await, Err(Error::Denied)));
    let status = caller.status(actor).await.unwrap();
    assert_eq!(status.setups.len(), 1);
    assert!(status.setups[0].instructions.is_empty());
    assert!(status.setups[0].options.environment.is_empty());
    let command = conversations::Send {
        version: 1,
        request_id: jj_store_model::RequestId::new([3; 16]),
        conversation_id: "a".repeat(32),
        origin: conversations::Origin {
            scope: "project:test".into(),
            home: Some(RepositoryId::new([1; 16])),
            thread: None,
            agent_target: None,
            context: String::new(),
        },
        thread_codebase: None,
        agent_ids: vec!["build-agent".into()],
        attachments: vec![],
        text: "hello".into(),
        comments: None,
        max_duration_ms: 1000,
    };
    assert!(matches!(
        caller.send_conversation(actor, command).await,
        Err(Error::Denied)
    ));
    assert!(broker.0.scopes.lock().unwrap().is_empty());
    directory
        .revoke(
            &DirectoryCaller::Operator(operator),
            jj_store_model::RequestId::new([4; 16]),
            token.id(),
        )
        .await
        .unwrap();
    assert!(matches!(caller.status(actor).await, Err(Error::Denied)));
    broker.shutdown().await.unwrap();
    let bytes = std::fs::read(root.path().join("remote/scopes.json")).unwrap_or_default();
    assert!(!String::from_utf8_lossy(&bytes).contains(token.expose().as_str()));
}
