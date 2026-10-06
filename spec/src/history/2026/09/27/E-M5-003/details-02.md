---
schema: 1
id: E-M5-003
---

# E-M5-003 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation_acceptance:: --locked
```

**Notes**

Two initial acceptance tests returned 404 for absent mutation routes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation_acceptance:: --locked
```

**Notes**

Both original route/authorization fixtures pass after implementation; expanded target subsequently passed six tests.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle http_mutation --locked
```

**Notes**

Seven native HTTP fixtures pass, including shared read/command admission and deadline after successful manifest CAS.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --locked
```

**Notes**

Four compiled contract tests and three existing asset tests pass.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

34 actual response schemas and six Node tests passed; formatter moved TypeScript directive off its intended property, yielding unused-directive and intentional invalid-command diagnostics. Moved directive to the property; assertion unchanged.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm check:progress
```

**Notes**

Expected missing same-change ledger/evidence errors before E-M5-003.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:check
```

**Notes**

Exact rerun passes after fixing TypeScript directive location: 34 actual HTTP/SSE responses, six schema tests and TypeScript consumer checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-http -p jj-store-service -p jj-store-wal --all-targets --locked -- -D warnings
```

**Notes**

Owning-package Clippy passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All normal lanes pass, including rustfmt, workspace all-target Clippy, boundaries, specification/progress, contracts and zero Svelte diagnostics.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

275 Rust tests pass including 120 native lifecycle tests, with six external tests explicitly ignored. 16 governance, three storage-harness, six schema and eight browser client tests also pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production assets and Rust workspace build pass. Browser runtime bundle unchanged.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt, contracts and workspace checks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

Pinned rustfmt and Prettier pass.
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
pnpm check:progress
```

**Notes**

32 tasks validate; this is bookkeeping, not completion of remaining mutation gates.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:first
```

**Notes**

Mapped protected file/chapter gate passes.
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
jj --no-pager diff --name-only -- web
```

**Notes**

Only web/src/lib/api/generated.ts changed; no browser behavior, component, test or visual changes.
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; jj-native-0.45.1-v1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Adds V1 HTTP preview/apply/status and capability entries for five commands, generated OpenAPI/TypeScript and WAL Snapshot schema metadata. Adds existing schemars dependency to WAL lock entry; persisted native/WAL/intent/StoredResult formats unchanged. No browser feature or visual changes; generated TypeScript declaration is the sole web edit. Enclosing Jujutsu change is the revision reference. ReconcileOperations, Abandon and ResolveConflicts remain M5 gates. Real RustFS/TLS SSE qualification awaits stopped shared Docker; browser feature and deployment gates remain Deferred M4.3.
<!-- /evidence-field -->

