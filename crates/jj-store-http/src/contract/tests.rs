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
fn interactive_execution_contract_is_strict_and_has_no_browser_process_configuration() {
    let doc = openapi();
    let paths = &doc["paths"];
    for (path, method) in [
        ("/v1/local/agents", "get"),
        ("/v1/local/agents/targets", "get"),
        ("/v1/local/agents/setups", "post"),
        ("/v1/local/agents/launch", "post"),
        ("/v1/local/agents/command", "post"),
        ("/v1/local/agents/outcome", "get"),
        ("/v1/local/agents/events", "get"),
        ("/v1/local/agents/stream", "get"),
    ] {
        assert!(paths[path][method].is_object(), "{path}");
        assert_eq!(paths[path][method]["security"], json!([{}]));
    }
    let schemas = &doc["components"]["schemas"];
    for path in [
        "/v1/local/agents/launch",
        "/v1/local/agents/command",
        "/v1/local/agents/setups",
    ] {
        let body = &paths[path]["post"]["requestBody"]["content"]["application/json"]["schema"];
        assert_eq!(body["additionalProperties"], false);
        for unsafe_field in ["cwd", "executable", "environment", "actor", "token"] {
            assert!(body["properties"].get(unsafe_field).is_none());
        }
    }
    for field in [
        "base_commit",
        "review_commit",
        "published_sequence",
        "last_sequence",
    ] {
        assert!(
            schemas["AgentExecutionSession"]["properties"]
                .get(field)
                .is_some()
        );
    }
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
    for path in [
        "/v1/directory/profile",
        "/v1/directory/profiles",
        "/v1/directory/emojis",
        "/v1/directory/emoji-snapshot/{id}",
        "/v1/directory/commands",
        "/v1/directory/commands/{request_id}",
        "/v1/directory/media/{digest}",
        "/v1/directory/identities",
        "/v1/directory/identity-revoke",
        "/v1/local/capture",
        "/v1/local/capture/sources",
        "/v1/local/capture/control",
        "/v1/local/capture/rescan",
        "/v1/local/repos/{repo}/working-tree/observe",
        "/v1/repos/{repo}/local/workspaces",
        "/v1/repos/{repo}/local/workspaces/link",
        "/v1/repos/{repo}/local/workspaces/pick",
        "/v1/repos/{repo}/local/search",
        "/v1/repos/{repo}/local/search/save",
        "/v1/repos/{repo}/local/search/control",
        "/v1/repos/{repo}/local/sync",
        "/v1/repos/{repo}/local/sync/control",
        "/v1/repos/{repo}/threads",
        "/v1/repos/{repo}/threads/{thread_id}/timeline",
        "/v1/repos/{repo}/reactions/query",
        "/v1/repos/{repo}/reaction-emojis/{emoji_id}",
        "/v1/repos/{repo}/reaction-assets/{asset}",
        "/v1/repos/{repo}/work/activity",
        "/v1/local/repos/{repo}/work/activity",
        "/v1/directory/image-uploads/{request_id}",
        "/v1/local/connections",
        "/v1/local/connections/connect",
        "/v1/local/connections/disconnect",
        "/v1/operator/access",
        "/v1/operator/agents",
        "/v1/repos/{repo}/agent-access",
        "/v1/repos/{repo}/agent-access/status",
        "/v1/configuration/reload",
        "/v1/configuration/reload/outcome",
        "/v1/agents",
        "/v1/agents/targets",
        "/v1/agents/chat",
        "/v1/agents/command",
        "/v1/agents/workspace/inspect",
        "/v1/agents/outcome",
        "/v1/agents/events",
        "/v1/agents/feedback",
        "/v1/agents/stream",
    ] {
        assert!(paths.contains_key(path), "missing integrated route: {path}");
    }
    assert_eq!(paths.len(), 200);
    assert_eq!(
        paths
            .keys()
            .filter(|name| name.starts_with("/v1/local/"))
            .count(),
        86
    );
    assert_eq!(
        paths
            .keys()
            .filter(|name| !name.starts_with("/v1/local/"))
            .count(),
        114
    );
    let mut ids = std::collections::BTreeSet::new();
    for (name, path) in paths {
        for (method, operation) in path.as_object().unwrap() {
            assert!(ids.insert(operation["operationId"].as_str().unwrap()));
            if name == "/v1/bootstrap" {
                assert_eq!(operation["parameters"], json!([]));
                assert_eq!(operation["security"], json!([{}]));
            } else if name == "/v1/events/ws" {
                assert_eq!(method, "get");
                assert_eq!(operation["security"], json!([{}]));
                assert_eq!(operation["parameters"][0]["name"], "Origin");
                assert_eq!(
                    operation["parameters"][1]["schema"]["const"],
                    "jj-store-events-v1"
                );
                assert!(operation["responses"]["101"].is_object());
                assert!(operation["x-websocket-client-schema"].is_object());
                assert!(operation["x-websocket-event-schema"].is_object());
            } else if name == "/v1/events" {
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]},{}]));
                assert_eq!(operation["parameters"][0]["name"], "after");
            } else if name == "/v1/repositories" {
                assert!(
                    operation["parameters"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|parameter| parameter["in"] == "query")
                );
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]},{}]));
            } else if name.starts_with("/v1/directory/") {
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]},{}]));
                assert!(
                    operation["parameters"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|p| p["name"] != "repo")
                );
            } else if name.starts_with("/v1/operator/") || name.starts_with("/v1/configuration/") {
                assert_eq!(
                    operation["security"],
                    if name.starts_with("/v1/operator/") {
                        json!([{"repositoryBearer":[]}])
                    } else {
                        json!([{"repositoryBearer":[]},{}])
                    }
                );
                assert!(
                    operation["parameters"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|p| p["name"] != "repo")
                );
            } else if name.starts_with("/v1/local/") {
                assert_eq!(operation["security"], json!([{}]));
                assert_eq!(operation["tags"], json!(["local"]));
            } else if name.starts_with("/v1/agents") {
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]}]));
                assert_eq!(operation["tags"], json!(["agents"]));
                assert!(
                    operation["parameters"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .all(|p| p["name"] != "repo")
                );
            } else if matches!(
                name.as_str(),
                "/v1/repos/{repo}/agent-access" | "/v1/repos/{repo}/agent-access/status"
            ) {
                assert_eq!(operation["parameters"][0]["name"], "repo");
                assert_eq!(operation["security"], json!([{"repositoryBearer":[]}]));
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
    let activity = &paths["/v1/agents/stream"]["get"];
    assert_eq!(activity["parameters"][1]["name"], "after");
    assert_eq!(activity["parameters"][1]["schema"]["type"], "integer");
    assert!(
        activity["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["name"] != "Last-Event-ID")
    );
    assert_eq!(
        activity["responses"]["200"]["content"]["text/event-stream"]["x-sse-events"],
        json!({"agent-activity":"activity"})
    );
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
fn avatar_upload_contract_is_binary_personal_and_receipt_bound() {
    let doc = openapi();
    let op = &doc["paths"]["/v1/directory/image-uploads/{request_id}"]["post"];
    assert_eq!(
        op["requestBody"]["content"]["application/octet-stream"]["schema"]["format"],
        "binary"
    );
    assert!(
        op["description"]
            .as_str()
            .unwrap()
            .contains("before reading bytes")
    );
    assert!(
        op["description"]
            .as_str()
            .unwrap()
            .contains("original request ID")
    );
    assert!(
        op["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["name"] == "request_id" && p["required"] == true)
    );
    assert!(
        op["requestBody"]["description"]
            .as_str()
            .unwrap()
            .contains("20 MiB")
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

#[test]
fn local_contracts_preserve_separate_profiles_and_strict_pinned_queries() {
    let doc = openapi();
    let schemas = &doc["components"]["schemas"];
    assert_eq!(schemas["CommitId"]["minLength"], 128);
    assert_eq!(schemas["GitSha1Id"]["minLength"], 40);
    assert_eq!(
        schemas["LocalCodeSnapshot"]["properties"]["operation"]["$ref"],
        "#/components/schemas/OperationId"
    );
    assert!(
        schemas["LocalCodeSnapshot"]["properties"]
            .get("epoch")
            .is_none()
    );
    assert!(
        schemas["LocalCodeSnapshot"]["properties"]
            .get("published_seq")
            .is_none()
    );
    assert!(
        schemas["LocalGraphPage"]["properties"]
            .get("cache")
            .is_none()
    );
    let query = &doc["paths"]["/v1/local/repos/{repo}/query"]["post"];
    assert_eq!(query["operationId"], "local_code_query");
    assert_eq!(
        doc["paths"]["/v1/local/repositories"]["get"]["operationId"],
        "local_repository_list"
    );
    assert_eq!(
        doc["paths"]["/v1/local/repositories"]["post"]["operationId"],
        "local_repository_register"
    );
    assert_eq!(query["security"], json!([{}]));
    assert_eq!(
        query["requestBody"]["content"]["application/json"]["schema"]["additionalProperties"],
        false
    );
    assert_eq!(
        schemas["LocalRepositoryCapabilities"]["properties"]["search"]["type"],
        "boolean"
    );
    assert!(
        schemas["Capabilities"]["properties"]
            .get("native_profile")
            .is_some()
    );
}

#[test]
fn work_activity_contracts_are_body_free_and_pin_independent_profiles() {
    let doc = openapi();
    let native = &doc["paths"]["/v1/repos/{repo}/work/activity"]["get"];
    let local = &doc["paths"]["/v1/local/repos/{repo}/work/activity"]["post"];
    assert_eq!(native["operationId"], "work_activity");
    assert_eq!(local["operationId"], "local_work_activity");
    let input = &local["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(input["additionalProperties"], false);
    assert!(
        input["required"]
            .as_array()
            .unwrap()
            .contains(&json!("metadata_commit"))
    );
    assert_eq!(input["properties"]["query"]["additionalProperties"], false);
    let schemas = &doc["components"]["schemas"];
    let group = schemas["WorkActivityGroup"]["properties"]
        .as_object()
        .unwrap();
    assert_eq!(group.len(), 4);
    for name in ["repository", "project", "tasks", "updated_at_millis"] {
        assert!(group.contains_key(name));
    }
    assert_eq!(
        schemas["WorkActivityPage"]["properties"]["snapshot"]["$ref"],
        "#/components/schemas/WorkReadSnapshot"
    );
    assert_eq!(
        schemas["LocalWorkActivityPage"]["properties"]["snapshot"]["$ref"],
        "#/components/schemas/LocalWorkSnapshot"
    );
}

#[test]
fn kind_preparation_contract_is_strict_actor_free_and_metadata_only() {
    let doc = openapi();
    let paths = &doc["paths"];
    let start = &paths["/v1/repos/{repo}/index/kinds/tasks"]["post"];
    let schema = &start["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(schema["required"], json!(["operation", "kind"]));
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["properties"].as_object().unwrap().len(), 2);
    assert!(start["responses"].get("202").is_some());
    for (path, method) in [
        ("/v1/repos/{repo}/index/kinds/tasks/{task}", "get"),
        ("/v1/repos/{repo}/index/kinds/tasks/{task}/cancel", "post"),
    ] {
        assert!(paths[path][method]["responses"].get("200").is_some());
        assert!(paths[path][method].get("requestBody").is_none());
    }
    let task = &doc["components"]["schemas"]["KindPreparationTask"];
    assert!(task["properties"].get("progress").is_some());
    assert!(task["properties"].get("ids").is_none());
}

#[test]
fn working_tree_observation_contract_is_registered_pinned_and_path_free() {
    let document = openapi();
    let operation = &document["paths"]["/v1/local/repos/{repo}/working-tree/observe"]["post"];
    assert_eq!(operation["operationId"], "local_working_tree_observe");
    assert_eq!(operation["security"], json!([{}]));
    let request = &operation["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(request["additionalProperties"], false);
    let required = request["required"].as_array().unwrap();
    for key in ["version", "binding_generation", "operation"] {
        assert!(required.contains(&json!(key)));
    }
    assert!(request["properties"].get("path").is_none());
    let schemas = &document["components"]["schemas"];
    let snapshot = &schemas["WorkingTreeSnapshot"]["properties"];
    for key in [
        "repository",
        "binding_generation",
        "operation",
        "view",
        "workspace",
        "working_copy",
        "trees",
        "tree_labels",
        "physical_workspace",
        "metadata_fingerprint",
        "physical_fingerprint",
    ] {
        assert!(snapshot.get(key).is_some(), "{key}");
    }
    let response = &schemas["ObservedWorkingTree"]["properties"];
    assert_eq!(response["changes"]["maxItems"], 4096);
    for key in [
        "root",
        "path",
        "base_ignores",
        "global_ignore",
        "contents",
        "prepared",
        "selection_token",
    ] {
        assert!(response.get(key).is_none(), "{key}");
    }
}
