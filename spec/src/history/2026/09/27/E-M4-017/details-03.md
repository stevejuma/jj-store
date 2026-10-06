---
schema: 1
id: E-M4-017
---

# E-M4-017 details 3

<!-- evidence-field:runs runs -->
## Validation

<!-- run -->
### Run 1

```yaml
result: pass
failures: 0
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser
```

**Notes**

Eight cold browser interactions pass before final reset/empty-file additions; includes axe desktop/mobile, pagination, index retry and stale file completion. Full final verification remains pending.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --test assets --locked
```

**Notes**

Three asset allowlist/preload, invalid-manifest and symlink tests pass.
<!-- /run -->

<!-- run -->
### Run 3

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

Expected missing ledger update and evidence entry before recording E-M4-017.
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
pnpm format:rust:check
```

**Notes**

Pinned rustfmt rejects the newly authored unformatted Rust sources; formatting is required before acceptance.
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
pnpm format:rust:check
```

**Notes**

Pinned rustfmt now passes all Rust source, examples and tests after pnpm format.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

58 requirements, package boundaries, progress, rustfmt, 25 serialized contract responses, Svelte and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

246 Rust tests pass / 6 explicit external tests ignored; 98 native lifecycle cases, 16 governance tests, 3 storage-harness self-tests, 5 schema tests, TypeScript consumer checks and 8 browser-client unit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep graph_and_directory_pages_retain_exact_snapshots_and_query_context
```

**Notes**

Extended regression showed snapshot bar using editable operation text rather than loaded snapshot identity.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep graph_and_directory_pages_retain_exact_snapshots_and_query_context
```

**Notes**

Loaded snapshot label remains exact while operation/revset inputs change. Continuations retain original operation, query/time and reject mixed snapshots for graph and directory pages.
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

Production Svelte assets and complete locked Rust workspace build pass; JS 75.63 kB / 27.95 kB gzip and CSS 7.87 kB / 2.42 kB gzip.
<!-- /run -->

<!-- run -->
### Run 11

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

Browser analysis/unit/eight cold interactions, contracts, Rust formatting/tests and mdBook pass; final spec-first rejected new docs/browser.md without an owning map.
<!-- /run -->

<!-- run -->
### Run 12

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

Reproduced docs/browser.md missing chapter mapping. Mapped it to api.md and made governance table pipe escaping valid Markdown.
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
pnpm spec:first
```

**Notes**

41 protected files and 7 owning canonical chapters pass.
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
pnpm spec:validate
```

**Notes**

Updated map/table validate with 58 requirements and exactly one verification row per requirement.
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

All 13 lanes pass on final implementation: eight cold browser interactions including axe and reset/refresh/snapshot-label regressions; eight client unit tests; 246 Rust tests with six external tests explicitly ignored; contract conformance, rustfmt, governance, mdBook and mapped spec-first.
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
pnpm format:check
```

**Notes**

Pinned cargo fmt --all -- --check and Prettier pass across authored Rust, Svelte, TypeScript, Markdown and tooling.
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

Rust 1.97.1 with rustfmt; jj-cli/jj-lib 0.45.1 at 7c41cdeb16b6b321c64e789a966b6adf723816a5; Svelte 5.57.1; Vite 8.3.1; Playwright 1.63.0; Chromium headless shell 1243; axe 4.13.0; lossless-json 4.3.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Browser asset manifest V1 added. Existing native, RPC and HTTP wire formats unchanged; OpenAPI reference siblings normalized to allOf for discriminator-preserving client generation. Enclosing Jujutsu change is the revision reference. M4.2 retains real RustFS/SSE deployment qualification; E-M4-015 records stopped shared Docker engine and pending restart approval. M5 mutations, M6 search and M8a hardening remain unfinished. Browser visual review remains visual-pending.
<!-- /evidence-field -->

