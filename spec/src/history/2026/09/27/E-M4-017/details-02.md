---
schema: 1
id: E-M4-017
---

# E-M4-017 details 2

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
pnpm web:check
```

**Notes**

Before browser implementation the web:check command did not exist.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

Initial implementation: recursive task result inference and two missing operation-diff discriminator errors.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:contracts
```

**Notes**

TypeScript consumer operationParent reproduced loss of reference-sibling discriminator; fixed schemars reference normalization and regenerated artifacts.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm contracts:generate
```

**Notes**

Initial RemoveRefSiblings construction failed E0603; corrected the non-exhaustive transform to Default construction.
<!-- /run -->

<!-- run -->
### Run 5

```yaml
result: fail
failures: 4
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser
```

**Notes**

Matching Playwright browser absent; installed chromium headless shell 1243.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser
```

**Notes**

Actual browser fetch called with Client receiver, preventing login. Default fetch now forwards through a closure.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible
```

**Notes**

Binary file locator incorrectly included an aria-hidden marker; corrected to the actual accessible name bin file.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible
```

**Notes**

Axe identified scrollable-region-focusable on operation metadata; labeled keyboard-focusable text regions fix this.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep native_graphs_files_and_source_rendering_are_accessible
```

**Notes**

Native views, source rendering, metadata, disconnect and axe pass under production asset CSP.
<!-- /run -->

<!-- run -->
### Run 10

```yaml
result: fail
failures: 3
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:check
```

**Notes**

TextPreview refactor initially missed three multiline pre tags, causing two parse diagnostics and a missing App export; corrected every tag.
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
pnpm web:check
```

**Notes**

Retained query union needed an explicit record type; fixed without weakening generated DTO types.
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
pnpm web:check
```

**Notes**

Svelte/TypeScript reports zero errors and zero warnings.
<!-- /run -->

<!-- run -->
### Run 13

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_serves_only_the_configured_browser_bundle --locked -- --exact
```

**Notes**

Unknown ordinary HTTP asset path inherited Tonic HTTP 200 fallback; expected 404.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p jj-store-cli --test server_executable cold_tls_server_serves_only_the_configured_browser_bundle --locked -- --exact
```

**Notes**

Cold real TLS executable serves configured assets/CSP, denies unauthenticated reads, returns ordinary HTTP 404 and preserves unknown gRPC service status 12.
<!-- /run -->

<!-- run -->
### Run 15

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep event_reconnect_resumes_and_reset_requires_explicit_refresh
```

**Notes**

Extended regression confirmed explicit refresh after reset did not resubscribe.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm web:test:browser --grep event_reconnect_resumes_and_reset_requires_explicit_refresh
```

**Notes**

Refresh aborts old feed and starts a fresh subscription; reconnect retains the fully parsed prior cursor before reset.
<!-- /run -->

<!-- run -->
### Run 17

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test
```

**Notes**

Empty file range returned file_unavailable; retry only bytes */0 with a bounded authenticated full read.
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
pnpm web:test
```

**Notes**

Eight focused client/layout tests pass, including empty files, lossless numbers, bounded graph rendering and revocation.
<!-- /run -->
<!-- /evidence-field -->

