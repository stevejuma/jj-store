---
schema: 1
id: E-M10-011
---

# E-M10-011

<!-- evidence-field:id markdown -->
## Id

E-M10-011
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M10
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-30
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

browser-acceptance-token-discovery-fixture-repair
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/tests/browser/native-browser-fixture.ts
- web/tests/browser/navigation.spec.ts
- web/tests/browser/mutations.spec.ts
- web/tests/browser/reads.spec.ts
- spec/src/ui.md
- spec/src/milestones.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- web/tests/browser/local-bootstrap.spec.ts#local trusted bootstrap auto-opens
  the sole repository without credentials
- web/tests/browser/navigation.spec.ts#hash route restores snapshot and file
  through controls reload and browser history
- web/tests/browser/mutations.spec.ts#repository command preview and apply
  retain the captured head and request
- web/tests/browser/reads.spec.ts#disconnect_discards_inflight_history_and_revocation_clears_data
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
pnpm spec:check
```

**Notes**

The first run reached Storybook, but browser module loading failed because this new workspace temporarily linked node_modules to the default checkout. A local offline frozen-lockfile install removed that path mismatch.
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
pnpm spec:check
```

**Notes**

Catalog and Storybook then passed, but the browser preview example exceeded Playwright's 30-second startup limit during its first Cargo build. Explicitly building that example took 60 seconds.
<!-- /run -->

<!-- run -->
### Run 3

```yaml
result: fail
failures: 6
hasNote: true
noteEncoding: markdown

```

```shell
pnpm web:test:browser
```

**Notes**

With the preview built, 14 of 20 browser tests passed. Six stale expectations still used the retired repository ID/token form, omitted code_state_version or expected an operation ID in repository context.
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
pnpm exec playwright test --config web/playwright.config.ts --grep 'repository command preview|hash route restores|malformed owned route|native_graphs_files|multiple_heads_require|disconnect_discards'
```

**Notes**

All six previously failing browser cases pass after aligning fixtures and assertions with JST-UI-010 and JST-UI-011.
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
pnpm web:test:browser
```

**Notes**

A cold preview process and all 20 browser interaction cases pass.
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
pnpm check
```

**Notes**

The complete repository check lane passes after the shared browser fixture repair.
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
pnpm test
```

**Notes**

The complete repository test lane passes after the shared browser fixture repair.
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
pnpm build
```

**Notes**

The complete repository build lane passes after the shared browser fixture repair.
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
pnpm spec:check
```

**Notes**

All 18 specification lanes pass, including the cold 20-case browser acceptance suite.
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

pnpm 10.32.1; Storybook 10.5.7; Playwright browser acceptance; Rust 1.98.1.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production API, persistence or wire format change; fixtures now use version-1 repository discovery pages and code_state_version in typed mutation preconditions.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


