use super::*;
use serde_json::json;

#[tokio::test]
async fn provider_preview_is_ephemeral_credential_scoped_and_reaped() {
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let resources = fixture.resources.clone();
    let helper = resources.executable.with_file_name("jj-store-local-agent");
    fs::write(&helper, r#"#!/usr/bin/python3
import os, sys, json, time
assert sys.argv[1:] == ['--models']
with open('probes', 'a') as f: f.write('probe\n')
if os.environ.get('FIXTURE_WAIT'): time.sleep(60)
secret = os.environ['ANTHROPIC_API_KEY']
assert secret.startswith('preview-')
print(json.dumps({'status':'ready','facts':{'models':{'availableModels':[{'modelId':os.environ['BUZZ_AGENT_PROVIDER'],'name':secret}]}}}))
"#).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let supervisor = Supervisor::open(fixture.resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let journal = resources.root.join("state.json");
    let before = fs::read(&journal).unwrap();
    let mut preview: catalog::Preview = serde_json::from_value(json!({
        "version":1,"runner":"local","preferences":{"model":null,"effort":null,"mode":null},
        "options":{"provider":"anthropic","environment":{"ANTHROPIC_API_KEY":{"source":"keychain","value":"preview"}}},
        "secrets":{"preview":"preview-secret-one"}
    })).unwrap();
    let first = supervisor
        .catalog_preview(actor, preview.clone())
        .await
        .unwrap();
    assert_eq!(first.status.as_deref(), Some("ready"));
    assert_eq!(
        first.facts["models"]["availableModels"][0]["modelId"],
        "anthropic"
    );
    assert!(!first.facts.to_string().contains("preview-secret"));
    let probes = resources.root.join("catalog/probes");
    assert_eq!(fs::read_to_string(&probes).unwrap().lines().count(), 1);
    supervisor
        .catalog_preview(actor, preview.clone())
        .await
        .unwrap();
    assert_eq!(fs::read_to_string(&probes).unwrap().lines().count(), 1);
    preview
        .secrets
        .insert("preview".into(), "preview-secret-two".into());
    supervisor
        .catalog_preview(actor, preview.clone())
        .await
        .unwrap();
    assert_eq!(fs::read_to_string(&probes).unwrap().lines().count(), 2);
    assert_eq!(fs::read(&journal).unwrap(), before);
    assert_eq!(
        fs::read_dir(resources.root.join("workspaces"))
            .unwrap()
            .count(),
        0
    );
    let status = supervisor.status(actor).await.unwrap();
    assert!(
        status.setups.is_empty() && status.sessions.is_empty() && status.conversations.is_empty()
    );
    preview.options.environment.insert(
        "FIXTURE_WAIT".into(),
        settings::EnvironmentValue::Literal("yes".into()),
    );
    let reader = supervisor.clone();
    let probe = tokio::spawn(async move { reader.catalog_preview(actor, preview).await });
    tokio::time::timeout(Duration::from_secs(5), async {
        while fs::read_to_string(&probes).unwrap().lines().count() < 3 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    supervisor.shutdown().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_secs(1), probe)
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
}

#[tokio::test]
async fn managed_catalog_does_not_start_compute_or_mutate_the_journal() {
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let root = fixture.resources.root.clone();
    fs::write(
        fixture
            .resources
            .executable
            .with_file_name("jj-store-local-agent"),
        "unused helper",
    )
    .unwrap();
    let supervisor = Supervisor::open(fixture.resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let before = fs::read(root.join("state.json")).unwrap();
    let preview = serde_json::from_value(json!({
        "version":1,"runner":"local","preferences":{"model":null,"effort":null,"mode":null},
        "options":{"provider":"local"},"secrets":{}
    }))
    .unwrap();
    let result = supervisor.catalog_preview(actor, preview).await.unwrap();
    assert_eq!(result.status.as_deref(), Some("unavailable"));
    assert!(
        !root.join("compute").exists(),
        "read-only discovery started compute preparation"
    );
    assert_eq!(fs::read(root.join("state.json")).unwrap(), before);
    supervisor.shutdown().await.unwrap();
}
