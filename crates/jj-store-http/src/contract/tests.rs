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
fn contract_inventory_covers_security_methods_media_and_task_bodies() {
    let doc = openapi();
    assert_eq!(doc["security"], json!([{"repositoryBearer":[]}]));
    let paths = doc["paths"].as_object().unwrap();
    assert_eq!(paths.len(), 17);
    let mut ids = std::collections::BTreeSet::new();
    for path in paths.values() {
        for (method, operation) in path.as_object().unwrap() {
            assert!(ids.insert(operation["operationId"].as_str().unwrap()));
            assert_eq!(operation["parameters"][0]["name"], "repo");
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
