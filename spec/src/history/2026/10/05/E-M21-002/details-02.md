---
schema: 1
id: E-M21-002
---

# E-M21-002 details 2

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: fail
failures: 11
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser stacks::local_stack_history_is_bounded_pinned_and_read_only --locked -- --exact
```

**Notes**

Missing module/variant before implementation; reads-before.log
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
cargo test -p jj-store-cli --test local_browser stacks::local_stack_history_is_bounded_pinned_and_read_only --locked -- --exact
```

**Notes**

One real-JJ test passes: five commits/limit-two pages, explicit range and target history, wrong anchor, empty range, revert-to-base, and all source metadata/files byte-identical including unsnapshotted edits. reads-after.log
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
cargo test -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked -- --exact
```

**Notes**

Intermediate test expected incorrect status 409; canonical admission returns 400. reads-http.log
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
cargo test -p jj-store-cli --test local_browser filesystem_runtime_registers_and_reads_without_native_storage --locked -- --exact
```

**Notes**

One actual HTTP test passes; continuation, wrong-range and raw-anchor rejection included. reads-http-after.log
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
pnpm catalog:test web/tests/vite/local-stack-history.test.ts
```

**Notes**

One pass/two missing validation failures; reads-client-before.log
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:test web/tests/vite/local-stack-history.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts web/tests/vite/read-query.test.ts
```

**Notes**

Intermediate two assertion-message mismatches plus two cancellation timeouts, resolved by parent uvzrpntl and exact code assertions; reads-client-after.log
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
pnpm catalog:test web/tests/vite/local-stack-history.test.ts web/tests/vite/local-browser.test.ts web/tests/vite/history-query-transport.test.ts web/tests/vite/read-query.test.ts
```

**Notes**

Final 27 tests/four files pass; reads-client-final.log
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
pnpm contracts:generate
```

**Notes**

Generated public schema/client refreshed; reads-generate.log
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

Unchanged full inventory: 259 actual HTTP/SSE responses conform, seven contract tests pass and contract TypeScript check passes; reads-contracts-check.log
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
cargo check -p jj-store-jj -p jj-store-http --all-targets --locked
```

**Notes**

Affected packages pass; reads-check.log
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
cargo fmt --all -- --check
```

**Notes**

Pass; reads-rustfmt.log
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
pnpm check:boundaries
```

**Notes**

17 package boundaries pass; reads-boundaries.log
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
pnpm web:check
```

**Notes**

Zero errors/warnings; reads-web-check.log
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
pnpm catalog:check
```

**Notes**

Zero errors/warnings; reads-catalog-check.log
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
pnpm check:architecture
```

**Notes**

1052 modules pass; reads-architecture.log
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
pnpm check:ui-ownership
```

**Notes**

One validator passes; reads-ownership.log
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
pnpm docs:generate
```

**Notes**

Generated API manual refreshed; reads-docs-generate.log
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
pnpm docs:check
```

**Notes**

139 files, 13720379 bytes, nine light/dark Mermaid diagrams, valid links/anchors; reads-docs-check.log
<!-- /run -->
<!-- /evidence-field -->

