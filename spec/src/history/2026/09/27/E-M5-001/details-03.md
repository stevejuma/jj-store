---
schema: 1
id: E-M5-001
---

# E-M5-001 details 3

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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Specification, progress, rustfmt, 25 actual contract responses, five schema tests, Svelte and all-target Clippy pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

255 Rust tests pass, six external tests explicitly ignored; 107 native lifecycle tests, 16 governance tests, three storage-harness self-tests, five schema tests, TypeScript consumer checks and eight browser-client unit tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Production browser bundle and locked Rust workspace build pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 13 lanes pass, including eight cold Playwright interactions with axe, 255 Rust tests with six explicit external ignores, native DTO contracts, rustfmt, governance, mdBook and spec-first.
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
pnpm format:check
```

**Notes**

Pinned rustfmt and repository Prettier checks pass.
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

No persistence, WAL, native RPC or public HTTP format changes. Internal owned command and impact-report DTOs introduced. Enclosing Jujutsu change is the revision reference. M5 remains In progress: durable preview/apply/status, reconciliation, abandon, conflict resolution and transport/browser integration remain. M4 real RustFS/SSE/browser qualification remains open in E-M4-015; no shared Docker restart approval has been received.
<!-- /evidence-field -->

