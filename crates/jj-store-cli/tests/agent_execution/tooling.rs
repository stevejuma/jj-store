use super::*;
use jj_store_service::agent_execution::tooling as dto;
use sha2::{Digest as _, Sha256};
#[tokio::test]
async fn runtime_receipts_recover_without_execution_or_automatic_install_replay() {
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let resources = fixture.resources;
    let supervisor = Supervisor::open(resources.clone()).unwrap();
    let observed = supervisor.status(actor).await.unwrap();
    let inventory = supervisor.runtime_status(actor).await.unwrap();
    assert_eq!(
        inventory
            .runtimes
            .iter()
            .map(|r| r.id.as_str())
            .collect::<Vec<_>>(),
        vec!["codex", "cursor", "local"]
    );
    let local = inventory.runtimes.iter().find(|r| r.id == "local").unwrap();
    assert_eq!(local.label, "jj-agent");
    assert_eq!(local.state, "bundle_needed");
    assert!(local.action.is_none());
    let bin = resources.executable.parent().unwrap();
    for name in ["jj-store-local-agent", "jj-store-agent-tools"] {
        fs::write(bin.join(name), "fixture companion").unwrap();
    }
    let bundled = supervisor.runtime_status(actor).await.unwrap();
    assert_eq!(
        bundled
            .runtimes
            .iter()
            .find(|r| r.id == "local")
            .unwrap()
            .state,
        "ready"
    );
    assert_eq!(
        supervisor
            .status(actor)
            .await
            .unwrap()
            .runners
            .iter()
            .find(|r| r.id == "local")
            .unwrap()
            .label,
        "jj-agent"
    );
    fs::remove_file(bin.join("jj-store-agent-tools")).unwrap();
    assert_eq!(
        supervisor
            .runtime_status(actor)
            .await
            .unwrap()
            .runtimes
            .iter()
            .find(|r| r.id == "local")
            .unwrap()
            .state,
        "bundle_needed"
    );
    assert!(inventory.pending.is_none());
    assert!(matches!(
        supervisor.runtime_status(PrincipalId::new([99; 16])).await,
        Err(Error::Denied)
    ));
    let command = dto::Command {
        version: 1,
        request_id: RequestId::new([88; 16]),
        runtime_id: "not-catalogued".into(),
        action: dto::Action::Install,
    };
    assert!(matches!(
        supervisor.install_runtime(actor, command.clone()).await,
        Err(Error::Invalid)
    ));
    assert!(matches!(
        supervisor
            .install_runtime(PrincipalId::new([99; 16]), command.clone())
            .await,
        Err(Error::Denied)
    ));
    assert!(!resources.root.join("tooling").exists());
    supervisor.shutdown().await.unwrap();
    drop(supervisor);
    let command = dto::Command {
        runtime_id: "codex".into(),
        ..command
    };
    let digest = hex::encode(Sha256::digest(serde_json::to_vec(&command).unwrap()));
    let path = resources.root.join("state.json");
    let mut state: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    state["runtime_installs"] = serde_json::json!({command.request_id.to_string():{
        "digest":digest,"outcome":{"version":1,"request_id":command.request_id,"runtime_id":"codex","state":"installing","error":null}
    }});
    fs::write(&path, serde_json::to_vec(&state).unwrap()).unwrap();
    let reopened = Supervisor::open(resources.clone()).unwrap();
    let outcome = reopened
        .runtime_outcome(actor, command.request_id)
        .await
        .unwrap();
    assert_eq!(outcome.state, "interrupted");
    assert_eq!(
        reopened
            .install_runtime(actor, command.clone())
            .await
            .unwrap()
            .state,
        "interrupted"
    );
    let different = dto::Command {
        action: dto::Action::Update,
        ..command.clone()
    };
    assert!(matches!(
        reopened.install_runtime(actor, different).await,
        Err(Error::Conflict)
    ));
    assert!(!resources.root.join("tooling").exists());
    let status = reopened.status(actor).await.unwrap();
    assert_eq!(status.revision, observed.revision);
    assert!(status.sessions.is_empty() && status.setups.is_empty());
    reopened.shutdown().await.unwrap();
}
