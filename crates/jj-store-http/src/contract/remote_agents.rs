use super::*;

impl Document {
    pub(crate) fn authenticated_agent_endpoint<Q: JsonSchema, R: JsonSchema>(
        &mut self,
        method: &str,
        path: &str,
        id: &str,
    ) {
        self.endpoint::<Q, R>(method, path, id, Reply::Json(200));
        let item = self.paths.get_mut(&format!("/v1{path}")).unwrap();
        for operation in item.as_object_mut().unwrap().values_mut() {
            operation["security"] = json!([{"repositoryBearer":[]}]);
            operation["tags"] = json!(["agents"]);
            operation["description"] = json!(
                "Authenticated execution only. Requires operator-enabled agent admission for the token's principal and independent current native repository grants. Requests, sessions, outcomes and streams are caller-owned. Unknown query fields are rejected. Retain the original request identity across transport failure; reconnect never replays commands. Installation configuration remains LocalTrusted only."
            );
        }
    }

    pub(crate) fn authenticated_agent_events<Q: JsonSchema, R: JsonSchema>(
        &mut self,
        path: &str,
        id: &str,
    ) {
        self.authenticated_agent_endpoint::<Q, R>("get", path, id);
        let schema = self.output.subschema_for::<R>().to_value();
        let operation = self
            .paths
            .get_mut(&format!("/v1{path}"))
            .unwrap()
            .get_mut("get")
            .unwrap();
        operation["responses"]["200"]["description"] = json!(
            "Bounded caller-owned execution activity. Reconnect using the last delivered integer sequence in after. Admission and current repository grants are checked throughout delivery; revoked streams terminate. Publication flags report AgentStore commitment separately. No command replay."
        );
        operation["responses"]["200"]["content"] = json!({"text/event-stream":{
            "schema":{"type":"string"}, "x-sse-data-schema":schema,
            "x-sse-events":{"agent-activity":"activity"},
            "x-sse-id-schema":{"type":"string", "pattern":"^(0|[1-9][0-9]*)$"}
        }});
    }
}
