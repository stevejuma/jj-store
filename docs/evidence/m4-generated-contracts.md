# M4 generated read contracts

The HTTP route declaration generates both Axum registration and OpenAPI paths.
The document covers the 17 registered paths, implicit HEAD methods, Bearer
security, query defaults, required index-task bodies, snapshot envelopes,
errors, binary ranges and SSE framing. The same compiled Rust DTOs drive JSON
serialization and schema generation. Input and output generators use their
respective Serde contracts, with stable schema names for generic envelopes.

`contract::tests` checks ID widths, tuple order, native-versus-graph metadata,
required nullable response fields, optional request fields, media and route
inventory. `scripts/contracts.test.mjs` validates OpenAPI without external
references and checks positive/negative metadata examples against the frozen V1
envelopes. `scripts/contracts.types.ts` exercises generated unions, tuple order,
required time context and predecessor nullability as a TypeScript consumer.

The `http_contract_capture_matches_registered_routes` native lifecycle fixture
executes real authenticated Axum handlers against the fault-store/native stack.
It exports disposable response bodies only when explicitly requested by the
contract checker. This exercises all seven native object kinds, three graphs,
detail/content responses, task start/status/cancel, ready/reset SSE data and
error bodies. The checker validates those actual serializations against the
generated document, and the fixture verifies zero manifest changes.

These checks qualify the declared contract and current serialization. Existing
HTTP, SSE, codec, authorization and deployment tests remain required for
semantic and storage correctness. No browser interaction or new real RustFS
deployment pass is claimed by this report. The single progress ledger records
actual commands, failures, results, remaining gaps and the enclosing change.
