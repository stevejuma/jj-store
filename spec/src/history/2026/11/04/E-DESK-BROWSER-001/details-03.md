---
schema: 1
id: E-DESK-BROWSER-001
---

# E-DESK-BROWSER-001 details 3

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
MISE_PNPM_VERSION=10.32.1 MISE_RUST_VERSION=1.99.0 pnpm docs:check
```

**Notes**

Default validation dispatch started a Docker docs lane with Rust reference compilation. Stopped the exact owned validation run; UI qualification uses existing explicit native reference binary. An initial stop targeting a lease container was refused as unowned; no unrelated process was stopped.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=/home/steve/Projects/jj-store/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

References/review and nine diagrams passed; mise required an explicit uv version.
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
MISE_PNPM_VERSION=10.32.1 pnpm evidence:new -- --id E-DESK-BROWSER-001 --task M17 --task M17.2 --requirement JST-DESK-004 --requirement JST-DESK-006 --requirement JST-UI-018 --requirement JST-CLI-004
```

**Notes**

Scaffold created; global navigation regeneration could not fit the existing E-UI-CATALOG-HELP-001 index. Restored only generator-owned unrelated edits, preserved committed history, and updated this evidence/day navigation narrowly through repository rendering helpers.
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
node scripts/web-architecture.mjs
```

**Notes**

Initial one-line additions exceeded two 300-line module limits; extracted connection options and consolidated controller type imports. New acceptance helper also kept launcher under its existing limit.
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
node scripts/web-architecture.mjs
```

**Notes**

Current production/script limits pass without baseline changes.
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
node --test scripts/dev-ui-presentation.test.mjs scripts/dev-ui.test.mjs scripts/web-manifest.test.mjs scripts/web-architecture.test.mjs
```

**Notes**

18 tests, including inventory bounds and architecture checks.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm web:check
```

**Notes**

Initial and final checks: zero errors/warnings.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm catalog:check
```

**Notes**

Initial and final type check: zero errors/warnings; catalog boundary and story parity pass.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_VALIDATION_RUNTIME=native pnpm check:ui-ownership
```

**Notes**

Owned production boundary passes; no Community runtime imports.
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
node scripts/docs.mjs review application "Reviewed explicit host and connection context plus appearance bundling. Production defaults and service-backed screens remain unchanged; browser desktop debugging and native capability limits are documented in the development guide."
```

**Notes**

Reviewed affected application documentation and refreshed only its fingerprint.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_BROWSER_PREVIEW_EXECUTABLE=/home/steve/Projects/jj-store/target/debug/examples/browser_preview pnpm exec playwright test --config web/playwright.config.ts
```

**Notes**

Escalation: shared production chunk grouping can affect other built routes, so ran the complete app browser suite. 112 passed, one existing skip; capture-observation and local-sync tests timed out waiting for main bookmark during concurrent compilation/documentation activity.
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
MISE_PNPM_VERSION=10.32.1 node .local/desktop-browser-validation/baseline-browser.mjs
```

**Notes**

Same two exact tests passed against the compiled parent source overlay, 2/2. Temporarily swapped ignored build directories with guaranteed restoration; no tracked files modified.
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
MISE_PNPM_VERSION=10.32.1 JJ_STORE_BROWSER_PREVIEW_EXECUTABLE=/home/steve/Projects/jj-store/target/debug/examples/browser_preview pnpm exec playwright test --config web/playwright.config.ts web/tests/browser/local-capture-settings.spec.ts web/tests/browser/local-sync.spec.ts --grep "visible capture follows observations|local sync controls and grouped progress"
```

**Notes**

Both exact targets also pass on current code without product or test edits, 2/2. Final complete rerun required to clear transient broad-lane failures.
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
MISE_PNPM_VERSION=10.32.1 MISE_UV_VERSION=0.9.28 JJ_STORE_VALIDATION_RUNTIME=native JJ_STORE_DOC_REFERENCE=/home/steve/Projects/jj-store/target/debug/examples/docs_reference pnpm docs:check
```

**Notes**

Strict manual build and nine light/dark Mermaid diagrams pass; 144-file 26736832-byte Help closure, links and anchors valid. Reused existing explicit native reference binary.
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
MISE_PNPM_VERSION=10.32.1 MISE_UV_VERSION=0.9.28 JJ_STORE_DOC_REFERENCE=/home/steve/Projects/jj-store/target/debug/examples/docs_reference node scripts/docs.mjs stage
```

**Notes**

Built documentation staged successfully into ignored web/dist/help.
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
MISE_PNPM_VERSION=10.32.1 MISE_UV_VERSION=0.9.28 JJ_STORE_DOC_REFERENCE=/home/steve/Projects/jj-store/target/debug/examples/docs_reference VITE_JJ_STORE_DESKTOP_PREVIEW=1 pnpm web:build
```

**Notes**

Complete final production build, bounded inventory and documentation staging pass. Production build deliberately supplied the preview environment flag: browser acceptance still uses ordinary browser behavior because import.meta.env.DEV is false. Manifest is v1, 256 files, 11516 bytes. Existing large-chunk advisory retained.
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
MISE_PNPM_VERSION=10.32.1 pnpm exec vitest run --config catalog/vitest.config.ts --project catalog-unit web/tests/vite/host-capabilities.test.ts
```

**Notes**

Final strengthened power regression: 2 tests pass; preserves active-agent gating, maximum activity timestamp, explicit disable, disposal release and non-blocking host refusal.
<!-- /run -->

<!-- run -->
### Run 18

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
MISE_PNPM_VERSION=10.32.1 pnpm exec spec-validator first --base @- --head @
```

**Notes**

First gate identified missing mapped UI and documentation chapters. Added owning terminal/presentation and development documentation contracts; next attempt correctly required the new files in fresh evidence, which is now refreshed.
<!-- /run -->
<!-- /evidence-field -->

