---
schema: 1
id: E-M9-001
---

# E-M9-001 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 28
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-wal --test publication domain_publications_preserve_roots_without_staling_code --locked
```

**Notes**

The test-first compile proved that the repository had no independent domain-root publication contract.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication legacy_repository_enables_collaboration_without_rewriting_code_history --locked -- --exact
```

**Notes**

A repository with committed format-1 history gains canonical empty agent/review roots through one receipt-bearing WAL event; an already prepared unchanged-code mutation then commits and cold recovery preserves all records.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --locked
```

**Notes**

All 30 WAL inventory/publication tests pass, including domain isolation, enablement retries, before/after unknown manifest outcomes and selected-history recovery.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check --workspace --all-targets --locked
```

**Notes**

Every Rust workspace target compiles with the code-state token propagated through service, HTTP and RPC contracts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-service conflict_ --locked
```

**Notes**

Both canonical legacy receipt and conflict-upload digest regressions remain byte-stable when code_state_version is zero.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-rpc administration --locked
```

**Notes**

Both administration snapshot and credential-safety tests pass with the new RPC field.
<!-- /run -->

<!-- run -->
### Run 7

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

The first generation attempt reached the linker under the unaccepted full-Xcode developer directory; no contract assertion ran.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate && pnpm test:contracts
```

**Notes**

OpenAPI and TypeScript contracts regenerate and all six schema/client validation tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

The complete contract-capture lane exposed that its conflict-upload HTTP fixture did not yet send the required code_state_version query binding; the request failed closed with HTTP 400.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked -p jj-store-cli --test native_lifecycle content_acceptance::diff_acceptance::conflict_acceptance::resolve_acceptance::uploads_acceptance::conflict_upload_http_stages_verified_binary_without_publication_then_resolves -- --exact
```

**Notes**

The exact contract-capture regression passes after binding the upload fixture to the selected snapshot code-state version.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

Generated OpenAPI and TypeScript artifacts match, all 51 captured HTTP/SSE responses conform, and all six contract tests pass.
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
pnpm exec tsx --test web/tests/production-store-mutations.test.ts web/tests/production-store-conflicts.test.ts web/tests/production-store-loaders.test.ts web/tests/production-inspection-pages.test.ts
```

**Notes**

All 15 directly affected controller tests pass with exact code-state mutation and conflict-upload bindings.
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
node scripts/web-architecture.mjs
```

**Notes**

All 196 governed web modules remain within the Community-derived size and dependency boundaries.
<!-- /run -->

<!-- run -->
### Run 14

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

The mapped gate required API, CLI, compatibility, JJ integration and mutation chapters for the propagated code-state contracts.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:rust:check; pnpm exec prettier --check <changed files>
```

**Notes**

Rustfmt reported the new modules and tests, while Prettier reported the four new/extended canonical chapters; all formatting was applied through reviewed patches.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first; pnpm test:governance; pnpm check:progress; pnpm web:check; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:rust:check
```

**Notes**

Nine mapped canonical chapters, 24 governance tests, all 60 ledger tasks, production Svelte diagnostics and Rust formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; jj-lib/jj-cli 0.45.1; manifest format 2; OpenAPI 3.1; native RPC v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Manifest format 2 selects independent code, agent and review state with a domain-history boundary. New repositories start with canonical empty agent/review roots; format-1 repositories adopt them through a receipt-bearing CollaborationEnabled WAL event without native ID or history rewrite. Snapshot code_state_version is omitted at zero for canonical legacy bytes and carried by current HTTP/RPC contracts. AgentStore and ReviewStore object schemas remain pending M9.2/M9.3.
<!-- /evidence-field -->

