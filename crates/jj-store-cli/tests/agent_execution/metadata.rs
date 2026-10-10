//! Real local companion publication and restart; controlled inference is fixture evidence.
use super::*;
use jj_store_service::agent_execution::{conversations as chat, metadata as api};
use serde_json::json;
#[path = "metadata_recovery.rs"]
mod recovery;
const HELPER: &str = r#"#!/usr/bin/python3
import json, os, sys, time
from pathlib import Path
assert sys.argv[1:] == ['--metadata']
assert os.environ['BUZZ_AGENT_MODEL'] == 'metadata-model'
input = json.load(sys.stdin)
assert input['version'] == 1 and set(input) == {'version','transcript'}
with open('inputs.jsonl','a') as f: f.write(json.dumps(input)+'\n')
if os.environ.get('METADATA_WAIT'): time.sleep(60)
if os.environ.get('METADATA_GATE'):
    Path('generation-started').touch()
    while not Path('generation-release').exists(): time.sleep(.02)
if os.environ.get('METADATA_BAD'): print('{"foreign":"invalid"}')
else: print(json.dumps(dict(title='Review fixture conversation',summary='The user requested a fixture conversation. The assistant responded.',keywords=['fixture','discovery-keyword'],goals=['Review captured behavior'])))
"#;
async fn start() -> (Fixture, Supervisor) {
    start_with_title(false).await
}
async fn start_with_title(title: bool) -> (Fixture, Supervisor) {
    let f = Fixture::new().await;
    if title {
        let script = fs::read_to_string(&f.resources.executable).unwrap().replace(
            "        emit('started', request)",
            "        emit('started', request)\n        emit('activity', request, update=dict(sessionUpdate='session_info_update', title='ACP suggested title'))",
        );
        fs::write(&f.resources.executable, script).unwrap();
    }
    let helper = f
        .resources
        .executable
        .with_file_name("jj-store-local-agent");
    fs::write(&helper, HELPER).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        fs::set_permissions(helper, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let s = Supervisor::open(f.resources.clone()).unwrap();
    s.start_publication();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !s.ready() {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    let revision = s.status(f.actor).await.unwrap().revision;
    s.save_setup(
        f.actor,
        SetupCommand {
            version: 1,
            request_id: RequestId::new([90; 16]),
            expected_revision: revision,
            setup: Some(Setup {
                options: Default::default(),
                id: "helper".into(),
                name: "Agent identity".into(),
                avatar: "".into(),
                description: "".into(),
                instructions: "".into(),
                runner: "codex".into(),
                preferences: Preferences {
                    model: None,
                    effort: None,
                    mode: Some("ask".into()),
                },
            }),
            remove: None,
        },
    )
    .await
    .unwrap();
    (f, s)
}
async fn configure(s: &Supervisor, actor: PrincipalId, n: u8, option: Option<&str>) {
    let mut value = s.settings(actor).await.unwrap();
    let revision = value.revision;
    value.metadata=Some(serde_json::from_value(json!({"model":"metadata-model","options":{"provider":"openai-compat","endpoint":"https://metadata.example/v1","environment":option.map(|v|json!({(v):{"source":"literal","value":"yes"}})).unwrap_or(json!({}))}})).unwrap());
    s.save_settings(
        actor,
        settings::SettingsCommand {
            version: 1,
            request_id: RequestId::new([n; 16]),
            expected_revision: revision,
            settings: value,
        },
    )
    .await
    .unwrap();
}
async fn send(s: &Supervisor, actor: PrincipalId, n: u8) -> String {
    let outcome = s
        .send_conversation(
            actor,
            chat::Send {
                thread_codebase: None,
                version: 1,
                request_id: RequestId::new([n; 16]),
                comments: None,
                conversation_id: format!("{n:02x}").repeat(16),
                origin: chat::Origin {
                    scope: "personal".into(),
                    home: None,
                    thread: None,
                    agent_target: None,
                    context: String::new(),
                },
                agent_ids: vec!["helper".into()],
                attachments: vec![],
                text: "Review this fixture 🦀".into(),
                max_duration_ms: 5000,
            },
        )
        .await
        .unwrap();
    assert!(outcome.agents[0].error.is_none(), "{outcome:?}");
    let id = outcome.agents[0].session_id.clone().unwrap();
    wait(s, actor, |v| {
        v.sessions.iter().any(|a| {
            a.id == id
                && a.state == "idle"
                && a.last_sequence > 3
                && a.last_sequence == a.published_sequence
        })
    })
    .await;
    id
}
fn query(id: &str) -> api::Query {
    api::Query {
        session: id.into(),
        repository: None,
    }
}
async fn terminal(s: &Supervisor, actor: PrincipalId, id: RequestId) -> api::Job {
    tokio::time::timeout(Duration::from_secs(40), async {
        loop {
            let j = s.metadata_job(actor, id).await.unwrap();
            if !matches!(j.state.as_str(), "prepared" | "generating" | "publishing") {
                return j;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap()
}
async fn command(
    s: &Supervisor,
    actor: PrincipalId,
    id: &str,
    n: u8,
    action: api::Action,
) -> api::Command {
    let view = s.metadata(actor, query(id)).await.unwrap();
    api::Command {
        version: 1,
        request_id: RequestId::new([n; 16]),
        query: query(id),
        expected: view.revision,
        action,
    }
}
#[tokio::test]
async fn metadata_generates_once_preserves_custom_titles_and_recovers_repository_state() {
    let (f, s) = start_with_title(true).await;
    configure(&s, f.actor, 91, None).await;
    let id = send(&s, f.actor, 92).await;
    let view = tokio::time::timeout(Duration::from_secs(40), async {
        loop {
            let v = s.metadata(f.actor, query(&id)).await.unwrap();
            if v.value.generated.is_some() {
                return v;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    assert!(view.fresh);
    assert_eq!(view.title.as_deref(), Some("Review fixture conversation"));
    assert_eq!(view.value.agent_titles[&id], "ACP suggested title");
    assert!(view.value.agent_title_sources.contains_key(&id));
    let inputs = f.resources.root.join("metadata-runtime/inputs.jsonl");
    assert_eq!(fs::read_to_string(&inputs).unwrap().lines().count(), 1);
    let input: serde_json::Value =
        serde_json::from_str(fs::read_to_string(&inputs).unwrap().lines().next().unwrap()).unwrap();
    assert!(
        input["transcript"]
            .as_str()
            .unwrap()
            .contains("Review this fixture 🦀")
    );
    assert!(
        !input["transcript"]
            .as_str()
            .unwrap()
            .contains("JJ_STORE_AGENT_JJ_EXECUTABLE")
    );
    let rename = command(
        &s,
        f.actor,
        &id,
        93,
        api::Action::Rename {
            title: "Custom 🦀 title".into(),
        },
    )
    .await;
    s.metadata_command(f.actor, rename.clone()).await.unwrap();
    assert_eq!(
        terminal(&s, f.actor, rename.request_id).await.state,
        "completed"
    );
    let refresh = command(&s, f.actor, &id, 94, api::Action::Refresh).await;
    s.metadata_command(f.actor, refresh.clone()).await.unwrap();
    assert_eq!(
        terminal(&s, f.actor, refresh.request_id).await.state,
        "completed"
    );
    let v = s.metadata(f.actor, query(&id)).await.unwrap();
    assert_eq!(v.title.as_deref(), Some("Custom 🦀 title"));
    assert!(matches!(
        s.metadata_command(
            f.actor,
            api::Command {
                request_id: RequestId::new([95; 16]),
                ..rename.clone()
            }
        )
        .await,
        Err(Error::Stale)
    ));
    let mut conflict = refresh.clone();
    conflict.action = api::Action::ResetTitle;
    assert!(matches!(
        s.metadata_command(f.actor, conflict).await,
        Err(Error::Conflict)
    ));
    let alias = v.sources[0].session.to_string();
    assert_eq!(
        s.metadata(
            f.actor,
            api::Query {
                session: alias,
                repository: Some(v.repository)
            }
        )
        .await
        .unwrap()
        .revision,
        v.revision
    );
    for _ in 0..3 {
        s.metadata(f.actor, query(&id)).await.unwrap();
        s.status(f.actor).await.unwrap();
    }
    let count = fs::read_to_string(&inputs).unwrap().lines().count();
    s.shutdown().await.unwrap();
    let reopened = Supervisor::open(f.resources.clone()).unwrap();
    assert_eq!(
        reopened
            .metadata(f.actor, query(&id))
            .await
            .unwrap()
            .title
            .as_deref(),
        Some("Custom 🦀 title")
    );
    assert_eq!(
        reopened
            .metadata_command(f.actor, refresh)
            .await
            .unwrap()
            .state,
        "completed"
    );
    assert_eq!(fs::read_to_string(&inputs).unwrap().lines().count(), count);
    reopened.shutdown().await.unwrap();
}
#[tokio::test]
async fn metadata_does_not_backfill_and_cancellation_retains_titles_without_replaying_inference() {
    let (f, s) = start().await;
    let id = send(&s, f.actor, 96).await;
    configure(&s, f.actor, 97, Some("METADATA_WAIT")).await;
    assert!(
        s.metadata(f.actor, query(&id))
            .await
            .unwrap()
            .value
            .generated
            .is_none()
    );
    let refresh = command(&s, f.actor, &id, 98, api::Action::Refresh).await;
    s.metadata_command(f.actor, refresh.clone()).await.unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while !f
            .resources
            .root
            .join("metadata-runtime/inputs.jsonl")
            .exists()
        {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    let rename = command(
        &s,
        f.actor,
        &id,
        99,
        api::Action::Rename {
            title: "Keep this title".into(),
        },
    )
    .await;
    s.metadata_command(f.actor, rename.clone()).await.unwrap();
    assert_eq!(
        terminal(&s, f.actor, rename.request_id).await.state,
        "completed"
    );
    let cancel = command(
        &s,
        f.actor,
        &id,
        100,
        api::Action::Cancel {
            job: refresh.request_id,
        },
    )
    .await;
    s.metadata_command(f.actor, cancel.clone()).await.unwrap();
    assert_eq!(
        s.metadata_command(f.actor, cancel).await.unwrap().state,
        "completed"
    );
    assert_eq!(
        s.metadata_job(f.actor, refresh.request_id)
            .await
            .unwrap()
            .state,
        "cancelled"
    );
    configure(&s, f.actor, 101, None).await;
    let regenerate = command(&s, f.actor, &id, 102, api::Action::RegenerateTitle).await;
    s.metadata_command(f.actor, regenerate.clone())
        .await
        .unwrap();
    assert_eq!(
        terminal(&s, f.actor, regenerate.request_id).await.state,
        "completed"
    );
    assert_eq!(
        s.metadata(f.actor, query(&id))
            .await
            .unwrap()
            .title
            .as_deref(),
        Some("Review fixture conversation")
    );
    configure(&s, f.actor, 103, Some("METADATA_BAD")).await;
    let malformed = command(&s, f.actor, &id, 104, api::Action::Refresh).await;
    s.metadata_command(f.actor, malformed.clone())
        .await
        .unwrap();
    assert_eq!(
        terminal(&s, f.actor, malformed.request_id).await.state,
        "failed"
    );
    assert!(
        s.metadata(f.actor, query(&id))
            .await
            .unwrap()
            .value
            .generated
            .is_some()
    );
    configure(&s, f.actor, 105, Some("METADATA_WAIT")).await;
    let interrupted = command(&s, f.actor, &id, 106, api::Action::Refresh).await;
    s.metadata_command(f.actor, interrupted.clone())
        .await
        .unwrap();
    s.shutdown().await.unwrap();
    let count = fs::read_to_string(f.resources.root.join("metadata-runtime/inputs.jsonl"))
        .unwrap()
        .lines()
        .count();
    let reopened = Supervisor::open(f.resources.clone()).unwrap();
    assert_eq!(
        reopened
            .metadata_command(f.actor, interrupted)
            .await
            .unwrap()
            .state,
        "outcome_unknown"
    );
    assert_eq!(
        fs::read_to_string(f.resources.root.join("metadata-runtime/inputs.jsonl"))
            .unwrap()
            .lines()
            .count(),
        count
    );
    let abandon = command(
        &reopened,
        f.actor,
        &id,
        118,
        api::Action::Cancel {
            job: RequestId::new([106; 16]),
        },
    )
    .await;
    reopened
        .metadata_command(f.actor, abandon.clone())
        .await
        .unwrap();
    assert_eq!(
        terminal(&reopened, f.actor, abandon.request_id).await.state,
        "completed"
    );
    assert_eq!(
        reopened
            .metadata_job(f.actor, RequestId::new([106; 16]))
            .await
            .unwrap()
            .state,
        "cancelled"
    );
    assert_eq!(
        fs::read_to_string(f.resources.root.join("metadata-runtime/inputs.jsonl"))
            .unwrap()
            .lines()
            .count(),
        count
    );
    reopened.shutdown().await.unwrap();
}
