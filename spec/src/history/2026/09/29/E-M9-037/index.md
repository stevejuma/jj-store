---
schema: 1
id: E-M9-037
---

# E-M9-037

<!-- evidence-field:id markdown -->
## Id

E-M9-037
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M9
- M9.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-29
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

production-avatar-assets
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- crates/jj-store-http/src/assets.rs
- crates/jj-store-http/tests/assets.rs
- scripts/web-manifest.mjs
- spec/src/api.md
- spec/src/milestones.md
- spec/src/spec-governance.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-http/tests/assets.rs#browser_assets_are_allowlisted_preloaded_and_sandboxed
- crates/jj-store-http/tests/assets.rs#browser_manifest_rejects_escape_missing_duplicate_and_oversized_assets
- crates/jj-store-http/tests/assets.rs#browser_agent_icon_directory_symlink_is_rejected
<!-- /evidence-field -->

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
cargo test -p jj-store-http --test assets --locked
```

**Notes**

All four asset-router tests pass. Manifest-listed agent SVGs return image/svg+xml, unknown and malformed paths remain rejected, intermediate directory symlinks fail closed and the document policy names only the exact Gravatar image origin.
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
pnpm web:build; validate web/dist/asset-manifest.json
```

**Notes**

The production build succeeds and its bounded manifest includes the Codex, Cursor and OpenCode SVGs under stable agent-icons paths.
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
pnpm spec:first
```

**Notes**

The protected asset router, manifest generator and canonical documentation remain mapped to the three owning chapters.
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
pnpm check:progress
```

**Notes**

The updated milestone evidence ledger is structurally valid.
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

Rust formatting and repository-wide Prettier checks pass after the final evidence update.
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

The aggregate static, formatting, Rust, web, architecture and specification checks pass.
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

The aggregate Rust workspace, web unit, browser, Storybook interaction and accessibility, catalog, lifecycle and adapter lanes pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

The complete Rust, web application and documentation build passes; the known Lightning CSS :global(svg) and large-chunk warnings remain informational.
<!-- /run -->

<!-- run -->
### Run 9

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

Specification validation and cold catalog browser acceptance pass, then the same inherited 20 captured production-browser cases time out while waiting for removed Repository ID and Repository token controls; this slice introduces no new failure category.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The checkout-owned development application restarted without reseeding and preserved repository 3afe5080b9388eeba0688b6514aea1b4.
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
python3 urllib live HTTP probe against http://127.0.0.1:7460
```

**Notes**

Codex, Cursor and OpenCode SVG routes each return 200 image/svg+xml; SOURCES.md remains 404 and the document CSP permits only same-origin images plus exact https://www.gravatar.com.
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
live browser DOM inspection of agent and Gravatar images
```

**Notes**

The restarted production application renders Cursor, Codex and OpenCode images with complete=true and non-zero intrinsic dimensions; five email-derived Gravatar author images also load completely at 80 by 80 pixels.
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

Rust 1.98.1; Node.js 24.10.0; pnpm 10.32.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No native object, manifest, WAL, receipt, AgentStore or ReviewStore encoding changes. Browser bundle manifest version 1 now admits only flat regular agent-icons/*.svg files in addition to the existing index, JavaScript, CSS and font allowlist. The CSP image source adds only https://www.gravatar.com.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


