//! Offline OpenAPI generation from the executable route inventory and owned DTOs.
use crate::{dto::ErrorBody, m, routes};
use schemars::{JsonSchema, SchemaGenerator, generate::SchemaSettings};
use serde_json::{Map, Value, json};

pub(crate) enum Reply {
    Json(u16),
    Binary,
    Events,
}
pub(crate) struct Document {
    output: SchemaGenerator,
    input: SchemaGenerator,
    paths: Map<String, Value>,
}
impl Document {
    fn new() -> Self {
        let mut settings = SchemaSettings::draft2020_12().for_serialize();
        settings.definitions_path = "/components/schemas".into();
        // Keep sibling constraints on references visible to OpenAPI client generators.
        settings
            .transforms
            .push(Box::<schemars::transform::RemoveRefSiblings>::default());
        let mut input = SchemaSettings::draft2020_12().for_deserialize();
        // HTTP request DTOs are finite records. Inline them so a type used in
        // both directions cannot accidentally reuse its opposite contract.
        input.inline_subschemas = true;
        Self {
            output: settings.into_generator(),
            input: input.into_generator(),
            paths: Map::new(),
        }
    }
    fn path_parameter(&mut self, name: &str) -> Value {
        let schema = match name {
            "repo" => self.output.subschema_for::<m::RepositoryId>().to_value(),
            "commit" | "id" => self.output.subschema_for::<m::CommitId>().to_value(),
            "request_id" => self.output.subschema_for::<m::RequestId>().to_value(),
            "file" => self.output.subschema_for::<m::FileId>().to_value(),
            "operation" => self.output.subschema_for::<m::OperationId>().to_value(),
            "kind" => {
                json!({"type":"string", "enum":["commit","tree","file","symlink","copy","view","operation"]})
            }
            "task" => {
                json!({"type":"string", "description":"Opaque process-local task ID; grants no access."})
            }
            _ => panic!("undocumented path parameter: {name}"),
        };
        json!({"name":name,"in":"path","required":true,"schema":schema})
    }
    pub(crate) fn endpoint<Q: JsonSchema, R: JsonSchema>(
        &mut self,
        method: &str,
        path: &str,
        id: &str,
        reply: Reply,
    ) {
        let query = self.input.subschema_for::<Q>().to_value();
        assert_eq!(
            query["additionalProperties"], false,
            "strict query DTO required"
        );
        let mut parameters = vec![];
        for part in path.split('/') {
            if let Some(name) = part.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                parameters.push(self.path_parameter(name));
            }
        }
        assert_eq!(query["type"], "object", "query object");
        for (name, schema) in query["properties"].as_object().into_iter().flatten() {
            let required = query["required"]
                .as_array()
                .is_some_and(|fields| fields.contains(&json!(name)));
            parameters.push(json!({"name":name,"in":"query","required":required,
                "schema":non_null(schema.clone()), "style":"form", "explode":true}));
        }
        let error = self.output.subschema_for::<ErrorBody>();
        let mut responses = Map::new();
        for (status, description) in [
            (
                400,
                "Invalid request, unknown query field or invalid cursor.",
            ),
            (401, "Missing repository Bearer credential."),
            (
                403,
                "Access denied, including revoked credentials or disallowed Origin.",
            ),
            (
                404,
                "Object not committed in the selected snapshot or unknown route.",
            ),
            (405, "Unsupported method."),
            (408, "Native query canceled."),
            (
                409,
                "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
            ),
            (410, "Index task expired or was lost on server restart."),
            (413, "Request or response resource limit exceeded."),
            (415, "Unsupported request media type or content encoding."),
            (503, "Busy, unavailable, corrupt or incomplete history."),
            (504, "Request or response deadline exceeded."),
        ] {
            responses.insert(
                status.to_string(),
                json!({"description":description,
                "content":{"application/json":{"schema":error}}}),
            );
        }
        let success = match reply {
            Reply::Json(status) => {
                responses.insert(
                    status.to_string(),
                    json!({"description":"Authorized result at the reported snapshot.",
                    "content":{"application/json":{"schema":self.output.subschema_for::<R>()}}}),
                );
                status
            }
            Reply::Events => {
                parameters.push(json!({"name":"Last-Event-ID", "in":"header", "required":false,
                    "description":"Must agree with after when both are supplied.", "schema":event_id_schema()}));
                for parameter in &mut parameters {
                    if parameter["name"] == "after" {
                        parameter["schema"] = event_id_schema();
                    }
                }
                responses.insert("200".into(), json!({"description":"Bounded SSE stream. JSON data appears inside ready, repository-changed and reset frames. Authorized heartbeat comments do not advance IDs. Reset closes the stream. Resume the last fully parsed ID after EOF; post-header failures terminate the stream.",
                    "content":{"text/event-stream":{"schema":{"type":"string"},
                        "x-sse-data-schema":self.output.subschema_for::<R>(),
                        "x-sse-events":{"ready":"ready", "repository-changed":"changed", "reset":"reset"},
                        "x-sse-id-schema":event_id_schema()}}}));
                200
            }
            Reply::Binary => {
                parameters.push(json!({"name":"Range", "in":"header", "required":false,
                    "schema":{"type":"string"}, "description":"One bytes=start-end, bytes=start- or bytes=-count range. Bounds are clamped to EOF after verifying the complete native file."}));
                for status in [200, 206] {
                    responses.insert(status.to_string(), json!({"description":"Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery.",
                        "content":{"application/octet-stream":{"schema":{"type":"string","format":"binary"}}},
                        "headers":self.file_headers(status == 206)}));
                }
                responses.insert("416".into(), json!({"description":"Invalid or unsatisfiable byte range.",
                    "headers":{"Content-Range":{"schema":{"type":"string"},"description":"bytes */<full length>"}},
                    "content":{"application/json":{"schema":error}}}));
                200
            }
        };
        let mutation = path.contains("/mutations");
        let mut operation = json!({"operationId":id, "tags":["reads"],
            "description":"Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
            "parameters":parameters, "responses":responses, "x-jj-store-query-schema":query});
        if mutation {
            operation["tags"] = json!(["mutations"]);
            operation["description"] = json!(
                "Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred."
            );
            operation["responses"]["409"]["description"] = json!(
                "stale_snapshot, stale_bookmark, stale_conflict, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt."
            );
        }
        if path.ends_with("/conflict-uploads") {
            operation["tags"] = json!(["mutations"]);
            operation["description"] = json!(
                "Requires current repository write permission before body consumption. Binary staging only; never publishes history. Binding must name the exact current single-head snapshot and a committed target. Repeating identical bytes/binding returns the same scoped handle. Deadline or unavailable responses permit safe re-upload; no mutation receipt is created. Upload limits are advertised by capabilities."
            );
            operation["requestBody"] = json!({"required":true,"content":{"application/octet-stream":{"schema":{"type":"string","format":"binary"}}}});
        }
        let mut item = json!({method:operation});
        if method == "get" {
            let mut head = operation;
            head["operationId"] = json!(format!("{id}_head"));
            head["description"] = json!(
                "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned."
            );
            for response in head["responses"].as_object_mut().unwrap().values_mut() {
                response.as_object_mut().unwrap().remove("content");
            }
            assert!(head["responses"].get(success.to_string()).is_some());
            item["head"] = head;
        }
        assert!(
            self.paths.insert(format!("/v1{path}"), item).is_none(),
            "duplicate route"
        );
    }
    pub(crate) fn body<T: JsonSchema>(&mut self, path: &str, method: &str) {
        let operation = self.paths.get_mut(path).unwrap().get_mut(method).unwrap();
        operation["requestBody"] = json!({"required":true,
            "content":{"application/json":{"schema":self.input.subschema_for::<T>()}}});
    }
    fn file_headers(&mut self, partial: bool) -> Value {
        let mut headers = json!({
            "Content-Disposition":{"schema":{"type":"string"}, "description":"attachment; filename=\"<file ID>.bin\""},
            "Accept-Ranges":{"schema":{"const":"bytes"}},
            "Content-Length":{"schema":{"type":"string", "pattern":"^[0-9]+$"}},
            "x-jj-store-repository":{"schema":self.output.subschema_for::<m::RepositoryId>()},
            "x-jj-store-epoch":{"schema":self.output.subschema_for::<m::EpochId>()},
            "x-jj-store-operation":{"schema":self.output.subschema_for::<m::OperationId>()},
            "x-jj-store-position":{"schema":{"type":"string", "pattern":"^[0-9]+$"}},
            "x-jj-store-revision":{"schema":{"type":"string", "pattern":"^[0-9]+$"}}
        });
        if partial {
            headers["Content-Range"] = json!({"schema":{"type":"string"},"description":"bytes <start>-<inclusive end>/<full length>"});
        }
        headers
    }
    fn finish(mut self) -> Value {
        assert!(
            self.input.definitions().is_empty(),
            "recursive input schemas require explicit separate naming"
        );
        json!({"openapi":"3.1.0", "jsonSchemaDialect":"https://json-schema.org/draft/2020-12/schema",
            "info":{"title":"jj-store application API", "version":"1.0.0", "license":{"name":"Apache-2.0","identifier":"Apache-2.0"},
                "description":"Native JJ read and typed mutation contracts. Same-origin Bearer authentication; tokens never appear in paths, query strings or cookies. Shared deployments require TLS. Numeric JSON values retain Rust widths: clients must handle precision beyond JavaScript safe integers explicitly."},
            "servers":[{"url":"/"}], "tags":[{"name":"reads"},{"name":"mutations"}],
            "security":[{"repositoryBearer":[]}],
            "paths":self.paths,
            "components":{"securitySchemes":{"repositoryBearer":{"type":"http","scheme":"bearer","description":"Revocable repository-scoped token; read routes require read permission, mutation routes require write permission."}},
                "schemas":self.output.take_definitions(true)}})
    }
}
fn event_id_schema() -> Value {
    json!({"type":"string","maxLength":128,"pattern":"^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
        "description":"Committed replay cursor, not a credential. Position must fit u64."})
}
/// Query parameters can be omitted but cannot encode a JSON null value.
fn non_null(mut value: Value) -> Value {
    if let Some(types) = value.get_mut("type").and_then(Value::as_array_mut) {
        types.retain(|t| t != "null");
        if types.len() == 1 {
            value["type"] = types[0].clone();
        }
    }
    if let Some(options) = value.get_mut("anyOf").and_then(Value::as_array_mut) {
        options.retain(|v| v.get("type") != Some(&json!("null")));
        if options.len() == 1 {
            let mut selected = options[0].as_object().unwrap().clone();
            value.as_object_mut().unwrap().remove("anyOf");
            selected.extend(value.as_object().unwrap().clone());
            value = json!(selected);
        }
    }
    value
}
/// Deterministic, credential-free generation. No listener or object store is opened.
pub fn openapi() -> Value {
    let mut document = Document::new();
    routes::document(&mut document);
    document.finish()
}

#[cfg(test)]
mod tests;
