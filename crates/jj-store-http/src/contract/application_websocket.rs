use super::*;

impl Document {
    pub(crate) fn application_websocket<Q: JsonSchema, R: JsonSchema>(&mut self) {
        let hello = self.input.subschema_for::<Q>().to_value();
        let frame = self.output.subschema_for::<R>().to_value();
        self.paths.insert("/v1/events/ws".into(), json!({"get": {
            "operationId": "application_events_websocket",
            "tags": ["reads"],
            "description": "Same-origin WebSocket freshness. Exact Host and mandatory Origin, WebSocket Fetch Metadata, and jj-store-events-v1 subprotocol replace the HTTP protocol header on this route only. No queries, cookies or Authorization headers. Authenticate in the first JSON message within five seconds (8192 bytes maximum); credentials remain memory-only. No frames precede current authorization. Version-1 event frames retain the SSE scope, cursor, replay/reset and per-delivery authorization. Heartbeat JSON every 15 seconds; finite admission 300 seconds. Close 1000 renews, 4403 denies, 4400 rejects protocol, 1013 retries. Client messages after hello cannot issue commands.",
            "parameters": [
                {"in":"header", "name":"Origin", "required":true, "schema":{"type":"string"}},
                {"in":"header", "name":"Sec-WebSocket-Protocol", "required":true, "schema":{"const":"jj-store-events-v1"}}
            ],
            "security": [{}],
            "x-websocket-client-schema": hello,
            "x-websocket-event-schema": frame,
            "x-websocket-heartbeat-schema": {"type":"object", "required":["version","type"], "properties":{"version":{"const":1},"type":{"const":"heartbeat"}},"additionalProperties":false},
            "responses": {
                "101": {"description":"WebSocket upgraded; first-message authorization still required."},
                "400": {"description":"Invalid upgrade."},
                "403": {"description":"Origin, authority or protocol admission denied."},
                "503": {"description":"Bounded event admission exhausted."}
            }
        }}));
    }
}
