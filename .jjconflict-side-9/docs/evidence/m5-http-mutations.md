# Typed mutation HTTP API

The enclosing Jujutsu change exposes authenticated preview, apply and status
over Axum V1 routes. The shared native service implements New, Describe and
bookmark create/move/delete. Capability discovery lists exactly those five
commands. Reconciliation, abandon and conflict resolution remain M5 work. UI
behavior and visuals remain deferred under M4.3; the only web-file change is the
mechanically generated API TypeScript declaration.

Write authorization precedes JSON body decoding. Bodies are capped at 512 KiB,
strict DTOs reject unknown fields/commands, and the command executor shares the
same native worker permits as graph/content/file reads. Exact snapshot and
bookmark expectations, protected-history rules, current principal and revocation
are enforced through the existing service/publication guard. Actors are never
accepted from JSON. Receipts and status remain scoped to the current principal.

Preview never publishes. Apply and status return tagged committed, unknown or
not-known results with HTTP 200; not-known does not establish rejection.
Semantic rejections are explicit errors. A dispatched apply/status HTTP deadline
or a result exceeding the configured JSON response budget returns a small
unknown outcome with the original request ID. Interrupted response delivery
still requires lookup/retry using the original ID. No transport result issues a
fresh request on the caller's behalf.

The deadline fixture applies the real manifest CAS, then indefinitely withholds
its response until the HTTP deadline cancels the future. A rebuilt HTTP service
recovers that original committed receipt and repeats the exact request without
another publication. This uses fault-injecting memory storage, not RustFS/TLS
deployment qualification.

Seven focused native HTTP tests cover the three routes, all five commands,
preview zero-publication, restart/retry, stale/conflicting intent, protected
history, reader/revoked/cross-repository denial, principal-scoped status, strict
body/query validation, larger admitted input, response caps, HEAD behavior and
shared native worker admission. Compiled schema tests and generated TypeScript
acceptance cover strict input types and tagged outcomes. The contract lane
validates 34 actual HTTP/SSE responses, including nine mutation/capability
responses, against generated schemas.

Exact commands, initial failures and final lane outcomes are recorded in
E-M5-003 of the [single progress ledger](../../spec/src/milestones.md). Native
codecs, canonical intent, StoredResult V1 and persisted WAL formats are
unchanged.

Aggregate validation passed: normal checks, all 275 Rust tests (six external
tests explicitly ignored), build, formatting and all 13 specification lanes.
Existing browser acceptance passed eight cold interaction/accessibility tests
without UI behavior changes. Six schema tests and generated TypeScript checks
passed.
