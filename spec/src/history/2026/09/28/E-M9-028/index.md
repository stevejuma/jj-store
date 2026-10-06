---
schema: 1
id: E-M9-028
---

# E-M9-028

<!-- evidence-field:id markdown -->
## Id

E-M9-028
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

root-pinned-agent-checkpoints
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_checkpoints.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_checkpoints.rs
- crates/jj-store-service/src/collaboration/checkpoint_models.rs
- crates/jj-store-service/src/collaboration/cursor_position.rs
- crates/jj-store-service/tests/agent_checkpoints.rs
- crates/jj-store-wal/src/publication.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/cli.md
- spec/src/collaboration.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/mutations.md
- spec/src/publication.md
- spec/src/spec-governance.md
- spec/src/verification.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-service/tests/agent_checkpoints.rs#checkpoint_pages_and_versions_retain_exact_root
- crates/jj-store-http/src/contract/tests.rs#collaboration_contract_exposes_root_pinned_agent_session_pages
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration_checkpoints.rs#local_trusted_http_pages_current_and_historical_agent_checkpoints
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

The test-first OpenAPI regression failed because checkpoint collection, current-detail and exact-version routes were absent.
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
cargo test -p jj-store-service --test agent_checkpoints --locked
```

**Notes**

The focused service regression proves catalog continuity across a later publication, current selection lookup, historical version traversal and rejection of a foreign checkpoint version.
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

The generated contract includes all three additive checkpoint paths and service-owned page, detail, summary and code-anchor DTOs.
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
cargo test -p jj-store-cli --test native_lifecycle http_collaboration_checkpoint_acceptance::local_trusted_http_pages_current_and_historical_agent_checkpoints --locked -- --exact
```

**Notes**

The LocalTrusted registered handlers page checkpoints without bearer credentials, retain their root after publication, resolve current and historical versions and return 404 for another checkpoint chain.
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
CC=<pinned-clang> CXX=<pinned-clang++> CMAKE=<qualified-cmake> BOOST_ROOT=<qualified-boost> pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript regenerate with the additive AgentStore checkpoint routes and DTOs.
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
cargo fmt --all -- --check; cargo test -p jj-store-service --test agent_sessions --locked; cargo test -p jj-store-service --test agent_relations --locked; cargo test -p jj-store-service --test agent_checkpoints --locked; cargo test -p jj-store-http contract::tests --locked; cargo test -p jj-store-cli --test native_lifecycle http_collaboration_checkpoint_acceptance::local_trusted_http_pages_current_and_historical_agent_checkpoints --locked -- --exact; cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Rust formatting, all five affected AgentStore service tests, all nine HTTP contract tests, the exact LocalTrusted checkpoint fixture and every affected WAL/service/HTTP/CLI target pass. New production modules are 188 and 127 lines; focused tests are 324 and 294 lines.
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
pnpm contracts:check; pnpm check:boundaries; pnpm check:architecture; pnpm spec:validate
```

**Notes**

Generated OpenAPI and TypeScript match; 63 actual HTTP/SSE responses and all six contract tests pass. All 13 package boundaries, 198 checked web modules and 89 requirements validate.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same-change gate required spec-governance coverage because the generated-contract capture runner gained a separate exact checkpoint fixture.
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
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed governance and milestone files>
```

**Notes**

All 89 requirements, 15 protected files across eight canonical chapters and 61 progress tasks validate; the changed governance and milestone files are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- ReviewStore reads and typed writes, collaboration CLI commands, receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5.
- Real harness and cold native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; OpenAPI 3.1; TypeScript generated contracts; checkpoint page/detail response v1; cursor token jsa1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds three JSON GET checkpoint routes and generated schemas. Collection cursors bind the exact AgentStore root, page size and checkpoint radix key. Current detail resolves a stable 16-byte checkpoint ID; exact-version detail accepts a full 32-byte immutable object ID only within the selected checkpoint's bounded supersedes chain. Native objects, manifest format 2, WAL, receipts and AgentStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


