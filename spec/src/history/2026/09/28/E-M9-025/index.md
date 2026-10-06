---
schema: 1
id: E-M9-025
---

# E-M9-025

<!-- evidence-field:id markdown -->
## Id

E-M9-025
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

root-pinned-agent-session-detail
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_details.rs
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

- crates/jj-store-service/tests/agent_sessions.rs#session_detail_resolves_current_stable_identity_and_exact_links
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

Test-first OpenAPI regression failed because the stable-session detail path and response schema were absent.
<!-- /run -->

<!-- run -->
### Run 2

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

The first implementation compile reached contract generation and rejected the undocumented session path identity before its exact schema was registered.
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

The contract now exposes the canonical stable-session path and required root snapshot, summary and immutable-link fields.
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
cargo test -p jj-store-service --test agent_sessions --locked
```

**Notes**

Both focused service tests pass: root-pinned collection continuity and current-root stable-session detail with exact immutable links and explicit not-found behavior.
<!-- /run -->

<!-- run -->
### Run 5

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

The registered LocalTrusted handler lists two pages, resolves one detail without credentials and returns 404 for a missing canonical session ID.
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
pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript regenerate with the additive session-detail route and owned DTOs.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 5
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same-change gate required api, cli, compatibility, mutations and publication updates before the slice could close.
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
cargo fmt --all -- --check; cargo test -p jj-store-service --test agent_sessions --locked; cargo test -p jj-store-http contract::tests --locked; cargo test -p jj-store-cli --test native_lifecycle http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked -- --exact; cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Rust formatting, both service regressions, all nine HTTP contract tests, the one LocalTrusted lifecycle fixture and every affected WAL/service/HTTP/CLI target pass. Production collaboration modules are 45, 112, 235 and 290 lines; focused tests are 169 and 159.
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

Generated OpenAPI and TypeScript match; 54 actual HTTP/SSE responses conform and all six schema/client tests pass.
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
pnpm check:boundaries; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed specification, contract and generated files>
```

**Notes**

All 13 package boundaries, 89 requirements, 13 protected files across seven canonical chapters and 61 progress tasks validate; changed prose and generated contracts are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Transcript, relation and checkpoint reads remain M9.4 work.
- ReviewStore reads and typed writes, collaboration CLI commands, receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5; real harness and cold
  native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; OpenAPI 3.1; TypeScript generated contracts; session-detail response v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds one JSON GET detail route and generated schemas. The stable path identity is a full 16-byte AgentSessionId; returned immutable evidence, event, transcript and tool links retain full 32-byte AgentObjectId values. Native objects, manifest format 2, WAL, receipts and AgentStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


