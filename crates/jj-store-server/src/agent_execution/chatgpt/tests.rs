//! Generated synthetic keys exercise OAuth validation; no user credentials.
use super::*;
#[test]
fn dynamic_registration_returns_issued_identity_and_reauthorization_is_bound() {
    let url = oauth::authorize(
        None,
        "urn:uuid:00000000-0000-4000-8000-000000000001",
        "http://127.0.0.1:55123/auth/callback",
        "state",
        "nonce",
        "verifier",
    )
    .unwrap();
    let q = url.query_pairs().collect::<BTreeMap<_, _>>();
    assert_eq!(q["client_id"], "dynamic_agent_client");
    assert_eq!(q["agent_name_hint"], "JJ Agent");
    assert_eq!(q["resource"], oauth::RESOURCE);
    assert!(q["scope"].contains(oauth::PLAN_SCOPE));
    assert_eq!(q["code_challenge_method"], "S256");
    assert!(oauth::client_id(None, Some("dynamic_agent_client")).is_err());
    assert!(oauth::client_id(Some("issued-one"), Some("issued-two")).is_err());
    assert_eq!(
        oauth::client_id(Some("issued-one"), None).unwrap(),
        "issued-one"
    );
}
fn token(claims: Value) -> String {
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some("fixture".into());
    let key = {
        use base64::Engine as _;
        let pem = include_str!("fixtures/synthetic-rsa.fixture")
            .lines()
            .filter(|s| !s.starts_with("---"))
            .collect::<String>();
        jsonwebtoken::EncodingKey::from_rsa_der(
            &base64::engine::general_purpose::STANDARD
                .decode(pem)
                .unwrap(),
        )
    };
    jsonwebtoken::encode(&header, &claims, &key).unwrap()
}
#[test]
fn signed_identity_requires_issuer_client_expiry_nonce_and_valid_signature() {
    let keys: jsonwebtoken::jwk::JwkSet =
        serde_json::from_str(include_str!("fixtures/synthetic-jwks.json")).unwrap();
    let claims = json!({"iss":oauth::ISSUER,"aud":"issued-one","exp":now()+300,"sub":"subject-one","nonce":"nonce-one","email":"fixture@example.invalid"});
    assert_eq!(
        oauth::identity(&token(claims.clone()), &keys, "issued-one", "nonce-one")
            .unwrap()
            .sub,
        "subject-one"
    );
    assert!(oauth::identity(&token(claims.clone()), &keys, "issued-two", "nonce-one").is_err());
    assert!(oauth::identity(&token(claims.clone()), &keys, "issued-one", "nonce-two").is_err());
    for (key, value) in [
        ("iss", json!("https://invalid.example")),
        ("exp", json!(now() - 1)),
        ("azp", json!("issued-two")),
    ] {
        let mut changed = claims.clone();
        changed[key] = value;
        assert!(oauth::identity(&token(changed), &keys, "issued-one", "nonce-one").is_err());
    }
    let mut forged = token(claims).into_bytes();
    let last = forged.len() - 8;
    forged[last] = if forged[last] == b'A' { b'B' } else { b'A' };
    assert!(
        oauth::identity(
            std::str::from_utf8(&forged).unwrap(),
            &keys,
            "issued-one",
            "nonce-one"
        )
        .is_err()
    );
}
#[test]
fn plan_scope_is_distinct_from_identity_and_token_bundles_are_bounded() {
    let mut tokens:oauth::Tokens=serde_json::from_value(json!({"access_token":"fixture-access","refresh_token":"fixture-refresh","id_token":"fixture-id-token","expires_in":3600,"scope":"openid email"})).unwrap();
    assert!(tokens.valid());
    assert!(!tokens.plan());
    tokens.scope = oauth::PLAN_SCOPE.into();
    assert!(tokens.plan());
    tokens.access_token = "x".repeat(16385);
    assert!(!tokens.valid());
}
mod harness;

#[tokio::test]
async fn sign_out_cancels_reauthorization_of_disconnected_accounts() {
    let mut f = harness::Fixture::new().await;
    let account = "disconnected-account";
    f.supervisor
        .update(|s| {
            s.chatgpt_accounts.insert(
                account.into(),
                Record {
                    view: contract::Account {
                        id: account.into(),
                        label: "Synthetic disconnected account".into(),
                        email: None,
                        connected: false,
                        plan_enabled: true,
                    },
                    client_id: "issued-one".into(),
                    subject: "subject-one".into(),
                    generation: 0,
                },
            );
            Ok(())
        })
        .unwrap();
    let request = RequestId::new([15; 16]);
    f.supervisor
        .chatgpt_connect(request, Some(account.into()))
        .await
        .unwrap();
    let _authorization = f.authorization().await;
    f.supervisor.chatgpt_sign_out(account).await.unwrap();
    assert_eq!(
        journal::read(&f.supervisor.0.resources.root)
            .unwrap()
            .chatgpt_requests[&request]
            .1
            .state,
        "cancelled"
    );
    let record = f.supervisor.chatgpt_record(account).unwrap();
    assert!(!record.view.connected);
    assert_eq!(record.generation, 1);
    assert!(f.io.vault.lock().unwrap().is_empty());
}
use harness::Fixture;
pub(super) use harness::FixtureIo;
use std::sync::atomic::Ordering;
#[tokio::test]
async fn cold_authorization_receipts_serialized_rotation_and_account_bound_broker() {
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
    let mut f = Fixture::new().await;
    let request = RequestId::new([9; 16]);
    let id = request.to_string();
    let (a, b) = tokio::join!(
        f.supervisor.chatgpt_connect(request, None),
        f.supervisor.chatgpt_connect(request, None)
    );
    a.unwrap();
    b.unwrap();
    assert!(matches!(
        f.supervisor
            .chatgpt_connect(request, Some("other".into()))
            .await,
        Err(Error::Conflict)
    ));
    let url = f.authorization().await;
    assert_eq!(
        f.callback(&url, Some("wrong-state")).await,
        reqwest::StatusCode::BAD_REQUEST
    );
    assert_eq!(f.callback(&url, None).await, reqwest::StatusCode::OK);
    assert_eq!(f.settled(&id).await.state, "connected");
    assert!(f.opened.try_recv().is_err());
    let persisted = journal::read(&f.supervisor.0.resources.root).unwrap();
    assert_eq!(persisted.chatgpt_requests[&request].1.state, "connected");
    let original = f.supervisor.chatgpt_token(&id, None).await.unwrap();
    let calls = (0..12).map(|_| f.supervisor.chatgpt_token(&id, Some(original.as_str())));
    for result in futures::future::join_all(calls).await {
        assert_eq!(result.unwrap(), "fixture-access-renewed-1");
    }
    assert_eq!(f.io.refreshes.load(Ordering::SeqCst), 1);
    assert_eq!(
        f.io.vault.lock().unwrap()[&id].tokens.refresh_token,
        "fixture-refresh-renewed-1"
    );
    f.supervisor.0.chatgpt.slots.lock().unwrap().clear();
    assert_eq!(
        f.supervisor.chatgpt_token(&id, None).await.unwrap(),
        "fixture-access-renewed-1"
    );
    assert_eq!(
        f.supervisor.chatgpt_models(&id).await.unwrap().facts["models"]["availableModels"],
        json!([{"modelId":"account-model","name":"Account model"}])
    );
    let setup:Setup=serde_json::from_value(json!({"id":"worker-one","name":"Fixture","avatar":"","instructions":"","runner":"local","preferences":{"model":null,"effort":null,"mode":null},"options":{"provider":"chatgpt-plan","account":id}})).unwrap();
    let mut command = tokio::process::Command::new("unused");
    let mut lease = f
        .supervisor
        .chatgpt_launch(&mut command, &setup, "worker-one", 1)
        .await
        .unwrap();
    let env = command
        .as_std()
        .get_envs()
        .map(|(k, v)| {
            (
                k.to_string_lossy().to_string(),
                v.unwrap().to_string_lossy().to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let capability = &env["JJ_STORE_CHATGPT_BROKER_TOKEN"];
    let socket = &env["JJ_STORE_CHATGPT_SOCKET"];
    let mut stream = tokio::net::UnixStream::connect(socket).await.unwrap();
    stream
        .write_all(format!("{}\n", json!({"capability":capability,"rejected":null})).as_bytes())
        .await
        .unwrap();
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&bytes).unwrap()["access_token"],
        "fixture-access-renewed-1"
    );
    f.io.mode.store(1, Ordering::SeqCst);
    assert!(matches!(
        f.supervisor
            .chatgpt_token(&id, Some("fixture-access-renewed-1"))
            .await,
        Err(Error::Unavailable)
    ));
    assert!(f.supervisor.chatgpt_record(&id).unwrap().view.connected);
    let (sender, _commands) = mpsc::channel(1);
    let (stop, stopping) = tokio::sync::watch::channel(false);
    f.supervisor.0.fleet.lock().unwrap().insert(
        "worker-one".into(),
        fleet::Worker {
            configuration: "older-saved-options".into(),
            inference_host: None,
            sender,
            state: "idle".into(),
            generation: 1,
            error: None,
            last_used: tokio::time::Instant::now(),
            stop,
        },
    );
    // No current setup references the account; only the live lease does.
    assert!(f.supervisor.read(|s| Ok(s.setups.is_empty())).unwrap());
    f.io.mode.store(3, Ordering::SeqCst);
    let this = f.supervisor.clone();
    let account = id.clone();
    let renewal = tokio::spawn(async move {
        this.chatgpt_token(&account, Some("fixture-access-renewed-1"))
            .await
    });
    f.io.started.notified().await;
    let this = f.supervisor.clone();
    let account = id.clone();
    let sign_out = tokio::spawn(async move { this.chatgpt_sign_out(&account).await });
    tokio::time::timeout(Duration::from_secs(2), broker::cancelled(&mut lease))
        .await
        .unwrap();
    f.io.release.notify_one();
    assert!(matches!(renewal.await.unwrap(), Err(Error::Stale)));
    sign_out.await.unwrap().unwrap();
    assert!(*stopping.borrow());
    assert_eq!(
        f.supervisor.0.fleet.lock().unwrap()["worker-one"].state,
        "stopping"
    );
    assert!(f.supervisor.0.chatgpt.leases.lock().unwrap().is_empty());
    assert!(f.io.vault.lock().unwrap().is_empty());
    assert!(!f.supervisor.chatgpt_record(&id).unwrap().view.connected);
    let journal_text =
        fs::read_to_string(f.supervisor.0.resources.root.join("state.json")).unwrap();
    for secret in [
        "fixture-access",
        "fixture-refresh",
        "id_token",
        "code_verifier",
        capability,
    ] {
        assert!(!journal_text.contains(secret));
    }
    let safe = runtime::safe(
        f.supervisor.0.sanitizer.lock().unwrap().as_ref().unwrap(),
        json!({"text":format!("{original} fixture-access-renewed-1 fixture-refresh-renewed-1")}),
    )
    .unwrap();
    assert!(!safe.to_string().contains("fixture-access"));
    let cancel = RequestId::new([10; 16]);
    f.supervisor.chatgpt_connect(cancel, None).await.unwrap();
    let _ = f.authorization().await;
    f.supervisor.chatgpt_cancel(&cancel.to_string()).unwrap();
    assert_eq!(
        journal::read(&f.supervisor.0.resources.root)
            .unwrap()
            .chatgpt_requests[&cancel]
            .1
            .state,
        "cancelled"
    );
    f.io.mode.store(4, Ordering::SeqCst);
    let mismatch = RequestId::new([11; 16]);
    f.supervisor
        .chatgpt_connect(mismatch, Some(id.clone()))
        .await
        .unwrap();
    let returning = f.authorization().await;
    let fields = returning.query_pairs().collect::<BTreeMap<_, _>>();
    assert_eq!(fields["client_id"], "issued-one");
    assert!(!fields.contains_key("agent_name_hint"));
    assert_eq!(
        fields["ext_agent_host_id"],
        url.query_pairs()
            .find(|(k, _)| k == "ext_agent_host_id")
            .unwrap()
            .1
    );
    f.callback(&returning, None).await;
    assert_eq!(
        f.settled(&mismatch.to_string()).await.error.as_deref(),
        Some("account_changed")
    );
    assert!(f.io.vault.lock().unwrap().is_empty());
    f.io.mode.store(0, Ordering::SeqCst);
    let reconnect = RequestId::new([12; 16]);
    f.supervisor
        .chatgpt_connect(reconnect, Some(id.clone()))
        .await
        .unwrap();
    let returning = f.authorization().await;
    f.callback(&returning, None).await;
    assert_eq!(f.settled(&reconnect.to_string()).await.state, "connected");
    f.io.mode.store(2, Ordering::SeqCst);
    assert!(matches!(
        f.supervisor
            .chatgpt_token(&id, Some("fixture-access-original"))
            .await,
        Err(Error::Denied)
    ));
    assert!(!f.supervisor.chatgpt_record(&id).unwrap().view.connected);
    assert!(f.io.vault.lock().unwrap().is_empty());
    f.io.mode.store(5, Ordering::SeqCst);
    let identity_only = RequestId::new([13; 16]);
    f.supervisor
        .chatgpt_connect(identity_only, None)
        .await
        .unwrap();
    let returning = f.authorization().await;
    f.callback(&returning, None).await;
    assert_eq!(
        f.settled(&identity_only.to_string()).await.state,
        "connected"
    );
    assert!(
        !f.supervisor
            .chatgpt_record(&identity_only.to_string())
            .unwrap()
            .view
            .plan_enabled
    );
    assert!(matches!(
        f.supervisor
            .chatgpt_token(&identity_only.to_string(), None)
            .await,
        Err(Error::Denied)
    ));
    f.supervisor.0.stopping.store(true, Ordering::Release);
    assert!(matches!(
        f.supervisor
            .chatgpt_connect(RequestId::new([14; 16]), None)
            .await,
        Err(Error::Unavailable)
    ));
    assert!(f.opened.try_recv().is_err());
}
