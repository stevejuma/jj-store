---
schema: 1
id: E-UI-007
---

# E-UI-007

<!-- evidence-field:id markdown -->
## Id

E-UI-007
<!-- /evidence-field -->

<!-- evidence-field:tasks list -->
## Tasks

- M4
- M4.3
- M4.9
<!-- /evidence-field -->

<!-- evidence-field:date markdown -->
## Date

2026-09-27
<!-- /evidence-field -->

<!-- evidence-field:kind markdown -->
## Kind

architecture
<!-- /evidence-field -->

<!-- evidence-field:files list -->
## Files

- AGENTS.md
- package.json
- scripts/web-architecture.mjs
- scripts/web-architecture-baseline.json
- scripts/web-architecture.test.mjs
- spec/src/architecture.md
- spec/src/spec-governance.md
- spec/src/ui.md
- spec/src/verification.md
- spec/src/milestones.md
- docs/implementation-plan.md
<!-- /evidence-field -->

<!-- evidence-field:tests list -->
## Tests

- scripts/web-architecture.test.mjs#rejects a new oversized production module
- scripts/web-architecture.test.mjs#rejects production imports from catalog and
  fixtures
- scripts/web-architecture.test.mjs#allows stories to consume production
  presentation modules
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
node --test scripts/web-architecture.test.mjs && node scripts/web-architecture.mjs
```

**Notes**

The first focused run proved counted-line enforcement but exposed that the import audit did not recognize side-effect imports.
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
node --test scripts/web-architecture.test.mjs && node scripts/web-architecture.mjs
```

**Notes**

Four focused architecture tests and the 104-module repository scan pass with Community-derived production/story/test limits, exact hotspot no-growth budgets and production isolation from catalog fixtures.
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
pnpm test:governance
```

**Notes**

All 21 governance and architecture tests pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm check
```

**Notes**

Type checking, architecture enforcement, contract drift, progress and source audits pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm test
```

**Notes**

Governance, RustFS harness, generated contracts, 8 web tests, 20 catalog unit tests, 37 Storybook interaction/accessibility tests, cold catalog browser acceptance and the full Rust workspace pass; three OS credential tests remain intentionally ignored.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm build
```

**Notes**

Cold Storybook, production web and locked Rust workspace builds pass. Existing Lightning CSS :global(svg) and large Storybook chunk warnings remain non-fatal.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm format:check
```

**Notes**

rustfmt and Prettier checks pass.
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
DEVELOPER_DIR=/Library/Developer/CommandLineTools pnpm spec:check
```

**Notes**

All 18 canonical validation lanes pass, including the cold catalog browser, Storybook interactions/accessibility, full Rust workspace and mdBook.
<!-- /run -->
<!-- /evidence-field -->

<!-- evidence-field:result markdown -->
## Result

pending
<!-- /evidence-field -->

<!-- evidence-field:remaining list -->
## Remaining

- Promote the reviewed presentation modules into production ownership and
  introduce the shared asynchronous controller contract.
<!-- /evidence-field -->

<!-- evidence-field:versions markdown -->
## Versions

Community architecture metric and limits reviewed at d859f33a484a9238bb11f974f4ead8e9be729ec6; jj-store parent e187e24b1b48db7ee8486847596056b8c84183b9.
<!-- /evidence-field -->

<!-- evidence-field:formats markdown -->
## Formats

No storage, native identity, HTTP wire, credential or browser persistence format changes. New architecture checks are wired into normal validation; the enclosing Jujutsu change is the version-control evidence reference.
<!-- /evidence-field -->
<!-- generated-navigation -->
## Pages


