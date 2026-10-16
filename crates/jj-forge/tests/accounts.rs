use jj_forge::{
    Error, Result,
    accounts::{http::Transport, store::SecretStore, *},
};
use serde_json::{Value, json};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Default)]
struct Secrets {
    values: Mutex<HashMap<String, String>>,
    fail: bool,
}
impl SecretStore for Secrets {
    fn get(&self, key: &str) -> Result<Option<String>> {
        if self.fail {
            return Err(Error::State);
        }
        Ok(self.values.lock().unwrap().get(key).cloned())
    }
    fn set(&self, key: &str, value: &str) -> Result<()> {
        if self.fail {
            return Err(Error::State);
        }
        self.values.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn remove(&self, key: &str) -> Result<()> {
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}
#[derive(Default)]
struct Network {
    replies: Mutex<VecDeque<Value>>,
    calls: Mutex<Vec<String>>,
}
#[async_trait::async_trait(?Send)]
impl Transport for Network {
    async fn request(&self, url: &str, _: Option<&str>, _: Option<Value>) -> Result<Value> {
        self.calls.lock().unwrap().push(url.into());
        self.replies
            .lock()
            .unwrap()
            .pop_front()
            .ok_or(Error::Denied)
    }
}
fn user() -> Value {
    json!({"id":42,"login":"someone","name":"A Person","email":"hello@example.test","avatar_url":"https://avatars.githubusercontent.com/u/42"})
}
fn verification() -> Value {
    json!({"device_code":"private-device-code", "user_code":"ABCD-1234", "verification_uri":"https://github.com/login/device", "expires_in":900,"interval":5})
}
fn fixture(replies: Vec<Value>) -> (tempfile::TempDir, Manager, Arc<Secrets>, Arc<Network>) {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let network = Arc::new(Network {
        replies: Mutex::new(replies.into()),
        ..Default::default()
    });
    let manager = Manager::new(
        root.path().join("accounts"),
        secrets.clone(),
        network.clone(),
        Some("our-client-id".into()),
    );
    (root, manager, secrets, network)
}
fn token(api: &str) -> Command {
    Command::AddToken {
        api_url: api.into(),
        token: "fixture-secret-token".into(),
    }
}

#[tokio::test]
async fn validated_accounts_survive_restart_without_secrets_in_metadata_or_responses() {
    let (root, manager, secrets, network) = fixture(vec![user(), user(), user()]);
    let first = manager
        .execute(token("https://api.github.com"), 10)
        .await
        .unwrap();
    let id = first.accounts[0].id.clone();
    let replaced = manager
        .execute(token("https://api.github.com/"), 10)
        .await
        .unwrap();
    assert_eq!(replaced.accounts.len(), 1);
    assert_eq!(replaced.accounts[0].revision, 2);
    manager
        .execute(token("https://enterprise.example/api/v3"), 10)
        .await
        .unwrap();
    let restarted = Manager::new(manager.root.clone(), secrets, network, None);
    let accounts = restarted.inventory().unwrap();
    assert_eq!(accounts.accounts.len(), 2);
    assert_ne!(accounts.accounts[0].id, accounts.accounts[1].id);
    let file = std::fs::read_to_string(root.path().join("accounts/accounts.json")).unwrap();
    for text in [file, serde_json::to_string(&accounts).unwrap()] {
        assert!(!text.contains("fixture-secret"));
        assert!(!text.contains("access_token"));
    }
    restarted
        .execute(
            Command::Forget {
                account: id.clone(),
            },
            11,
        )
        .await
        .unwrap();
    assert!(restarted.credential(&id, 11).await.is_err());
    assert_eq!(restarted.inventory().unwrap().accounts.len(), 1);
}
#[tokio::test]
async fn invalid_tokens_or_locked_keychain_never_create_connected_accounts() {
    let (_, manager, _, _) = fixture(vec![]);
    assert!(
        manager
            .execute(token("https://api.github.com"), 10)
            .await
            .is_err()
    );
    assert!(manager.inventory().unwrap().accounts.is_empty());
    let root = tempfile::tempdir().unwrap();
    let manager = Manager::new(
        root.path().to_owned(),
        Arc::new(Secrets {
            fail: true,
            ..Default::default()
        }),
        Arc::new(Network {
            replies: Mutex::new(vec![user()].into()),
            ..Default::default()
        }),
        None,
    );
    assert!(
        manager
            .execute(token("https://api.github.com"), 10)
            .await
            .is_err()
    );
    assert!(manager.inventory().unwrap().accounts.is_empty());
    assert!(!root.path().join("accounts.json").exists());

    let (_, manager, _, network) = fixture(vec![user()]);
    manager
        .execute(token("https://api.github.com"), 10)
        .await
        .unwrap();
    let locked = Manager::new(
        manager.root.clone(),
        Arc::new(Secrets {
            fail: true,
            ..Default::default()
        }),
        network,
        None,
    );
    // A locked store must reach the error/toast boundary, rather than being
    // silently reported as a disconnected account.
    assert!(matches!(locked.inventory(), Err(Error::State)));
}
#[tokio::test]
async fn device_polling_obeys_pending_slowdown_and_returns_only_public_authorization() {
    let (_, manager, _, network) = fixture(vec![
        verification(),
        json!({"error":"authorization_pending"}),
        json!({"error":"slow_down"}),
        json!({"access_token":"oauth-secret","refresh_token":"refresh-secret","expires_in":100}),
        user(),
    ]);
    let auth = manager
        .execute(Command::BeginOauth, 100)
        .await
        .unwrap()
        .authorization
        .unwrap();
    assert!(
        !serde_json::to_string(&auth)
            .unwrap()
            .contains("private-device-code")
    );
    for now in [100, 104, 105, 106, 110, 115, 120] {
        let result = manager
            .execute(
                Command::PollOauth {
                    session: auth.session.clone(),
                },
                now,
            )
            .await
            .unwrap();
        if now == 110 {
            assert_eq!(result.authorization.as_ref().unwrap().interval, 10);
        }
        if now == 120 {
            assert_eq!(result.authorization.unwrap().state, "complete");
            assert_eq!(result.accounts.len(), 1);
        }
    }
    assert_eq!(network.calls.lock().unwrap().len(), 5);
}
#[tokio::test]
async fn cancellation_expiry_and_denial_do_not_save_accounts() {
    for response in [
        json!({"error":"access_denied"}),
        json!({"error":"expired_token"}),
    ] {
        let (_, manager, _, _) = fixture(vec![verification(), response]);
        let auth = manager
            .execute(Command::BeginOauth, 100)
            .await
            .unwrap()
            .authorization
            .unwrap();
        let result = manager
            .execute(
                Command::PollOauth {
                    session: auth.session.clone(),
                },
                105,
            )
            .await
            .unwrap();
        assert_ne!(result.authorization.unwrap().state, "pending");
        assert!(result.accounts.is_empty());
        manager
            .execute(
                Command::CancelOauth {
                    session: auth.session.clone(),
                },
                106,
            )
            .await
            .unwrap();
        assert!(
            manager
                .execute(
                    Command::PollOauth {
                        session: auth.session
                    },
                    110
                )
                .await
                .is_err()
        );
    }
    let (_, manager, _, network) = fixture(vec![verification()]);
    let auth = manager
        .execute(Command::BeginOauth, 100)
        .await
        .unwrap()
        .authorization
        .unwrap();
    let result = manager
        .execute(
            Command::PollOauth {
                session: auth.session,
            },
            1001,
        )
        .await
        .unwrap();
    assert_eq!(result.authorization.unwrap().state, "expired");
    assert_eq!(network.calls.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn cancelling_an_inflight_device_poll_discards_its_late_token_response() {
    struct Delayed {
        network: Arc<Network>,
        entered: Mutex<Option<futures::channel::oneshot::Sender<()>>>,
        release: Mutex<Option<futures::channel::oneshot::Receiver<()>>>,
    }
    #[async_trait::async_trait(?Send)]
    impl Transport for Delayed {
        async fn request(
            &self,
            url: &str,
            token: Option<&str>,
            body: Option<Value>,
        ) -> Result<Value> {
            if url.ends_with("/access_token") {
                self.entered
                    .lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .send(())
                    .unwrap();
                let release = self.release.lock().unwrap().take().unwrap();
                release.await.unwrap();
            }
            self.network.request(url, token, body).await
        }
    }
    let (_, original, secrets, network) = fixture(vec![
        verification(),
        json!({"access_token":"late-token"}),
        user(),
    ]);
    let (entered, waiting) = futures::channel::oneshot::channel();
    let (release, blocked) = futures::channel::oneshot::channel();
    let manager = Manager::new(
        original.root.clone(),
        secrets.clone(),
        Arc::new(Delayed {
            network,
            entered: Mutex::new(Some(entered)),
            release: Mutex::new(Some(blocked)),
        }),
        Some("our-client-id".into()),
    );
    let auth = manager
        .execute(Command::BeginOauth, 100)
        .await
        .unwrap()
        .authorization
        .unwrap();
    let (poll, ()) = futures::join!(
        manager.execute(
            Command::PollOauth {
                session: auth.session.clone()
            },
            105
        ),
        async {
            waiting.await.unwrap();
            manager
                .execute(
                    Command::CancelOauth {
                        session: auth.session.clone(),
                    },
                    106,
                )
                .await
                .unwrap();
            release.send(()).unwrap();
        }
    );
    assert!(matches!(poll, Err(Error::Stale)));
    assert!(manager.inventory().unwrap().accounts.is_empty());
    assert!(secrets.values.lock().unwrap().is_empty());
}

#[tokio::test]
async fn expiring_oauth_rotates_credentials_without_returning_them_in_inventory() {
    let (_, manager, secrets, _) = fixture(vec![
        verification(),
        json!({"access_token":"old","refresh_token":"old-refresh","expires_in":61}),
        user(),
        json!({"access_token":"new","refresh_token":"new-refresh","expires_in":1000}),
        user(),
    ]);
    let auth = manager
        .execute(Command::BeginOauth, 100)
        .await
        .unwrap()
        .authorization
        .unwrap();
    let result = manager
        .execute(
            Command::PollOauth {
                session: auth.session,
            },
            105,
        )
        .await
        .unwrap();
    let id = &result.accounts[0].id;
    assert_eq!(
        manager.credential(id, 110).await.unwrap().access_token,
        "new"
    );
    assert!(
        !secrets
            .values
            .lock()
            .unwrap()
            .get(id)
            .unwrap()
            .contains("old-refresh")
    );
    assert!(
        !serde_json::to_string(&manager.inventory().unwrap())
            .unwrap()
            .contains("new-refresh")
    );
}
#[test]
fn remote_detection_requires_known_hosts_and_normalizes_transports() {
    for url in [
        "https://github.com/team/project.git",
        "ssh://git@github.com/team/project.git",
        "git@github.com:team/project.git",
    ] {
        let remote = detect_remote("origin", url, &[]).unwrap();
        assert_eq!(remote.repository, "team/project");
        assert_eq!(remote.api_url, "https://api.github.com");
    }
    for url in [
        "git@unknown.example:team/project.git",
        "https://token@github.com/team/project.git",
        "file:///team/project",
        "https://github.com/team/project?token=x",
    ] {
        assert!(detect_remote("origin", url, &[]).is_none());
    }
    for api in [
        "http://api.github.com",
        "https://api.github.com/other",
        "https://token@enterprise.example/api/v3",
        "https://enterprise.example/api/v3?token=x",
    ] {
        assert!(api_root(api).is_err());
    }
}
