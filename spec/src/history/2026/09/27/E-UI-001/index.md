---
schema: 1
id: E-UI-001
---

# E-UI-001

<!-- evidence-field:id markdown -->
## Id

E-UI-001
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.4
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

storybook-foundation
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- catalog/foundation.test.ts
- .storybook/main.ts
- docs/catalog.md
- package.json
- web/src/lib/ui/community-chip/model.ts
- scripts/governance.test.mjs
- scripts/check-progress.mjs
- scripts/check-catalog.mjs
- spec-validator.config.mjs
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/foundation.test.ts#recognizes_svelte_diagnostics
- catalog/foundation.test.ts#portable_canvas_is_scoped
- scripts/governance.test.mjs#requires_catalog_evidence
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
pnpm test:storybook
```

**Notes**

Before implementation: command absent; no stories executed.
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
pnpm build-storybook
```

**Notes**

Initial static build lacked an explicit Svelte Vite config; three raw Svelte parse errors. Corrected catalog builder config.
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
pnpm catalog:check
```

**Notes**

Zero Svelte errors or warnings; presentation boundary passes.
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
pnpm catalog:test
```

**Notes**

2 tests pass.
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
pnpm test:storybook
```

**Notes**

1 story passes, including blocking axe checks.
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
pnpm build-storybook
```

**Notes**

Cold static build passes.
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

All normal checks pass.
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

Governance, web, catalog and native suites pass.
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

Storybook, existing browser and Rust builds pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 17 lanes pass, including cold existing browser acceptance and rustfmt.
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
pnpm format:check
```

**Notes**

Pinned rustfmt and Prettier pass.
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

Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core ^0.2.0; Community d859f33a484a9238bb11f974f4ead8e9be729ec6.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No production formats or APIs changed. Enclosing Jujutsu change is evidence reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


