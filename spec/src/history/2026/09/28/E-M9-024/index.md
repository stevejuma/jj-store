---
schema: 1
id: E-M9-024
---

# E-M9-024

<!-- evidence-field:id markdown -->
## Id

E-M9-024
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

root-pinned-agent-session-api
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-cli/tests/native_lifecycle.rs
- crates/jj-store-cli/tests/native_lifecycle/http_collaboration.rs
- crates/jj-store-http/src/collaboration.rs
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-service/src/collaboration.rs
- crates/jj-store-service/src/collaboration/agent_cursors.rs
- crates/jj-store-service/src/lib.rs
- crates/jj-store-service/tests/agent_sessions.rs
- crates/jj-store-wal/src/publication/agent_catalog.rs
- crates/jj-store-wal/src/publication.rs
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/api.md
- spec/src/architecture.md
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

- crates/jj-store-service/tests/agent_sessions.rs#session_pages_retain_one_agent_root_across_later_publication
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
cargo test -p jj-store-http contract::tests::collaboration_contract_exposes_root_pinned_agent_session_pages -- --exact
```

**Notes**

Test-first OpenAPI regression failed because /v1/repos/{repo}/agent/sessions was absent.
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
cargo test -p jj-store-service --test agent_sessions
```

**Notes**

The service pager retains one audited AgentStore root across a later publication and rejects page-size reuse and cursor tampering.
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
cargo test -p jj-store-http contract::tests::collaboration_contract_exposes_root_pinned_agent_session_pages -- --exact
```

**Notes**

The registered OpenAPI route exposes full root-pinned snapshot and stable session identities with bounded cursor errors.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test native_lifecycle local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials -- --exact
```

**Notes**

The first exact filter omitted the module path and executed zero tests, so it was rejected as evidence.
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
cargo test -p jj-store-cli --test native_lifecycle http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials -- --exact
```

**Notes**

One LocalTrusted registered-handler test lists both root-pinned pages without a bearer credential and captures them for contract checking.
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
cargo check -p jj-store-http
```

**Notes**

The service-owned collaboration DTO and HTTP registration compile across the affected package boundary.
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
pnpm contracts:generate
```

**Notes**

OpenAPI and generated TypeScript include the additive AgentStore session collection.
<!-- /run -->

<!-- run -->
### Run 8

```yaml
result: fail
failures: 7
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

The same-change mapping correctly required missing updates to api, architecture, cli, compatibility, mutations, publication and spec-governance before this slice could close.
<!-- /run -->

<!-- run -->
### Run 9

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:boundaries
```

**Notes**

The package checker rejected a direct service-to-jj-agent dependency. The service now consumes the required owned model types through the existing WAL facade without weakening the boundary policy.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo fmt --all -- --check
```

**Notes**

The facade import rewrite needed rustfmt ordering before the final focused validation.
<!-- /run -->

<!-- run -->
### Run 11

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

Rust formatting, the one service pager regression, all nine HTTP contract tests, the one exact LocalTrusted handler fixture and every affected WAL/service/HTTP/CLI target pass. Production collaboration modules are 285 and 290 physical lines; tests are 136 and 138.
<!-- /run -->

<!-- run -->
### Run 12

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:governance; pnpm check:boundaries
```

**Notes**

The governance suite and all 13 package boundaries pass with AgentStore model types entering service only through the WAL facade.
<!-- /run -->

<!-- run -->
### Run 13

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

Generated OpenAPI and TypeScript match; 53 actual registered-handler HTTP/SSE responses conform and all six schema/client tests pass.
<!-- /run -->

<!-- run -->
### Run 14

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:validate; pnpm spec:first; pnpm check:progress; pnpm exec prettier --check <changed specification, contract and generated files>
```

**Notes**

All 89 requirements, 17 protected files across nine canonical chapters and 61 progress tasks validate; changed prose, contract tooling and generated files are formatted.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Session detail, transcript, relation and checkpoint reads remain M9.4 work.
- ReviewStore reads and typed writes, collaboration CLI commands, receipts and
  live hints remain M9.4 work.
- Shared Community/Design Core presentation remains M9.5; real harness and cold
  native recovery remain M9.6.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; AgentStore schema v1; OpenAPI 3.1; TypeScript generated contracts; session-page response v1; cursor token jsa1.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-M9-024 details 2](details-02.md)
