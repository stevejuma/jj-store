use super::*;

#[test]
fn capture_management_contract_is_installation_local_and_actor_free() {
    let document = openapi();
    let paths = &document["paths"];
    let status = &paths["/v1/local/capture"]["get"];
    assert!(
        status.is_object(),
        "missing installation-local capture status"
    );
    assert_eq!(status["security"], json!([{}]));
    let command = &paths["/v1/local/capture/control"]["post"];
    assert_eq!(command["security"], json!([{}]));
    let schemas = &document["components"]["schemas"];
    let input = &command["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(input["additionalProperties"], false);
    assert!(input["properties"].get("principal").is_none());
    assert!(input["properties"].get("actor").is_none());
    assert_eq!(
        input["properties"]["generation"]["pattern"],
        "^[0-9a-f]{32}$"
    );
    let required = input["required"].as_array().unwrap();
    for name in [
        "version",
        "request_id",
        "generation",
        "expected_revision",
        "action",
    ] {
        assert!(required.contains(&json!(name)));
    }
    let save = &paths["/v1/local/capture/settings"]["post"];
    assert_eq!(save["security"], json!([{}]));
    let body = &save["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(body["additionalProperties"], false);
    assert_eq!(
        body["properties"]["expected_revision"]["pattern"],
        "^[0-9a-f]{64}$"
    );
    assert!(body["properties"].get("actor").is_none());
    assert_eq!(
        schemas["CaptureConfiguration"]["properties"]["harnesses"]["maxItems"],
        2
    );
    let phases = &schemas["CaptureRuntimePhase"]["enum"];
    assert!(phases.as_array().unwrap().contains(&json!("draining")));
    let rescan = &paths["/v1/local/capture/rescan"]["post"];
    assert_eq!(rescan["security"], json!([{}]));
    let hint = &rescan["requestBody"]["content"]["application/json"]["schema"];
    assert_eq!(hint["additionalProperties"], false);
    assert_eq!(hint["properties"].as_object().unwrap().len(), 3);
    let required = hint["required"].as_array().unwrap();
    assert_eq!(required.len(), 3);
    for name in ["version", "generation", "expected_revision"] {
        assert!(
            required.contains(&json!(name)),
            "missing required field: {name}"
        );
    }
    assert_eq!(
        hint["properties"]["generation"]["pattern"],
        "^[0-9a-f]{32}$"
    );
    for field in ["discovery", "degraded_sources", "harnesses"] {
        assert!(
            schemas["CaptureRuntimeStatus"]["required"]
                .as_array()
                .unwrap()
                .contains(&json!(field))
        );
    }
    let groups = &schemas["CaptureRuntimeStatus"]["properties"]["harnesses"];
    assert_eq!(groups["maxItems"], 2);
    assert_eq!(
        schemas["CaptureHarnessCoverage"]["additionalProperties"],
        false
    );
    let properties = &schemas["CaptureHarnessCoverage"]["properties"];
    assert!(properties.get("path").is_none() && properties.get("native_session_key").is_none());
    assert_eq!(properties["configured_roots"]["maxItems"], 16);
    assert_eq!(schemas["CaptureRootPath"]["maxLength"], 4096);
    assert_eq!(properties["product_version"]["maxLength"], 32);
    // POST inputs must remain bodies, never documented query parameters.
    for operation in [command, rescan] {
        assert_eq!(
            operation["x-jj-store-query-schema"],
            status["x-jj-store-query-schema"]
        );
        assert_eq!(operation["parameters"], json!([]));
    }
    let sources = &paths["/v1/local/capture/sources"]["get"];
    assert_eq!(sources["security"], json!([{}]));
    let query = &sources["x-jj-store-query-schema"];
    assert_eq!(query["additionalProperties"], false);
    assert_eq!(query["properties"]["limit"]["minimum"], 1);
    assert_eq!(query["properties"]["limit"]["maximum"], 100);
    assert_eq!(
        query["properties"]["cursor"]["pattern"],
        "^[0-9a-f]{32}:[0-9a-f]{16}$"
    );
    let page = &schemas["CaptureSourcePage"];
    assert_eq!(page["additionalProperties"], false);
    assert_eq!(page["properties"]["total"]["maximum"], 4096);
    assert_eq!(page["properties"]["sources"]["maxItems"], 100);
    let row = &schemas["CaptureSourceCoverage"];
    assert_eq!(row["additionalProperties"], false);
    assert_eq!(row["properties"]["selector"]["maxLength"], 49);
    for field in [
        "path",
        "native_session_key",
        "source_instance",
        "fingerprint",
        "proof",
        "actor",
    ] {
        assert!(row["properties"].get(field).is_none());
    }
}

#[test]
fn snapshot_source_coverage_is_optional_closed_and_has_no_admission_identity() {
    let contract = super::openapi();
    let schemas = &contract["components"]["schemas"];
    let row = &schemas["CaptureSourceCoverage"];
    assert!(row["properties"].get("snapshot_publication").is_some());
    assert!(
        !row["required"]
            .as_array()
            .unwrap()
            .contains(&json!("snapshot_publication"))
    );
    let progress = &schemas["CaptureSnapshotPublication"];
    assert_eq!(progress["additionalProperties"], false);
    assert_eq!(
        progress["properties"]["outstanding_ranges"]["maximum"],
        4096
    );
    for name in [
        "captured_sequence",
        "captured_logical_records",
        "committed_sequence",
        "receipts",
    ] {
        assert_eq!(progress["properties"][name]["type"], "integer");
        assert_eq!(progress["properties"][name]["minimum"], 0);
    }
    for name in [
        "path",
        "source",
        "native_session_key",
        "generation_digest",
        "proof",
        "request_id",
    ] {
        assert!(progress["properties"].get(name).is_none());
    }
}
