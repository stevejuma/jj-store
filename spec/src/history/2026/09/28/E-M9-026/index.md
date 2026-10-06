---
schema: 1
id: E-M9-026
---

# E-M9-026

<!-- evidence-field:id markdown -->
## Id

E-M9-026
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

bounded-root-pinned-agent-transcripts
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_cursors.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
- crates/jj-store-service/src/collaboration/agent_transcripts.rs
- crates/jj-store-service/src/collaboration/cursor_position.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-wal/src/publication.rs
- schemas/openapi.json
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/agent_sessions.rs#transcript_pages_retain_exact_root_and_validate_stream_cursor
- crates/jj-store-http/src/contract/tests.rs#collaboration_contract_exposes_root_pinned_agent_session_pages
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs#local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials
<!-- /evidence-field -->

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-http contract::tests::collaboration_contract_exposes_root_pinned_agent_session_pages --locked -- --exact
```

**Notes**

Test-first OpenAPI regression failed because the bounded transcript path and response schema were absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-service --test agent_sessions --locked
```

**Notes**

All three focused service tests pass, including two transcript pages pinned across a later publication and explicit rejection of a query-mismatched cursor.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-http contract::tests::collaboration_contract_exposes_root_pinned_agent_session_pages --locked -- --exact
```

**Notes**

The contract exposes full stable session, manifest and source identities with the required bounded transcript page fields.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked -- --exact
```

**Notes**

The registered LocalTrusted handler resolves transcript descriptors and serves both pinned transcript pages without bearer credentials.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:generate
```

**Notes**

The first generation attempt used the blocked system C compiler and failed at the unaccepted Xcode license before contract generation.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
CC=<pinned-clang> CXX=<pinned-clang++> CMAKE=<qualified-cmake> BOOST_ROOT=<qualified-boost> pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript regenerate with the additive bounded transcript route and service-owned DTOs.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check; cargo test -p jj-store-http contract::tests --locked
```

**Notes**

Rust formatting and all nine focused HTTP contract tests pass after route registration and generated-schema updates.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Every affected WAL, service, HTTP and CLI target passes Clippy. New production modules are 96 and 242 lines; focused tests remain below 500 lines.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm contracts:check
```

**Notes**

Generated OpenAPI and TypeScript match; 57 actual HTTP/SSE responses, including detail and two transcript pages, conform and all six schema/client tests pass.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries; pnpm check:architecture; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed specification and generated contract files>
```

**Notes**

All 13 package boundaries, 198 checked web modules, 89 requirements, 15 protected files across seven canonical chapters and 61 progress tasks validate; changed prose and generated contracts are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Relation and checkpoint reads remain M9.4 work.
- ReviewStore reads and typed writes, collaboration CLI commands, receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5; real harness and cold
  native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; OpenAPI 3.1; TypeScript generated contracts; transcript-page response v1; cursor token jsa1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds one JSON GET transcript route and generated schemas. Its process-local signed continuation binds repository, epoch, publication position, manifest revision, AgentStore root/state version, stable session, immutable transcript manifest, source instance/generation, page size, byte ceiling and last sequence. Native objects, manifest format 2, WAL, receipts and AgentStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


