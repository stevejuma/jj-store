---
schema: 1
id: E-M8A-003
---

# E-M8A-003 details 2

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
cargo test -p jj-store-cli --test local_trusted_cli --locked
```

**Notes**

The first physical-workspace acceptance completed the tokenless create and attach flow, then exposed macOS /var to /private/var canonicalization in the expected workspace, cache and socket paths.
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
cargo test -p jj-store-cli --test local_trusted_cli --locked
```

**Notes**

The tokenless Unix RPC client creates one native repository and physical workspace, persists a format-2 local connection with no credential ID, and reads its heads.
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
cargo test -p jj-store-http --locked; cargo test -p jj-store-cli --test native_lifecycle http_ --locked; pnpm contracts:check
```

**Notes**

All 31 HTTP tests, the complete HTTP lifecycle filter and 51 captured HTTP/SSE responses pass with bootstrap, deployment-dependent access, exact loopback admission, media types and SSE protections.
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
JJ_STORE_RUSTFS_RUNTIME=$PWD/runtime/embedded-rustfs/target/debug/jj-store-rustfs cargo test -p jj-store-cli --test server_executable --locked -- --ignored --exact local_trusted_executable_bootstraps_registers_and_serves_without_tokens
```

**Notes**

The cold executable starts pinned embedded RustFS, exposes a mode-0600 Unix socket, creates and reads a native repository without tokens, registers the verified authority, restarts and returns the installation catalog from bootstrap.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook
```

**Notes**

The first 71-case interaction/accessibility run found that the new loading status had visible text but no computed accessible name. The composed status now owns an explicit name and hides the nested spinner status.
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
pnpm exec vitest run --config catalog/vitest.config.ts --project storybook catalog/components/store-application/StoreConnection.stories.svelte
```

**Notes**

All six connection stories pass, including bootstrapping, local chooser and empty-catalog states under blocking accessibility checks.
<!-- /run -->

<!-- run -->
### Run 7

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser
```

**Notes**

The widened cold browser lane passed the new tokenless LocalTrusted auto-open and axe scenario, then exposed one old malformed-route test that had no fixture for the newly mandatory bootstrap request.
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
pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/navigation.spec.ts --grep 'malformed owned route'
```

**Notes**

The original invalid-route assertion passes after its harness supplies the authenticated bootstrap phase; the application still reports the route error before accepting credentials.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration older_configuration_formats_are_explicitly_rejected --locked
```

**Notes**

The narrowed pre-release contract initially entered the obsolete format-1 parser and returned a generic malformed-config error instead of rejecting the unsupported format.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-server --test configuration older_configuration_formats_are_explicitly_rejected --locked; cargo test -p jj-store-cli --test server_executable local_init_creates_strict_format_two_configuration --locked
```

**Notes**

Format 1 is now rejected explicitly, the migration command and fixture are removed, and local initialization still emits strict secret-free format 2.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_ --locked
```

**Notes**

The first focused rerun proved the protocol header was present, then exposed that the diagnostic fixture still composed its router for a different authority than its real listener.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test native_lifecycle server_search_ --locked
```

**Notes**

All five search lifecycle cases pass; diagnostics use the mandatory protocol header and the exact configured listener authority, with denied and unavailable states preserved.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

All embedded, LocalTrusted, contract, web, catalog, Storybook interaction/accessibility and 175 native lifecycle cases passed before two cold TLS probes reached API routes with the temporary listener port as Host instead of the configured authority.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_serves_only_the_configured_browser_bundle --locked; DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_http_reads_support_http1_and_http2 --locked
```

**Notes**

Both cold TLS HTTP/1.1 and HTTP/2 probes pass with the mandatory protocol header and exact configured Host while still connecting to their isolated ephemeral listeners.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All specification, boundary, 188-module architecture, progress, formatting, embedded-helper Clippy, 51 captured HTTP/SSE response, production/catalog diagnostic and locked-workspace Clippy checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm test
```

**Notes**

The complete non-visual aggregate passes 23 governance, three storage-harness, six contract, 41 production web, 36 catalog unit, 71 Storybook interaction/accessibility and 175 native lifecycle cases, plus cold catalog, LocalTrusted executable, embedded RustFS and complete Rust workspace lanes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Cold Storybook and production browser bundles, the separately locked embedded RustFS helper and the complete locked Rust workspace build successfully. Existing Design Core :global selector and bundle-size diagnostics remain non-failing warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools VITEST_MAX_WORKERS=1 pnpm spec:check
```

**Notes**

All 18 canonical lanes pass, including 75 requirements, architecture, contracts, diagnostics, 71 Storybook interaction/accessibility cases, cold catalog and production-browser acceptance, the locked Rust workspace and mdBook. Visual baselines remain skipped by user direction.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->

