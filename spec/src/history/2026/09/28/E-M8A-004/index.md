---
schema: 1
id: E-M8A-004
---

# E-M8A-004

<!-- evidence-field:id markdown -->
## Id

E-M8A-004
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M8a
- M8a.1
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-28
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

local-macos-release
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- README.md
- package.json
- scripts/dev-demo.mjs
- scripts/dev-demo-services.mjs
- scripts/dev-demo-repository.mjs
- scripts/real-browser-acceptance.mjs
- crates/jj-store-cli/src/server_commands.rs
- crates/jj-store-cli/tests/server_executable.rs
- crates/jj-store-server/src/search.rs
- crates/jj-store-server/src/storage.rs
- crates/jj-store-server/tests/embedded_storage.rs
- web/tests/real/native-repository.spec.ts
- docs/browser.md
- docs/cli/server.md
- docs/cli/server-config.md
- docs/cli/server-commands.md
- docs/cli/serve-local.md
- docs/embedded-rustfs-qualification.md
- docs/evidence/m8a-local-release.md
- docs/implementation-plan.md
- spec/src/api.md
- spec/src/architecture.md
- spec/src/cli.md
- spec/src/compatibility.md
- spec/src/milestones.md
- spec/src/spec-governance.md
- spec/src/verification.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- crates/jj-store-server/tests/embedded_storage.rs#managed_local_storage_locks_verifies_and_fails_closed
- crates/jj-store-cli/tests/server_executable.rs#local_trusted_executable_bootstraps_registers_and_serves_without_tokens
- crates/jj-store-cli/tests/native_lifecycle/server_search.rs#server_search_diagnostics_use_authorized_live_status_and_handle_unavailability
- web/tests/real/native-repository.spec.ts#LocalTrusted bootstrap auto-opens
  without browser credentials
- web/tests/real/native-repository.spec.ts#external publication waits for
  explicit refresh and preserves selection
- web/tests/real/native-repository.spec.ts#native conflict selection previews
  and publishes one atomic resolution
- web/tests/real/native-repository.spec.ts#browser mutation previews and
  publishes one native request
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm dev:restart
```

**Notes**

The first LocalTrusted restart built and started successfully, but the next restart rejected its own new local/server.toml process because the ownership guard still expected the legacy runtime-root server.toml path. Runtime metadata now records the exact owned configuration path with a bounded compatibility check for the previously running stack.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked -- --ignored --exact local_trusted_executable_bootstraps_registers_and_serves_without_tokens
```

**Notes**

The real executable creates, diagnoses, registers, stops and restarts one token-free native repository through the mode-0600 Unix socket and protected loopback HTTP.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_ --locked
```

**Notes**

All five existing authenticated search lifecycle/diagnostic cases pass after the shared diagnostic request became deployment-aware.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-server --test embedded_storage --locked -- --ignored --exact managed_local_storage_locks_verifies_and_fails_closed
```

**Notes**

The first strengthened diagnostic assertion expected the generic filesystem prefix for the exclusive-lock failure; the platform reports the stable lock category instead. The assertion now checks that explicit category and retains exact diagnostics for every other startup fault.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-server --test embedded_storage --locked -- --ignored --exact managed_local_storage_locks_verifies_and_fails_closed
```

**Notes**

Persistent graceful/abrupt restart, exclusive ownership, missing volume/identity, private credential permissions, occupied port, sentinel/bucket and public-listener shutdown diagnostics pass against embedded RustFS.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools JJ_STORE_REAL_KEEP=1 pnpm test:local-release
```

**Notes**

Cold macOS ARM64 LocalTrusted acceptance passes all four built-browser scenarios in 21.2 seconds. Empty-volume startup was 10790 ms, populated restart 408 ms and combined server/RustFS RSS 142896 KiB, within the 30-second and 512 MiB ceilings. The token-free demo remains running for review.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

The first same-change gate found missing JST-GOV-002 mappings for api.md, cli.md and spec-governance.md. Those canonical chapters now describe the LocalTrusted release and deferred shared/search scope and are included in this evidence record.
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
pnpm format; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

Spec-first passed after the canonical update; progress then found E-UI-017 still naming the removed real-browser Tantivy case. Historical search evidence retains its controller, captured-browser and Storybook tests while local search-worker activation is deferred.
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

The complete static aggregate passes all 75 canonical requirements, package and 188-module web architecture boundaries, progress and formatting gates, embedded-runtime and workspace Clippy, 51 captured HTTP/SSE contracts plus six contract tests, both Svelte checks and catalog boundaries.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual aggregate passes 23 governance, three storage-harness, embedded/managed/LocalTrusted real-runtime targets, six contract, 41 production-web, 36 catalog-unit, 71 Storybook interaction/accessibility, cold catalog browser and full Rust workspace lanes. Visual baselines remained skipped by user direction.
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

Cold Storybook and production browser bundles, the pinned embedded RustFS runtime and the complete Rust workspace build successfully. Existing non-fatal Design Core :global(svg) minifier and bundle-size warnings remain outside this slice.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including cold catalog browser, interaction/accessibility, contracts, complete Rust tests, mdBook and same-change spec-first verification.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm format; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

The closure pass correctly rejected marking the M8a umbrella Done while M8a.5-M8a.7 remain deferred. M8a is now the deferred umbrella and completed LocalTrusted delivery remains independently Done as M8a.1.
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
pnpm format; DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:first; pnpm check:progress; pnpm format:check
```

**Notes**

The final same-change gate passes with 20 protected files mapped to six canonical chapters, 53 milestone tasks consistent, and all Rust and Prettier formatting current.
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
<!-- generated-navigation -->
## Pages

- [E-M8A-004 details 2](details-02.md)
