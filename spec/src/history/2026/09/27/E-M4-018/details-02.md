---
schema: 1
id: E-M4-018
---

# E-M4-018 details 2

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
cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::native_browser_windows_preserve_references_paths_groups_and_exact_diffs --locked -- --exact
```

**Notes**

The first exact native regression exposed an invalid even-term test bookmark merge; the production reader was not relaxed and the fixture was corrected to preserve native alternating terms.
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
cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::native_browser_windows_preserve_references_paths_groups_and_exact_diffs --locked -- --exact
```

**Notes**

Native bookmark/change lookup, conflicted targets, recursive path windows, bounded last-change attribution, exact operation rewrite grouping, metadata-only operations and changed-path summaries pass.
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
cargo test -p jj-store-cli --test native_lifecycle browsing_acceptance::signed_browser_cursors_bind_queries_support_backward_windows_and_reset_on_restart --locked -- --exact
```

**Notes**

Signed continuations preserve the exact snapshot, support backward windows, reject changed filters and reset after a serving-process restart.
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
cargo test -p jj-store-cli --test native_lifecycle http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured --locked -- --exact
```

**Notes**

All four registered HTTP browsing routes enforce repository authorization, retain the selected operation, reject cross-query cursors, publish nothing and emit contract samples.
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
cargo test -p jj-store-http contract::tests::browsing_contracts_keep_complete_reads_and_expose_bound_windows --locked -- --exact
```

**Notes**

OpenAPI exposes required operation parents, explicit diff snapshots and nullable bidirectional cursors while leaving the complete diff route cursor-free.
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
pnpm check:architecture && pnpm test:contracts
```

**Notes**

The 106-module architecture scan and generated TypeScript/schema consumer tests pass.
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
pnpm contracts:check
```

**Notes**

Generated OpenAPI and TypeScript match; 51 actual HTTP/SSE responses, including the four new browsing handlers, conform.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The first aggregate check exposed a clippy large-enum-variant error in LastChange; its two signatures were boxed without changing JSON or schema shape.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The next aggregate check reached a clippy too-many-arguments error in the browser pager; cursor/query inputs were consolidated into an owned internal request.
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
cargo clippy --workspace --all-targets --locked -- -D warnings
```

**Notes**

The exact workspace lint boundary passes after both internal ownership fixes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification, 11-package boundaries, 106-module architecture, progress, rustfmt, 51-response contract drift, web/catalog diagnostics and workspace clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

21 governance/architecture, 3 storage-harness, 6 contract, 8 web, 20 catalog-unit, 37 Storybook interaction/accessibility and cold catalog tests pass; the complete Rust workspace passes, including 174 native-lifecycle tests.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Cold Storybook, production browser and complete Rust workspace builds pass. Existing Design Core :global minifier and Storybook chunk-size warnings remain non-failing.
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
pnpm format:check
```

**Notes**

Tracked rustfmt and Prettier checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including spec-first coverage for 23 protected files across eight chapters, catalog interaction/accessibility, cold catalog and the complete Rust workspace.
<!-- /run -->

<!-- run -->
### Run 16

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

After marking M4.10 done, spec-first required the task's bounded-data versus production-wiring boundary in ui.md; no executable lane failed.
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

Native profile jj-0.45.1; cursor envelope jsr1; OpenAPI 3.1; generated openapi-typescript 7.13.0; enclosing Jujutsu change is the revision reference.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Additive browsing HTTP routes and signed continuation kinds for references, recursive paths, operation groups and diff summaries. Existing complete graph/tree/operation-diff/file-diff responses, native object envelopes, WAL, receipts, credentials and storage formats remain unchanged.
<!-- /evidence-field -->

