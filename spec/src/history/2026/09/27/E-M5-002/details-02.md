---
schema: 1
id: E-M5-002
---

# E-M5-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 15
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked
```

**Notes**

Acceptance fixture initially lacked ApplicationIntent, publish/status application methods and receipt-by-request lookup.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked
```

**Notes**

First three candidate retry, race and unknown-outcome fixtures pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked
```

**Notes**

Five fixtures pass after adding stable-input/native-profile and current-guard checks.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact
```

**Notes**

Native service fixture initially lacked the mutations module and typed result service.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact
```

**Notes**

Service incorrectly referenced private WAL encoding helper; added service-owned bounded JSON encoding.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance::typed_apply_returns_original_native_result_after_restart_and_later_publication --locked -- --exact
```

**Notes**

Native empty-repository preview, New, Describe and retry after later publication/restart pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked
```

**Notes**

Five native service fixtures pass, including stale/protected/reader/revocation checks and both preparation/CAS races.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-wal --test publication application_acceptance:: --locked
```

**Notes**

Six final WAL fixtures pass, including frozen independently calculated intent digest, zero-write preview checks and selected identity/position/index corruption.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked
```

**Notes**

Six final native service fixtures pass, including checkpoint restart and late bookmark create/move receipts after deletion.
<!-- /run -->

<!-- run -->
### Run 10

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

Before adding E-M5-002: missing same-change ledger update and evidence entry.
<!-- /run -->

<!-- run -->
### Run 11

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

Clippy found two large enum variants in Outcome/Status (352-byte receipt versus 16-byte request ID). Boxed receipt values without changing serialization.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo clippy -p jj-store-wal -p jj-store-service --all-targets --locked -- -D warnings
```

**Notes**

Exact owning-package all-target Clippy target passes after boxing retained results.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle mutation_acceptance:: --locked
```

**Notes**

All six final native service fixtures pass again after bounded enum layout repair.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Rustfmt, workspace all-target Clippy, governance/progress/boundaries, unchanged generated contracts with 25 actual HTTP/SSE responses, five schema tests and Svelte analysis pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

267 Rust tests including 113 native lifecycle tests; six external tests explicitly ignored. Also 16 governance, three storage-harness, five schema and eight browser client tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production browser assets and locked Rust workspace build pass; no browser source changes.
<!-- /run -->

<!-- run -->
### Run 17

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

All 13 lanes pass, including eight cold Playwright interaction/accessibility tests, mdBook, spec-first, rustfmt and workspace checks.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format:check
```

**Notes**

Pinned rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

