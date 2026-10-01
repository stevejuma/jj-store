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
            "session" => self
                .output
                .subschema_for::<jj_store_service::collaboration::AgentSessionIdentity>()
                .to_value(),
            "transcript" => self
                .output
                .subschema_for::<jj_store_service::collaboration::AgentTranscriptIdentity>()
                .to_value(),
            "checkpoint" => self
                .output
                .subschema_for::<jj_store_service::collaboration::AgentCheckpointIdentity>()
                .to_value(),
            "version" => self
                .output
                .subschema_for::<jj_store_service::collaboration::AgentCheckpointVersionIdentity>()
                .to_value(),
            "thread" => self
                .output
                .subschema_for::<jj_store_service::collaboration::ReviewThreadIdentity>()
                .to_value(),
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
            "parameters":parameters, "responses":responses, "x-jj-store-query-schema":query,
            "security":[{"repositoryBearer":[]},{}]});
        if mutation {
            operation["tags"] = json!(["mutations"]);
            operation["description"] = json!(
                "Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred."
            );
            operation["responses"]["409"]["description"] = json!(
                "stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt."
            );
        }
        if path.ends_with("/search") || path.ends_with("/search/status") {
            operation["tags"] = json!(["search"]);
            operation["description"] = json!(
                "Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data."
            );
            operation["responses"]["501"] = json!({"description":"Search is not configured for this repository.", "content":{"application/json":{"schema":error}}});
            operation["responses"]["409"]["description"] = json!(
                "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed."
            );
            operation["responses"]["503"]["description"] =
                json!("search_not_ready, busy, unavailable, corrupt_index, or incomplete history.");
        }
        if path.contains("/work") {
            operation["tags"] = json!(["work"]);
            operation["description"] = json!(
                "Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported."
            );
            operation["responses"]["409"]["description"] =
                json!("stale_work, cursor_reset, intent_conflict or work_replication_unsupported.");
        } else if path.contains("/reviews/commands") {
            operation["tags"] = json!(["collaboration"]);
            operation["description"] = json!(
                "Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication."
            );
            operation["responses"]["409"]["description"] = json!(
                "stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt."
            );
        } else if path.contains("/agent/") || path.contains("/reviews/") {
            operation["tags"] = json!(["collaboration"]);
            operation["description"] = json!(
                "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state."
            );
            operation["responses"]["409"]["description"] = json!(
                "cursor_reset after expiry, process restart, or repository epoch replacement."
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
        let existing = self
            .paths
            .entry(format!("/v1{path}"))
            .or_insert_with(|| json!({}));
        for (method, operation) in item.as_object().unwrap() {
            assert!(
                existing
                    .as_object_mut()
                    .unwrap()
                    .insert(method.clone(), operation.clone())
                    .is_none(),
                "duplicate route method"
            );
        }
    }
    pub(crate) fn public_endpoint<Q: JsonSchema, R: JsonSchema>(
        &mut self,
        method: &str,
        path: &str,
        id: &str,
        reply: Reply,
    ) {
        self.endpoint::<Q, R>(method, path, id, reply);
        let item = self.paths.get_mut(&format!("/v1{path}")).unwrap();
        for operation in item.as_object_mut().unwrap().values_mut() {
            operation["security"] = json!([{}]);
            operation["description"] = json!(
                "Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog."
            );
        }
    }
    pub(crate) fn local_endpoint<Q: JsonSchema, R: JsonSchema>(
        &mut self,
        method: &str,
        path: &str,
        id: &str,
    ) {
        self.endpoint::<Q, R>(method, path, id, Reply::Json(200));
        let item = self.paths.get_mut(&format!("/v1{path}")).unwrap();
        for verb in if method == "get" {
            vec!["get", "head"]
        } else {
            vec![method]
        } {
            let operation = item.get_mut(verb).unwrap();
            operation["security"] = json!([{}]);
            operation["tags"] = json!(["local"]);
            operation["description"] = json!(
                "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay."
            );
        }
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
                "description":"Native JJ read and typed mutation contracts. Access is deployment-dependent: authenticated deployments require repository Bearer credentials, while LocalTrusted installations admit only protected literal-loopback requests and bind an installation principal. Tokens never appear in paths, query strings or cookies. Shared deployments require TLS. Numeric JSON values retain Rust widths: clients must handle precision beyond JavaScript safe integers explicitly."},
            "servers":[{"url":"/"}], "tags":[{"name":"reads"},{"name":"mutations"},{"name":"search"},{"name":"collaboration"}],
            "security":[],
            "x-jj-store-access-policies":["authenticated","local_trusted"],
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
    crate::local_browser::document(&mut document);
    document.finish()
}

#[cfg(test)]
#[path = "contract/review_tests.rs"]
mod review_tests;
#[cfg(test)]
mod tests;
