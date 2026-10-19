use jj_forge::{
    Result,
    accounts::{Manager, Method, http::Transport, store::SecretStore},
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
#[derive(Default)]
struct Secrets(Mutex<HashMap<String, String>>);
impl SecretStore for Secrets {
    fn get(&self, id: &str) -> Result<Option<String>> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }
    fn set(&self, id: &str, value: &str) -> Result<()> {
        self.0.lock().unwrap().insert(id.into(), value.into());
        Ok(())
    }
    fn remove(&self, id: &str) -> Result<()> {
        self.0.lock().unwrap().remove(id);
        Ok(())
    }
}
#[derive(Default)]
struct Network(Mutex<Vec<String>>);
#[async_trait::async_trait(?Send)]
impl Transport for Network {
    async fn request(&self, url: &str, token: Option<&str>, _: Option<Value>) -> Result<Value> {
        assert_eq!(token, Some("synthetic-forgejo-token"));
        self.0.lock().unwrap().push(url.into());
        if url.ends_with("/user") {
            Ok(json!({"id":42,"login":"lapis-fixture","name":"Fixture"}))
        } else {
            Ok(
                json!({"full_name":"lapis-fixture/notes","clone_url":"https://forge.example/lapis-fixture/notes.git","default_branch":"main","permissions":{"push":true}}),
            )
        }
    }
}
#[tokio::test]
async fn explicit_forgejo_accounts_keep_native_custody_and_use_the_approved_instance() {
    let root = tempfile::tempdir().unwrap();
    let secrets = Arc::new(Secrets::default());
    let network = Arc::new(Network::default());
    let manager = Manager::new(
        root.path().join("private"),
        secrets.clone(),
        network.clone(),
        None,
    );
    for endpoint in [
        "http://forge.example/api/v1",
        "https://forge.example/api/v3",
        "https://forge.example/api/v1?token=secret",
        "https://secret@forge.example/api/v1",
        "https://api.github.com/api/v1",
    ] {
        assert!(
            manager
                .add_forgejo_token(endpoint, "synthetic-forgejo-token".into())
                .await
                .is_err()
        );
    }
    assert!(network.0.lock().unwrap().is_empty());
    let inventory = manager
        .add_forgejo_token(
            "https://forge.example/api/v1/",
            "synthetic-forgejo-token".into(),
        )
        .await
        .unwrap();
    let account = &inventory.accounts[0];
    assert_eq!(account.method, Method::Pat);
    assert!(!inventory.oauth_available);
    assert_eq!(account.api_url, "https://forge.example/api/v1");
    manager
        .repository(account, "lapis-fixture/notes", 10)
        .await
        .unwrap();
    assert_eq!(
        *network.0.lock().unwrap(),
        vec![
            "https://forge.example/api/v1/user",
            "https://forge.example/api/v1/repos/lapis-fixture/notes"
        ]
    );
    let metadata = std::fs::read_to_string(root.path().join("private/accounts.json")).unwrap();
    assert!(!metadata.contains("synthetic-forgejo-token"));
    assert!(
        !serde_json::to_string(&inventory)
            .unwrap()
            .contains("synthetic-forgejo-token")
    );
    assert!(secrets.get(&account.id).unwrap().is_some());
    let reopened = Manager::new(root.path().join("private"), secrets, network, None);
    assert!(reopened.inventory().unwrap().accounts[0].available);
    assert!(
        jj_forge::accounts::api_root("https://forge.example/api/v1").is_err(),
        "GitHub admission must stay strict"
    );
}
