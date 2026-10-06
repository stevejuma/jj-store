//! Actual registered-folder command responses for generated contract validation.
use super::jj;
use serde_json::{Value, json};
use std::{fs, path::Path};
pub async fn qualify(endpoint: &str, id: &str, folder: &Path) -> Vec<Value> {
    let client = reqwest::Client::new();
    let snapshot: Value = client
        .get(format!("{endpoint}/local/repos/{id}/snapshot"))
        .header("x-jj-store-protocol", "1")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    jj(folder, &["bookmark", "create", "core-topic"]);
    let mut samples = vec![];
    let mut state = json!({"operation":null,"revision":null});
    for command in [
        json!({"kind":"refresh"}),
        json!({"kind":"setup","name":"Core primary","target":"main"}),
        json!({"kind":"apply","branch":"core-topic"}),
        json!({"kind":"unapply","branch":"core-topic"}),
        json!({"kind":"apply","branch":"core-topic"}),
        json!({"kind":"unapply_branches","branches":["core-topic"]}),
        json!({"kind":"create","name":"Core second"}),
        json!({"kind":"refresh"}),
    ] {
        let response = client
            .post(format!("{endpoint}/local/repos/{id}/stacks"))
            .header("x-jj-store-protocol", "1")
            .json(
                &json!({"version":1,"binding_generation":snapshot["generation"],
                "operation":state["operation"],"revision":state["revision"],"command":command}),
            )
            .send()
            .await
            .unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();
        assert!(status.is_success(), "{command}: {status}: {body}");
        state = serde_json::from_str(&body).unwrap();
        assert_eq!(state["version"], 1);
        assert!(state["warning"].is_null(), "{state}");
        if !state["active"].is_null() {
            assert_eq!(state["topology"]["complete"], true);
            assert!(state["topology"]["lanes"].is_array());
        }
        samples.push(
            json!({"path":"/v1/local/repos/{repo}/stacks","method":"post",
            "status":200,"media":"application/json","data":state}),
        );
    }
    assert_eq!(state["workspaces"].as_array().unwrap().len(), 2);
    assert!(
        fs::read_to_string(folder.join(".jj/jj-store/stacks.toml"))
            .unwrap()
            .contains("Core primary")
    );
    let workspace = state["active"].as_str().unwrap().to_owned();
    for command in [
        json!({"kind":"update","workspace":workspace,"name":"Updated second","target":"core-topic"}),
        json!({"kind":"delete","workspace":workspace}),
    ] {
        state=client.post(format!("{endpoint}/local/repos/{id}/stacks"))
          .header("x-jj-store-protocol","1")
          .json(&json!({"version":1,"binding_generation":snapshot["generation"],"operation":state["operation"],"revision":state["revision"],"command":command}))
          .send().await.unwrap().error_for_status().unwrap().json().await.unwrap();
        assert!(state["warning"].is_null(), "{state}");
        samples.push(json!({"path":"/v1/local/repos/{repo}/stacks","method":"post","status":200,"media":"application/json","data":state}));
    }
    assert!(state["active"].is_null());
    assert_eq!(state["workspaces"].as_array().unwrap().len(), 1);
    assert!(
        !fs::read_to_string(folder.join(".jj/jj-store/stacks.toml"))
            .unwrap()
            .contains("Updated second")
    );
    samples
}
