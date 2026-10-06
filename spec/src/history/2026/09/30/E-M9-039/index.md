---
schema: 1
id: E-M9-039
---

# E-M9-039

<!-- evidence-field:id markdown -->
## Id

E-M9-039
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

native-diff-line-coordinates-and-emoji-picker-csp
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- spec/src/api.md
- spec/src/collaboration.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- web/src/lib/controller/native-presentation.ts
- web/tests/production-store-loaders.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/production-store-loaders.test.ts#native byte offsets become
  one-based unified diff line coordinates
- web/tests/review-comment-anchor.test.ts#after-line anchors preserve UTF-8
  offsets, digest and operation pair
- crates/jj-store-http/tests/assets.rs#browser_assets_are_allowlisted_preloaded_and_sandboxed
<!-- /evidence-field -->

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
live browser inspection of diff gutters and emoji-mart shadow root before implementation
```

**Notes**

The five-line sample rendered native byte offsets 0, 43, 71, 73 and 74 as line numbers, so exact ReviewStore resolution rejected the selected row. The emoji picker element existed, but its 13,391-byte inline shadow style had no CSSStyleSheet under the document CSP and browser defaults produced an unstyled control.
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
pnpm exec tsx --test web/tests/production-store-loaders.test.ts web/tests/review-comment-anchor.test.ts
```

**Notes**

All 12 focused presentation and anchor tests pass. Sequential native hunks with byte starts 0, 4, 8, 14 and 23 present one-based source coordinates with the correct zero-length insertion position, while exact UTF-8 anchors retain their existing behavior.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-http --test assets --locked
```

**Notes**

All four asset-router tests pass. The CSP admits the exact SHA-256 hash of the pinned emoji-mart 5.6.0 shadow stylesheet while continuing to reject unsafe-inline.
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
pnpm web:check; pnpm spec:first; pnpm check:progress
```

**Notes**

Svelte reports zero diagnostics, the four protected implementation files map to the API, collaboration, governance and milestone chapters, and the evidence ledger is structurally valid across all 61 tasks.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned development application rebuilt and restarted without reseeding, preserved repository 3afe5080b9388eeba0688b6514aea1b4 and reported search ready.
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
live in-app browser verification at http://127.0.0.1:7460/
```

**Notes**

The rebuilt production app renders the five-line fixture as source lines 1 through 5. A first comment on the formerly misnumbered native byte-71 row committed as an exact inline ReviewStore thread with no stale-anchor error. The emoji picker renders its bounded search, emoji grid, category navigation, typography and spacing instead of browser defaults.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

The aggregate specification, package-boundary, architecture, progress, Rust formatting and Clippy, contract, web and catalog checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

The complete governance, embedded and managed RustFS, LocalTrusted, contract, 54-case web, 60-case catalog, 85-case Storybook, cold catalog browser and Rust workspace suites pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The complete Storybook, production web, embedded RustFS and Rust workspace build passes; the known Lightning CSS :global(svg) and large-chunk warnings remain informational.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 20
hasNote: true
noteEncoding: markdown

```

```shell
pnpm spec:check
```

**Notes**

Specification validation and cold catalog browser acceptance advance to the captured production-browser lane, where the same inherited 20 cases fail on removed Repository ID or Repository token controls. The Playwright last-run artifact records exactly 20 failed test IDs and every captured failure remains in that existing fixture category.
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
pnpm format:check; pnpm spec:first; pnpm check:progress
```

**Notes**

Rust and repository-wide Prettier formatting pass, the final documentation-only evidence update introduces no unmapped protected implementation, and the ledger remains structurally valid across all 61 tasks.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Visual baselines remain unapproved.
- The inherited captured production-browser fixtures still require migration
  from removed Repository ID and Repository token controls before pnpm
  spec:check can pass.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Rust 1.98.1; Node.js 24.10.0; pnpm 10.32.1; Svelte 5.57.1; Community ^0.1.0; emoji-mart 5.6.0.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore or ReviewStore encoding changes. FileDiffHunk before_start and after_start remain native byte offsets on the transport. Presentation alone converts ordered hunk contents to unified one-based source coordinates. The browser CSP adds one exact SHA-256 style hash and does not admit arbitrary inline style or script.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


