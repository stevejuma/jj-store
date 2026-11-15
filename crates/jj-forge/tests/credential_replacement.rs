use jj_forge::{
    Error, Result,
    accounts::{Command, Manager, http::Transport, store::SecretStore},
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

#[derive(Default)]
struct Secrets(Mutex<HashMap<String, String>>, AtomicBool);
impl SecretStore for Secrets {
    fn get(&self, id: &str) -> Result<Option<String>> {
        if self.1.load(Ordering::SeqCst) {
            return Err(Error::State);
        }
        Ok(self.0.lock().unwrap().get(id).cloned())
    }
    fn set(&self, id: &str, value: &str) -> Result<()> {
        if self.1.load(Ordering::SeqCst) {
            return Err(Error::State);
        }
        self.0.lock().unwrap().insert(id.into(), value.into());
        Ok(())
    }
    fn remove(&self, id: &str) -> Result<()> {
        self.0.lock().unwrap().remove(id);
        Ok(())
    }
}
type Hook = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<()>>>> + Send>;
#[derive(Default)]
struct Network(Mutex<Vec<String>>, Mutex<Option<Hook>>);
#[async_trait::async_trait(?Send)]
impl Transport for Network {
    async fn request(&self, url: &str, token: Option<&str>, _: Option<Value>) -> Result<Value> {
        self.0.lock().unwrap().push(url.into());
        let hook = self.1.lock().unwrap().take();
        if let Some(hook) = hook {
            hook().await?;
        }
        if url == "https://github.com/login/device/code" {
            return Ok(
                json!({"device_code":"private-device-fixture", "user_code":"ABCD-EFGH",
                "verification_uri":"https://github.com/login/device", "expires_in":900, "interval":5}),
            );
        }
        if url == "https://github.com/login/oauth/access_token" {
            return Ok(
                json!({"access_token":"oauth-fixture", "refresh_token":"refresh-fixture", "expires_in":900}),
            );
        }
        Ok(
            json!({"id":if token == Some("different-fixture") {99} else {42},
            "login":if token == Some("replacement-fixture") {"renamed"} else {"original"}}),
        )
    }
}

#[tokio::test]
async fn manual_replacement_preserves_an_oauth_account_reference_without_old_refresh_material() {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let network = Arc::new(Network::default());
    let manager = Manager::new(
        root.path().join("accounts"),
        secrets,
        network.clone(),
        Some("fixture-client".into()),
    );
    let authorization = manager
        .execute(Command::BeginOauth, 10)
        .await
        .unwrap()
        .authorization
        .unwrap();
    let authorized = manager
        .execute(
            Command::PollOauth {
                session: authorization.session,
            },
            15,
        )
        .await
        .unwrap();
    let account = &authorized.accounts[0];
    let replaced = manager
        .replace_token(&account.id, "replacement-fixture".into())
        .await
        .unwrap();
    assert_eq!(replaced.accounts[0].id, account.id);
    assert_eq!(
        replaced.accounts[0].method,
        jj_forge::accounts::Method::Oauth
    );
    let before = network.0.lock().unwrap().len();
    let credential = manager.credential(&account.id, 16).await.unwrap();
    assert!(credential.access_token == "replacement-fixture");
    assert!(credential.refresh_token.is_none() && credential.expires_at.is_none());
    assert_eq!(network.0.lock().unwrap().len(), before);
}

#[tokio::test]
async fn locked_custody_does_not_advance_metadata_or_fall_back_to_plaintext() {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let manager = Manager::new(
        root.path().join("accounts"),
        secrets.clone(),
        Arc::new(Network::default()),
        None,
    );
    let saved = manager
        .execute(
            Command::AddToken {
                api_url: "https://api.github.com".into(),
                token: "initial-fixture".into(),
            },
            10,
        )
        .await
        .unwrap();
    let id = &saved.accounts[0].id;
    let secret = secrets.get(id).unwrap();
    let metadata = std::fs::read(manager.root.join("accounts.json")).unwrap();
    secrets.1.store(true, Ordering::SeqCst);
    assert!(matches!(
        manager
            .replace_token(id, "replacement-fixture".into())
            .await,
        Err(Error::State)
    ));
    assert!(std::fs::read(manager.root.join("accounts.json")).unwrap() == metadata);
    secrets.1.store(false, Ordering::SeqCst);
    assert!(secrets.get(id).unwrap() == secret);
    assert_eq!(std::fs::read_dir(&manager.root).unwrap().count(), 2);
}

#[tokio::test]
async fn invalid_or_different_identity_never_changes_the_selected_credential() {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let network = Arc::new(Network::default());
    let manager = Manager::new(
        root.path().join("accounts"),
        secrets.clone(),
        network.clone(),
        None,
    );
    let inventory = manager
        .execute(
            Command::AddToken {
                api_url: "https://api.github.com".into(),
                token: "initial-fixture".into(),
            },
            10,
        )
        .await
        .unwrap();
    let id = &inventory.accounts[0].id;
    let metadata = std::fs::read(manager.root.join("accounts.json")).unwrap();
    let secret = secrets.get(id).unwrap();
    for token in ["", "a\nb", "a\tb", "a\0b"] {
        assert!(matches!(
            manager.replace_token(id, token.into()).await,
            Err(Error::Invalid)
        ));
    }
    assert!(matches!(
        manager
            .replace_token("unknown", "replacement-fixture".into())
            .await,
        Err(Error::Denied)
    ));
    assert_eq!(network.0.lock().unwrap().len(), 1);
    assert!(matches!(
        manager.replace_token(id, "different-fixture".into()).await,
        Err(Error::Denied)
    ));
    assert_eq!(network.0.lock().unwrap().len(), 2);
    assert!(std::fs::read(manager.root.join("accounts.json")).unwrap() == metadata);
    assert!(secrets.get(id).unwrap() == secret);

    // A changed metadata endpoint cannot redirect the replacement token.
    let mut saved = jj_forge::accounts::store::Store::open(&manager.root).unwrap();
    saved.accounts[0].api_url = "https://unexpected.example/api/v3".into();
    saved.save().unwrap();
    drop(saved);
    assert!(matches!(
        manager
            .replace_token(id, "replacement-fixture".into())
            .await,
        Err(Error::Denied)
    ));
    assert_eq!(network.0.lock().unwrap().len(), 2);
    assert!(secrets.get(id).unwrap() == secret);
}

#[tokio::test]
async fn replacement_never_resurrects_a_forgotten_account_or_overwrites_a_newer_credential() {
    for forget in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let secrets = Arc::new(Secrets::default());
        let network = Arc::new(Network::default());
        let manager = Arc::new(Manager::new(
            root.path().join("accounts"),
            secrets.clone(),
            network.clone(),
            None,
        ));
        let inventory = manager
            .execute(
                Command::AddToken {
                    api_url: "https://api.github.com".into(),
                    token: "initial-fixture".into(),
                },
                10,
            )
            .await
            .unwrap();
        let id = inventory.accounts[0].id.clone();
        let concurrent = manager.clone();
        let selected = id.clone();
        *network.1.lock().unwrap() = Some(Box::new(move || {
            Box::pin(async move {
                let command = if forget {
                    Command::Forget { account: selected }
                } else {
                    Command::AddToken {
                        api_url: "https://api.github.com".into(),
                        token: "concurrent-fixture".into(),
                    }
                };
                concurrent.execute(command, 11).await?;
                Ok(())
            })
        }));
        assert!(matches!(
            manager
                .replace_token(&id, "replacement-fixture".into())
                .await,
            Err(Error::Stale)
        ));
        let current = manager.inventory().unwrap();
        if forget {
            assert!(current.accounts.is_empty());
            assert!(secrets.get(&id).unwrap().is_none());
        } else {
            assert_eq!(current.accounts.len(), 1);
            assert_eq!(current.accounts[0].revision, 2);
            assert_eq!(current.accounts[0].login, "original");
            let credential = manager.credential(&id, 12).await.unwrap();
            assert!(credential.access_token == "concurrent-fixture");
        }
    }
}
#[tokio::test]
async fn replacement_preserves_the_selected_account_and_scope() {
    for api in [
        "https://api.github.com",
        "https://enterprise.example/api/v3",
        "https://forge.example/api/v1",
    ] {
        let root = tempfile::tempdir().unwrap();
        let secrets = Arc::new(Secrets::default());
        let network = Arc::new(Network::default());
        let manager = Manager::new(
            root.path().join("accounts"),
            secrets.clone(),
            network.clone(),
            None,
        );
        let before = if api.ends_with("/api/v1") {
            manager
                .add_forgejo_token(api, "initial-fixture".into())
                .await
                .unwrap()
        } else {
            manager
                .execute(
                    Command::AddToken {
                        api_url: api.into(),
                        token: "initial-fixture".into(),
                    },
                    10,
                )
                .await
                .unwrap()
        };
        let account = &before.accounts[0];
        let updated = manager
            .replace_token(&account.id, "replacement-fixture".into())
            .await
            .unwrap();
        assert_eq!(updated.accounts.len(), 1);
        assert_eq!(updated.accounts[0].id, account.id);
        assert_eq!(updated.accounts[0].api_url, account.api_url);
        assert_eq!(updated.accounts[0].user_id, account.user_id);
        assert_eq!(updated.accounts[0].revision, account.revision + 1);
        assert_eq!(updated.accounts[0].login, "renamed");
        assert_eq!(
            *network.0.lock().unwrap(),
            vec![format!("{api}/user"), format!("{api}/user")]
        );
        let metadata = std::fs::read_to_string(root.path().join("accounts/accounts.json")).unwrap();
        assert!(!metadata.contains("replacement-fixture"));
        assert!(
            !serde_json::to_string(&updated)
                .unwrap()
                .contains("replacement-fixture")
        );
        let restarted = Manager::new(root.path().join("accounts"), secrets, network, None);
        assert!(restarted.inventory().unwrap().accounts[0].available);
    }
}

#[tokio::test]
async fn replacement_refuses_a_secret_refreshed_without_a_metadata_revision() {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let network = Arc::new(Network::default());
    let manager = Manager::new(
        root.path().join("accounts"),
        secrets.clone(),
        network.clone(),
        None,
    );
    let saved = manager
        .execute(
            Command::AddToken {
                api_url: "https://api.github.com".into(),
                token: "initial-fixture".into(),
            },
            10,
        )
        .await
        .unwrap();
    let id = saved.accounts[0].id.clone();
    let selected = id.clone();
    let custody = secrets.clone();
    *network.1.lock().unwrap() = Some(Box::new(move || {
        Box::pin(async move { custody.set(&selected, "refreshed-fixture") })
    }));
    assert!(matches!(
        manager
            .replace_token(&id, "replacement-fixture".into())
            .await,
        Err(Error::Stale)
    ));
    assert_eq!(manager.account(&id).unwrap().revision, 1);
    assert!(secrets.get(&id).unwrap().as_deref() == Some("refreshed-fixture"));
}
