# Rust formatting

The pinned Rust toolchain includes rustfmt. The root rustfmt.toml fixes Rust and
formatting editions to 2024 and uses Unix line endings. `pnpm format:rust`
formats every workspace crate and the separately locked embedded RustFS runtime,
including examples and tests; `pnpm format:rust:check` checks those files
without changing them. `pnpm format` and `pnpm format:check` include Rust
alongside Prettier. Both `pnpm check` and `pnpm spec:check` reject Rust
formatting drift.

The RPC layer may consume the owned object streaming interface. Its native JJ
trait adapters remain in the CLI crate, and direct upstream JJ dependencies in
RPC are rejected by the existing boundary checker.

The boundary checker treats `jj-agent` and `jj-review` as owned layers even
though their names do not use the `jj-store-` prefix. `jj-agent` may depend on
the neutral model layer. `jj-review` may additionally depend on `jj-agent` for
immutable activity-object references. Neither can depend on WAL, service, native
JJ, transport or presentation layers. WAL may depend on the domain crates to
validate typed roots and immutable objects before shared publication. The
boundary checker admits the exact `jj-store-wal` to `jj-review` dependency and
continues to reject review dependencies on WAL or higher layers.

Capture tooling owns the pinned MongoDB Kingfisher Rust integration directly in
the CLI crate. The local pnpm hook has no agent-redaction package exception, and
its focused tests continue to prove Design Core sibling resolution and the
no-sibling fallback. The in-process sanitizer module stays below the production
line budget and returns only validated sanitized payloads with the fixed policy
identity or a fixed omission state.

Evidence entries can explicitly identify removed files and retired named tests.
The progress checker accepts historical references only when a later entry
records the exact removal, verifies removed files are absent and keeps every
other historical evidence path and named regression live. Its Jujutsu reads use
a fixed 16 MiB output ceiling so the single growing ledger can be validated
without an unbounded child-process buffer.

Server composition has an explicit layer in the boundary checker. Regression
coverage rejects dependencies on CLI presentation and direct upstream JJ types,
while permitting the owned native/service/RPC composition interfaces.

The standalone server storage lane qualifies TLS using a generated certificate
and disposable credentials. Retained reports state transport and topology scope;
previous plaintext-loopback reports remain unchanged. Protocol-rejection
fixtures assert a local transport failure rather than timing-dependent gRPC
status codes. No generated private keys are retained in reports or checkouts.

Query resource measurements run through `scripts/qualify-query-resources.py`
against an explicitly built release fixture binary. Reports retain the actual
process result and scope, including resource/admission failures. Ordinary green
unit checks do not imply a successful release resource run.

The resource harness supports separate graph and content fixture processes.
Content evidence includes positive boundary inputs and explicit oversized-input
rejections, preserving checksums, exact commands, CPU, elapsed time and RSS.

The predicate resource scenario runs the explicitly compiled release native
lifecycle test binary under an independent process timeout. It requires each
named admission/parity test and records actual process CPU/RSS separately from
compilation. A hung debug fixture must be diagnosed and recorded, not counted as
a passing deadline test.

Large graph resource qualification records cold admission and explicit index
preparation separately from prepared/warm graph queries. The harness requires a
successful preparation report for both linear and wide-head fixtures; an
explicit rebuild-required response alone is not a passing query or completed
task.

The server may depend on the HTTP transport for listener composition. HTTP keeps
its existing service/model dependency boundary and consumes only owned
contracts; it cannot depend directly on storage, CLI presentation or upstream JJ
libraries.

Generated OpenAPI and TypeScript contracts have a drift check in both normal and
specification checks. The generator uses the compiled HTTP route inventory;
request and response schema modes preserve Serde's optionality rules. Contract
tests validate the OpenAPI document and representative serialized native DTOs.
The M4.10 capture also executes the registered reference, recursive-path,
operation-change and diff-summary handlers. It proves repository authorization,
exact snapshot pinning, signed query-bound forward/backward continuations and
zero publication rather than validating generated shapes alone.

The generated-contract lane also executes the registered LocalTrusted AgentStore
session, transcript, relation and checkpoint handlers plus ReviewStore
collection, detail, command and durable-status handlers. Separate exact fixtures
capture collection/detail responses, preserve one root snapshot across resumed
pages, validate current plus historical immutable checkpoint detail and replay a
review receipt after transport reconstruction against generated OpenAPI. Every
fixture must report one executed test. A zero-test filter, schema-only assertion
or uncaptured response cannot count as collaboration transport evidence.

Browser checks include Svelte/TypeScript analysis, focused client tests and cold
Playwright interactions/accessibility against built assets. Functional and
accessibility failures remain blocking; visual review is separate and remains
visual-pending. Browser sources and tooling require same-change ledger evidence.

User-requested scope deferrals must retain original requirements and evidence.
The 27 September 2026 browsing resumption covers M4.9–M4.14; M4.15–M4.17
subsequently connect search, core repository mutations and native conflict
editing. Backend tasks may depend on completed API foundations independently of
browser progress.

Contract checks validate both read/SSE and typed mutation responses captured
from real registered handlers. Mutation request schemas and generated TypeScript
narrowing reject untyped commands and caller-selected principals. Regenerating
API declarations does not close any deferred browser feature or deployment gate.

The contract lane captures typed abandon preview/apply/no-change responses as
well as foundational mutations and reads. New nonempty optional report fields
must conform to generated schemas while older V1 report bytes remain unchanged.

Contract capture includes atomic conflict preview/apply and rejection responses.
The generated command union exposes only implemented resolution actions.
Conflict-editor acceptance separately covers descriptor loading, complete
multi-path selection, stale fingerprints and the real native workflow.

The generated-contract lane also captures actual binary-upload results and
uploaded-file mutation responses. Upload IDs have canonical-width schema checks
and the staging record has an independently calculated digest fixture. Browser
tests additionally require the upload query binding and atomic command body.

Dependency checks permit service-to-adapter authorization wiring while retaining
the adapter-to-service/native/UI prohibition. Adapter runner and fault fixtures
join normal Rust and specification validation; their tests cannot substitute for
real storage deployment or Tantivy-specific durability qualification.

Generated-contract checks capture registered search page, status and freshness
error responses through a real native fixture. The capture must execute its
exact named test and produce samples; a zero-test invocation cannot count as
contract evidence. Generated API declarations remain permitted during UI
deferral.

Server composition may depend on owned adapter and Tantivy interfaces. The layer
checker continues to reject upstream JJ imports in server code and all search or
transport dependencies in authoritative storage/publication layers.
