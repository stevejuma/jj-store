use super::*;

#[test]
fn contract_distinguishes_optional_inputs_from_nullable_outputs() {
    let doc = openapi();
    let log = &doc["paths"]["/v1/repos/{repo}/log"]["get"];
    let params = log["parameters"].as_array().unwrap();
    let time = params
        .iter()
        .find(|p| p["name"] == "at_time_millis")
        .unwrap();
    assert_eq!(time["required"], true);
    let limit = params.iter().find(|p| p["name"] == "limit").unwrap();
    assert_eq!(limit["required"], false);
    assert_eq!(limit["schema"]["default"], 50);
    let selection = params.iter().find(|p| p["name"] == "at_op").unwrap();
    assert_eq!(selection["schema"]["type"], "string");
    assert_eq!(selection["schema"]["minLength"], 128);
    let schema = &doc["components"]["schemas"]["NativeOperation"];
    assert!(
        schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("commit_predecessors"))
    );
    assert!(
        schema["properties"]["commit_predecessors"]["type"]
            .as_array()
            .unwrap()
            .contains(&json!("null"))
    );
    let error = &doc["components"]["schemas"]["ErrorBody"];
    assert!(
        !error["required"]
            .as_array()
            .unwrap()
            .contains(&json!("snapshot"))
    );
}

#[test]
fn contract_preserves_native_shapes_and_hex_id_widths() {
    let doc = openapi();
    let schemas = &doc["components"]["schemas"];
    for (name, length) in [
        ("RepositoryId", 32),
        ("ChangeId", 32),
        ("CommitId", 128),
        ("ConflictFingerprint", 128),
        ("ConflictUploadId", 64),
    ] {
        assert_eq!(schemas[name]["minLength"], length);
        assert_eq!(schemas[name]["maxLength"], length);
        assert_eq!(schemas[name]["pattern"], format!("^[0-9a-f]{{{length}}}$"));
    }
    let tuple = &schemas["NativeTree"]["properties"]["entries"]["items"];
    assert_eq!(tuple["minItems"], 2);
    assert_eq!(tuple["maxItems"], 2);
    assert_eq!(tuple["prefixItems"][0]["type"], "string");
    assert!(
        schemas["NativeTreeValue"]["oneOf"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["properties"]["type"]["const"] == "submodule")
    );
    assert_ne!(schemas["NativeCommit"], schemas["GraphCommit"]);
    assert_eq!(
        schemas["NativeRemoteRef"]["properties"]["target"]["type"],
        "array"
    );
}

#[test]
fn collaboration_contract_exposes_root_pinned_agent_session_pages() {
    let doc = openapi();
    let path = &doc["paths"]["/v1/repos/{repo}/agent/sessions"];
    assert!(path.get("get").is_some());
    let operation = &path["get"];
    let parameters = operation["parameters"].as_array().unwrap();
    assert_eq!(
        parameters
            .iter()
            .find(|parameter| parameter["name"] == "limit")
            .unwrap()["schema"]["default"],
        50
    );
    assert!(
        parameters
            .iter()
            .any(|parameter| parameter["name"] == "cursor")
    );
    let page = &doc["components"]["schemas"]["AgentSessionPage"];
    for field in ["version", "snapshot", "sessions", "next_cursor"] {
        assert!(page["required"].as_array().unwrap().contains(&json!(field)));
    }
    let summary = &doc["components"]["schemas"]["AgentSessionSummary"];
    for field in ["first_activity_at_millis", "last_activity_at_millis"] {
        assert!(
            summary["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
    let snapshot = &doc["components"]["schemas"]["AgentReadSnapshot"];
    for field in [
        "repo_id",
        "epoch",
        "published_seq",
        "manifest_revision",
        "agent_root",
        "agent_state_version",
    ] {
        assert!(
            snapshot["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }

    let detail = &doc["paths"]["/v1/repos/{repo}/agent/sessions/{session}"]["get"];
    assert_eq!(detail["tags"], json!(["collaboration"]));
    assert_eq!(
        detail["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/AgentSessionDetail"
    );
    let schema = &doc["components"]["schemas"]["AgentSessionDetail"];
    for field in [
        "version",
        "snapshot",
        "session",
        "evidence",
        "event_batches",
        "transcripts",
        "tools",
        "activity",
        "activity_truncated",
        "code",
        "code_truncated",
    ] {
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }

    let transcript =
        &doc["paths"]["/v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}"]["get"];
    assert_eq!(transcript["tags"], json!(["collaboration"]));
    let parameters = transcript["parameters"].as_array().unwrap();
    for required in ["source", "generation", "limit", "max_bytes"] {
        assert!(parameters.iter().any(|value| value["name"] == required));
    }
    assert!(parameters.iter().any(|value| value["name"] == "cursor"));
    assert_eq!(
        transcript["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/AgentTranscriptPage"
    );
    let page = &doc["components"]["schemas"]["AgentTranscriptPage"];
    for field in [
        "version",
        "snapshot",
        "session_id",
        "transcript_id",
        "source_instance_id",
        "source_generation",
        "chunks",
        "next_cursor",
    ] {
        assert!(page["required"].as_array().unwrap().contains(&json!(field)));
    }

    let relations = &doc["paths"]["/v1/repos/{repo}/agent/associations"]["get"];
    assert_eq!(relations["tags"], json!(["collaboration"]));
    let parameters = relations["parameters"].as_array().unwrap();
    for field in [
        "session",
        "tool_session",
        "tool",
        "operation",
        "commit",
        "checkpoint",
        "cursor",
        "limit",
    ] {
        assert!(parameters.iter().any(|value| value["name"] == field));
    }
    assert_eq!(
        relations["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/AgentRelationPage"
    );
    let relation_page = &doc["components"]["schemas"]["AgentRelationPage"];
    for field in ["version", "snapshot", "subject", "relations", "next_cursor"] {
        assert!(
            relation_page["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }

    let checkpoints = &doc["paths"]["/v1/repos/{repo}/agent/checkpoints"]["get"];
    assert_eq!(checkpoints["tags"], json!(["collaboration"]));
    assert_eq!(
        checkpoints["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
        "#/components/schemas/AgentCheckpointPage"
    );
    let checkpoint_page = &doc["components"]["schemas"]["AgentCheckpointPage"];
    for field in ["version", "snapshot", "checkpoints", "next_cursor"] {
        assert!(
            checkpoint_page["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
    for path in [
        "/v1/repos/{repo}/agent/checkpoints/{checkpoint}",
        "/v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}",
    ] {
        let detail = &doc["paths"][path]["get"];
        assert_eq!(detail["tags"], json!(["collaboration"]));
        assert_eq!(
            detail["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
            "#/components/schemas/AgentCheckpointDetail"
        );
    }
    let checkpoint_detail = &doc["components"]["schemas"]["AgentCheckpointDetail"];
    for field in [
        "version",
        "snapshot",
        "checkpoint",
        "session_snapshot_id",
        "transcript_id",
        "event_batches",
        "tools",
        "associations",
    ] {
        assert!(
            checkpoint_detail["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
}

#[test]
fn contract_inventory_covers_security_methods_media_and_task_bodies() {
    let doc = openapi();
    assert_eq!(doc["security"], json!([]));
    assert_eq!(
        doc["x-jj-store-access-policies"],
        json!(["authenticated", "local_trusted"])
    );
    let paths = doc["paths"].as_object().unwrap();
    assert_eq!(paths.len(), 40);
    let mut ids = std::collections::BTreeSet::new();
    for (name, path) in paths {
        for (method, operation) in path.as_object().unwrap() {
            assert!(ids.insert(operation["operationId"].as_str().unwrap()));
            if name == "/v1/bootstrap" {
                assert_eq!(operation["parameters"], json!([]));
                assert_eq!(operation["security"], json!([{}]));
            } else if name == "/v1/repositories" {
                assert!(
                    operation["parameters"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|parameter| parameter["in"] == "query")
                );
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]},{}]));
            } else {
                assert_eq!(operation["parameters"][0]["name"], "repo");
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]},{}]));
            }
            if method == "head" {
                for response in operation["responses"].as_object().unwrap().values() {
                    assert!(response.get("content").is_none());
                }
            }
        }
    }
    let events = &paths["/v1/repos/{repo}/events"]["get"]["responses"]["200"]["content"];
    assert!(events.get("application/json").is_none());
    assert!(
        events["text/event-stream"]["x-sse-data-schema"]
            .get("$ref")
            .is_some()
    );
    let file = &paths["/v1/repos/{repo}/files/{file}"]["get"];
    assert_eq!(
        file["responses"]["206"]["content"]["application/octet-stream"]["schema"]["format"],
        "binary"
    );
    assert!(
        file["responses"]["416"]["headers"]
            .get("Content-Range")
            .is_some()
    );
    let prepare = &paths["/v1/repos/{repo}/index/tasks"]["post"];
    assert_eq!(prepare["requestBody"]["required"], true);
    assert_eq!(
        prepare["requestBody"]["content"]["application/json"]["schema"]["required"],
        json!(["operation"])
    );
    assert!(prepare["responses"].get("202").is_some());
    assert!(
        paths["/v1/repos/{repo}/index/tasks/{task}/cancel"]["post"]
            .get("requestBody")
            .is_none()
    );
}

#[test]
fn bootstrap_contract_separates_access_storage_catalog_and_backup_state() {
    let doc = openapi();
    let operation = &doc["paths"]["/v1/bootstrap"]["get"];
    assert_eq!(operation["security"], json!([{}]));
    let schema = &doc["components"]["schemas"]["Bootstrap"];
    for field in [
        "version",
        "access_mode",
        "capabilities",
        "storage",
        "backups",
    ] {
        assert!(
            schema["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
    assert_eq!(
        doc["components"]["schemas"]["AccessMode"]["enum"],
        json!(["authenticated", "local_trusted"])
    );
    let repositories = &doc["paths"]["/v1/repositories"]["get"];
    assert_eq!(
        repositories["security"],
        json!([{"repositoryBearer":[]},{}])
    );
    assert_eq!(
        repositories["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .find(|parameter| parameter["name"] == "limit")
            .unwrap()["schema"]["default"],
        50
    );
    let page = &doc["components"]["schemas"]["RepositoryPage"];
    for field in ["version", "generation", "repositories"] {
        assert!(page["required"].as_array().unwrap().contains(&json!(field)));
    }
    assert!(
        !page["required"]
            .as_array()
            .unwrap()
            .contains(&json!("next_cursor"))
    );
    assert_eq!(
        doc["components"]["schemas"]["EffectivePermission"]["enum"],
        json!(["read", "write", "admin"])
    );
    let backup = &doc["components"]["schemas"]["BackupStatus"];
    assert!(
        backup["required"]
            .as_array()
            .unwrap()
            .contains(&json!("last_verified_restore_point"))
    );
}

#[test]
fn browsing_contracts_keep_complete_reads_and_expose_bound_windows() {
    let doc = openapi();
    let paths = doc["paths"].as_object().unwrap();
    for path in [
        "/v1/repos/{repo}/references",
        "/v1/repos/{repo}/commits/{commit}/paths",
        "/v1/repos/{repo}/operations/{operation}/changes",
        "/v1/repos/{repo}/diff/paths",
    ] {
        assert!(
            paths[path]["get"]["responses"]["200"]["content"]["application/json"]["schema"]
                .get("$ref")
                .is_some()
        );
    }
    let references = paths["/v1/repos/{repo}/references"]["get"]["parameters"]
        .as_array()
        .unwrap();
    assert_eq!(
        references
            .iter()
            .find(|parameter| parameter["name"] == "limit")
            .unwrap()["schema"]["default"],
        50
    );
    assert_eq!(
        references
            .iter()
            .find(|parameter| parameter["name"] == "direction")
            .unwrap()["schema"]["default"],
        "forward"
    );
    let changes = paths["/v1/repos/{repo}/operations/{operation}/changes"]["get"]["parameters"]
        .as_array()
        .unwrap();
    assert_eq!(
        changes
            .iter()
            .find(|parameter| parameter["name"] == "parent")
            .unwrap()["required"],
        true
    );
    let diff_paths = paths["/v1/repos/{repo}/diff/paths"]["get"]["parameters"]
        .as_array()
        .unwrap();
    assert_eq!(
        diff_paths
            .iter()
            .find(|parameter| parameter["name"] == "at_op")
            .unwrap()["required"],
        true
    );
    let complete_diff = paths["/v1/repos/{repo}/diff"]["get"]["parameters"]
        .as_array()
        .unwrap();
    assert!(
        complete_diff
            .iter()
            .all(|parameter| parameter["name"] != "cursor")
    );
    let reference = paths["/v1/repos/{repo}/references"]["get"]["responses"]["200"]
        ["content"]["application/json"]["schema"]["$ref"]
        .as_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap();
    let window = &doc["components"]["schemas"][reference];
    assert!(
        window["required"]
            .as_array()
            .unwrap()
            .contains(&json!("previous_cursor"))
    );
    assert!(
        window["required"]
            .as_array()
            .unwrap()
            .contains(&json!("next_cursor"))
    );
}

#[test]
fn mutation_contracts_require_typed_intent_and_write_permission() {
    let doc = openapi();
    for path in [
        "/v1/repos/{repo}/mutations",
        "/v1/repos/{repo}/mutations/preview",
    ] {
        let op = &doc["paths"][path]["post"];
        assert_eq!(op["tags"], json!(["mutations"]));
        assert!(
            op["description"]
                .as_str()
                .unwrap()
                .contains("write permission")
        );
        let schema = &op["requestBody"]["content"]["application/json"]["schema"];
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(
            schema["required"],
            json!(["version", "request_id", "expected", "command"])
        );
        assert_eq!(
            schema["properties"]["expected"]["additionalProperties"],
            false
        );
        assert_eq!(
            schema["properties"]["command"]["oneOf"]
                .as_array()
                .unwrap()
                .len(),
            9
        );
    }
    let status = &doc["paths"]["/v1/repos/{repo}/mutations/{request_id}"]["get"];
    assert_eq!(status["parameters"][1]["name"], "request_id");
    assert!(
        status["description"]
            .as_str()
            .unwrap()
            .contains("not_known never proves rejection")
    );
}

#[test]
fn conflict_upload_contract_has_binary_body_exact_snapshot_and_write_permission() {
    let doc = openapi();
    let op = &doc["paths"]["/v1/repos/{repo}/conflict-uploads"]["post"];
    assert!(
        op["description"]
            .as_str()
            .unwrap()
            .contains("write permission before body consumption")
    );
    assert_eq!(
        op["requestBody"]["content"]["application/octet-stream"]["schema"]["format"],
        "binary"
    );
    for name in [
        "epoch",
        "position",
        "operation",
        "commit",
        "path",
        "fingerprint",
    ] {
        assert!(
            op["parameters"]
                .as_array()
                .unwrap()
                .iter()
                .any(|p| p["name"] == name && p["required"] == true)
        );
    }
    assert_eq!(
        doc["components"]["schemas"]["ConflictUploadId"]["pattern"],
        "^[0-9a-f]{64}$"
    );
}

#[test]
fn search_contract_requires_query_and_exposes_freshness_and_resets() {
    let doc = openapi();
    let op = &doc["paths"]["/v1/repos/{repo}/search"]["get"];
    let query = &op["x-jj-store-query-schema"];
    assert_eq!(query["additionalProperties"], false);
    assert_eq!(query["required"], json!(["q"]));
    assert_eq!(query["properties"]["page_size"]["default"], 50);
    assert_eq!(query["properties"]["mode"]["default"], "visible_commits");
    assert_eq!(query["properties"]["wait_millis"]["default"], 0);
    assert!(op["responses"].get("501").is_some());
    let schemas = &doc["components"]["schemas"];
    for field in [
        "indexed_position",
        "index_generation",
        "selected_view",
        "scope_snapshots",
        "is_stale",
        "degraded",
        "next_cursor",
        "truncated",
    ] {
        assert!(
            schemas["SearchPage"]["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
    assert!(
        schemas["ErrorBody"]["properties"]
            .get("search_status")
            .is_some()
    );
}
