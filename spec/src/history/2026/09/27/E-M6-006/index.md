---
schema: 1
id: E-M6-006
---

# E-M6-006

<!-- evidence-field:id markdown -->
## Id

E-M6-006
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M6
- M6.2
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

http-search
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- Cargo.lock
- crates/jj-store-cli/tests/native_lifecycle/http_search.rs
- crates/jj-store-cli/tests/native_lifecycle/search_paging.rs
- crates/jj-store-http/Cargo.toml
- crates/jj-store-http/src/contract.rs
- crates/jj-store-http/src/contract/tests.rs
- crates/jj-store-http/src/dto.rs
- crates/jj-store-http/src/error.rs
- crates/jj-store-http/src/lib.rs
- crates/jj-store-http/src/routes.rs
- crates/jj-store-http/src/search.rs
- docs/evidence/m6-http-search.md
- schemas/openapi.json
- scripts/contracts.mjs
- spec/src/adapters.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- web/src/lib/api/generated.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_pins_results_across_writes_and_captures_contracts
- crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_rejects_bad_inputs_and_unconfigured_repositories
- crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_revocation_and_repository_isolation_precede_query_details
- crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_reports_degradation_and_head_runs_without_publication
- crates/jj-store-cli/tests/native_lifecycle/http_search.rs#http_search_requires_head_selection_and_reports_missing_reader
- crates/jj-store-http/src/contract/tests.rs#search_contract_requires_query_and_exposes_freshness_and_resets
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search_pins_results_across_writes_and_captures_contracts --locked
```

**Notes**

Initial acceptance cannot compile: missing SearchIndex and HttpApi::with_search (two diagnostics).
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search_pins_results_across_writes_and_captures_contracts --locked
```

**Notes**

Original named acceptance passes after implementation and again with the committed dependency lock.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search --locked
```

**Notes**

Initial four HTTP search fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --lib --locked
```

**Notes**

Six HTTP contract tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate
```

**Notes**

OpenAPI and TypeScript declarations regenerated.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

47 actual HTTP/SSE responses conform; six contract validators and TypeScript checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_search --locked
```

**Notes**

All five HTTP search fixtures pass, including multiple-head selection and an absent reader.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All checks pass: rustfmt, boundaries, progress, generated contracts, existing Svelte and all-target Clippy.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

344 Rust tests passed, zero failures, six external tests ignored; governance, harness, contract and existing web unit checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Existing browser bundle and Rust workspace build successfully.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 specification lanes pass, including existing cold browser interaction/accessibility, Rust tests, mdBook and spec-first.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Rustfmt and repository Prettier formatting pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Configure server search runners, credentials, lifecycle and diagnostics.
- Implement immutable remote index snapshots and restore; record cold deployment
  and measured performance evidence. RustFS/TLS external qualification is
  blocked; UI remains deferred.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1; Tantivy 0.26.2 at 72d1ef9a6468aa68bbc69dcc80cdf60aaf64364d.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds registered V1 HTTP search and status routes, per-repository search capability and optional search_status error data; generated OpenAPI and client declarations updated. Local schema 2, jss1 cursors and authoritative native/WAL/receipt formats unchanged. Enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


