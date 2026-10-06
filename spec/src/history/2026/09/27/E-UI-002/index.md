---
schema: 1
id: E-UI-002
---

# E-UI-002

<!-- evidence-field:id markdown -->
## Id

E-UI-002
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4.5
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

storybook-shell
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- web/src/lib/ui/store-application/StoreApplication.svelte
- catalog/components/store-application/StoreApplication.stories.svelte
- docs/catalog.md
- .storybook/vite.config.ts
- catalog/vitest.config.ts
- catalog/components/community-app-shell/shell-header-geometry.test.ts
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- catalog/components/community-app-shell/shell-header-geometry.test.ts#coalesces_and_cancels_header_measurement
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Before implementation: no matching story file; shell acceptance absent.
<!-- /run -->

<!-- run -->
### Run 2

```yaml
result: fail
failures: 8
hasNote: true
noteEncoding: markdown

```

```shell
pnpm catalog:check
```

**Notes**

Initial adaptation: unsupported Close side prop and invalid test query exact options; fixed.
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Suite import failed with four dependency optimizer errors. Reused Community source-package exclusions for rune compilation.
<!-- /run -->

<!-- run -->
### Run 4

```yaml
result: fail
failures: 2
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Expanded selector and compact-picker keyboard setup mismatch; corrected control queries and actual opening interaction.
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Expanded story asserted before collapse animation settled; added waitFor.
<!-- /run -->

<!-- run -->
### Run 6

```yaml
result: fail
failures: 1
hasNote: true
noteEncoding: markdown

```

```shell
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

Expanded details correctly dismissed on focus leaving constrained overlay; test now closes details before resizing navigation.
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
pnpm test:storybook catalog/components/store-application/StoreApplication.stories.svelte
```

**Notes**

5 shell stories pass: Default, Expanded, CompactRepositoryPicker, Mobile, HoverReveal; blocking axe passes.
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
pnpm catalog:test
```

**Notes**

3 tests pass, including copied header geometry cancellation.
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
pnpm catalog:check
```

**Notes**

Zero errors or warnings; presentation boundaries pass.
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
pnpm test:storybook
```

**Notes**

6 stories pass including Chip; final cold process after optimizer configuration.
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
pnpm build-storybook
```

**Notes**

Final cold static shell build passes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

17 lanes pass, including existing browser acceptance.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Normal checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

All suites pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Catalog, served browser and Rust builds pass.
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

Rustfmt and Prettier pass.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pass
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages

- [E-UI-002 details 2](details-02.md)
