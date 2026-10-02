use serde_json::{Value, json};
use std::path::Path;

pub async fn qualify(
    endpoint: &str,
    id: &str,
    other: &str,
    folder: &Path,
    old_pin: Value,
) -> Vec<Value> {
    let client = reqwest::Client::new();
    let path = |suffix: &str| format!("{endpoint}/local/repos/{id}/{suffix}");
    let post = |suffix: &str, value: Value| {
        client
            .post(path(suffix))
            .header("x-jj-store-protocol", "1")
            .json(&value)
    };
    let get = |suffix: &str| client.get(path(suffix)).header("x-jj-store-protocol", "1");
    let snapshot: Value = get("snapshot")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let anchor = json!({"schema_version":1,"repository_id":id,"target":{"target":"repository"},"created_context":{"publication_position":null,"code_state_version":null,"agent_root":null},"sensitivity":"repository"});
    let body = json!({"schema_version":1,"format":"safe_markdown","text":"Browser repository review","sanitizer_version":"review-markdown-v1"});
    let request = json!({"version":1,"request_id":"28282828282828282828282828282828","command":{"command":"create_thread","anchor":anchor,"body":body}});
    let outcome: Value = post("reviews/commands", request.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(outcome["status"], "committed");
    assert_eq!(outcome["receipt"]["actor"], snapshot["actor"]);
    let replay: Value = post("reviews/commands", request)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(replay, outcome);
    let status: Value = get("reviews/commands/28282828282828282828282828282828")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(status, outcome);
    let cli_status: Value = serde_json::from_str(&super::jj(
        folder,
        &[
            "review",
            "--format=json",
            "status",
            "28282828282828282828282828282828",
        ],
    ))
    .unwrap();
    assert_eq!(cli_status, outcome);
    let _: Value = serde_json::from_str(&super::jj(
        folder,
        &[
            "review",
            "--format=json",
            "repository",
            "--request-id",
            "29292929292929292929292929292929",
            "--body",
            "CLI repository review",
        ],
    ))
    .unwrap();
    let current: Value = get("snapshot")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(current["heads"], snapshot["heads"]);
    let pin = &current["metadata_commit"];
    let page_request = json!({"metadata_commit":pin,"limit":1,"cursor":null});
    let page: Value = post("reviews/page", page_request.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(page["data"]["threads"].as_array().unwrap().len(), 1);
    let mut next = page_request.clone();
    next["cursor"] = page["next_cursor"].clone();
    let continued: Value = post("reviews/page", next.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(continued["data"]["snapshot"], page["data"]["snapshot"]);
    assert_ne!(
        continued["data"]["threads"][0]["thread_id"],
        page["data"]["threads"][0]["thread_id"]
    );
    next["limit"] = json!(2);
    assert_eq!(
        post("reviews/page", next).send().await.unwrap().status(),
        400
    );
    let mut next = page_request;
    next["cursor"] = page["next_cursor"].clone();
    assert_eq!(
        client
            .post(format!("{endpoint}/local/repos/{other}/reviews/page"))
            .header("x-jj-store-protocol", "1")
            .json(&next)
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let old: Value = post(
        "reviews/page",
        json!({"metadata_commit":old_pin,"limit":10,"cursor":null}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    assert!(old["data"]["threads"].as_array().unwrap().is_empty());
    let thread = &outcome["receipt"]["command"]["thread_id"];
    let detail: Value = post(
        "reviews/detail",
        json!({"metadata_commit":pin,"thread_id":thread}),
    )
    .send()
    .await
    .unwrap()
    .error_for_status()
    .unwrap()
    .json()
    .await
    .unwrap();
    let cli_detail: Value = serde_json::from_str(&super::jj(
        folder,
        &[
            "review",
            "--format=json",
            "show",
            thread.as_str().unwrap(),
            "--at-metadata",
            pin.as_str().unwrap(),
        ],
    ))
    .unwrap();
    assert_eq!(cli_detail, detail);
    assert_eq!(
        detail["comments"][0]["content"]["text"],
        "Browser repository review"
    );
    let stale = json!({"version":1,"request_id":"30303030303030303030303030303030","command":{"command":"resolve_thread","thread_id":thread,"expected_thread_version":"00".repeat(32),"reason":null}});
    assert_eq!(
        post("reviews/commands", stale)
            .send()
            .await
            .unwrap()
            .status(),
        409
    );
    let capabilities: Value = get("capabilities")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(capabilities["reviews"], true);
    assert_eq!(capabilities["review_actions"], true);
    assert_eq!(capabilities["review_code_anchors"], false);
    assert!(capabilities.get("synchronization").is_none());
    let sample = |suffix: &str, method: &str, data: Value| json!({"path":format!("/v1/local/repos/{{repo}}/{suffix}"),"method":method,"status":200,"media":"application/json","data":data});
    vec![
        sample("reviews/page", "post", page),
        sample("reviews/detail", "post", detail),
        sample("reviews/commands", "post", outcome),
        sample("reviews/commands/{request_id}", "get", status),
        sample("capabilities", "get", capabilities),
    ]
}
