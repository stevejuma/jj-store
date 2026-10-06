---
schema: 1
id: E-M9-027
---

# E-M9-027

<!-- evidence-field:id markdown -->
## Id

E-M9-027
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

root-pinned-agent-relations
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_relations.rs
- crates/jj-store-service/src/collaboration/cursor_position.rs
- crates/jj-store-service/src/collaboration/models.rs
- crates/jj-store-service/tests/agent_relations.rs
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

- crates/jj-store-service/tests/agent_relations.rs#relation_pages_retain_root_and_reject_subject_cursor_reuse
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

The test-first OpenAPI regression failed because the AgentStore association route and relation response schemas were absent.
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
cargo test -p jj-store-service --test agent_relations --locked
```

**Notes**

The focused service regression proves two ordered relation pages retain their original audited root after a later publication and reject cursor reuse with a different typed subject.
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

The generated contract includes the additive association route, exact selector query and service-owned typed subject, link and page schemas.
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

The LocalTrusted lifecycle fixture pages relations without a bearer credential, retains the original root after publication, covers all five subject kinds and rejects contradictory selectors.
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

OpenAPI and TypeScript regenerate with the additive AgentStore association route and relation DTOs.
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
cargo fmt --all -- --check; cargo test -p jj-store-service --test agent_relations --locked; cargo test -p jj-store-http contract::tests --locked; cargo test -p jj-store-cli --test native_lifecycle http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials --locked -- --exact; cargo clippy -p jj-store-wal -p jj-store-service -p jj-store-http -p jj-store-cli --all-targets --locked -- -D warnings
```

**Notes**

Rust formatting, the focused relation regression, all nine HTTP contract tests, the exact LocalTrusted lifecycle fixture and every affected WAL/service/HTTP/CLI target pass. The new production relation module is 151 lines and its focused test is 291 lines.
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
pnpm contracts:check; pnpm check:boundaries; pnpm check:architecture; pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed specification, contract and generated files>
```

**Notes**

Generated OpenAPI and TypeScript match; 59 actual HTTP/SSE responses and all six contract tests pass. All 13 package boundaries, 198 checked web modules, 89 requirements, 12 protected files across seven canonical chapters and 61 progress tasks validate; changed files are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Checkpoint detail reads remain M9.4 work.
- ReviewStore reads and typed writes, collaboration CLI commands, receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5; real harness and cold
  native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; OpenAPI 3.1; TypeScript generated contracts; relation-page response v1; cursor token jsa1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds one JSON GET association route and generated schemas. Its process-local signed continuation binds repository, epoch, publication position, manifest revision, AgentStore root/state version, complete typed relation subject, page size and root-local page/offset. Native objects, manifest format 2, WAL, receipts and AgentStore persistence remain unchanged.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


