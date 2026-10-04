use super::*;

#[test]
fn review_activity_contract_is_compact_bounded_and_independently_pinned() {
    let doc = openapi();
    let native = &doc["paths"]["/v1/repos/{repo}/reviews/activity"]["get"];
    let parameters = native["parameters"].as_array().unwrap();
    assert!(!parameters.iter().any(|value| value["name"] == "cursor"));
    assert_eq!(
        parameters
            .iter()
            .find(|value| value["name"] == "limit")
            .unwrap()["schema"]["default"],
        50
    );
    assert_eq!(
        native["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ReviewActivitySummary"
    );
    let local = &doc["paths"]["/v1/local/repos/{repo}/reviews/activity"]["post"];
    assert_eq!(
        local["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/LocalReviewActivitySummary"
    );
    let activity = &doc["components"]["schemas"]["ReviewActivity"];
    assert_eq!(activity["properties"].as_object().unwrap().len(), 3);
    for field in ["threads", "threads_partial", "updated_at_millis"] {
        assert!(activity["properties"].get(field).is_some());
    }
}

#[test]
fn collaboration_contract_exposes_root_pinned_review_threads() {
    let doc = openapi();
    let list = &doc["paths"]["/v1/repos/{repo}/reviews/threads"]["get"];
    assert_eq!(list["tags"], json!(["collaboration"]));
    let parameters = list["parameters"].as_array().unwrap();
    assert!(parameters.iter().any(|value| value["name"] == "cursor"));
    assert_eq!(
        parameters
            .iter()
            .find(|value| value["name"] == "limit")
            .unwrap()["schema"]["default"],
        50
    );
    assert_eq!(
        list["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ReviewThreadPage"
    );

    let page = &doc["components"]["schemas"]["ReviewThreadPage"];
    for field in ["version", "snapshot", "threads", "next_cursor"] {
        assert!(page["required"].as_array().unwrap().contains(&json!(field)));
    }
    let snapshot = &doc["components"]["schemas"]["ReviewReadSnapshot"];
    for field in [
        "repo_id",
        "epoch",
        "published_seq",
        "manifest_revision",
        "review_root",
        "review_state_version",
    ] {
        assert!(
            snapshot["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }

    let detail = &doc["paths"]["/v1/repos/{repo}/reviews/threads/{thread}"]["get"];
    assert_eq!(detail["tags"], json!(["collaboration"]));
    assert_eq!(
        detail["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ReviewThreadDetail"
    );
    let detail_schema = &doc["components"]["schemas"]["ReviewThreadDetail"];
    for field in [
        "version", "snapshot", "thread", "anchor", "comments", "events",
    ] {
        assert!(
            detail_schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
}

#[test]
fn collaboration_contract_exposes_receipt_backed_review_commands() {
    let doc = super::openapi();
    let apply = &doc["paths"]["/v1/repos/{repo}/reviews/commands"]["post"];
    assert_eq!(apply["tags"], serde_json::json!(["collaboration"]));
    assert_eq!(
        apply["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ReviewCommandOutcome"
    );
    let request = &apply["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(
        request["required"],
        serde_json::json!(["version", "request_id", "command"])
    );
    let variants = request["properties"]["command"]["oneOf"]
        .as_array()
        .unwrap();
    let commands = variants
        .iter()
        .map(|variant| variant["properties"]["command"]["const"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        commands,
        std::collections::BTreeSet::from([
            "create_thread",
            "append_reply",
            "edit_comment",
            "resolve_comment_conflict",
            "tombstone_comment",
            "hide_comment",
            "resolve_thread",
            "reopen_thread",
        ])
    );
    let resolution = variants
        .iter()
        .find(|variant| variant["properties"]["command"]["const"] == "resolve_comment_conflict")
        .unwrap();
    assert!(
        resolution["required"]
            .as_array()
            .unwrap()
            .contains(&json!("expected_revision_ids"))
    );

    let status = &doc["paths"]["/v1/repos/{repo}/reviews/commands/{request_id}"]["get"];
    assert_eq!(status["tags"], serde_json::json!(["collaboration"]));
    assert_eq!(
        status["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/ReviewCommandStatus"
    );
    assert!(
        status["description"]
            .as_str()
            .unwrap()
            .contains("not_known never proves rejection")
    );
}
