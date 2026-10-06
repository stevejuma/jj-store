---
schema: 1
id: E-UI-003
---

# E-UI-003 details 3

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
pnpm catalog:check
```

**Notes**

71 catalog modules; 0 Svelte errors and 0 warnings.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

18 lanes pass including 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, Rust tests and rustfmt. Before final author-spacing and shutdown fixes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

All checks and Clippy pass before the final author-spacing and shutdown fixes.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

UI/catalog lanes passed; existing cold_search_worker_polls_failed_storage_and_stops_with_foreground_server exited with SIGTERM before handler installation. Source repair and focused evidence recorded in E-M3-015.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

New Full app cold assertion used the Testing Library toHaveTextContent matcher in Playwright; corrected to toContainText. 19 unit tests and 34 interaction/axe stories passed; the cold script failed one test.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Regex text assertion compared raw DOM whitespace; the rendered author spacing was correct. Switched to Playwright string matching with its standard whitespace normalization, retaining the required separator.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

Concurrent final validation had already loaded that same raw-whitespace assertion and stopped at catalog-cold-browser.
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
node --test scripts/catalog-browser.test.mjs
```

**Notes**

Cold static catalog acceptance passes, including the dedicated Full app initial view and author-spacing assertion; screenshots captured, with no unexpected requests or runtime errors.
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

Final source checks pass: all canonical/boundary/progress/contracts checks, rustfmt, Svelte checks with 0 errors/warnings and workspace Clippy.
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

Final 18 lanes pass: 19 catalog unit tests, 34 interaction/axe stories, cold static catalog, 8 existing browser cases, 369 Rust tests (6 external-service/credential tests ignored), rustfmt, contracts, governance, boundaries and mdBook.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Final aggregate passes: governance, storage harness self-tests, contracts, browser unit tests, 19 catalog unit tests, 34 stories, cold catalog and 369 Rust tests; 6 external-service/credential tests ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Static Storybook, existing browser bundle/manifest and all locked Rust workspace binaries build successfully.
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

<!-- evidence-field:remaining list -->
## Remaining

[]
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Community d859f33a484a9238bb11f974f4ead8e9be729ec6; Storybook 10.5.7; Svelte CSF 5.1.2; Vitest 4.1.11; Design Core 0.2.0 via ^0.2.0; Tailwind and Tailwind Vite 4.3.3 via ^4.3.3.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

Presentation-only contracts; native/server/API formats unchanged. Enclosing Jujutsu change is the evidence reference.
<!-- /evidence-field -->

